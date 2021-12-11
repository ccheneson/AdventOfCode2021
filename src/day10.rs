use anyhow::Result;


pub fn run() -> Result<()> {
    let file = include_str!("../input/day10.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}

mod part01 {
    use anyhow::Result;

    pub fn check_corruption(n: Vec<&str>) -> Option<(&str,&str)> {
        let mut check : Vec<&str> = vec!();
        let mut result: Option<(&str, &str)> = None;

        fn expected(c: &str) -> & str {
            match c {
                "("  => ")",
                "["  => "]",
                "{"  => "}",
                "<"  => ">",
                _ => panic!("Invalid char")
            }
        }

        for c in n {
            match c {
                "(" | "[" | "{" | "<" => {
                    check.push(c);
                },
                _ => {
                    match check.last() {
                        Some(latest) => {
                            let should = expected(latest);  

                            if c == should {
                                check.remove(check.len() - 1);
                                continue;
                            } else {
                                result = Some((should, c));
                                break;
                            }
                        },
                        _ => {
                            continue
                        }
                    }
                }
            };
        }
        result
    }
   

    pub fn run(file: &str) -> Result<i32> {
        let input : Vec<Vec<&str>> = 
            file.lines().into_iter()
                .map(|line| 
                        line
                            .split("")
                            .filter(|&e| ! e.is_empty())
                            .collect::<Vec<&str>>()
                ).collect();

        let mut illegal_characters : Vec<&str> = vec!();
                
        for line in input {
            let check = check_corruption(line);
            if check.is_some() {
                illegal_characters.push(check.unwrap().1);
            }
        }


        let score : i32 = illegal_characters
            .into_iter()
            .map(|c| {
                let score = match c {
                    ")" => 3,
                    "]" => 57,
                    "}" => 1197,
                    ">" => 25137,
                    _ => 0
                };
                score
            })
            .sum();

        println!("What is the total syntax error score for those errors? {}", score);

        Ok(score)
    }

    #[test]
    fn test() {
        let file ="{([(<{}[<>[]}>{[]{[(<()>
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    [(()[<>])]({[<{<<[]>>(
    ";
        let input : Vec<Vec<&str>> = 
        file.lines().into_iter()
            .map(|line| 
                    line
                        .split("")
                        .filter(|&e| ! e.is_empty())
                        .collect::<Vec<&str>>()                        
            ).collect();

        assert_eq!(check_corruption(input.get(0).unwrap().to_vec()), Some(("]", "}")));
        assert_eq!(check_corruption(input.get(1).unwrap().to_vec()), Some(("]", ")")));
        assert_eq!(check_corruption(input.get(2).unwrap().to_vec()), Some((")", "]")));
        assert_eq!(check_corruption(input.get(3).unwrap().to_vec()), Some((">", ")")));
        assert_eq!(check_corruption(input.get(4).unwrap().to_vec()), Some(("]", ">")));
        assert_eq!(check_corruption(input.get(5).unwrap().to_vec()), None);

        assert_eq!(run(file).unwrap(), 26397);
    }


}

mod part02 {
    use anyhow::Result;

    pub fn check_corruption(n: Vec<&str>) -> Option<(&str,&str)> {
        let mut check : Vec<&str> = vec!();
        let mut result: Option<(&str, &str)> = None;

        fn expected(c: &str) -> & str {
            match c {
                "("  => ")",
                "["  => "]",
                "{"  => "}",
                "<"  => ">",
                _ => panic!("Invalid char")
            }
        }

        for c in n {
            match c {
                "(" | "[" | "{" | "<" => {
                    check.push(c);
                },
                _ => {
                    match check.last() {
                        Some(latest) => {
                            let should = expected(latest);  

                            if c == should {
                                check.remove(check.len() - 1);
                                continue;
                            } else {
                                result = Some((should, c));
                                break;
                            }
                        },
                        _ => {
                            continue
                        }
                    }
                }
            };
        }
        result
    }
   
    pub fn complete_closing_chars(n: Vec<&str>) -> Vec<&str> {
        let mut check : Vec<&str> = vec!();

        fn expected(c: &str) -> & str {
            match c {
                "("  => ")",
                "["  => "]",
                "{"  => "}",
                "<"  => ">",
                _ => panic!("Invalid char")
            }
        }

        for c in n {
            match c {
                "(" | "[" | "{" | "<" => {
                    check.push(c);
                },
                _ => {
                    match check.last() {
                        Some(latest) => {
                            let should = expected(latest);  

                            if c == should {
                                check.remove(check.len() - 1);
                                continue;
                            }
                        },
                        _ => {
                            continue
                        }
                    }
                }
            };
        }

        check.into_iter().rev().map(|e| expected(e)).collect()
    }

    fn closing_chars_table(c: &str) -> u64 {
        match c {
            ")" => 1,
            "]" => 2,
            "}" => 3,
            ">" => 4,
            _ => 0
        }
    }

    pub fn run(file: &str) -> Result<u64> {
        let input : Vec<Vec<&str>> = 
            file.lines().into_iter()
                .map(|line| 
                        line
                            .split("")
                            .filter(|&e| ! e.is_empty())
                            .collect::<Vec<&str>>()
                ).collect();

        let mut illegal_characters : Vec<&str> = vec!();
        let mut scores : Vec<u64> = vec!();
                
        for line in input {
            let line_clone = line.clone();
            let check = check_corruption(line);
            if check.is_some() {
                illegal_characters.push(check.unwrap().1);
            } else {
                //incomplete but not corrupted
                let result = complete_closing_chars(line_clone)
                                    .into_iter()
                                    .fold(0 as u64, |mut acc, elem| {
                    acc = (acc * 5) + closing_chars_table(elem);
                    acc
                });
                scores.push(result);
            }
        }

        scores.sort();
      
        let score = scores.get(scores.len() / 2).unwrap();
        println!("What is the middle score? {}", score);

        Ok(*score)
   }

    #[test]
    fn test() {
        let file ="{([(<{}[<>[]}>{[]{[(<()>
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    [(()[<>])]({[<{<<[]>>(
    ";
        let input : Vec<Vec<&str>> = 
        file.lines().into_iter()
            .map(|line| 
                    line
                        .split("")
                        .filter(|&e| ! e.is_empty())
                        .collect::<Vec<&str>>()                        
            ).collect();

        assert_eq!(check_corruption(input.get(0).unwrap().to_vec()), Some(("]", "}")));
        assert_eq!(check_corruption(input.get(1).unwrap().to_vec()), Some(("]", ")")));
        assert_eq!(check_corruption(input.get(2).unwrap().to_vec()), Some((")", "]")));
        assert_eq!(check_corruption(input.get(3).unwrap().to_vec()), Some((">", ")")));
        assert_eq!(check_corruption(input.get(4).unwrap().to_vec()), Some(("]", ">")));
        assert_eq!(check_corruption(input.get(5).unwrap().to_vec()), None);

    }

    #[test]
    fn test_completion() {
        let file ="[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
(((({<>}<{<{<>}{[]{[]{}
{<[[]]>}<{[{[{[]{()[[[]
<{([{{}}[<[[[<>{}]]]>[]]
";

    let input : Vec<Vec<&str>> = 
    file.lines().into_iter()
        .map(|line| 
                line
                    .split("")
                    .filter(|&e| ! e.is_empty())
                    .collect::<Vec<&str>>()                        
        ).collect();

    assert_eq!(complete_closing_chars(input.get(0).unwrap().to_vec()).join(""), "}}]])})]");
    assert_eq!(complete_closing_chars(input.get(1).unwrap().to_vec()).join(""), ")}>]})");
    assert_eq!(complete_closing_chars(input.get(2).unwrap().to_vec()).join(""), "}}>}>))))");
    assert_eq!(complete_closing_chars(input.get(3).unwrap().to_vec()).join(""), "]]}}]}]}>");
    assert_eq!(complete_closing_chars(input.get(4).unwrap().to_vec()).join(""), "])}>");

    assert_eq!(run(file).unwrap(), 288957);


    }


}

