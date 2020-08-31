use std::{fs::File, io, path::Path, collections::HashSet};
use io::BufRead;

fn main() {
    let mut counter = 0;

    let lines = read_lines("input")
        .expect("Read file error")
        .map(|x| x.expect("Read line error"));

    for mut row in lines {
        // let mut check = false;
        let mut spin = true;
        let mut aba_set: HashSet<String> = HashSet::new();
        let mut bab_set: HashSet<String> = HashSet::new();
        loop {
            let splits: Vec<&str> = row.splitn(2, |c| c == '[' || c == ']').collect();
            // if check_row(splits[0]) {
            //     if !spin { 
            //         check = false;
            //         break;
            //     }
            //     check = true;
            // }
            if spin {
                process_row(splits[0], &mut aba_set, true);
            } else {
                process_row(splits[0], &mut bab_set, false);
            }

            if splits.len() == 1 { break; }
            row = splits[1].to_string();
            spin = !spin;
        }

        let mut check = false;
        for aba in aba_set {
            for bab in bab_set.clone() {
                if aba == bab {                    
                    check = true;
                    break;
                }
            }
            if check { break; }
        }
        if check { counter = counter + 1; }
    }

    println!("{}", counter);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_row(row: &str) -> bool {
    let (window, sub_str) = row.split_at(3);
    let mut window: Vec<char> = window.chars().collect();

    for c in sub_str.chars() {
        window.push(c);
        if window[0] != window [1] && window[1] == window[2] && window[0] == window[3] {
            return true;
        }
        window.remove(0);
    }

    false
}

fn process_row(row: &str, set: &mut HashSet<String>, spin: bool) {
    let (window, sub_str) = row.split_at(2);
    let mut window: Vec<char> = window.chars().collect();

    for c in sub_str.chars() {
        window.push(c);
        if window[0] != window[1] && window[0] == window[2] {
            let mut s: String = window[0].to_string();
            if spin {
                s.push(window[1]);
            } else {
                s.insert(0, window[1]);
            }
            
            if !set.contains(&s) { set.insert(s); }
        }
        window.remove(0);
    }
}
