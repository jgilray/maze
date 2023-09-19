// Simple maze generator, outputs a list of internal walls
use clap::Parser;
use rand::prelude::*;

// custom error type
#[derive(Debug)]
struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Clone, Debug, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Square {
    top_open: bool,
    left_open: bool,
    visited: bool,
    loc: Point,
}

impl Square {
    fn new(x: usize, y: usize) -> Self {
        Self {
            top_open: false,
            left_open: false,
            visited: false,
            loc: Point { x, y },
        }
    }
}

// Program arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of maze
    #[arg(long, default_value_t = 20)]
    width: usize,

    /// Height of maze
    #[arg(long, default_value_t = 10)]
    height: usize,

    /// Approximate percentage of walls to remove,
    /// resulting in a maze which can be solved in more than one way
    #[arg(long, default_value_t = 0.0)]
    remove_percentage: f64,

    /// number of randomly placed rooms
    #[arg(long, default_value_t = 0)]
    num_rooms: usize,

    /// Size of optional randomly placed rooms
    #[arg(long, default_value_t = 0)]
    room_size: usize,

    /// Option to support harp lab's maze game,
    /// Overrides all other options
    #[clap(long, action)]
    harp: bool,
}

struct Maze {
    cells: Vec<Vec<Square>>,
    rng: ThreadRng,
}

impl Maze {
    fn new(x: usize, y: usize) -> Self {
        let mut s = Self {
            cells: vec![vec![Square::new(0, 0); y]; x],
            rng: thread_rng(),
        };

        for xx in 0..x {
            for yy in 0..y {
                s.cells[xx][yy].loc = Point { x: xx, y: yy };
            }
        }

        s
    }

    fn recursive_build(&mut self, current: Point) {
        self.cells[current.x][current.y].visited = true;
        while let Some(chosen) = self.find_rand_unvisited_neighbor(current) {
            self.remove_wall(current, chosen);
            self.recursive_build(chosen);
        }
    }

    fn find_rand_unvisited_neighbor(&mut self, loc: Point) -> Option<Point> {
        let mut neighbor: Point;
        let r = self.rng.gen_range(0..4);
        let mut n = r;
        loop {
            neighbor = get_neighbor(loc, n);
            if neighbor.x < self.cells.len()
                && neighbor.y < self.cells[neighbor.x].len()
                && !self.cells[neighbor.x][neighbor.y].visited
            {
                return Some(self.cells[neighbor.x][neighbor.y].loc);
            }

            n = if n == 3 { 0 } else { n + 1 };
            if n == r {
                break;
            }
        }
        None
    }

    fn remove_wall(&mut self, a: Point, b: Point) {
        if a.x != b.x {
            if a.x == b.x + 1 {
                self.cells[a.x][a.y].left_open = true;
            } else if a.x == b.x - 1 {
                self.cells[b.x][b.y].left_open = true;
            }
        } else if a.y == b.y + 1 {
            self.cells[a.x][a.y].top_open = true;
        } else if a.y == b.y - 1 {
            self.cells[b.x][b.y].top_open = true;
        }
    }

    fn remove_all_walls_from_cell(&mut self, rc: Point) {
        self.cells[rc.x][rc.y].top_open = true;
        self.cells[rc.x][rc.y].left_open = true;
    }
}

fn get_neighbor(candidate: Point, n: usize) -> Point {
    let mut neighbor = candidate;
    match n {
        0 => {
            neighbor.x = if candidate.x > 0 {
                candidate.x - 1
            } else {
                candidate.x
            }
        }
        1 => {
            neighbor.y = if candidate.y > 0 {
                candidate.y - 1
            } else {
                candidate.y
            }
        }
        2 => neighbor.x = candidate.x + 1,
        3 => neighbor.y = candidate.y + 1,
        _ => unreachable!(),
    }
    neighbor
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get arguments to the prograam
    let args = Args::parse();
    let width;
    let height;
    let num_rooms;
    let room_size;
    let remove_percentage;
    if args.harp {
        // harp lab option is set, overrides all settings
        width = 11;
        height = 11;
        num_rooms = (width + height) / 4;
        room_size = 2;
        remove_percentage = 8.0;
    } else {
        width = args.width;
        height = args.height;
        num_rooms = args.num_rooms;
        room_size = args.room_size;
        remove_percentage = args.remove_percentage;
    }

    if room_size > height - 1 || room_size > width - 1 {
        return Err(Box::new(Error::new("room size too large for maze")));
    }

    // initialize maze with all walls in place
    let mut m = Maze::new(width, height);

    // remove random walls starting from a random location
    let start = Point {
        x: m.rng.gen_range(0..width),
        y: m.rng.gen_range(0..height),
    };
    m.recursive_build(start);

    // optionally create rooms
    for _ in 0..num_rooms {
        let xs = m.rng.gen_range(1..(width - room_size + 1));
        let ys = m.rng.gen_range(1..(height - room_size + 1));

        for x in xs..(xs + room_size) {
            for y in ys..(ys + room_size) {
                let rc = Point { x, y };
                m.remove_all_walls_from_cell(rc);
            }
        }
    }

    // print out walls still remaining - optionally skipping some walls
    let mut range = 0.9 * width as f64 * height as f64;
    range -= (room_size * room_size * num_rooms) as f64;
    let limit = (range * remove_percentage / 100.0) as usize;
    dbg!(range, limit, num_rooms, room_size);
    for i in 0..m.cells.len() {
        for j in 0..m.cells[i].len() {
            let rndnum = m.rng.gen_range(0..range as usize);
            if rndnum >= limit {
                if !m.cells[i][j].top_open && j > 0 {
                    println!("wall-tile {} {} {} {}", i, j - 1, i, j);
                }
                if !m.cells[i][j].left_open && i > 0 {
                    println!("wall-tile {} {} {} {}", i - 1, j, i, j);
                }
            }
        }
    }

    Ok(())
}
