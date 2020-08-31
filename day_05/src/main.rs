fn main() {
    let mut password = vec!['_'; 8];
    let mut counter = 0;

    let id = String::from("ojvtpuvg");
    let mut index = 0;

    while counter < 8 {
        let mut input = id.clone();
        input.push_str(&index.to_string());

        let digest = format!("{:x}", md5::compute(input.as_bytes()));
        
        if digest.starts_with("00000") {
            println!("{}, {}", digest, index);
            let mut iter = digest.chars();
            let pos = iter.nth(5).unwrap();
            if pos > '7' {
                index = index + 1;
                continue;
            }
            
            let pos = pos as usize - '0' as usize;
            let val = iter.nth(0).unwrap();

            if password[pos] == '_' {
                password[pos] = val;
                let s: String = password.iter().collect();
                println!("{}", s);
                counter = counter + 1;
            }
        }

        index = index + 1;
    }

    println!("{:?}", password);
}
