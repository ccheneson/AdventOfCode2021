use anyhow::Result;


pub fn run() -> Result<()> {
    let file = include_str!("../input/day11.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}

mod common {

    pub fn parse(file: &str) -> Vec<Vec<i8>> {
            file.lines().into_iter()
                .map(|line| 
                        line
                            .split("")
                            .filter(|&x| !x.is_empty()
                        )
                        .map(|tokens| tokens.trim().parse::<i8>().unwrap())
                        .collect()
                ).collect()
    }
}

mod part01 {
    use std::collections::HashMap;
    use colored::*;
    use anyhow::Result;

    use super::common::parse;

    #[derive(Debug)]
    struct Grid {
        map: HashMap<(usize, usize), i8>,
        count_flash : usize
    }
    impl<'a> Grid {
        fn load(m: HashMap<(usize, usize), i8>) -> Self {
            Self {
                map: m, count_flash : 0
            }
        }

        fn increase_energy(&mut self) {
            self.map = self.map.iter()
                               .map(|(c, v)| {  
                                    let plus_1 = v + 1;
                                    if plus_1 == 10 {
                                        self.count_flash += 1;
                                        (*c, 0)
                                    } else {
                                        (*c, plus_1)
                                    }                                
                               })
                               .collect();
        }

        fn increase_energy_cell(&mut self, x: usize, y: usize) {
            let value = self.get_value(x as isize, y as isize).unwrap().0;
            let energy_plus = if (value + 1 ) == 10 { 
                self.count_flash += 1;
                0
            } else if value == 0 { 
                value 
            } else {
                value + 1
            };
            self.map.insert((x,y), energy_plus);
        }

        fn filter(&self, v: Vec<Option<(i8, (usize, usize))>>) -> Vec<(i8, (usize, usize))> {
            v.into_iter()
                .filter(|&e| e.is_some())
                .map(|e| {
                    let (v,(x, y)) = e.unwrap();
                    (v, (x, y))
                }).collect()
        }

        fn get_value(&self, x: isize, y:isize) -> Option<(i8, (usize, usize))> {
            if x < 0 || y < 0 { 
                None
            } else { 
                let x = x as usize;
                let y = y as usize;
                if self.map.contains_key(&(x,y)) {
                    self.map.get(&(x,y)).map(|e| (*e, (x, y)))
                } else {
                    None
                }
            }
        }
  
    

        fn get_adjacent_points(&self, x:usize, y: usize) -> Vec<(i8, (usize, usize))> {
            let x = x as isize;
            let y = y as isize;

            let value_01 = self.get_value(x -1, y);
            let value_02 = self.get_value(x + 1, y);
            let value_03 = self.get_value(x, y - 1);
            let value_04 = self.get_value(x, y + 1);
            let value_05 = self.get_value(x-1, y - 1);
            let value_06 = self.get_value(x-1, y + 1);
            let value_07 = self.get_value(x + 1, y - 1);
            let value_08 = self.get_value(x + 1, y + 1);

    
            self.filter(vec!(value_01, value_02, value_03, value_04, value_05, value_06, value_07, value_08))
        }

        fn collect_with_value(&self, n: i8) ->  Vec<(&(usize, usize), &i8)> {
            self.map.iter()
                    .filter(|&e| *e.1 == n).collect()
        }

        fn check_lights(&mut self) {

            let mut tracker : HashMap<(usize, usize), i8> = HashMap::new();

            loop {
                //Get all point == 0 and unhandled
                //An octopus can only flash at most once per step.
                let mut buffer : Vec<(&(usize, usize), &i8)> = self.collect_with_value(0).into_iter()
                                                                    .filter(|&e| ! tracker.contains_key(e.0)).collect();

                //If buffer is empty, we have treated everything
                if buffer.is_empty() {
                    break;
                }

                let mut increase_energy : Vec<(i8, (usize, usize))> = vec!();

                while let Some(((x, y), v)) = buffer.pop() {
                    let  adjacents: Vec<(i8, (usize, usize))> = 
                        self.get_adjacent_points(*x, *y)
                            .into_iter()
                            .filter(|e| e.0 != 0)
                            .collect();                            
                    adjacents.iter().for_each(|e| increase_energy.push(*e));
                    tracker.insert((*x,*y), *v);
                    
                }

                increase_energy.into_iter().for_each(|e| self.increase_energy_cell(e.1.0, e.1.1));

            }
            
        }

        #[allow(unused)]
        fn pretty_print(&self) {
            for i in 0..10 {
                for j in 0..10 {
                    let v = self.get_value(j, i).unwrap().0;
                    if v == 0 {
                        print!("{}", "0".blue().bold());
                    } else if v == 9 {
                        print!("{}", "9".yellow().bold());
                    } else {
                        print!("{:?}", self.get_value(j, i).unwrap().0);
                    }
                }
                println!()
            }
        }
    }


    pub fn run(file: &str) -> Result<usize> {
        let input = parse(file);

        let mut arr : HashMap<(usize, usize), i8> = HashMap::new();

        for (i, row) in input.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                //j is x and i is y
                arr.insert((j, i), *value);
            }    
        }
    
        let mut grid = Grid::load(arr);    
        for _ in 0..100 {
            grid.increase_energy();
            grid.check_lights();
        }


        let result = grid.count_flash;
        println!("How many total flashes are there after 100 steps? {}", result);
        Ok(result)
    }


    #[test]
    fn test() {
        let file ="5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    assert_eq!(run(file).unwrap(), 1656);

    }

}

