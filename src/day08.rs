use anyhow::Result;

#[derive(Debug, PartialEq, Clone)]
struct SignalPattern(String);
impl SignalPattern {
    fn pattern(&self) -> String {
        self.0.to_string()
    }
    fn length(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
struct Digit(String);
impl Digit {
    fn segments(&self) -> String {
        self.0.to_string()
    }
}

pub fn run() -> Result<()> {
    let file = include_str!("../input/day08.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}

mod part01 {
    use super::{Digit, SignalPattern};
    use anyhow::Result;

    pub fn run(file: &str) -> Result<usize> {
        let inputs: Vec<(Vec<SignalPattern>, Vec<Digit>)> = file
            .lines()
            .map(|e| {
                let split: Vec<&str> = e.split("|").collect();
                let patterns: Vec<SignalPattern> = split[0]
                    .trim()
                    .split_whitespace()
                    .map(|signal| SignalPattern(signal.to_string()))
                    .collect();
                let segments: Vec<Digit> = split[1]
                    .trim()
                    .split_whitespace()
                    .map(|signal| Digit(signal.to_string()))
                    .collect();
                (patterns, segments)
            })
            .collect();

        //Nbr of segment with unique count for 1,4,7,8
        let check_length: Vec<usize> = vec![2, 3, 4, 7];

        let digits: Vec<Digit> = inputs
            .into_iter()
            .flat_map(|e| e.1)
            .filter(|digit| check_length.contains(&digit.segments().len()))
            .collect();

        let result = digits.iter().count();

        println!("how many times do digits 1, 4, 7, or 8 appear ? {}", result);

        Ok(result)
    }
}

mod part02 {
    use std::collections::HashMap;
    use anyhow::Result;
    use itertools::Itertools;

    use super::{Digit, SignalPattern};

    pub fn run(file: &str) -> Result<u32> {
        let inputs: Vec<(Vec<SignalPattern>, Vec<Digit>)> = file
            .lines()
            .map(|e| {
                let split: Vec<&str> = e.split("|").collect();
                let patterns: Vec<SignalPattern> = split[0]
                    .trim()
                    .split_whitespace()
                    .map(|signal| {
                        let sorted = signal.chars();
                        let mut s = sorted.sorted();
                        SignalPattern(s.join("").to_string())
                    })
                    .collect();
                let segments: Vec<Digit> = split[1]
                    .trim()
                    .split_whitespace()
                    .map(|signal| Digit(signal.to_string()))
                    .collect();
                (patterns, segments)
            })
            .collect();

        let mut final_result : u32 = 0;

        for (mut patterns, digits) in inputs.into_iter() {
            
            let mut base : Vec<(u8, String)> = vec!();
            let mut index_to_remove: Vec<usize> = vec!();

            for (index, p) in patterns.iter().enumerate() {
                match p.length() {
                    2 => base.push((1, p.pattern())),
                    3 => base.push((7, p.pattern())),
                    4 => base.push((4, p.pattern())),
                    7 => base.push((8, p.pattern())),
                    _ => continue
                }
                index_to_remove.push(index);
            }
            patterns = patterns.into_iter().enumerate().filter(|e| ! index_to_remove.contains(&e.0) ).map(|e| e.1).collect();

            let mut base_map : HashMap<u8, String> = base.into_iter().collect();

            //--------------------------------------------------
            //Check 3
            let length_05 : Vec<SignalPattern> = find_pattern_with_length(&patterns, 5);
            let pattern_07 = base_map.get(&7).unwrap().to_string();
            let result : Vec<SignalPattern> = 
                length_05.into_iter()
                    .filter(|sp| 
                        contain(sp.pattern().as_str(), pattern_07.as_str())
                    ).collect();
            let match_03 = &result[0];
            patterns.retain(|x| ! (x == match_03));
            base_map.insert(3,  match_03.pattern());

            //Check 9
            let length_06 : Vec<SignalPattern> = find_pattern_with_length(&patterns, 6);
            let pattern_03 = base_map.get(&3).unwrap().to_string();
            let pattern_04 = base_map.get(&4).unwrap().to_string();
            let pattern_09 = concat(pattern_03.as_str(), pattern_04.as_str());
            let result : Vec<SignalPattern> = 
                length_06.into_iter()
                    .filter(|sp| contain(sp.pattern().as_str(), pattern_09.as_str()))
                    .collect();
            let match_09 = &result[0];
            patterns.retain(|x| ! (x == match_09));
            base_map.insert(9,  match_09.pattern());

            //Check 6        
            let length_06 : Vec<SignalPattern> = find_pattern_with_length(&patterns, 6);        
            let pattern_08 = base_map.get(&8).unwrap().to_string();
            let pattern_01 = base_map.get(&1).unwrap().to_string();
            let pattern_01_tokens : Vec<&str> = pattern_01.split("").filter(|e|!e.is_empty()).collect();
            let pattern_06_a = sort(diff(pattern_08.as_str(), pattern_01_tokens[0]).as_str());
            let pattern_06_b = sort(diff(pattern_08.as_str(), pattern_01_tokens[1]).as_str());


            let check_pattern_06 = vec!(pattern_06_a, pattern_06_b);


            let result : Vec<SignalPattern> = 
                length_06.into_iter()
                    .filter(|sp| check_pattern_06.contains(&sp.pattern()))
                    .collect();
            let match_00 = &result[0];
            patterns.retain(|x| ! (x == match_00));
            base_map.insert(6,  match_00.pattern());

            //Check 0 
            //The remaining pattern must be 0
            let length_06 = find_pattern_with_length(&patterns, 6); 
            let match_06 = &length_06[0];
            patterns.retain(|x| ! (x == match_06));
            base_map.insert(0,  match_06.pattern());


            //Check 5
            let length_05 = find_pattern_with_length(&patterns, 5); 
            let pattern_09 = base_map.get(&9).unwrap().to_string();
            let result : Vec<SignalPattern> = 
                length_05.into_iter()
                    .filter(|sp| contain(pattern_09.as_str(), sp.pattern().as_str()))
                    .collect();
            let match_05 = &result[0];
            patterns.retain(|x| ! (x == match_05));
            base_map.insert(5,  match_05.pattern());


            //Check 2
            //The remaining pattern must be 0
            let match_02 = &patterns[0];
            base_map.insert(2,  match_02.pattern());


            let mut reverse_map : HashMap<String, u8> = HashMap::new();
            base_map.into_iter().for_each(|(num, pattern)| { reverse_map.insert(sort(pattern.as_str()), num); });


            let output = 
                digits.into_iter()
                        .map(|dig| reverse_map[sort(dig.segments().as_str()).as_str()].to_string())
                        .join("")
                        .parse::<u32>()?;
            
            final_result += output;            
        }

        println!("What do you get if you add up all of the output values {:?}", final_result);

        Ok(final_result)
    }

    pub fn diff<'a>(v1: &'a str, v2: &'a str) -> String {
        v1.chars().filter(|c| ! v2.chars().contains(c)).join("")
    }

    pub fn concat<'a>(v1: &'a str, v2: &'a str) -> String {
        let v1_len = v1.len();
        let v2_len = v2.len();
        let (from, to) = if v1_len > v2_len { (v1, v2) } else { (v2, v1) };
        let mut v1s = from.to_string();
        v1s.push_str(to);
        v1s.chars().sorted().join("")
    }

    pub fn contain<'a>(v1: &'a str, v2: &'a str) -> bool {
        let v1_len = v1.len();
        let v2_len = v2.len();
        let (from, to) = if v1_len > v2_len { (v2, v1) } else { (v1, v2) };
        from.chars().all(|e| to.chars().contains(&e))
    }

    pub fn sort(v: &str) -> String {
        v.chars().sorted().join("")
    }

    fn find_pattern_with_length(line01: &Vec<SignalPattern>, length: usize) -> Vec<SignalPattern>{
        line01.iter().filter(|&sp| sp.length() == length).map(|e| e.clone()).collect()
    }
  
}

#[test]
fn test() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    assert_eq!(part02::diff("abcdef", "bcd"), "aef".to_string());
    assert_eq!(part02::concat("abc", "def"), "abcdef".to_string());
    assert_eq!(part02::contain("abced", "cd"), true);
    assert_eq!(part02::contain("bcdef", "bde"), true);
    assert_eq!(part01::run(input).unwrap(), 26);
    assert_eq!(part02::run(input).unwrap(), 61229);
}
