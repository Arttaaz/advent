// use std::io::prelude::*;
use std::io::Seek;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::io::SeekFrom;

fn count_times(mut file : &File, times : u32) -> u64 {
    file.seek(SeekFrom::Start(0));
    let buffer = BufReader::new(file);
    let mut seen_chars : HashMap<char, u32> = HashMap::new();
    let mut word_sum : u64 = 0;

    for line in buffer.lines() {
        let line = line.unwrap();
        for letter in line.chars() {
            if seen_chars.contains_key(&letter) {
                *seen_chars.get_mut(&letter).unwrap() += 1;
            }
            else {
                seen_chars.insert(letter, 1);
            }
        }
        for value in seen_chars.values() {
            if *value == times {
                word_sum += 1;
                break;
            }
        }
        seen_chars.clear();
    }
    word_sum
}

fn part_two(mut file : &File) -> String {
    file.seek(SeekFrom::Start(0));
    let buffer = BufReader::new(file);

    let mut words = Vec::new();
    for line in buffer.lines() {
        words.push(line.unwrap());
    }

    let mut diff = Vec::new();
    for i in 0..words.len() {
        let line_bytes = &words[i].as_bytes();
        for line_2 in &words {
            diff.clear();
            let line_2 = line_2.as_bytes();
            if *line_bytes != line_2 {
                for j in 0..line_bytes.len() {
                    if line_bytes[j] != line_2[j] {
                        diff.push(j);
                    }
                }
            }
            if diff.len() == 1 {
                let mut result : String = words[i][..(diff[0])].to_string();
                result += &words[i][(diff[0]+1)..].to_string();
                return result;
            }
        }
    }
    "".to_owned()
}

fn main() {
    let f = File::open("input").expect("file not found");
    let mut checksum : u64 = count_times(&f, 2);
    checksum *= count_times(&f, 3);

    println!("checksum : {}", checksum);
    println!("letters in commun : {}", part_two(&f));
}
