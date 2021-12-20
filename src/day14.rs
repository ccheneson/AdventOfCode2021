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

// mod part02 {
    
//     use std::collections::{HashMap, BTreeMap, VecDeque};

//     use anyhow::Result;
//     use itertools::Itertools;

//     pub fn parse(file: &str) -> (Vec<&str>, Vec<(&str, &str)>) {
//         let mut poly_template : Vec<&str> = vec!();
//         let mut rules : Vec<(&str, &str)> = vec!();
//         file.lines().into_iter().for_each(|e| {
//             if e.contains("->") {
//                 let tokens = e.split("->")
//                                         .map(|e| e.trim()).collect::<Vec<&str>>();
//                 //let c : Vec<&str> = tokens[0].split("").filter(|e| !e.is_empty()).collect();
//                 rules.push((tokens[0], tokens[1]));
//             }
//             else if e.is_empty() {
//             } else {
//                 poly_template = e.split("").filter(|e|!e.is_empty()).collect();
//             }
//         });
//         (poly_template, rules)
//     }



//     pub fn run(file: &str) -> Result<usize> {
    
//         let (template, rules) = parse(file);

//         let m : HashMap<&str, &str> = rules.into_iter().collect();
//         let mut template_it = template.into_iter();
//         let template_first_char = template_it.next().unwrap();

//         let mut tracker : HashMap<String, u16> = HashMap::new();


//         //----------------- Get initial map --------------------

//         let (result, _left) = template_it.fold((vec!(), template_first_char.to_string()), |(mut acc, mut prev), elem| {
//             prev.push_str( elem); 
//             {
//                 let value = m.get(prev.as_str());
//                 if let Some(&v) = value {
//                     *tracker.entry(prev.clone()).or_insert(0) += 1;
//                     acc.push(prev);
//                     acc.push(v.to_string());
//                 }
//             }
//             (acc, elem.to_string())
            
//         });

//         println!("Initial map {:?}", tracker);
        
//         //------------------------------------------------------

//         for _ in 1..10 {
//             tracker = process(tracker, &m);
//         }

//         println!("Tracker {:?}", tracker);


//         //Stats to rules
//         tracker = tracker.into_iter().map(|e| {
//                 if let Some(&v) = m.get(e.0.as_str()) {
//                     (v.to_string(), e.1)
//                 } else {
//                     e
//                 }
//         })
//         .group_by(|e| e.0.clone())
//         .into_iter()
//         .map(|(key, items)| (key, items.map(|(e, value)| value).sum()))
//         .collect();

//         println!("Tracker {:?}", tracker);


        


//         //https://stackoverflow.com/questions/54936304/finding-most-frequently-occurring-string-in-a-structure-in-rust
//         //Count number of occurence of each elemnts
//         let mut counts = BTreeMap::new();
//         for word in tracker.iter() {
//             *counts.entry(word).or_insert(0) += 1;
//         }

//         let max = counts.iter().max_by_key(|&(_, count)| count).map(|e| e.1).unwrap();
//         let min = counts.iter().min_by_key(|&(_, count)| count).map(|e| e.1).unwrap();

//         let result = max - min;

//         println!("What do you get if you take the quantity of the most common element and subtract the quantity of the least common element? {}", result);

//         Ok(result)
//     }


    
//     //{"CB": 1, "NN": 1, "NC": 1}
//     fn process<'a>(input: HashMap<String, u16>, rules : &HashMap<&str, &str>) -> HashMap<String, u16> {

//         let mut result = input.clone();

//         input.into_iter().for_each(|e| {
//             if let Some(v) = rules.get(e.0.as_str()) {   //get("CH")
//                 let tokens : Vec<&str> = e.0.split("").filter(|e| !e.is_empty()).collect(); //["C","H"]                
//                 let mut key01 = tokens[0].to_string();
//                 key01.push_str(v);
//                 let mut key02 = v.to_string();
//                 key02.push_str(tokens[1]);
//                 *result.entry(key01).or_insert(0) += 1;
//                 *result.entry(key02).or_insert(0) += 1;
//             }
//         });

//         result

//     }

