use anyhow::Result;

pub fn run() -> Result<()> {
    part01::run()?;
    part02::run()?;
    Ok(())
}

#[derive(Debug, PartialEq , Eq, Hash)]
struct Coordinates {
    x: u16,
    y: u16
}

impl Coordinates {
    fn from_slice(s: &[u16]) -> Self {
        Self { x: s[0], y : s[1] }
    }
}

mod part01 {
    use anyhow::Result;
    use itertools::Itertools;

    use crate::day05::Coordinates;

    #[derive(Debug)]
    struct Lines {
        start: Coordinates,
        end: Coordinates
    }

    #[derive(Debug)]
    enum CoordinateBase {
        X(u16), Y(u16)
    }

    impl Lines {

        fn generate_coordinates(&self, base: CoordinateBase, from_incl: u16, to_incl: u16) -> Vec<Coordinates> {
            //The below is needed for .rev()
            let (from_incl, to_incl, up) = if from_incl < to_incl { (from_incl, to_incl, true) } else { (to_incl, from_incl, false) };            
            let gen : Vec<u16> = if up { (from_incl..=to_incl).collect() } else { (from_incl..=to_incl).rev().collect()};
            match base {
                CoordinateBase::X(value) => gen.into_iter().map(|p| Coordinates { x: value, y: p }).collect(),
                CoordinateBase::Y(value) => gen.into_iter().map(|p| Coordinates { x: p, y:value }).collect()
            }
        }

        fn points_covered(&self) -> Vec<Coordinates> {
            if self.start.x == self.end.x {
                let base = self.start.x;
                self.generate_coordinates(CoordinateBase::X(base), self.start.y, self.end.y)
            } else if self.start.y == self.end.y {
                let base = self.start.y;
                self.generate_coordinates(CoordinateBase::Y(base), self.start.x, self.end.x)
            } else  {
                vec!()
            }
        }
    }


    fn parse_line_to_coordinates(line: &str) -> Lines  {
        let coordinates   = line
                                    .split("->")
                                    .map(|e|e.trim())
                                    .map(|e| {
                                        e.split(",")
                                            .map(|e| e.parse::<u16>().unwrap())
                                            .collect::<Vec<u16>>()
                                    })
                                    .collect::<Vec<Vec<u16>>>();

        let start = &coordinates[0];
        let end = &coordinates[1];    
        Lines { start: Coordinates::from_slice(start) , end: Coordinates::from_slice(end)  }        
    }

    pub fn run() -> Result<()> {
        let file = include_str!("../input/day05.txt");
        let lines: Vec<Lines> = file.lines().map(|line| parse_line_to_coordinates(line)).collect();
        let covered_points: Vec<Coordinates> = 
            lines.into_iter().flat_map(|e| e.points_covered()).collect();

        //Get duplicates - if duplicates, it means there is at least 2 occurences
        let total = covered_points.iter().duplicates().count();        

        println!("At how many points do at least two lines overlap? {}", total);
        
        Ok(())
    }

    #[test]
    fn test_entry_point() {
        let line1 = Lines {
            start:  Coordinates { x:1 , y: 1 },
            end:    Coordinates { x:1 , y: 3 }
        };
        let line2 = Lines {
            start:  Coordinates { x:9 , y: 7 },
            end:    Coordinates { x:7 , y: 7 }
        };
        let line3 = Lines {
            start:  Coordinates { x:3 , y: 8 },
            end:    Coordinates { x:5 , y: 9 }
        };

        assert_eq!(line1.points_covered(), vec!(Coordinates { x:1, y: 1}, Coordinates { x:1, y: 2}, Coordinates { x:1, y: 3}));
        assert_eq!(line2.points_covered(), vec!(Coordinates { x:9, y: 7}, Coordinates { x:8, y: 7}, Coordinates { x:7, y: 7}));
        assert_eq!(line3.points_covered(), vec!());
    }

}

mod part02 {
    use anyhow::Result;
    use itertools::{Itertools, enumerate};

    use crate::day05::Coordinates;

    #[derive(Debug)]
    struct Lines {
        start: Coordinates,
        end: Coordinates
    }

    #[derive(Debug)]
    enum CoordinateBase {
        X(u16), Y(u16)
    }

    enum Alignment {
        HORIZENTAL, VERTICAL, DIAGONAL, NONE
    }


    impl Lines {

        fn get_alignment(&self) -> Alignment {
            if self.start.x == self.end.x {
                Alignment::HORIZENTAL
            } else if self.start.y == self.end.y {
                Alignment::VERTICAL
            } else if (self.start.x as i16 - self.end.x as i16).abs() == (self.start.y as i16 - self.end.y as i16).abs() {
                Alignment::DIAGONAL
            } else {
                Alignment::NONE
            }
        }

