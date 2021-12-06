use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day01.txt");

    part01::run(file)?;
    part02::run(file)?;
    part02::run_bis(file)?;

    Ok(())
}

mod part01 {
    use anyhow::Result;

    pub fn run(file: &str) -> Result<usize> {
        let mut lines = file.lines();
        let mut previous = lines.next().unwrap().parse::<u16>()?;
        let mut larger_count: usize = 0;

        for line in lines {
            let line = line.parse::<u16>()?;
            if line > previous {
                larger_count += 1;
            }
            previous = line;
        }

        println!(
            "How many measurements are larger than the previous measurement - part1 ? {}",
            larger_count
        );

        Ok(larger_count)
    }
}

mod part02 {
    use anyhow::Result;
    use std::{collections::VecDeque, iter::Sum};


    pub fn run(file: &str) -> Result<usize> {
        let file = file.lines();

        let mut queue = FixSizeQueue::<u16, { 3 as usize }>::new();
        let mut previous: Option<u16> = None;
        let mut larger_count: usize = 0;

        for line in file {
            let line = line.parse::<u16>()?;
            queue.push(line);
            if let Some(sum) = queue.sum() {
                match previous {
                    None => previous = queue.sum(), //N/A No previous sum - no increase of counter
                    Some(value) if value < sum => {
                        larger_count += 1;
                        previous = Some(sum);
                    }
                    _ => previous = Some(sum),
                }
            }
        }

        println!(
            "How many measurements are larger than the previous measurement - part2 ? {}",
            larger_count
        );

        Ok(larger_count)
    }

    // I discovered the array_windows function
    // and wanted to use it here for reference
    pub fn run_bis(file: &str) -> Result<usize> {
        let file = file.lines();

        let larger_count = file
            .map(|line| line.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
            .array_windows()
            .map(|[a, b, c]| a + b + c)
            .collect::<Vec<u16>>()
            .array_windows()
            .filter(|[a, b]| a < b)
            .count();

        println!(
            "How many measurements are larger than the previous measurement - part2 bis ? {}",
            larger_count
        );

        Ok(larger_count)
    }

    struct FixSizeQueue<T, const N: usize> {
        queue: VecDeque<T>,
    }

    impl<T: Sum + Copy, const N: usize> FixSizeQueue<T, N> {
        fn push(&mut self, value: T) {
            if self.queue.len() >= N {
                self.queue.pop_back();
            }
            self.queue.push_front(value);
        }

        fn sum(&self) -> Option<T> {
            if self.queue.len() >= N {
                Some(self.queue.iter().copied().sum())
            } else {
                None
            }
        }

        fn new() -> Self {
            Self {
                queue: VecDeque::with_capacity(N),
            }
        }
    }
}

#[test]
fn test() {
   let input = r#"199
200
208
210
200
207
240
269
260
263"#;

   assert_eq!(part01::run(input).unwrap(), 7);
   assert_eq!(part02::run(input).unwrap(), 5);
   assert_eq!(part02::run_bis(input).unwrap(), 5);
   
}
