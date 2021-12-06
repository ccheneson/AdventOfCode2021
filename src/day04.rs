use anyhow::Result;

pub fn run() -> Result<()> {
    let file = include_str!("../input/day04.txt");

    part01::run(file)?;
    part02::run(file)?;    
    Ok(())
}

#[derive(Debug)]
struct BoardBox {
    number: u8,
    tick: bool
}

impl BoardBox {
    fn new(number: u8) -> Self { Self { number, tick : false}}
    fn set_ticked(&mut self) { self.tick = true }
    fn is_ticked(&self) -> bool { self.tick }
}

mod part01 {
    use std::borrow::BorrowMut;

    use anyhow::Result;

    use super::BoardBox;

    type Row = Vec<BoardBox>;
    type Board = Vec<Row>;


    //Tick the boards that have the drawn number
    fn check_boards(draw: u8, boards: &mut Vec<Board>) {
        boards
            .iter_mut()
            .for_each(|board|   //For all Boards
                board
                    .iter_mut()
                    .for_each(|boardboxes|  //For each rows of 1 board
                        boardboxes
                            .iter_mut()                            
                            .for_each(|boardbox | {
                                if boardbox.number == draw {
                                    boardbox.set_ticked()
                                }
                            })                    
                    )
            )
    }

    fn verify_winner(boards: &Vec<Board>) -> Option<&Board> {
        let mut result = None;
        for board in boards {            
            for row in board {       
                // Check all matching horizental boxes 
                let check_rows = row.iter().all(|boardbox| boardbox.is_ticked());
                if check_rows == true {
                    result = Some(board);
                    break;
                }
            }
            for i in 0..board.len() {
                // Check all matching vertical boxes
                let mut check_cols: bool = true;
                for j in 0..board[i].len() {
                    check_cols = check_cols && board[j][i].is_ticked()
                }
                if check_cols == true {
                    result = Some(board);
                    break;
                }
            }

        }
        result
    }

    pub fn run(file: &str) -> Result<u32> {
        let mut lines_it = file.lines();

        let input_numbers : Vec<u8> = lines_it.next().unwrap().split(",").map(|e| e.parse::<u8>().unwrap()).collect();

        let mut playing_boards : Vec<Board> = vec!();

        //----- Build new boards 
        while let Some(mut line) = lines_it.next() {
            if line.is_empty() {
                continue;
            }
     
            let mut new_board: Vec<_> = vec!();

            for _ in 0..5 {
                let board_row: Vec<BoardBox> = line
                                                .split_whitespace()
                                                .map(|e| e.parse::<u8>().map(|n| BoardBox::new(n)).unwrap())
                                                .collect();
                new_board.push(board_row);
                if let Some(line_next) = lines_it.next() {
                    line = line_next;
                } else {
                    break;
                }
            }
            
            playing_boards.push(new_board);
        }


        let mut result: u32 = 0;

        //----- Let play now !!!
        for draw_number in input_numbers {
            check_boards(draw_number, playing_boards.borrow_mut());            
            if let Some(winner) = verify_winner(&playing_boards) {
                let sum_unticked : u32 = 
                    winner
                        .into_iter()
                        .flat_map(|e|
                            e.into_iter()
                            .filter(|boardbox| !boardbox.is_ticked())
                            .map(|e|e.number as u32)
                            .collect::<Vec<u32>>()
                        )
                        .sum();
                
                result =  sum_unticked * draw_number as u32;
                
                println!("What will your final score be if you choose that board? {}", result);
                break;
            }
        }

        Ok(result)
    }

    
}


mod part02 {
    use std::borrow::BorrowMut;

    use anyhow::Result;

    use super::BoardBox;

    type Row = Vec<BoardBox>;
    type Board = Vec<Row>;


    //Tick the boards that have the drawn number
    fn check_boards(draw: u8, boards: &mut Vec<Board>) {
        boards
            .iter_mut()
            .for_each(|board|
                board
                    .iter_mut()
                    .for_each(|boardboxes|
                        boardboxes
                            .iter_mut()                            
                            .for_each(|boardbox | {
                                if boardbox.number == draw {
                                    boardbox.set_ticked()
                                }
                            })                    
                    )
            )
    }

    fn verify_winner(boards: &Vec<Board>) -> Option<(usize, &Board)> {
        let mut result = None;
        let mut index_board : usize = 0;
        for (index, board) in boards.iter().enumerate() {
            for row in board {       
                // Check all matching horizental boxes 
                let check_rows = row.iter().all(|boardbox| boardbox.is_ticked());
                if check_rows == true {
                    result = Some(board);
                    index_board = index;
                    break;
                }
            }
            for i in 0..board.len() {
                // Check all matching vertical boxes
                let mut check_cols: bool = true;
                for j in 0..board[i].len() {
                    check_cols = check_cols && board[j][i].is_ticked()
                }
                if check_cols == true {
                    result = Some(board);
                    index_board = index;
                    break;
                }
            }

        }
        result.map(|b| (index_board, b))
    }

    pub fn run(file: &str) -> Result<u32> {
        let mut lines_it = file.lines();

        let input_numbers : Vec<u8> = lines_it.next().unwrap().split(",").map(|e| e.parse::<u8>().unwrap()).collect();

        let mut playing_boards : Vec<Board> = vec!();

        //----- Build new boards 
        while let Some(mut line) = lines_it.next() {
            if line.is_empty() {
                continue;
            }
     
            let mut new_board: Vec<_> = vec!();     

            for _ in 0..5 {
                let board_row: Vec<BoardBox> = line
                                                .split_whitespace()
                                                .map(|e| e.parse::<u8>().map(|n| BoardBox::new(n)).unwrap())
                                                .collect();
                new_board.push(board_row);
                if let Some(line_next) = lines_it.next() {
                    line = line_next;
                } else {
                    break;
                }
            }
            
            playing_boards.push(new_board);
        }

        //----- Let play now !!!
        let mut last_drawn_number : u8 = 0;
        let mut last_board: Board = vec!();

        for draw_number in input_numbers {
            check_boards(draw_number, playing_boards.borrow_mut());

            //Here we need to remove the winning boards and we could also have several winning boards for 1 drawn number
            while let Some((index, _)) = verify_winner(&playing_boards) {
                //Discard the winning board
                last_board = playing_boards.remove(index);
                last_drawn_number = draw_number;
            }
        }

        let sum_unticked : u32 = last_board
                                    .iter()
                                    .flat_map(|e|
                                        e.iter()
                                        .filter(|boardbox| ! boardbox.is_ticked())
                                        .map(|e|e.number as u32)
                                        .collect::<Vec<u32>>()
                                    )
                                    .sum();

        let result = sum_unticked * last_drawn_number as u32;

        println!("What will your final score be if you choose that board - part 2 ? {}", result);
        Ok(result)
    }
}


#[test]
fn test() {
   let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#;

   assert_eq!(part01::run(input).unwrap(), 4512);
   assert_eq!(part02::run(input).unwrap(), 1924);
   
}