mod part02 {
    use std::collections::HashMap;
    use colored::*;
    use anyhow::Result;

    use super::common::parse;

    #[derive(Debug)]
    struct Grid {
        map: HashMap<(usize, usize), i8>,
        count_flash : usize
    }
    impl<'a> Grid {
        fn load(m: HashMap<(usize, usize), i8>) -> Self {
            Self {
                map: m, count_flash : 0
            }
        }

        fn increase_energy(&mut self) {
            self.map = self.map.iter()
                               .map(|(c, v)| {  
                                    let plus_1 = v + 1;
                                    if plus_1 == 10 {
                                        self.count_flash += 1;
                                        (*c, 0)
                                    } else {
                                        (*c, plus_1)
                                    }                                
                               })
                               .collect();
        }

        fn increase_energy_cell(&mut self, x: usize, y: usize) {
            let value = self.get_value(x as isize, y as isize).unwrap().0;
            let energy_plus = if (value + 1 ) == 10 { 
                self.count_flash += 1;
                0
            } else if value == 0 { 
                value 
            } else {
                value + 1
            };
            self.map.insert((x,y), energy_plus);
        }

        fn filter(&self, v: Vec<Option<(i8, (usize, usize))>>) -> Vec<(i8, (usize, usize))> {
            v.into_iter()
                .filter(|&e| e.is_some())
                .map(|e| {
                    let (v,(x, y)) = e.unwrap();
                    (v, (x, y))
                }).collect()
        }

        fn get_value(&self, x: isize, y:isize) -> Option<(i8, (usize, usize))> {
            if x < 0 || y < 0 { 
                None
            } else { 
                let x = x as usize;
                let y = y as usize;
                if self.map.contains_key(&(x,y)) {
                    self.map.get(&(x,y)).map(|e| (*e, (x, y)))
                } else {
                    None
                }
            }
        }
  
    

        fn get_adjacent_points(&self, x:usize, y: usize) -> Vec<(i8, (usize, usize))> {
            let x = x as isize;
            let y = y as isize;

            let value_01 = self.get_value(x -1, y);
            let value_02 = self.get_value(x + 1, y);
            let value_03 = self.get_value(x, y - 1);
            let value_04 = self.get_value(x, y + 1);
            let value_05 = self.get_value(x-1, y - 1);
            let value_06 = self.get_value(x-1, y + 1);
            let value_07 = self.get_value(x + 1, y - 1);
            let value_08 = self.get_value(x + 1, y + 1);

    
            self.filter(vec!(value_01, value_02, value_03, value_04, value_05, value_06, value_07, value_08))
        }

        fn collect_with_value(&self, n: i8) ->  Vec<(&(usize, usize), &i8)> {
            self.map.iter()
                    .filter(|&e| *e.1 == n).collect()
        }

        fn is_synchronized(&self) -> bool {
            self.map.iter().all(|e|*e.1 == 0)
        }

        fn check_lights(&mut self) {

            let mut tracker : HashMap<(usize, usize), i8> = HashMap::new();

            loop {
                //Get all point == 0 and unhandled
                //An octopus can only flash at most once per step.
                let mut buffer : Vec<(&(usize, usize), &i8)> = self.collect_with_value(0).into_iter()
                                                                    .filter(|&e| ! tracker.contains_key(e.0)).collect();

                //If buffer is empty, we have treated everything
                if buffer.is_empty() {
                    break;
                }

                let mut increase_energy : Vec<(i8, (usize, usize))> = vec!();

                while let Some(((x, y), v)) = buffer.pop() {
                    let  adjacents: Vec<(i8, (usize, usize))> = 
                        self.get_adjacent_points(*x, *y)
                            .into_iter()
                            .filter(|e| e.0 != 0)
                            .collect();                            
                    adjacents.iter().for_each(|e| increase_energy.push(*e));
                    tracker.insert((*x,*y), *v);
                    
                }

                increase_energy.into_iter().for_each(|e| self.increase_energy_cell(e.1.0, e.1.1));

            }
            
        }

        #[allow(unused)]
        fn pretty_print(&self) {
            for i in 0..10 {
                for j in 0..10 {
                    let v = self.get_value(j, i).unwrap().0;
                    if v == 0 {
                        print!("{}", "0".blue().bold());
                    } else if v == 9 {
                        print!("{}", "9".yellow().bold());
                    } else {
                        print!("{:?}", self.get_value(j, i).unwrap().0);
                    }
                }
                println!()
            }
        }
    }


    pub fn run(file: &str) -> Result<usize> {
        let input = parse(file);

        let mut arr : HashMap<(usize, usize), i8> = HashMap::new();

        for (i, row) in input.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                //j is x and i is y
                arr.insert((j, i), *value);
            }    
        }
    
        let mut grid = Grid::load(arr);    
        let mut counter = 0;

        loop {
            counter += 1;
            grid.increase_energy();
            grid.check_lights();
            if grid.is_synchronized() {
                break;
            }
        }
        
        println!("What is the first step during which all octopuses flash? {}", counter);
        Ok(counter)
    }


    #[test]
    fn test() {
        let file ="5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    assert_eq!(run(file).unwrap(), 195);

    }

}

