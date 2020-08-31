use std::{io, path::Path, fs::File, collections::HashMap};
use io::BufRead;

fn main() {
    let mut ans = 0;
    read_lines("input").expect("Read file error")
        .map(|x| x.expect("Read line error"))
        .for_each(|row| {
            ans = ans + process_row(&row);
        });
    println!("{}", ans);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_row(row: &String) -> usize {
    let mut splits: Vec<&str> = row.split("-").collect();
    
    let mut tmp = splits.pop().expect("Pop split vec error");
    tmp = tmp.strip_suffix("]").expect("Strip error");
    
    let id_and_checksum: Vec<&str> = tmp.split("[").collect();
    let id: usize = id_and_checksum[0].parse().expect("Parse id error");

    let mut map: HashMap<char, usize> = HashMap::new();

    splits.iter().for_each(|split| {
        split.chars().for_each(|c| {
            let counter = map.entry(c).or_insert(1);
            *counter = *counter + 1;
        });
    });

    let mut value_array: Vec<usize> = map.values().map(|x| *x).collect();
    for idx in 0..5 {
        let mut max = 0;
        let mut idx_max = 0;
        value_array.iter().skip(idx).enumerate().for_each(|(i, x)| {
            if *x > max {
                max = *x;
                idx_max = i;
            }
        });
        value_array.swap(idx, idx + idx_max);
    }

    let limit = value_array[4];

    let mut check = true;
    let checksum: Vec<(char, usize)> = id_and_checksum[1].chars().map(|c| {
        match map.get(&c) {
            Some(&val) => (c, val),
            None => { check = false; (c, 0) }
        }
    }).collect();

    if !check { return 0; }

    for idx in 0..4 {
        if checksum[idx].1 < limit ||
            (checksum[idx].1 == checksum[idx + 1].1 && checksum[idx].0 > checksum[idx + 1].0) {
            return 0;
        }
    }

    let mut realname = String::new();

    splits.iter().for_each(|split| {
        split.chars().for_each(|c| {
            let c_idx = c as u8 - 97;
            let offset = (id % 26) as u8;

            if c_idx + offset < 26 {
                realname.push((97 + c_idx + offset) as char);
            } else {
                realname.push((97 + c_idx + offset - 26) as char);
            }
        });
        realname.push(' ');
    });
    println!("{}", realname);

    if realname == "northpole object storage " {
        id
    } else {
        0
    }
}
