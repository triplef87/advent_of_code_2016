use std::{fs::File, io, path::Path, collections::HashMap};
use io::BufRead;

fn main() {
    let mut map_list: Vec<HashMap<char, usize>> = Vec::new(); 

    read_lines("input").expect("Read file error")
        .map(|x| x.expect("Read line error"))
        .for_each(|row| {
            if map_list.len() == 0 {
                for _ in 0..row.len() {
                    map_list.push(HashMap::new());
                }
            }

            row.chars().enumerate().for_each(|(i,c)| {
                let entry = map_list[i].entry(c).or_insert(1);
                *entry = *entry + 1;
            });
        });

    let mut message = String::new();
    map_list.iter().for_each(|map| {
        let max = map
            .iter()
            .min_by(|x, y| x.1.cmp(y.1))
            .unwrap();
        message.push(*max.0);
    });
    println!("{}", message);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}
