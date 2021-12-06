use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day06.txt");

    part01::run(file)?;
    part02::run(file)?;

    Ok(())
}


mod part01 {

    use anyhow::Result;

    
    struct LanternFish(u8);

    impl LanternFish {
        fn timer(&self) -> u8 { self.0 }
        fn reset_timer(&mut self)  { self.0  = 6 }
        fn decrease_timer(&mut self)  { self.0  -= 1 }
        fn create_new_fish() -> Self { Self(8) }

        fn check_after_one_day(&mut self) -> Option<Self> {
            match self.timer() {
                0 => { 
                    self.reset_timer();
                    Some(Self::create_new_fish())
                },
                _ => {
                    self.decrease_timer();
                    None
                }
            }
        }
    }
    pub fn run(file: &str) -> Result<usize> {        

        let mut lantern_fishes = 
            file.lines()
                .flat_map(|e| e.split(","))
                .map(|e| e.parse::<u8>().unwrap())
                .map(|e|LanternFish(e))
                .collect::<Vec<LanternFish>>();

        for _ in 0..80 {
            let new_fishes = 
                lantern_fishes
                    .iter_mut()
                    .map(|fish| fish.check_after_one_day())
                    .filter(|fish|fish.is_some())
                    .map(|fish|fish.unwrap())
                    .collect::<Vec<LanternFish>>();
                lantern_fishes.extend(new_fishes);
        }

        println!("How many lanternfish would there be after 80 days? {}", lantern_fishes.len());

        
        Ok(lantern_fishes.len())
    }
}

mod part02 {
    use anyhow::Result;

    
    struct LanternFish(u8, usize);

    impl LanternFish {
        fn timer(&self) -> u8 { self.0 }
        fn occurence(&self) -> usize   { self.1 }
        fn reset_timer(&mut self)  { self.0  = 6 }
        fn decrease_timer(&mut self)  { if self.0  != 0 { self.0 -= 1 } }
        fn create_new_fish(occurence: usize) -> Self { Self(8, occurence) }


        fn check_after_one_day(&mut self) -> Option<Self> {
            match self.timer() {
                0 => {
                    self.reset_timer();
                    Some(Self::create_new_fish(self.occurence()))
                },
                _ => {
                    self.decrease_timer();
                    None
                }
            }
        }
    }


    //The problem with this challenge (part2) is about performance
    //Using solution part1, we inject the newly-created LanternFish(es) in the lantern_fishes,
    //which makes the lantern_fishes to grow very quickly
    //Instead I added an occurence field so that we inject 1 struct reference with the number/count/occurence of that object
    //
    //LanternFish(8),LanternFish(8),LanternFish(8),LanternFish(8)
    // should be translated to
    //LanternFish(8, 4)
    //
    pub fn run(file: &str) -> Result<usize> {

        let mut lantern_fishes = 
            file.lines()
                .flat_map(|e| e.split(","))
                .map(|e| e.parse::<u8>().unwrap())
                .map(|e|LanternFish(e,1))
                .collect::<Vec<LanternFish>>();

        for _ in 0..256 {
            let count_new_fish8 : usize = 
                lantern_fishes
                    .iter_mut()
                    .map(|fish| fish.check_after_one_day())
                    .filter(|fish|fish.is_some())
                    .map(|fish|fish.unwrap().1)
                    .sum();

                lantern_fishes.push(LanternFish(8, count_new_fish8));
        
         }
 
         let fishes_count = lantern_fishes.into_iter().map(|f|f.1).sum::<usize>();
         println!("How many lanternfish would there be after 256 days? {:?}", fishes_count);

        
        Ok(fishes_count)
    }
}


#[test]
fn test() {
   let input = "3,4,3,1,2";

   assert_eq!(part01::run(input).unwrap(), 5934);
   assert_eq!(part02::run(input).unwrap(), 26984457539);
   
}
