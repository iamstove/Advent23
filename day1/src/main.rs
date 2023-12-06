use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut total = 0;
    let lines = read_lines("./input.txt");

    match lines {
        Ok(lines) => {
            for line in lines {
                //println!("{}", line.unwrap());
                total += parse_string(&line.unwrap());
            }
        },
        Err(e) => println!("File Error: {}", e),
    }

    
    println!("{}", total);
}


fn parse_string(str: &String) -> u32{
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