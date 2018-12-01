use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn part_one(freqs : &Vec<i64>) -> i64 {
    let mut freq : i64 = 0;

    for modifier in freqs {
        freq += modifier;
    }

    freq
}

fn part_two(freqs : &Vec<i64>) -> i64 {
    let mut found : bool = false;
    let mut freq  : i64  = 0;
    let mut known_freqs = HashSet::new();

    known_freqs.insert(0);
    while !found {
        for modifier in freqs {
            freq += modifier;
            if known_freqs.contains(&freq) {
                found = true;
                break;
            }
            known_freqs.insert(freq);

        }
    }
    freq
}


fn main() {


    let f = File::open("input").expect("file not found");
    let lines = BufReader::new(&f);
    let mut freqs = Vec::new();

    for line in lines.lines() {
        let modifier = line.unwrap().parse::<i64>();
        freqs.push(modifier.unwrap());
    }

    println!("final freq : {}", part_one(&freqs));
    println!("first freq reached twice : {}", part_two(&freqs));
}