        fn create_point_generator(from_incl: u16, to_incl: u16, up: bool) -> Vec<u16> {
            //The below is needed for .rev()
            let (from_incl, to_incl) = if from_incl < to_incl { (from_incl, to_incl) } else { (to_incl, from_incl) };
            if up { (from_incl..=to_incl).collect() } else { (from_incl..=to_incl).rev().collect() }
        }

        fn generate_coordinates(base: CoordinateBase, from_incl: u16, to_incl: u16) -> Vec<Coordinates> {
            let up = from_incl <  to_incl;//increment if true else decrement
            let gen : Vec<u16> = Self::create_point_generator(from_incl, to_incl, up);
            match base {
                CoordinateBase::X(value) => gen.into_iter().map(|p| Coordinates { x: value, y: p }).collect(),
                CoordinateBase::Y(value) => gen.into_iter().map(|p| Coordinates { x: p, y:value }).collect()
            }
        }

        fn generate_coordinates_diag(from: &Coordinates, to: &Coordinates) -> Vec<Coordinates> {
            let up_x : bool = from.x < to.x;
            let up_y : bool = from.y < to.y;

            //The below is needed for .rev()
            let (from_x, to_x) = if up_x { (from.x, to.x) } else { (to.x, from.x) };
            let (from_y, to_y) = if up_y { (from.y, to.y) } else { (to.y, from.y) };

            let gen_x : Vec<u16> = if up_x { (from_x..=to_x).collect() } else { (from_x..=to_x).rev().collect() };
            let gen_y : Vec<u16> = if up_y { (from_y..=to_y).collect() } else { (from_y..=to_y).rev().collect() };

            let mut result : Vec<Coordinates> = vec!();

            for (index, x) in enumerate(gen_x) {
                let y = gen_y[index];
                result.push(Coordinates { x, y });
            }
            result
        }

        fn points_covered(&self) -> Vec<Coordinates> {
            match self.get_alignment() {
                Alignment::HORIZENTAL => {
                    let base = self.start.x;
                    Self::generate_coordinates(CoordinateBase::X(base), self.start.y, self.end.y)
                },
                Alignment::VERTICAL => {
                    let base = self.start.y;
                    Self::generate_coordinates(CoordinateBase::Y(base), self.start.x, self.end.x)
                },
                Alignment::DIAGONAL => {
                    Self::generate_coordinates_diag(&self.start, &self.end)    
                }
                Alignment::NONE => vec!()
            }
        }
    }


    fn parse_line_to_coordinates(line: &str) -> Lines  {
        let coordinates   = line
                                    .split("->")
                                    .map(|e|e.trim())
                                    .map(|e| {
                                        e.split(",")
                                            .map(|e| e.parse::<u16>().unwrap())
                                            .collect::<Vec<u16>>()
                                    })
                                    .collect::<Vec<Vec<u16>>>();

        let start = &coordinates[0];
        let end = &coordinates[1];    
        Lines { start: Coordinates::from_slice(start) , end: Coordinates::from_slice(end)  }        
    }

    pub fn run() -> Result<()> {
        let file = include_str!("../input/day05.txt");
        let lines: Vec<Lines> = file.lines().map(|line| parse_line_to_coordinates(line)).collect();
        let covered_points: Vec<Coordinates> = 
            lines.into_iter().flat_map(|e| e.points_covered()).collect();

        //Get duplicates - if duplicates, it means there is at least 2 occurences
        let total = covered_points.iter().duplicates().count();        

        println!("At how many points do at least two lines overlap? {}", total);
        
        Ok(())
    }


    #[test]
    fn test_entry_point() {
        let line1 = Lines {
            start:  Coordinates { x:1 , y: 1 },
            end:    Coordinates { x:1 , y: 3 }
        };
        let line2 = Lines {
            start:  Coordinates { x:9 , y: 7 },
            end:    Coordinates { x:7 , y: 7 }
        };
        let line3 = Lines {
            start:  Coordinates { x:3 , y: 8 },
            end:    Coordinates { x:5 , y: 9 }
        };
        let line4 = Lines {
            start:  Coordinates { x:1 , y: 1 },
            end:    Coordinates { x:3 , y: 3 }
        };

        let line5 = Lines {
            start:  Coordinates { x:9 , y: 7 },
            end:    Coordinates { x:7 , y: 9 }
        };
        assert_eq!(line1.points_covered(), vec!(Coordinates { x:1, y: 1}, Coordinates { x:1, y: 2}, Coordinates { x:1, y: 3}));
        assert_eq!(line2.points_covered(), vec!(Coordinates { x:9, y: 7}, Coordinates { x:8, y: 7}, Coordinates { x:7, y: 7}));
        assert_eq!(line3.points_covered(), vec!());
        assert_eq!(line4.points_covered(), vec!(Coordinates { x:1, y: 1}, Coordinates { x:2, y: 2}, Coordinates { x:3, y: 3}));
        assert_eq!(line5.points_covered(), vec!(Coordinates { x:9, y: 7}, Coordinates { x:8, y: 8}, Coordinates { x:7, y: 9}));
    }


}
