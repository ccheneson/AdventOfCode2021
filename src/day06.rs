use anyhow::Result;

pub fn run() -> Result<()> {
    part01::run()?;
    part02::run()?;
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
    pub fn run() -> Result<()> {
        let file = include_str!("../input/day06.txt");

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

        
        Ok(())
    }
}

mod part02 {
    use anyhow::Result;

    
    #[derive(PartialEq, Eq, Hash, Debug)]
    struct LanternFish(u8, usize);

    impl LanternFish {
        fn timer(&self) -> u8 { self.0 }
        fn occurence(&self) -> usize   { self.1 }
        fn reset_timer(&mut self)  { self.0  = 6 }
        fn decrease_timer(&mut self)  { if self.0  != 0 { self.0 -= 1 } }

        fn check_after_one_day(&mut self) -> Option<usize> {
            match self.timer() {
                0 => {
                    self.reset_timer();
                    Some(self.occurence())
                },
                _ => {
                    self.decrease_timer();
                    None
                }
            }
        }
    }


    //The problem with thsi challenge (part2) is about performance
    //Using solution part1, we inject in the lantern_fishes the lanternfishes newly-created,
    //which makes the lantern_fishes to grow very quickly
    //Instead I added an occurence field so that we inject 1 struct reference the number(count) of that object
    //
    //LanternFish(8),LanternFish(8),LanternFish(8),LanternFish(8)
    // should be translated to
    //LanternFish(8, 3)
    //
    pub fn run() -> Result<()> {
        let file = include_str!("../input/day06.txt");

        let mut lantern_fishes = 
            file.lines()
                .flat_map(|e| e.split(","))
                .map(|e| e.parse::<u8>().unwrap())
                .map(|e|LanternFish(e,1))
                .collect::<Vec<LanternFish>>();


        //Initialize fishes_count with the fish parents already in the input file
        let mut fishes_count = lantern_fishes.len();

        for _ in 0..256 {
            let new_fishes = 
                lantern_fishes
                    .iter_mut()
                    .map(|fish| fish.check_after_one_day())
                    .filter(|fish|fish.is_some())
                    .map(|fish|fish.unwrap())
                    .collect::<Vec<usize>>();

                let new_fish_count:usize = new_fishes.iter().sum();

                if ! new_fishes.is_empty() {
                    fishes_count += new_fish_count;
                }

                lantern_fishes.push(LanternFish(8, new_fish_count));
        
         }

        
         println!("How many lanternfish would there be after 256 days? {:?}", fishes_count);
 
        
        Ok(())
    }
}
