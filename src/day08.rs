use std::fmt::Display;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct SignalPattern(String);
impl SignalPattern {
    fn pattern(&self) -> String { self.0.to_string() }
    fn length(&self) -> usize { self.0.len() }
    fn contains(&self, check: &str) -> bool { 
        check.chars().all(|c|self.0.contains(c))
    }
    fn diff(&mut self, diff: &str) -> String { 
        if self.contains(diff) {
            let mut d = diff.chars();
            self.pattern().chars().filter(|c| !d.contains(c)).join("")
        } else {
            self.pattern()
        }
    }
}

#[derive(Debug)]
struct Digit(String);
impl Digit {
    fn segments(&self) -> String { self.0.to_string() }
}



pub fn run() -> Result<()> {
    let file = include_str!("../input/day08.txt");

    part01::run(file)?;
    //part02::run(file)?;

    Ok(())
}



mod part01 {
    use anyhow::Result;
    use super::{SignalPattern, Digit};
   
    
    pub fn run(file: &str) -> Result<usize> {
  
        let inputs : Vec<(Vec<SignalPattern>, Vec<Digit>)>= 
            file.lines()
                .map(|e| {
                    let split: Vec<&str> = e.split("|").collect();
                    let patterns: Vec<SignalPattern> = split[0].trim().split_whitespace().map(|signal| SignalPattern(signal.to_string())).collect();
                    let segments: Vec<Digit> = split[1].trim().split_whitespace().map(|signal| Digit(signal.to_string())).collect();
                    (patterns, segments)
                })
                .collect();

        //Nbr of segment with unique count for 1,4,7,8
        let check_length : Vec<usize> = vec!(2,3,4,7);

        let digits:Vec<Digit> = inputs.into_iter()
                            .flat_map(|e| e.1)
                            .filter(|digit| check_length.contains(&digit.segments().len()) )
                            .collect();

        let result = digits.iter().count();

        println!("how many times do digits 1, 4, 7, or 8 appear ? {}", result);
        
        Ok(result)
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
   let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

   assert_eq!(part01::run(input).unwrap(), 26);
   //assert_eq!(part02::run(input).unwrap(), 168);
   
}
