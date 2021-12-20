use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day01.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}

mod part01 {
    use anyhow::Result;

    pub fn run(file: &str) -> Result<usize> {
        let larger_count = 
            file.lines()
                .map(|e| e.parse::<u16>().unwrap())                
                .collect::<Vec<u16>>()
                .windows(2)
                .filter(| window | window[0] < window[1])
                .count();

        println!(
            "How many measurements are larger than the previous measurement - part1 ? {}",
            larger_count
        );

        Ok(larger_count)
    }
}

mod part02 {
    use anyhow::Result; 
    pub fn run(file: &str) -> Result<usize> {
        let file = file.lines();

        let larger_count = file
            .map(|line| line.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .array_windows()
            .map(|[a, b, c]| a + b + c)
            .collect::<Vec<u16>>()
            .array_windows()
            .filter(|[a, b]| a < b)
            .count();

        println!(
            "How many measurements are larger than the previous measurement - part2 ? {}",
            larger_count
        );

        Ok(larger_count)
    }
}

#[test]
fn test() {
   let input = r#"199
200
208
210
200
207
240
269
260
263"#;

   assert_eq!(part01::run(input).unwrap(), 7);
   assert_eq!(part02::run(input).unwrap(), 5);
   
}
