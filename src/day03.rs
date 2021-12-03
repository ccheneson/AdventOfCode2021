use anyhow::Result;

pub fn run() -> Result<()> {
    part01::run()?;
    //part02::run()?;    
    Ok(())
}


mod part01 {
    use anyhow::Result;

    fn tokenize(line: &str) -> Vec<&str>{
        line.split("").filter(|&e| e != "").collect()
    }

    fn get_gama_bit(row: &Vec<&str>) -> &'static str {
        let (zeros, ones) = 
            row.iter().fold((0,0), |mut acc, &elem| {
                if elem == "0" {
                    acc.0 += 1;
                } else {
                    acc.1 += 1;
                }
                acc
            });

        if zeros > ones { "0" } else { "1" }
    }

    fn binary_rep_to_dec(input: &[&str]) -> isize {
        isize::from_str_radix(&input.join("").to_string(), 2).unwrap()
    }

    pub fn run() -> Result<()> {
        let lines = include_str!("../input/day03/input.txt").lines();
        let lines :Vec<Vec<&str>> = lines.map(tokenize).collect();
        let count_line = lines.len();
        let count_row = lines.get(0).unwrap().len();
        let mut switch_col_row : Vec<Vec<&str>>= vec![vec!["0";count_line]; count_row] ;
        
        for (i, line) in lines.into_iter().enumerate() {
            for (j, value) in line.into_iter().enumerate() {
                switch_col_row[j][i] = value;
            }
        }
        
        let gama : Vec<&str> = switch_col_row
                                .iter()
                                .map(|e| get_gama_bit(e)).collect();
        let epsilon : Vec<&str> = gama.iter().map(|&e| if e == "1" { "0" } else { "1" } ).collect();
        
        let gama_dec = binary_rep_to_dec(&gama);
        let epsilon_dec = binary_rep_to_dec(&epsilon);

        println!("What is the power consumption of the submarine? {}", gama_dec * epsilon_dec);

        Ok(())
    }

    
}