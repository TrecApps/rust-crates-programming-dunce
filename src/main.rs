extern crate passwords;

use passwords::hasher;

fn u8_to_hex(input: u8) -> Option<char>
{
    match input
    {
        0b0000 => Some('0'),
        0b0001 => Some('1'),
        0b0010 => Some('2'),
        0b0011 => Some('3'),
        0b0100 => Some('4'),
        0b0101 => Some('5'),
        0b0110 => Some('6'),
        0b0111 => Some('7'),
        0b1000 => Some('8'),
        0b1001 => Some('9'),
        0b1010 => Some('A'),
        0b1011 => Some('B'),
        0b1100 => Some('C'),
        0b1101 => Some('D'),
        0b1110 => Some('E'),
        0b1111 => Some('F'),

        _ => None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2
    {
        println!("Error! Needed second argument to demonstrate BCrypt Hash!");
        return;
    }

    let password = args.get(1).expect("Expected second argument to exist!").trim();

    let hash_res = hasher::bcrypt(10, "This_is_salt", password);

    match hash_res
    {
        Err(_) => {println!("Failed to generate a hash!");},
        Ok(hash) => { 
            let str_hash = String::from_utf8_lossy(&hash);

            let mut hex_str = String::from("");

            for bin in str_hash.bytes()
            {
                let bin_one = (bin & 0b11110000) >> 4;
                let bin_two = bin & 0b00001111;

                hex_str.push(u8_to_hex(bin_one).unwrap());
                hex_str.push(u8_to_hex(bin_two).unwrap());
            }

            println!("Hash generated from password {} is {}", password, hex_str);
        }
    }
}
