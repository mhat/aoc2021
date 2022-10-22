use std::{collections::{HashMap}, fs::File, io::BufRead, io::BufReader};

fn main() {

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut signal: Vec<Vec<String>> = Vec::new();
    let mut output: Vec<Vec<String>> = Vec::new();

    for(_, raw_row) in reader.lines().enumerate() {
        let mut srow: Vec<String> = Vec::new();
        let mut orow: Vec<String> = Vec::new();
        let mut pipe = false;

        let row = raw_row.unwrap();
        for (_, val) in row.split_ascii_whitespace().enumerate() {
            if val == "|" {
                pipe = true;
            }

            if !pipe {
                srow.push(val.to_string());
            } else {
                orow.push(val.to_string());
            }
        }
        signal.push(srow);
        output.push(orow);
    }

    // 0 - abc efg - 6 - three
    // 1 -   c  f  - 2 - unique
    // 2 - a cde g - 5 - three
    // 3 - a cd fg - 5 - three
    // 4 - b cd f  - 4 - unique
    // 5 - ab d fg - 5 - three
    // 6 - ab defg - 6 - three
    // 7 - a c  f  - 3 - unique
    // 8 - abcdefg - 8 - unique
    // 9 - abcd fg - 6 - three

    // PART 1

    // length -> number
    let mut mapping: HashMap<u32,u32> = HashMap::new();
    mapping.insert(2, 1);
    mapping.insert(4, 4);
    mapping.insert(3, 7);
    mapping.insert(7, 8);

    let mut sum = 0;
    // In the output values, how many times do digits 1, 4, 7, or 8 appear?
    for (_, row) in output.iter().enumerate() {
        for (_, val) in row.iter().enumerate() {
            let len = val.len() as u32;
            if mapping.contains_key(&len) {
                println!("{} with len {} must be number {}", val, len, mapping.get(&len).unwrap());
                sum += 1;
            } else {
                // println!("{} with len {} is tricky", val, len);
            }
        }
    }
    println!("occurrences={}", sum);

    // PART 2 
    // We can figure out what the remaining values must be! 
    // one has two values
    // four has four
    // seven has three 
    // eight has seven
    // 
    // we don't really learn much from eight
    // but 1, 4 and 7 are more useful.
    // 1 tells us what the cc and ff have to be
    // 4 tells us where bb and dddd are as well as cc and ff but we know those from (1)
    // 7 gives us aa, cc and ff but again (1)
    //
    // so by looking at those carefully we have fixes on aa, bb, cc, dd and ff
    // with those we can identify 9 which gives us gg
    // with those we can identify 0 which gives us ee
    // "" 
    // -1--4--7-- abcd f
    // -1--4--789 abcd fg
    // 01--4--789 abcdefg
    // from here all numbers are solvable
    // 
    // but we need some way to encode all of this nonsense
    // positions a b c d e f g
    // and based on what is set yields a number 0..9
    // have a mask for each 0b0000000
    // have a mapping from letter to 0b position
}
