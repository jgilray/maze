// simple 2D maze visualizer, it reads the output of the maze program
use clap::Parser;

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

/// Program arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of maze
    #[arg(long, default_value_t = 20)]
    width: usize,

    /// Height of maze
    #[arg(long, default_value_t = 10)]
    height: usize,

    /// Format of maze
    #[clap(long, short, action)]
    line: bool,
}

///Displays a wall
fn paint_wall(is_horiz: bool, active: bool) {
    if is_horiz {
        print!("{}", if active { "+---" } else { "+   " });
    } else {
        print!("{}", if active { "|   " } else { "    " });
    }
}

///Displays a final wall for a row
fn paint_close_wall(is_horiz: bool) {
    if is_horiz {
        println!("+")
    } else {
        println!()
    }
}

// Displays a whole row of walls
fn paint_row(is_horiz: bool, idx: usize, walls: &Vec<Vec<bool>>) {
    for w in walls[idx].iter() {
        paint_wall(is_horiz, *w);
    }
    paint_close_wall(is_horiz);
}

/// Paints the maze
fn paint(height: usize, wv: &Vec<Vec<bool>>, wh: &Vec<Vec<bool>>) {
    for i in 0..height {
        paint_row(true, i, wh);
        paint_row(false, i, wv);
    }
    paint_row(true, height, wh);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let reader = std::io::stdin();
    let mut inputstr = String::new();
    let mut walls_h: Vec<Vec<bool>> = vec![vec![false; args.width]; args.height + 1];
    let mut walls_v: Vec<Vec<bool>> = vec![vec![false; args.width + 1]; args.height];

    // put in edge walls
    for idx in 0..args.width {
        walls_h[0][idx] = true;
        walls_h[args.height][idx] = true;
    }
    for idx in 0..args.height {
        walls_v[idx][0] = true;
        walls_v[idx][args.width] = true;
    }

    // read_line returns 0 on EOF
    while reader.read_line(&mut inputstr)? != 0 {
        let instr = inputstr.trim_end().to_string(); // remove CRs at the end each line
        let vw = instr.split(' ').collect::<Vec<&str>>();
        if vw[0] != "wall" || vw.len() != 5 {
            return Err(Box::new(Error::new(
                "bad input, expecting \"wall x1 y1 x2 y2\"",
            )));
        }
        let x1 = vw[1].parse::<usize>()?;
        let y1 = vw[2].parse::<usize>()?;
        let x2 = vw[3].parse::<usize>()?;
        let y2 = vw[4].parse::<usize>()?;

        // put in walls read from input
        if args.line && x1 == x2 && (y1 == y2 + 1 || y2 == y1 + 1) {
            walls_v[std::cmp::min(y1, y2)][x1] = true;
        } else if !args.line && x1 == x2 && (y1 == y2 + 1 || y2 == y1 + 1) {
            walls_h[std::cmp::max(y1, y2)][x1] = true;
        } else if args.line && y1 == y2 && (x1 == x2 + 1 || x2 == x1 + 1) {
            walls_h[y1][std::cmp::min(x1, x2)] = true;
        } else if !args.line && y1 == y2 && (x1 == x2 + 1 || x2 == x1 + 1) {
            walls_v[y1][std::cmp::max(x1, x2)] = true;
        } else {
            return Err(Box::new(Error::new("bad input, non-adjacent cells")));
        }

        inputstr.clear();
    }

    // display maze as ascii text
    paint(args.height, &walls_v, &walls_h);

    Ok(())
}
