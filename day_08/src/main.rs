use std::{io, fs::File};
use io::BufRead;

fn main() {
    let mut screen = vec![vec![false; 50]; 6];
    let file = File::open("input").expect("Open file error!");

    io::BufReader::new(file)
        .lines()
        .map(|x| x.expect("Read line error!"))
        .for_each(|row| {
            let splits: Vec<&str> = row.split_whitespace().collect();
            if splits[0] == "rect" {
                let rect: Vec<usize> = splits[1].split("x").map(|s| s.parse().unwrap()).collect();

                for x in 0..rect[1] {
                    for y in 0..rect[0] {
                        screen[x][y] = true;
                    }
                }
            } else if splits[1] == "row" {
                let row_idx: Vec<&str> = splits[2].split("y=").collect();
                let row_idx: usize = row_idx[1].parse().unwrap();
                let offset: usize = splits[4].parse().unwrap();

                let mut copy = vec![false; 50];
                for (idx, light) in &mut screen[row_idx].iter().enumerate() {
                    if *light {
                        let new_idx = if idx + offset > 49 { idx + offset - 50 } else { idx + offset };
                        copy[new_idx] = true;
                    }
                }
                screen[row_idx] = copy;
            } else {
                let col_idx: Vec<&str> = splits[2].split("x=").collect();
                let col_idx: usize = col_idx[1].parse().unwrap();
                let offset: usize = splits[4].parse().unwrap();

                let mut copy = vec![false; 6];

                for (idx, row) in screen.iter().enumerate() {
                    if row[col_idx] {
                        let new_idx = if idx + offset > 5 { idx + offset - 6 } else { idx + offset };
                        copy[new_idx] = true;
                    }
                }

                for (idx, light) in copy.iter().enumerate() {
                    screen[idx][col_idx] = *light;
                }
            }
        });

    for row in screen {
        for col in row {
            if col { print!("#"); } else { print!("."); }
        }
        println!();
    }
}