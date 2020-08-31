use std::{fs::File, io, path::Path};
use io::BufRead;

fn main() {
    let numpad: Vec<Vec<char>> = vec![
        vec!['X','X','1','X','X'],
        vec!['X','2','3','4','X'],
        vec!['5','6','7','8','9'],
        vec!['X','A','B','C','X'],
        vec!['X','X','D','X','X']
        ];

    let mut position = Pos { x: 2, y: 0};
    let mut ans = String::from("");

    read_lines("input").expect("Read file error")
        .map(|x| x.expect("Read line error"))
        .for_each(|row| {
            get_num(&row, &mut position, &numpad);            
            ans.push(numpad[position.x][position.y]);
        });

    println!("{}", ans);
}

fn read_lines<P>(arg: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(arg)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone)]
struct Pos {
    x: usize,
    y: usize
}

fn get_num(row: &str, pos: &mut Pos, numpad: &Vec<Vec<char>>) {
    row.chars().for_each(|c| {
        let tmp = pos.clone();
        match c {            
            'L' => {
                pos.y = pos.y.saturating_sub(1);
            },
            'R' => {
                pos.y = if pos.y == 4 { pos.y } else { pos.y + 1 };
            },
            'D' => {
                pos.x = if pos.x == 4 { pos.x } else { pos.x + 1 };
            },
            'U' => {                
                pos.x = pos.x.saturating_sub(1);
            },
            _ => {}
        }
        if numpad[pos.x][pos.y] == 'X' {
            pos.x = tmp.x;
            pos.y = tmp.y;
        }
    });
}