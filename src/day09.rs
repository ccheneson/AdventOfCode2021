use std::collections::HashMap;
use anyhow::Result;
use itertools::Itertools;


struct HeightMap<'a> {
    map: &'a HashMap<(usize, usize), i8>,
    max_x: usize,
    max_y: usize
}
impl<'a> HeightMap<'a> {
    fn load(m: &'a HashMap<(usize, usize), i8>, max_x: usize, max_y : usize) -> Self {
        Self {
            map: m, max_x, max_y
        }
    }

    fn filter(&self, v: Vec<Option<&i8>>) -> Vec<i8> {
        v.into_iter().filter(|&e| e.is_some()).map(|e| *e.unwrap()).collect()
    }

    fn get_adjacent_points(&self, x:usize, y: usize) -> Vec<i8> {
        let xi = x as isize;
        let yi = y as isize;
        
        let value_01 = { if (xi - 1) < 0 {  None  }  else { self.map.get(&(x - 1,y)) } };
        let value_02 = { if (xi + 1) > self.max_x as isize {  None  }  else { self.map.get(&(x + 1,y)) } };
        let value_03 = { if (yi - 1) < 0 {  None  }  else { self.map.get(&(x,y - 1)) } };
        let value_04 = { if (yi + 1) > self.max_y as isize {  None  }  else { self.map.get(&(x,y + 1)) } };

        self.filter(vec!(value_01, value_02, value_03, value_04))
    }
}

pub fn run() -> Result<()> {
    let file = include_str!("../input/day09.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}

mod part01 {
    use std::collections::HashMap;
    use anyhow::Result;
    use super::HeightMap;

    pub fn run(file: &str) -> Result<i16> {

        let input : Vec<Vec<i8>> = 
            file.lines().into_iter()
                .map(|line| 
                        line
                            .split("")
                            .filter(|&x| !x.is_empty()
                        )
                        .map(|tokens| tokens.parse::<i8>().unwrap())
                        .collect()
                ).collect();

        let mut arr : HashMap<(usize, usize), i8> = HashMap::new();
        let max_x : usize = input[0].len();
        let max_y : usize = input.len();
        
        for (i, row) in input.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                //j is x and i is y
                arr.insert((j, i), *value);
            }    
        }
    
        let height_map = HeightMap::load(&arr, max_x, max_y);


        let mut lower_points : Vec<i16> = vec!();

        for (y, row) in input.into_iter().enumerate() {            
            for (x, point) in row.into_iter().enumerate() {
                let adjacents = height_map.get_adjacent_points(x,y);
                let lower: Vec<i8> = adjacents.into_iter().filter(|v| v <= &point).collect();
                if lower.is_empty() {               
                    lower_points.push(point as i16);
                }                
            }
        }

        let risk = lower_points.iter().map(|e| *e + 1 ).sum::<i16>();
        println!("What is the sum of the risk levels of all low points on your heightmap? {:?}", risk);

        Ok(risk)
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
    let file ="2199943210
3987894921
9856789892
8767896789
9899965678";

    let input : Vec<Vec<i8>> = 
        file.lines().into_iter()
            .map(|line| 
                    line
                        .split("")
                        .filter(|&x| !x.is_empty()
                    )
                    .map(|tokens| tokens.parse::<i8>().unwrap())
                    .collect()
            ).collect();


    let mut arr : HashMap<(usize, usize), i8> = HashMap::new();
    let max_x : usize = input[0].len();
    let max_y : usize = input.len();
    
    for (i, row) in input.into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate() {
            //j is x and i is y
            arr.insert((j, i), value);
        }    
    }

    let m = HeightMap::load(&arr, max_x, max_y);

    assert_eq!(sort(m.get_adjacent_points(0, 0)), sort(vec!(3,1)));
    assert_eq!(sort(m.get_adjacent_points(1, 0)), sort(vec!(2,9,9)));
    assert_eq!(sort(m.get_adjacent_points(6, 0)), sort(vec!(2,4,4)));
    assert_eq!(sort(m.get_adjacent_points(9, 0)), sort(vec!(1,1)));

    assert_eq!(sort(m.get_adjacent_points(9, 4)), sort(vec!(9,7)));
    assert_eq!(sort(m.get_adjacent_points(0, 4)), sort(vec!(8,8)));
    assert_eq!(sort(m.get_adjacent_points(9, 0)), sort(vec!(1,1)));
    assert_eq!(sort(m.get_adjacent_points(0, 3)), sort(vec!(7,9,9)));
    assert_eq!(sort(m.get_adjacent_points(2, 4)), sort(vec!(6,8,9)));
    assert_eq!(sort(m.get_adjacent_points(0, 3)), sort(vec!(7,9,9)));
    assert_eq!(sort(m.get_adjacent_points(9, 3)), sort(vec!(2,8,8)));
    assert_eq!(sort(m.get_adjacent_points(2, 2)), sort(vec!(6,6,8,8)));
    assert_eq!(sort(m.get_adjacent_points(3, 3)), sort(vec!(6,8,6,9)));
    assert_eq!(sort(m.get_adjacent_points(5, 4)), sort(vec!(9,9,5)));

    assert_eq!(sort(m.get_adjacent_points(5, 0)), sort(vec!(9,9,3)));
    
    fn sort(v: Vec<i8>) -> Vec<i8> {
        v.into_iter().sorted().collect::<Vec<i8>>()
    }
}
