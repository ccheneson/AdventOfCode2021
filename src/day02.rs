use anyhow::Result;


pub fn run() -> Result<()> {
    part01::run()?;
    part02::run()?;
    Ok(())
}

mod part01 {
    use anyhow::Result;

    #[derive(Debug)]
    struct Tracker {
        position: u32,
        depth: u32,
    }

    enum Commands {
        FORWARD(u32),
        DOWN(u32),
        UP(u32),
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

    pub fn run() -> Result<()> {
        let lines = include_str!("../input/day02/input.txt").lines();

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

        Ok(())
    }
}


mod part02 {
    use anyhow::Result;

    #[derive(Debug)]
    struct Tracker {
        position: u32,
        depth: u32,
        aim: u32
    }

    enum Commands {
        FORWARD(u32),
        DOWN(u32),
        UP(u32),        
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

    pub fn run() -> Result<()> {
        let lines = include_str!("../input/day02/input.txt").lines();

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

        Ok(())
    }
}
