use anyhow::Result;
use itertools::Itertools;


pub fn run() -> Result<()> {
    let file = include_str!("../input/day13.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}


type XY = (i32,i32);


#[derive(Debug)]
pub enum Fold {
    X(i32),
    Y(i32),
}

#[derive(Debug)]
//Should have used a HashSet<XY> instead of Vec<XY> not to worry about duplicates during folds
pub struct Coordinates(Vec<XY>);
impl Coordinates {
    fn length_y(&self) -> usize { self.0.iter().unique().map(|e|e.1).max().unwrap() as usize + 1}
    fn length_x(&self) -> usize { self.0.iter().unique().map(|e|e.0).max().unwrap() as usize + 1 }
    fn count_points(&self) -> usize { self.0.iter().unique().collect::<Vec<&XY>>().len() }
}

#[derive(Debug)]
pub struct FoldInstructions(Vec<Fold>);
impl FoldInstructions {
    fn folds(self) -> Vec<Fold> { self.0 }
}

mod common {
    use super::{FoldInstructions, Coordinates, Fold, XY};

    pub fn parse(file: &str) ->  (FoldInstructions, Coordinates) {
        let mut fold_instructions : Vec<Fold> = vec!();
        let mut coordonnates : Vec<XY> = vec!();
        for line in file.lines().filter(|e| !e.is_empty()) {
            if line.contains(",") {
                let coord : Vec<i32> = line.split(",")
                                            .map(|e| e.parse::<i32>().unwrap())
                                            .into_iter()
                                            .collect();
                coordonnates.push((coord[0], coord[1]));
            } else {
                let tokens : Vec<&str> = line.split("=").collect();
                let fold_alignment = tokens[0].split_whitespace().last().unwrap();
                let fold_value = tokens[1].parse::<i32>().unwrap();
                if fold_alignment == "x" {
                    fold_instructions.push(Fold::X(fold_value));
                } else {
                    fold_instructions.push(Fold::Y(fold_value));
                }

            }
        }
        (FoldInstructions(fold_instructions), Coordinates(coordonnates))
    }

}

mod part01 {
    
    use anyhow::Result;
    use itertools::Itertools;

    use crate::day13::Fold;

    use super::{common::parse, Coordinates, XY};


    fn fold_vertically(fold: i32, coordinates: Coordinates) -> Coordinates {

        let top_clone = coordinates.0.clone().into_iter().unique().sorted();
        let bottom_clone = coordinates.0.clone().into_iter().unique().sorted();

        let top :  Vec<XY> = top_clone.filter(|e| e.1 < fold).collect();


        let bottom :  Vec<XY> = bottom_clone.filter(|e| e.1 > fold)
                                            .map(|e| (e.0, (e.1 - ((e.1 - fold) * 2))))
                                            .collect();

        let new_coordinates : Vec<XY>  = [top, bottom ].concat();
        Coordinates(new_coordinates)
    }

    fn fold_horizentally(fold: i32, coordinates: Coordinates) -> Coordinates {

        let top_clone = coordinates.0.clone().into_iter().unique().sorted();
        let bottom_clone = coordinates.0.clone().into_iter().unique().sorted();

        let left :  Vec<XY> = top_clone.filter(|e| e.0 < fold).collect();


        let right :  Vec<XY> = bottom_clone.filter(|e| e.0 > fold)
                                            .map(|e| ((e.0 - ((e.0 - fold) * 2)), e.1))
                                            .collect();

        let new_coordinates : Vec<XY>  = [left, right ].concat();
        
        Coordinates(new_coordinates)
    }


    #[allow(unused)]
    fn draw(coordinates: &Coordinates) {
        for j in 0..coordinates.length_y() {
            for i in 0..coordinates.length_x() {                
                if coordinates.0.contains(&(i as i32 ,j as i32)) {
                     print!("#");
                 } else {
                     print!(".")
                 }
            }
            println!();
        }
    }


    pub fn run(file: &str) -> Result<usize> {
        let input = parse(file);

        let input_coordinates = input.1;
        let fold = &input.0.folds()[0];
        let coordinates = match *fold {
            Fold::X(x) => fold_horizentally(x, input_coordinates),
            Fold::Y(y) => fold_vertically(y, input_coordinates)
        };

        println!("How many dots are visible after completing just the first fold instruction on your transparent paper? {}", coordinates.count_points());

        Ok(coordinates.count_points())
    }


}

mod part02 {

    use anyhow::Result;
    use itertools::Itertools;

    use crate::day13::Fold;

    use super::{common::parse, Coordinates, XY};


    fn fold_vertically(fold: i32, coordinates: &Coordinates) -> Coordinates {

        let top_clone = coordinates.0.clone().into_iter().unique().sorted();
        let bottom_clone = coordinates.0.clone().into_iter().unique().sorted();

        let top :  Vec<XY> = top_clone.filter(|e| e.1 < fold).collect();


        let bottom :  Vec<XY> = bottom_clone.filter(|e| e.1 > fold)
                                            .map(|e| (e.0, (e.1 - ((e.1 - fold) * 2))))
                                            .collect();

        let new_coordinates : Vec<XY>  = [top, bottom ].concat();
        Coordinates(new_coordinates)
    }

    fn fold_horizentally(fold: i32, coordinates: &Coordinates) -> Coordinates {

        let top_clone = coordinates.0.clone().into_iter().unique().sorted();
        let bottom_clone = coordinates.0.clone().into_iter().unique().sorted();

        let left :  Vec<XY> = top_clone.filter(|e| e.0 < fold).collect();


        let right :  Vec<XY> = bottom_clone.filter(|e| e.0 > fold)
                                            .map(|e| ((e.0 - ((e.0 - fold) * 2)), e.1))
                                            .collect();

        let new_coordinates : Vec<XY>  = [left, right ].concat();
        
        Coordinates(new_coordinates)
    }


    #[allow(unused)]
    fn draw(coordinates: &Coordinates) {
        for j in 0..coordinates.length_y() {
            for i in 0..coordinates.length_x() {                
                if coordinates.0.contains(&(i as i32 ,j as i32)) {
                     print!("#");
                 } else {
                     print!(".")
                 }
            }
            println!();
        }
    }


    pub fn run(file: &str) -> Result<&str> {
        let (fold_instructions, mut coordinates) = parse(file);

        for fold in fold_instructions.folds() {
            coordinates = match fold {
                Fold::X(x) => fold_horizentally(x, &coordinates),
                Fold::Y(y) => fold_vertically(y, &coordinates)
            };
        }

        //draw(&coordinates); // Drawing "ARHZPCUH"

        println!("What code do you use to activate the infrared thermal imaging camera system? {}", "ARHZPCUH");

        Ok("ARHZPCUH")
    }



}

