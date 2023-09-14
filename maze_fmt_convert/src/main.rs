// simple 2D maze format converter, it reads the output of the maze program and changes TILE-based
// walls to LINE-based walls

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();

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

        // convert walls from "between tiles" to "line" format
        if x1 == x2 && y1 == y2 + 1 {
            println!("wall {} {} {} {}", x1, y1, x1 + 1, y1);
        } else if x1 == x2 && y2 == y1 + 1 {
            println!("wall {} {} {} {}", x1, y2, x1 + 1, y2);
        } else if y1 == y2 && x1 == x2 + 1 {
            println!("wall {} {} {} {}", x1, y1, x1, y1 + 1);
        } else if y1 == y2 && x2 == x1 + 1 {
            println!("wall {} {} {} {}", x2, y1, x2, y1 + 1);
        } else {
            return Err(Box::new(Error::new("bad input, non-adjacent cells")));
        }

        inputstr.clear();
    }

    Ok(())
}
