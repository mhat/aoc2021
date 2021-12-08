use std::{fs::File, io::BufRead, io::BufReader};

// src
// 1 1 1 1
// 2 2 2 2
// 3 3 3 3
//
// dst
// 1 2 3 
// 1 2 3
// 1 2 3
// 1 2 3
// 

fn transpose(src: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
  let mut dst: Vec<Vec<u32>> = Vec::new();

  for i in 0..src[0].len() {
    let mut dst_row: Vec<u32> = Vec::new();
    for (_, src_row) in src.iter().enumerate() {
        dst_row.push(src_row[i]);
    }
    dst.push(dst_row);
  }

  return dst
}

fn read_input() -> Vec<Vec<u32>> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut dst: Vec<Vec<u32>> = Vec::new();

    for(_, raw_row) in reader.lines().enumerate() {
        let mut dst_row: Vec<u32> = Vec::new();
        for (_, c) in raw_row.unwrap().char_indices() {
            let c = c.to_digit(10).unwrap();
            dst_row.push(c);
        }
        dst.push(dst_row);
    }

    return dst
}

fn most_common(r: &Vec<u32>, high_if_true: bool) -> (u32) {
    let mut zeros: Vec<u32> = Vec::new();
    let mut ones: Vec<u32> = Vec::new();
    
    for (i, _) in r.iter().enumerate() {
        if r[i] == 0 {
            zeros.push(r[i]);
        } else {
            ones.push(r[i]);
        }
    }

    // println!("0={:?},{} 1={:?},{}", zeros, zeros.len(), ones, ones.len());
    if high_if_true == true {
        if ones.len() >= zeros.len() {
            return 1
        }
        return 0
    }
    else {
        if ones.len() < zeros.len() {
            return 1
        }
        return 0
    }
}

fn vec2int(r: Vec<u32>) -> u32 {
    let mut value: u32 = 0;
    let mut rt = r;
    rt.reverse();
    for (i,v) in rt.iter().enumerate() {
        value = value | v << i;
    }

    return value
}

fn part1() {
    let input = read_input();
    let posed = transpose(&input);

    let mut v_gamma: Vec<u32> = Vec::new();
    let mut v_epislon: Vec<u32> = Vec::new();

    for (_, row) in posed.iter().enumerate() {
        v_gamma.push(most_common(row, true));
        v_epislon.push(most_common(row, false));
    }

    let gamma = vec2int(v_gamma);
    let epislon = vec2int(v_epislon);
    println!("g={} * e={} = {}", gamma, epislon, gamma*epislon);
}

fn reduce(input: &Vec<Vec<u32>>, high_if_true: bool) -> Vec<u32> {
    let val = reducer(input, 0, high_if_true);
    return val[0].clone()
}

fn reducer(input: &Vec<Vec<u32>>, pos: usize, high_if_true: bool) -> Vec<Vec<u32>> {
    let mut vals: Vec<Vec<u32>> = Vec::new(); 
    let transposed: Vec<Vec<u32>> = transpose(input);
    let common = most_common(&transposed[pos], high_if_true);

    for (_, v) in input.iter().enumerate() {
        if v[pos] == common {
            vals.push(v.clone())
        }
    }

    // println!("common={},pos={}, from={} to={} vals={:?}", common, pos, input.len(), vals.len(), vals);
    if vals.len() <= 1 {
        return vals
    }

    let new_pos = pos + 1;
    return reducer(&vals, new_pos, high_if_true);
}

fn part2() {
    let input = read_input();
    let o2 = vec2int(reduce(&input, true));
    let co2 = vec2int(reduce(&input, false));
    println!("o2={} * co2={} = {}", o2, co2, o2*co2);
}

fn main() {
    part1();
    part2();
}
