/// helper module to
/// - read the cli arguments
/// - read in the file, defining the start table
/// - and determine the table - width ( = height)
use std::env;
use std::fs;

/// works on user provied arguments
/// public function, returns
/// * start-state as a String (using only characters ., 0, 1)
/// * the number of solutions to look for
/// * the width ( = height ) of the board
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

/// Integer SQRT
/// internal function, returns
/// * the integer SQRT of the provided number
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

/// read the arguments, provided at startup
/// internal function, returns
/// * the input file name ( = first argument, mandatory)
/// * the number of solutions to search for ( = second argument, defaults to 1)
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
