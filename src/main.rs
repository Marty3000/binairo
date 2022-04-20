/// wrapper to illustrate the usage of board and pretty_print module
use std::time::Instant;

mod board;
mod init;
mod pretty_print;

fn main() {
    let (abc, num, width) = init::get_args();
    if width == 0 {
        println!("Configuration file seems to be invalid (no NxN valid input board). Exiting ..");
        return;
    }
    println!("  Starting for: {}", pretty_print::pprint(&abc, width));

    let now = Instant::now();
    let mut fld = board::Board::new(width);
    fld.init(&abc);

    let sols = fld.solve(num);
    let elapsed = now.elapsed();
    println!("  Found {} Solution(s):", sols.len());
    for sol in sols {
        println!("{}", pretty_print::pprint(&sol, width));
    }
    println!("  Time: {} sec\n", elapsed.as_secs_f64());
}
