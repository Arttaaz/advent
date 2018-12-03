extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

struct Rect {
    id : u64,
    x : u64,
    y : u64,
    width : u64,
    height : u64
}


fn main() {
    let f = File::open("input").expect("file not found");
    let buffer = BufReader::new(&f);
    let mut cases = HashMap::new();
    let mut rectangles = Vec::new();

    for line in buffer.lines() {
        let reg = Regex::new(r"#([0-9]*) @ ([0-9]*),([0-9]*): ([0-9]*)x([0-9]*)").unwrap();
        for cap in reg.captures_iter(&line.unwrap()) {
            let x = cap[2].parse::<u64>().unwrap();
            let y = cap[3].parse::<u64>().unwrap();
            let width = cap[4].parse::<u64>().unwrap();
            let height = cap[5].parse::<u64>().unwrap();
            let rect = Rect { id: cap[1].parse::<u64>().unwrap(), x: x, y: y, width: width, height: height};
            rectangles.push(rect);
            for i in x..(x+width) {
                for j in y..(y+height) {
                    if cases.contains_key(&format!("{},{}", i, j)) {
                        *cases.get_mut(&format!("{},{}", i, j)).unwrap() += 1;
                    }
                    else {
                        cases.insert(format!("{},{}", i, j), 1);
                    }
                }
            }
        }
    }

    let mut collisions : u64 = 0;

    for value in cases.values() {
        if *value >= 2 {
            collisions += 1;
        }
    }

    println!("Number of collisions : {}", collisions);

    let mut claimed = 0;
    for rec in rectangles {
        claimed = 0;
        for i in rec.x..(rec.x+rec.width) {
            for j in rec.y..(rec.y+rec.height) {
                if *cases.get(&format!("{},{}", i, j)).unwrap() != 1 {
                    claimed += 1;
                }
            }
        }
        if claimed == 0 {
            println!("{}", rec.id);
            // break;
        }
    }

}
