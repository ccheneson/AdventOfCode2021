use std::fmt::Display;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct SignalPattern(String);
impl SignalPattern {
    fn pattern(&self) -> String {
        self.0.to_string()
    }
    fn length(&self) -> usize {
        self.0.len()
    }
    fn contains(&self, check: &str) -> bool {
        check.chars().all(|c| self.0.contains(c))
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
    fn segments(&self) -> String {
        self.0.to_string()
    }
}

pub fn run() -> Result<()> {
    let file = include_str!("../input/day08.txt");

    //part01::run(file)?;
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
    use std::{
        borrow::{Borrow, BorrowMut},
        collections::HashMap,
        io::Sink,
        slice::SliceIndex,
    };

    use anyhow::Result;
    use itertools::Itertools;

    use super::{Digit, SignalPattern};

    pub fn run(file: &str) -> Result<u32> {
        // let inputs: Vec<(Vec<SignalPattern>, Vec<Digit>)> = file
        //     .lines()
        //     .map(|e| {
        //         let split: Vec<&str> = e.split("|").collect();
        //         let patterns: Vec<SignalPattern> = split[0]
        //             .trim()
        //             .split_whitespace()
        //             .map(|signal| {
        //                 let sorted = signal.chars();
        //                 let mut s = sorted.sorted();
        //                 SignalPattern(s.join("").to_string())
        //             })
        //             .collect();
        //         let segments: Vec<Digit> = split[1]
        //             .trim()
        //             .split_whitespace()
        //             .map(|signal| Digit(signal.to_string()))
        //             .collect();
        //         (patterns, segments)
        //     })
        //     .collect();

        // let mut signal_patterns = inputs.into_iter().map(|e| e.0);

        // for mut line01 in signal_patterns.next() {
        //     let mut base: Vec<(u8, String)> = vec![];
        //     let mut index_to_remove: Vec<usize> = vec![];

        //     for (index, p) in line01.iter().enumerate() {
        //         match p.length() {
        //             2 => base.push((1, p.pattern())),
        //             3 => base.push((7, p.pattern())),
        //             4 => base.push((4, p.pattern())),
        //             7 => base.push((8, p.pattern())),
        //             _ => continue,
        //         }
        //         index_to_remove.push(index);
        //     }

        //     //Input with the already defined 1,7,4,8 removed
        //     line01 = line01
        //         .into_iter()
        //         .enumerate()
        //         .filter(|e| !index_to_remove.contains(&e.0))
        //         .map(|e| e.1)
        //         .collect();

        //     let mut input_map  = 
        //         line01.into_iter()
        //         .map(|e| (e.pattern().len(), e.pattern()))
        //         .into_group_map();
            
        //     let mut base_map: HashMap<u8, String> = base.into_iter().collect();

        //     // Check 3 ----------------- 5 segments ---------------------------------------
        //     let mut segments5 = input_map[&5];
        //     let pattern_07 = base_map.get(&7).unwrap().to_string();
        //     let mut rfilter: Vec<String> = segments5.iter()
        //                                     .filter(|pattern| 
        //                                             check_pattern(
        //                                                     clean_pattern(&pattern).as_str(),
        //                                                     clean_pattern(&pattern_07).as_str()
        //                                             )
        //                                     ).collect();
        //     let result = rfilter[0];
        //     println!("segments5 {:?}", segments5);

        //     let segments5 :Vec<String> = segments5.into_iter().filter(|e| result == *e).map(|e| e.to_string()).collect();
        //     input_map.insert(5, segments5);
            
        //     println!("segments5 {:?}", segments5);
        //     println!("pattern_07 {:?}", pattern_07);
        //     println!("result {:?}", result);

        //     // // Check 9 ---------------------------------------------------------
        //     // let length_05: Vec<SignalPattern> = find_pattern_with_length(&line01, 6);
        //     // let pattern_09 = base_map.get(&3).unwrap().to_string();
        //     // let match_09 = check_pattern(&length_05, &pattern_09).unwrap();
        //     // line01.retain(|x| !(x == &match_09.1));
        //     // base_map.insert(9, match_09.1.pattern());

        //     // // Check 0 ---------------------------------------------------------
        //     // let length_05: Vec<SignalPattern> = find_pattern_with_length(&line01, 6);
        //     // let pattern_07 = base_map.get(&7).unwrap().to_string();
        //     // let match_07 = check_pattern(&length_05, &pattern_07).unwrap();
        //     // line01.retain(|x| !(x == &match_07.1));
        //     // base_map.insert(0, match_07.1.pattern());

        //     // // Check 6 ---------------------------------------------------------
        //     // let length_05: Vec<SignalPattern> = find_pattern_with_length(&line01, 6);
        //     // let pattern_08 = base_map.get(&8).unwrap().to_string();

        //     // let match_06: (usize, SignalPattern) = fit_pattern(&length_05, &pattern_08).unwrap();
        //     // line01.retain(|x| !(x == &match_06.1));
        //     // base_map.insert(6, match_06.1.pattern());

        //     // // Check 5 ---------------------------------------------------------
        //     // let length_05: Vec<SignalPattern> = find_pattern_with_length(&line01, 5);
        //     // let pattern_06 = base_map.get(&6).unwrap().to_string();

        //     // println!("base_map {:?}", base_map);
        //     // let match_05: (usize, SignalPattern) = fit_pattern(&length_05, &pattern_06).unwrap();
        //     // line01.retain(|x| !(x == &match_05.1));
        //     // base_map.insert(5, match_05.1.pattern());

        //     // // Check 2 ---------------------------------------------------------
        //     // if line01.len() == 1 {
        //     //     base_map.insert(2, line01[0].pattern());
        //     // }
        //     //println!("base_map {:?}", base_map);
        // }

        Ok(0)
    }

    fn check_pattern(pattern01: &str, pattern02: &str) -> bool {
        if  pattern01.len() > pattern02.len() {
            contains_pattern(pattern01,pattern02)
        } else {
            contains_pattern(pattern02,pattern01)
        }
    }

    fn fit_pattern(line01: &Vec<SignalPattern>, pattern: &str) -> Option<(usize, SignalPattern)> {
        let mut r: Option<(usize, SignalPattern)> = None;
        for (index, sp) in line01.into_iter().enumerate() {
            if clean_pattern(pattern).contains(&sp.pattern()) {
                r = Some((index, sp.clone()));
                break;
            }
        }
        r
    }

    fn clean_pattern(pattern: &str) -> String {
        pattern.chars().unique().sorted().join("")
    }

    fn merge_pattern(mut pattern1: String, pattern2: String) -> String {
        pattern1.push_str(pattern2.as_str());
        clean_pattern(pattern1.as_str())
    }

    fn contains_pattern(pattern: &str, matcher: &str) -> bool {
        matcher.chars().all(|e| pattern.chars().contains(&e))
    }

    fn diff_pattern(pattern: &str, to_remove: &str) -> String {
        pattern
            .chars()
            .filter(|c| !to_remove.chars().contains(c))
            .join("")
    }

    fn find_pattern_with_length(line01: &Vec<SignalPattern>, length: usize) -> Vec<SignalPattern> {
        line01
            .iter()
            .filter(|&sp| sp.length() == length)
            .map(|e| e.clone())
            .collect()
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

    assert_eq!(part01::run(input).unwrap(), 26);
    //assert_eq!(part02::run(input).unwrap(), 168);
}
