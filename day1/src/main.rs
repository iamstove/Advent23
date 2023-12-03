use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let total = read_and_calc();
    println!("{}", total);
}


fn read_and_calc () -> u32 {
    let mut ret_val = 0;

    //read file, find first and last number, add to total
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                //println!("{}", ip);
                ret_val += parse_string(ip);
            }
        }
    }

    return ret_val;
}

fn parse_string(str: String) -> u32{
    let mut ret_val;
    let mut ints_in_str: Vec<u32> = Vec::new();

    for char in str.chars() {
        if char.to_digit(10).is_some(){
            let as_int = char.to_digit(10).unwrap();
            ints_in_str.push(as_int);
            //println!("{}", as_int);
        }
    }

    //println!("{:?}", ints_in_str);
    ret_val = ints_in_str.first().unwrap() * 10;
    ret_val += ints_in_str.last().unwrap();

    //println!("{}", ret_val);

    return ret_val;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}