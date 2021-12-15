use anyhow::Result;


pub fn run() -> Result<()> {
    let file = include_str!("../input/day14.txt");

    part01::run(file)?;
    //part02::run(file)?;

    Ok(())
}


mod common {


    pub fn parse(file: &str) -> (Vec<&str>, Vec<((&str,&str), &str)>) {
        let mut poly_template : Vec<&str> = vec!();
        let mut rules : Vec<((&str,&str), &str)> = vec!();
        file.lines().into_iter().for_each(|e| {
            if e.contains("->") {
                let tokens = e.split("->")
                                        .map(|e| e.trim()).collect::<Vec<&str>>();
                let c : Vec<&str> = tokens[0].split("").filter(|e| !e.is_empty()).collect();
                rules.push(((c[0], c[1]), tokens[1]));
            }
            else if e.is_empty() {
            } else {
                poly_template = e.split("").filter(|e|!e.is_empty()).collect();
            }
        });
        (poly_template, rules)
    }
    

}

mod part01 {
    
    use std::collections::{HashMap, BTreeMap};

    use anyhow::Result;

    use super::common::parse;
   
    pub fn run(file: &str) -> Result<usize> {
    
        let (mut template, rules) = parse(file);

        let m : HashMap<(&str, &str), &str> = rules.into_iter().collect();


        for _ in 0..10 {
            let mut result_template = vec!();

            //Could easily be a .fold with the previous template character carried forward
            for (i, [a, b]) in template.array_windows().enumerate() {
                let value = m.get(&(a,b));
                if let Some(v) = value {
                    result_template.push(*a);
                    result_template.push(*v);
                }
                //because we treat 2 elements at a time (len - 1) and to match index (- 1)
                if (template.len() - 2) == i {
                    result_template.push(*b);
                }
            }
            template = result_template;
        }

        //https://stackoverflow.com/questions/54936304/finding-most-frequently-occurring-string-in-a-structure-in-rust
        //Count number of occurence of each elemnts
        let mut counts = BTreeMap::new();
        for word in template.iter() {
            *counts.entry(word).or_insert(0) += 1;
        }

        let max = counts.iter().max_by_key(|&(_, count)| count).map(|e| e.1).unwrap();
        let min = counts.iter().min_by_key(|&(_, count)| count).map(|e| e.1).unwrap();

        let result = max - min;

        println!("What do you get if you take the quantity of the most common element and subtract the quantity of the least common element? {}", result);

        Ok(result)
    }

    #[test]
    fn test() {
        let file ="NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    assert_eq!(run(file).unwrap(), 1588);

    }
}

