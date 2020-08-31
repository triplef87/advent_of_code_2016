use std::{fs::File, io, path::Path};
use io::BufRead;

fn main() {
    let mut count = 0;

    // Part 2
    let mut triangles: Vec<Vec<usize>> = vec![Vec::new(), Vec::new(), Vec::new()];
    read_file("input").expect("Read file error")
        .map(|x| x.expect("Read line error"))
        .for_each(|row| {
            let row_data: Vec<usize> = row.trim().split_whitespace()
                .map(|s| s.parse::<usize>().expect("Parse error"))
                .collect();
            
            triangles.iter_mut().enumerate().for_each(|(i, triangle)| triangle.push(row_data[i]));
            if triangles[0].len() == 3 {
                triangles.iter_mut().for_each(|triangle| {
                    if check(triangle) { count = count + 1; }
                    triangle.clear();
                });
            }
        });
    
    println!("{}", count);
}

fn read_file<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

fn check(triangle: &mut Vec<usize>) -> bool {
    let sum = triangle.iter().clone().fold(0, |acc, x| acc + x);
    let limit = sum / 2 + sum % 2;

    !triangle.iter().any(|&x| x >= limit)
}
