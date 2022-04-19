use std::env;
use std::fs;

pub fn get_args() -> (String, usize, usize) {
    let (inp_file, num) = read_args();
    if inp_file.is_empty() {
        println!("Please provide path to inputfile as argument.");
        return (inp_file, num, 0);
    } else {
        println!("  Will read input file: {}", inp_file);
    }

    let contents = fs::read_to_string(inp_file).expect("Something went wrong reading the file");

    let allowed_chars = ['.', '0', '1'];
    let abc: String = contents
        .chars()
        .filter(|x| allowed_chars.contains(x))
        .collect();
    let width = int_sqrt(abc.len());
    (abc, num, width)
}

fn int_sqrt(num: usize) -> usize {
    let mut sqrt: usize = 0;
    while sqrt * sqrt < num {
        sqrt += 1;
    }
    if sqrt * sqrt == num {
        return sqrt;
    }
    0
}

fn read_args() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    let mut fname: String = String::new();
    let mut num: usize = 1;

    match args.len() {
        1 => {}
        2 => {
            fname = args[1].parse().unwrap_or_default();
        }
        _ => {
            fname = args[1].parse().unwrap_or_default();
            num = args[2].parse().unwrap_or(1);
        }
    }
    (fname, num)
}
