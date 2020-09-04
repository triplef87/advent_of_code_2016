use std::{io, fs::File};
use io::BufRead;

fn main() {
    let file = File::open("input").expect("Open file error");
    let mut input = String::new(); 
    
    io::BufReader::new(file).lines()
        .map(|x| x.expect("Read line error!"))
        .for_each(|row| input.push_str(&row));
    
    // part 1
    // let mut total = 0;
    // while input.len() > 0 {
    //     let splits: Vec<&str> = input.splitn(4, |c| c == '(' || c == ')' || c == 'x').collect();
    //     // println!("{:?}", splits);
    //     total = total + splits[0].len();

    //     let sub_len: usize = splits[1].parse().unwrap();
    //     let repeat: usize = splits[2].parse().unwrap();
    //     total = total + sub_len * repeat;

    //     let (_, next) = splits[3].split_at(sub_len);
    //     input = next.to_string();
    // }

    // part 2
    println!("{}", get_decompress_len(input));            
}

fn get_decompress_len(mut input: String) -> usize {
    let mut total = 0;
    while input.len() > 0 {
        let splits: Vec<&str> = input.splitn(4, |c| c == '(' || c == ')' || c == 'x').collect();
        // println!("{:?}", splits);
        total = total + splits[0].len();

        if splits.len() == 1 { break; }

        let sub_len: usize = splits[1].parse().unwrap();
        let repeat: usize = splits[2].parse().unwrap();

        let (compress, next) = splits[3].split_at(sub_len);
        total = total + repeat * get_decompress_len(compress.to_string());
        input = next.to_string();
    }

    total
}