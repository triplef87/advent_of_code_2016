use std::{fs::File, io, collections::HashMap};

use io::BufRead;
#[derive(Debug, Copy, Clone)]
struct Bot {
    low: usize,
    high: usize
}

#[derive(Debug, Copy, Clone)]
struct Instruct {
    low: (usize, usize),
    high: (usize, usize)
}

impl Bot {
    fn add_val(&mut self, val: usize) {
        if self.low == 0 {
            self.low = val;
        } else if self.low > val {
            self.high = self.low;
            self.low = val;
        } else {
            self.high = val;
        }
    }
}

fn main() {
    let file = File::open("input").expect("Open file error!");
    
    let mut bots: Vec<Bot> = Vec::new();
    let mut outputs: Vec<usize> = Vec::new();
    let mut instructs: HashMap<usize, Instruct> = HashMap::new();

    io::BufReader::new(file).lines()
        .map(|x| x.expect("Read line error!"))
        .for_each(|row| {
            let splits: Vec<&str> = row.split(" ").collect();

            if splits.len() == 6 {
                process_bots(&splits, &mut bots);                
            } else {
                process_instructs(&splits, &mut instructs);
            }
        });

    loop {
        let mut conti = false;

        for (idx,bot) in bots.clone().iter().enumerate() {
            if bot.low > 0 && bot.high > 0 {
                if bot.low == 17 && bot.high == 61 { println!("{}", idx); }

                let instruct = instructs.get(&idx).unwrap();
                if instruct.low.0 == 0 {
                    if instruct.low.1 + 1 > bots.len() {
                        bots.resize(instruct.low.1 + 1, Bot{low: 0, high: 0})
                    }
                    bots[instruct.low.1].add_val(bot.low);
                } else {
                    if instruct.low.1 + 1 > outputs.len() {
                        outputs.resize(instruct.low.1 + 1, 0)
                    }
                    outputs[instruct.low.1] = bot.low;
                }
                bots[idx].low = 0;

                if instruct.high.0 == 0 {
                    if instruct.high.1 + 1 > bots.len() {
                        bots.resize(instruct.high.1 + 1, Bot{low: 0, high: 0})
                    }
                    bots[instruct.high.1].add_val(bot.high);
                } else {
                    if instruct.high.1 + 1 > outputs.len() {
                        outputs.resize(instruct.high.1 + 1, 0)
                    }
                    outputs[instruct.high.1] = bot.high;
                }
                bots[idx].high = 0;

                conti = true;
            }
        }
        if !conti { break; }
    }

    println!("{}", outputs[0] * outputs[1] * outputs[2]);
}

fn process_bots(splits: &Vec<&str>, bots: &mut Vec<Bot>) {
    let val: usize = splits[1].parse().unwrap();
    let num: usize = splits[5].parse().unwrap();

    if num + 1 > bots.len() {
        bots.resize(num + 1, Bot{low: 0, high: 0})
    }

    bots[num].add_val(val);
}

fn process_instructs(splits: &Vec<&str>, instructs: &mut HashMap<usize, Instruct>) {
    let bot: usize = splits[1].parse().unwrap();
    let low_num: usize = splits[6].parse().unwrap();
    let high_num: usize = splits[11].parse().unwrap();

    if instructs.contains_key(&bot) {
        println!("{}", bot);
    }

    let low_type = if splits[5] == "bot" { 0 } else { 1 };
    let high_type = if splits[10] == "bot" { 0 } else { 1 };

    instructs.insert(bot, Instruct { low: (low_type, low_num), high: (high_type, high_num)});
}
