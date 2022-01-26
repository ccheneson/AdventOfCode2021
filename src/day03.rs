use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day03.txt");

    part01::run(file)?;
    part02::run(file)?;    
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

    pub fn run(file: &str) -> Result<isize> {
        let lines = file.lines();
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

        Ok(gama_dec * epsilon_dec)
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


    fn process<'a>(base: Vec<Vec<&'a str>>, index: usize, fnc : &dyn Fn(Zeros, Ones) -> &'static str) -> Vec<Vec<&'a str>> {
        if base.len() == 1 {
            base
        } else {
            let transpose = transpose(&base);
            let flip_row = &transpose[index];
            let rating = calculate_rating(flip_row, fnc);
            let base : Vec<Vec<&str>> = base.into_iter().filter(|e|e[index] == rating).map(|e| e).collect();
            process(base, index + 1, fnc)
        }
    }

    fn transpose<'a>(base : &'a Vec<Vec<&str>>) -> Vec<Vec<&'a str>> {
        let mut transpose: Vec<Vec<&str>> = vec!();

        let count_rows = base.len();
        let count_cols = base[count_rows - 1].len();

        for j in 0..count_cols { 
            let mut row = vec!();
            for i in (0..count_rows).rev() { 
                row.push(base[i][j]);
            }
            transpose.push(row);
        }

        transpose
    }

    pub fn run(file: &str) -> Result<isize> {
        let lines = file.lines();
        let lines :Vec<Vec<&str>> = lines.map(tokenize).collect();   
        
        let oxygen_generator = |z : Zeros, o: Ones| if o.0 >= z.0 { "1" } else { "0" };
        let co2_scrubber = |z : Zeros, o: Ones| if o.0 >= z.0 { "0" } else { "1" };

        let base_ogr = process(lines.clone(),0, &oxygen_generator);
        let base_csr = process(lines.clone(),0, &co2_scrubber);


        // let mut base_test: Vec<Vec<&str>> = lines.clone();


        // process(base_test,0, &oxygen_generator);

        // for i in 0..count_row {
        //     if base_ogr.len() != 1 {
        //         let bits_ogr : Vec<&str> = base_ogr.iter().map(|line| line[i]).collect();    
        //         let rating_ogr = calculate_rating(&bits_ogr, oxygen_generator);
        //         base_ogr = base_ogr.into_iter().filter(|e|e[i] == rating_ogr).collect();
        //     }

        //     if base_csr.len() != 1 {
        //         let bits_csr : Vec<&str> = base_csr.iter().map(|line| line[i]).collect();            
        //         let rating_csr = calculate_rating(&bits_csr, co2_scrubber);
        //         base_csr = base_csr.into_iter().filter(|e|e[i] == rating_csr).collect();
        //     }

        // }

        let rating = binary_rep_to_dec(base_ogr[0].join("").as_str()) * binary_rep_to_dec(base_csr[0].join("").as_str());

        println!("What is the life support rating of the submarine? {}", rating);

        Ok(rating)
    }

    
}


#[test]
fn test() {
   let input = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

   assert_eq!(part01::run(input).unwrap(), 198);
   assert_eq!(part02::run(input).unwrap(), 230);
   
}
