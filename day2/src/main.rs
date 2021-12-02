use std::{fs::File, io::BufRead, io::BufReader};

fn part1() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut hoz = 0;
    let mut dep = 0;

    for(_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        let dir = tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();

        match dir {
            "forward" => hoz += val,
            "down" => dep += val,
            "up" => dep -= val,
            _ => println!("*****")
        }
    }
    println!("hoz={} dep={} mul={}", hoz, dep, hoz*dep);
}

fn part2() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut hoz = 0;
    let mut dep = 0;
    let mut aim = 0;

    for(_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        let dir = tokens.next().unwrap();
        let val = tokens.next().unwrap().parse::<i32>().unwrap();

        if dir == "forward" {
            hoz += val;
            dep  = dep + (aim * val);
        } else if dir == "down" {
            aim += val;
        } else if dir == "up" {
            aim -= val;
        } else { 
            println!("*****");
        }
        println!("val={}, dir={}, h={}, a={}, d={}", val, dir, hoz, aim, dep);

    }
    println!("hoz={} dep={} mul={}", hoz, dep, hoz*dep);
}

fn main() {
    part1();
    part2();
}