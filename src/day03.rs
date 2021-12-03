use anyhow::Result;

pub fn run() -> Result<()> {
    part01::run()?;
    part02::run()?;    
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


mod part02 {
    use anyhow::Result;

    fn tokenize(line: &str) -> Vec<&str>{
        line.split("").filter(|&e| e != "").collect()
    }

    struct Zeros(u16);
    struct Ones(u16);

    fn  calculate_rating(row: &Vec<&str>, fnct : impl Fn(Zeros, Ones) -> &'static str) -> &'static str {
        let (zeros, ones) = 
            row.iter().fold((0,0), |mut acc, &elem| {
                if elem == "0" {
                    acc.0 += 1;
                } else {
                    acc.1 += 1;
                }
                acc
            });

        fnct(Zeros(zeros), Ones(ones))
    }

    fn binary_rep_to_dec(input: &str) -> isize {        
        isize::from_str_radix(&input.to_string(), 2).unwrap()
    }

    pub fn run() -> Result<()> {
        let lines = include_str!("../input/day03/input.txt");
        let lines :Vec<Vec<&str>> = lines.lines().map(tokenize).collect();
        let count_line = lines.len();
        let count_row = lines.get(0).unwrap().len();
        let mut switch_col_row : Vec<Vec<&str>>= vec![vec!["0";count_line]; count_row] ;
        
        for (i, line) in lines.iter().enumerate() {
            for (j, value) in line.into_iter().enumerate() {
                switch_col_row[j][i] = value;
            }
        }

        let oxygen_generator = |z : Zeros, o: Ones| if o.0 >= z.0 { "1" } else { "0" };
        let co2_scrubber = |z : Zeros, o: Ones| if o.0 >= z.0 { "0" } else { "1" };


        let mut base_ogr: Vec<Vec<&str>> = lines.clone();
        let mut base_csr: Vec<Vec<&str>> = lines.clone();

        for i in 0..count_row {
            if base_ogr.len() != 1 {
                let bits_ogr : Vec<&str> = base_ogr.iter().map(|line| line[i]).collect();    
                let rating_ogr = calculate_rating(&bits_ogr, oxygen_generator);
                base_ogr = base_ogr.into_iter().filter(|e|e[i] == rating_ogr).collect();
            }

            if base_csr.len() != 1 {
                let bits_csr : Vec<&str> = base_csr.iter().map(|line| line[i]).collect();            
                let rating_csr = calculate_rating(&bits_csr, co2_scrubber);
                base_csr = base_csr.into_iter().filter(|e|e[i] == rating_csr).collect();
            }

        }

        println!(
            "What is the life support rating of the submarine? {}", 
            binary_rep_to_dec(base_ogr[0].join("").as_str()) * binary_rep_to_dec(base_csr[0].join("").as_str())
        );

        Ok(())
    }

    
}
