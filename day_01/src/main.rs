use std::collections::HashSet;

fn main() {
    let instrution_raw = "L4, L1, R4, R1, R1, L3, R5, L5, L2, L3, R2, R1, L4, R5, R4, L2, R1, R3, L5, R1, L3, L2, R5, L4, L5, R1, R2, L1, R5, L3, R2, R2, L1, R5, R2, L1, L1, R2, L1, R1, L2, L2, R4, R3, R2, L3, L188, L3, R2, R54, R1, R1, L2, L4, L3, L2, R3, L1, L1, R3, R5, L1, R5, L1, L1, R2, R4, R4, L5, L4, L1, R2, R4, R5, L2, L3, R5, L5, R1, R5, L2, R4, L2, L1, R4, R3, R4, L4, R3, L4, R78, R2, L3, R188, R2, R3, L2, R2, R3, R1, R5, R1, L1, L1, R4, R2, R1, R5, L1, R4, L4, R2, R5, L2, L5, R4, L3, L2, R1, R1, L5, L4, R1, L5, L1, L5, L1, L4, L3, L5, R4, R5, R2, L5, R5, R5, R4, R2, L1, L2, R3, R5, R5, R5, L2, L1, R4, R3, R1, L4, L2, L3, R2, L3, L5, L2, L2, L1, L2, R5, L2, L2, L3, L1, R1, L4, R2, L4, R3, R5, R3, R4, R1, R5, L3, L5, L5, L3, L2, L1, R3, L4, R3, R2, L1, R3, R1, L2, R4, L3, L3, L3, L1, L2";    
    let instructions: Vec<&str> = instrution_raw.split(", ").collect();

    let mut player = Player{direction: 0, x: 0, y: 0};
    let mut record: HashSet<(isize, isize)> = HashSet::new();
    record.insert((0, 0));

    for x in instructions {
        let (direct, distance_row) = x.split_at(1);
        let distance: usize = distance_row.parse().expect("Parse error");
        
        player.turn(direct);
        if player.walk(distance, &mut record) { break; }
    }

    println!("{}", player.x.abs() + player.y.abs());
}

struct Player {
    direction: usize,
    x: isize,
    y: isize
}

impl Player {
    fn turn(&mut self, direct: &str) {
        if direct == "L" {
            self.direction = if self.direction == 0 { 3 } else { self.direction - 1 }
        } else {
            self.direction = if self.direction == 3 { 0 } else { self.direction + 1 }
        }
    }

    fn walk(&mut self, distance: usize, record: &mut HashSet<(isize, isize)>) -> bool {
        let carry = if self.direction < 2 { 1 } else { -1 };
        for _ in 0..distance {
            if self.direction == 0 || self.direction == 2 {
                self.y = self.y + carry;
            } else {
                self.x = self.x + carry;
            }
            if record.contains(&(self.x, self.y)) { return true; }
            record.insert((self.x, self.y));
        }

        false
    }
}