use std::{fs::File, io::BufRead, io::BufReader, collections::HashSet};

const ROWS: [u32; 5] = [
    0b1111100000000000000000000,
    0b0000011111000000000000000,
    0b0000000000111110000000000,
    0b0000000000000001111100000,
    0b0000000000000000000011111,
];

const COLS: [u32; 5] = [
    0b1000010000100001000010000,
    0b0100001000010000100001000,
    0b0010000100001000010000100,
    0b0001000010000100001000010,
    0b0000100001000010000100001,
];

fn check(mask: u32) -> i8 {
    for i in 0..5 {
        // println!("mask={:#027b} rows={:#027b} &={:#027b}", mask, ROWS[i], ROWS[i]&mask);
        if ROWS[i] & mask == ROWS[i] {
            return 1
        }
        if COLS[i] & mask == COLS[i] {
            return -1
        }
    }
    return 0
}

struct Board {
    id: u32,
    nums: Vec<u32>,
    mask: u32,
}
struct BoardResult {
    last: u32,
    product: u32,
    sum: u32,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut bingo_nums: Vec<u32> = Vec::new();

    let mut board_id = 0;
    let mut boards: Vec<Board> = Vec::new();
    let mut board = Board {
        id: board_id,
        nums: Vec::new(),
        mask: 0,
    };

    for(idx, raw_row) in reader.lines().enumerate() {
        let raw_row_str = raw_row.unwrap();

        // ignore empty rows
        if raw_row_str.len() == 0 {
            if idx > 1 {
                board_id += 1;
                boards.push(board);
                board = Board {
                    id: board_id,
                    nums: Vec::new(),
                    mask: 0,
                };
            }
        }  

        // populate bingo number sequence
        if idx == 0 {
            for (_, str) in raw_row_str.split(',').enumerate() {
                bingo_nums.push(str.parse::<u32>().unwrap());
            }
            continue;
        }

        // parse bingo board row
        for (_, str) in raw_row_str.split_ascii_whitespace().enumerate() {
            board.nums.push(str.parse::<u32>().unwrap());
        }
    }
    boards.push(board);

    // play the bingo numbers until we get a winner
    // let mut last: u32 = 0; 
    let mut set: HashSet<u32> = HashSet::new();

    for (_,bingo_num) in bingo_nums.iter().enumerate() {
        // foreach board
        for (_, board) in boards.iter_mut().enumerate() {
            let r = play_number(*bingo_num, board);
            if r.product != 0 {
                if !set.contains(&board.id) {
                    set.insert(board.id);
                    println!("[{}] bingo_num={}, last={} * sum={} = product={}", board.id, bingo_num, r.last, r.sum, r.product);
                }
            }
        }
    }
}


fn play_number(bingo_num: u32, board: &mut Board) -> BoardResult {
    for (pos, board_num) in board.nums.iter().enumerate() {
        if bingo_num == *board_num {
            board.mask = board.mask | (1 << pos);

            if check(board.mask) != 0 {
                // now add up the unmatched values on the winning board
                let mut sum = 0;
                for (idx, board_num) in board.nums.iter().enumerate() {
                    if (board.mask & (1 << idx)) == 0 {
                        sum += board_num;
                    }
                }

                return BoardResult {
                    last: bingo_num,
                    product: sum*bingo_num,
                    sum: sum
                }
            }
        }
    }

    return BoardResult { last: 0, product: 0, sum: 0 }
}
