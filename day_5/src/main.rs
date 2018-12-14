    use std::io::BufRead;
use std::io::Seek;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;

fn main() {
    let f = File::open("input").expect("file not found");
    let mut chars = Vec::new();
    let mut pred_char : u8 = 0;
    let mut number_elem = 0;
    let mut reacted = true;



    let mut buffer = BufReader::new(&f);
    buffer.read_to_end(&mut chars).unwrap();

    let mut i = 0;
    while reacted {
        reacted = false;
        pred_char = 32;
        while i != chars.len() {
            let byte = chars[i];
            if (byte == (pred_char + 32)) || (byte == (pred_char - 32)) {
                chars.remove(i-1);
                chars.remove(i-1);
                reacted = true;
                i -= 1;
                if i > 0 {
                    pred_char = chars[i-1];
                } else {
                    pred_char = 32;
                }
            } else {
                pred_char = byte;
                i += 1;
            }
        }
    }
    println!("Number of elements: {}", chars.len());

    chars.clear();

    for c in 65..91 {
        buffer.seek(SeekFrom::Start(0)).unwrap();
        buffer.read_to_end(&mut chars).unwrap();
        i = 0;
        while i != chars.len() {
            if chars[i] == c || chars[i] == c + 32 {
                chars.remove(i);
            } else {
                i += 1;
            }
        }
        i = 0;
        reacted = true;
        while reacted {
            reacted = false;
            pred_char = 32;
            while i != chars.len() {
                let byte = chars[i];
                if (byte == (pred_char + 32)) || (byte == (pred_char - 32)) {
                    chars.remove(i-1);
                    chars.remove(i-1);
                    reacted = true;
                    i -= 1;
                    if i > 0 {
                        pred_char = chars[i-1];
                    } else {
                        pred_char = 32;
                    }
                } else {
                    pred_char = byte;
                    i += 1;
                }
            }
        }
        println!("lettre : {}, taille : {}", c as char, chars.len());
        chars.clear();
    }
}
