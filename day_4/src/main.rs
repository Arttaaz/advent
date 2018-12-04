extern crate regex;
extern crate chrono;

use chrono::Timelike;
use std::cmp::Ordering;
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use regex::Regex;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
#[derive(Clone)]
enum Action {
    NewShift,
    Asleep,
    WakesUp
}
#[derive(Eq)]
#[derive(Debug)]
struct Event {
    date    : DateTime<Utc>,
    event   : Action,
    id      : u64
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other : &Event) -> bool {
        self.date == other.date
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
struct Guard {
    id      : u64,
    asleep  : HashMap<u32, u32>,
}

impl Guard {
    fn sum(&self) -> u32 {
        let mut sum : u32 = 0;
        for minutes in self.asleep.values() {
            sum += minutes;
        }
        sum
    }

    fn max_freq(&self) -> (u32, u32) {
        let mut max = 0;
        let mut the_minute = 0;
        for (minute, occ) in &self.asleep {
            if *occ > max {
                the_minute = *minute;
                max = *occ;
            }
        }
        (the_minute, max)
    }
}

impl Ord for Guard {
    fn cmp(&self, other: &Guard) -> Ordering {
        self.sum().cmp(&other.sum())
    }
}

impl PartialOrd for Guard {
    fn partial_cmp(&self, other: &Guard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let f = File::open("input").expect("file not found");
    let buffer = BufReader::new(f);
    let mut events = Vec::new();
    let mut guards = HashMap::new();

    for line in buffer.lines() {
        let line = line.unwrap();
        let reg = Regex::new(r"\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\] (Guard #[0-9]+ begins shift|falls asleep|wakes up)").unwrap();
        for cap in reg.captures_iter(&line) {
            let date = Utc.datetime_from_str(&cap[1], "%F %R").unwrap();
            let action : Action;
            let mut id : u64;
            match &cap[2] {
                "falls asleep"  => action = Action::Asleep,
                "wakes up"      => action = Action::WakesUp,
                _               => action = Action::NewShift
            }
            let mut event = Event { date: date, event: action.clone(), id: 0};
            if action == Action::NewShift {
                let reg_2 = Regex::new(r"#(\d+)").unwrap();
                for result in reg_2.captures_iter(&cap[2]) {
                    id = result[1].parse::<u64>().unwrap();
                    event.id = id;
                    let guard = Guard { id: id, asleep: HashMap::new()};
                    if !guards.contains_key(&id) {
                        guards.insert(id, guard);
                    }
                }
            }
            events.push(event);
        }
    }
    events.sort();

    let mut current_id : u64 = 0;
    let mut minute_asleep : u32 = 0;
    let mut minute_awake : u32 = 0;
    for event in events {
        if event.event == Action::NewShift {
            current_id = event.id;
        } else if event.event == Action::Asleep {
            minute_asleep = event.date.minute();
        } else if event.event == Action::WakesUp {
            minute_awake = event.date.minute();
            let mut guard = guards.get_mut(&current_id).unwrap();
            for i in minute_asleep..minute_awake {
                if guard.asleep.contains_key(&i) {
                    *guard.asleep.get_mut(&i).unwrap() += 1;
                } else {
                    guard.asleep.insert(i, 1);
                }
            }
        }
    }
    let mut max = &Guard { id: 0, asleep: HashMap::new()};
    for guard in guards.values() {
        if guard > max {
            max = guard;
        }
    }
    let mut the_minute = 0;
    let mut max_occ = 0;
    for (minute, occ) in &max.asleep {
        if *occ > max_occ {
            max_occ = *occ;
            the_minute = *minute;
        } else if *occ == max_occ {
             if *minute < the_minute {
                 the_minute = *minute;
             }
        }
    }
    println!("Result : {}", max.id * the_minute as u64);

    for guard in guards.values() {
        let freq = guard.max_freq();
        if freq.1 > max.max_freq().1 {
            max = guard;
            the_minute = freq.0;
        }
    }
    println!("Second part : {}", max.id * the_minute as u64);
}
