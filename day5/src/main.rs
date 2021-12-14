use std::{fs::File, io::BufRead, io::BufReader, cmp};

fn calc(grid: &mut Vec<Vec<u8>>, part: i8) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for(_, raw_row) in reader.lines().enumerate() {
        // 0,9 -> 2,9
        let raw_row = raw_row.unwrap();
        let points: Vec<&str> = raw_row.split(" -> ").collect();
        let p1: Vec<&str> = points[0].split(",").collect();
        let p2: Vec<&str> = points[1].split(",").collect();

        let x1 = p1[0].parse::<usize>().unwrap();
        let y1 = p1[1].parse::<usize>().unwrap();
        let x2 = p2[0].parse::<usize>().unwrap();
        let y2 = p2[1].parse::<usize>().unwrap();

        if x1 == x2 || y1 == y2 { 
            // 90 degree
            if y1 == y2 {
                // y1==y2==horizontal
                let row2 = &mut grid[y1];
                for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                    // println!("X {},{} -> {},{}: {},{}",x1,y1,x2,y2,x,y1);
                    row2[x] += 1;
                }
            } else {
                // x1==x2==vertical
                // ensure that range y1..y2 is positive
                for y in cmp::min(y1,y2)..=cmp::max(y1,y2) {
                    let row2 = &mut grid[y];
                    // println!("X {},{} -> {},{}: {},{}",x1,y1,x2,y2,x1,y);
                    row2[x1] += 1;
                }
            }
        } else {
            if part == 2 {
                let steps = cmp::max(y2,y1) - cmp::min(y2, y1);
                let mut y = y1;
                let mut x = x1;
                for s in 0..=steps {
                    grid[y][x] += 1;
                    // println!("s={}/{}, x1={},y1={} | x2={},y2={} | x={},y={}", s,steps,x1,y1,x2,y2,x,y);

                    if s < steps {
                        if x2 > x1 {
                            x = x + 1;
                        } else {
                            x = x - 1;
                        }
                        if y2 > y1 {
                            y = y + 1;
                        } else {
                            y = y - 1;
                        }
                    }
                }
            }
        }
    }
}

fn main() {

    // init grid
    let mut grid1: Vec<Vec<u8>> = Vec::new();
    let mut grid2: Vec<Vec<u8>> = Vec::new();

    for _ in 0..1000 {
        grid1.push(vec![0; 1000]);
        grid2.push(vec![0; 1000]);
    }

    calc(&mut grid1, 1);
    let mut c = 0;
    for (_,r) in grid1.iter().enumerate() {
        for (_, p) in r.iter().enumerate() {
            if *p >= 2 {
                c += 1;
            }
        }
    }
    println!("part1={}", c);

    calc(&mut grid2, 2);
    let mut c = 0;
    for (_,r) in grid2.iter().enumerate() {
        for (_, p) in r.iter().enumerate() {
            if *p >= 2 {
                c += 1;
            }
        }
    }
    println!("part2={}", c);
}
