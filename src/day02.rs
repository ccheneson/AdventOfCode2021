use anyhow::Result;


pub fn run() -> Result<()> {
    let lines = include_str!("../input/day02.txt");


    part01::run(lines)?;
    part02::run(lines)?;
    part02::run_bis(lines)?;

    Ok(())
}


enum Commands {
    FORWARD(u32),
    DOWN(u32),
    UP(u32),
}

fn line_parser(line: &str) -> Result<Commands> {
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    if let (Some(command), Some(value)) = (tokens.get(0), tokens.get(1)) {
        let value = value.parse::<u32>()?;
        match *command {
            "forward" => Ok(Commands::FORWARD(value)),
            "down" => Ok(Commands::DOWN(value)),
            "up" => Ok(Commands::UP(value)),
            _ => Err(anyhow::anyhow!("Could not parse line '{}'", line)),
        }
    } else {
        Err(anyhow::anyhow!("Invalid number of token '{}'", line))
    }
}


mod part01 {
    use anyhow::Result;
    use crate::day02::line_parser;
    use super::Commands;

    #[derive(Debug)]
    struct Tracker {
        position: u32,
        depth: u32,
    }


    impl Tracker {
        fn new() -> Self {
            Self {
                position: 0,
                depth: 0,
            }
        }

        fn command(&mut self, cmd: Commands) {
            match cmd {
                Commands::FORWARD(val) => self.position += val,
                Commands::DOWN(val) => self.depth += val,
                Commands::UP(val) => self.depth -= val,
            }
        }

        fn result(&self) -> u32 {
            self.depth as u32 * self.position as u32
        }
    }

    pub fn run(file: &str) -> Result<u32> {
        let lines = file.lines();

        let tracker =
            lines
                .map(|line| line_parser(line))
                .fold(Tracker::new(), |mut acc, element| {
                    match element {
                        Ok(cmd) => acc.command(cmd),
                        Err(err) => panic!("{}", err),
                    };
                    acc
                });

        println!("What do you get if you multiply your final horizontal position by your final depth? {:?}", tracker.result());

        Ok(tracker.result())
    }
}


mod part02 {
    use anyhow::Result;
    use crate::day02::line_parser;
    use super::Commands;

    #[derive(Debug)]
    struct Tracker {
        position: u32,
        depth: u32,
        aim: u32
    }

    impl Tracker {
        fn new() -> Self {
            Self {
                position: 0,
                depth: 0,
                aim: 0
            }
        }

        fn command(&mut self, cmd: Commands) {
            match cmd {
                Commands::FORWARD(val) => {
                    self.depth += self.aim * val;
                    self.position += val
                },
                Commands::DOWN(val) => {
                    self.aim += val;
                },
                Commands::UP(val) => {
                    self.aim -= val;
                }
            }
        }

        fn result(&self) -> u32 {
            self.depth as u32 * self.position as u32
        }
    }

    pub fn run(file: &str) -> Result<u32> {
        let lines = file.lines();

        let tracker =
            lines
                .map(|line| line_parser(line))
                .fold(Tracker::new(), |mut acc, element| {
                    match element {
                        Ok(cmd) => acc.command(cmd),
                        Err(err) => panic!("{}", err),
                    };
                    acc
                });

        println!("What do you get if you multiply your final horizontal position by your final depth? {:?}", tracker.result());

        Ok(tracker.result())
    }


    // This is another implementation without any struct to manage the calculation
    // It also demonstrates the pattern matching with slice
    pub fn run_bis(file: &str) -> Result<u32> {

        fn parse(line: &str) -> Vec<&str> {
            line.split_whitespace().collect()
        }


        let lines = file.lines();

        //accumulator is tuple3( position, depth, aim)
        let tracker =
            lines
                .fold((0,0,0), | acc, element| {
                    match parse(element).as_slice() {                        
                        &["down", value] => (acc.0, acc.1, acc.2 + value.parse::<i32>().unwrap()),
                        &["up", value] => (acc.0, acc.1, acc.2 - value.parse::<i32>().unwrap()),
                        &["forward", value] => {
                            let value_i32 = value.parse::<i32>().unwrap();
                            (acc.0 + value_i32, acc.1 + acc.2 * value_i32, acc.2)
                        },
                        _ => panic!("Unknown command {}", element),
                    }
                });

        println!("What do you get if you multiply your final horizontal position by your final depth - part2 bis ? {:?}", tracker.0 * tracker.1);

        Ok((tracker.0 * tracker.1) as u32)
    }
}


#[test]
fn test() {
   let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2
"#;

   assert_eq!(part01::run(input).unwrap(), 150);
   assert_eq!(part02::run(input).unwrap(), 900);
   assert_eq!(part02::run_bis(input).unwrap(), 900);
   
}
