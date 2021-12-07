use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day07.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}


#[derive(Debug, PartialEq, Eq)]
struct Position(u16);
impl Position {
    fn position(&self) -> u16 { self.0 }
}

mod part01 {
    use anyhow::Result;
    use itertools::Itertools;
    use super::Position;

    
    pub fn run(file: &str) -> Result<i32> {        
        let positions : Vec<Position>   = file.lines()
                .flat_map(|line| line.split(",").collect::<Vec<&str>>())
                .map(|e| e.trim().parse::<u16>().unwrap())
                .map(|e| Position(e))
                .collect();
                

        let mut consummed_fuels: Vec<i32> = vec!();

        for target_position in positions.iter().dedup() {
            let mut moves = 0;
            positions.iter().for_each(|position| {
                moves += (target_position.position() as i32 - position.position() as i32).abs()
            });
            consummed_fuels.push(moves);
        }

        let min = consummed_fuels.into_iter().min().unwrap();
        println!("How much fuel must they spend to align to that position? part 1 {}", min);
        
        Ok(min)
    }
}

mod part02 {
    use anyhow::Result;
    use super::Position;

    
    pub fn run(file: &str) -> Result<i32> {        
        let positions : Vec<Position>   = file.lines()
                .flat_map(|line| line.split(",").collect::<Vec<&str>>())
                .map(|e| e.trim().parse::<u16>().unwrap())
                .map(|e| Position(e))
                .collect();
                

        let mut consummed_fuels: Vec<i32> = vec!();

        let max_position = positions.iter().max_by_key(|p| p.position()).unwrap().position();

        for target_position in 0..=max_position {
            let mut moves = 0;
            positions
                .iter()
                .for_each(|position| {
                    let count_moves = (target_position as i32 - position.position() as i32).abs();                
                    moves += (0..=count_moves).sum::<i32>();
                }
            );
            consummed_fuels.push(moves);
        }

        let min = consummed_fuels.into_iter().min().unwrap();
        println!("How much fuel must they spend to align to that position? part 2 (slow from build, but fast with --release) {}", min);
        
        Ok(min)
    }
}


#[test]
fn test() {
   let input = "16,1,2,0,4,2,7,1,2,14";

   assert_eq!(part01::run(input).unwrap(), 37);
   assert_eq!(part02::run(input).unwrap(), 168);
   
}
