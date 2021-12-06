use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day07.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}


mod part01 {

    use anyhow::Result;
    
    pub fn run(file: &str) -> Result<()> {        

        
        Ok(())
    }
}

mod part02 {
    use anyhow::Result;
    
    pub fn run(file: &str) -> Result<()> {        

        
        Ok(())
    }
}


#[test]
fn test() {
   //let input = "3,4,3,1,2";

   //assert_eq!(part01::run(input).unwrap(), 5934);
   //assert_eq!(part02::run(input).unwrap(), 26984457539);
   
}
