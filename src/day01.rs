
use std::{collections::VecDeque, iter::Sum};

use anyhow::Result;

pub fn day01() -> Result<()> {
    day01_part01()?;
    day01_part02()?;
    Ok(())
}

pub fn day01_part01() -> Result<()> {
    let mut file = include_str!("../input/day01/input01.txt").lines();
    let mut previous = file.next().unwrap().parse::<u16>()?;
    let mut larger_count:u16 = 0;

    for line in file {
        let line = line.parse::<u16>()?;
        if line > previous {
            larger_count += 1;
        }
        previous = line;
    }

    println!("How many measurements are larger than the previous measurement - part1 ? {}", larger_count);

    Ok(())
}




pub fn day01_part02() -> Result<()> {
    let file = include_str!("../input/day01/input01.txt").lines();

    
    let mut queue = FixSizeQueue::<u16, {3 as usize}>::new();
    let mut previous: Option<u16> = None;
    let mut larger_count = 0;

    for line in file {
        let line = line.parse::<u16>()?;
        queue.push(line);
        if let Some(sum) = queue.sum() {
            match previous {
                None => previous = queue.sum(), //N/A No previous sum - no increase of counter
                Some(value) if value < sum => {
                    larger_count += 1;
                    previous = Some(sum);
                },
                _ => previous = Some(sum)
            }            
        }        
    }

    println!("How many measurements are larger than the previous measurement - part2 ? {}", larger_count);

    Ok(())
}

struct FixSizeQueue<T, const N: usize> {
    queue : VecDeque<T>
}

impl<T: Sum + Copy, const N: usize> FixSizeQueue<T, N>{

    fn push(&mut self, value: T)  {
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
            queue : VecDeque::with_capacity(N)
        }
    }
}

