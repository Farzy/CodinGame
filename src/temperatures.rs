use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let _ = parse_input!(input_line, i32); // the number of temperatures to analyse
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();

    let mut temp_min: i32 = -10000;

    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        if t.abs() < temp_min.abs() || (t >0 && t + temp_min == 0) {
            temp_min = t;
        }
    }
    if temp_min == -10000 {
        temp_min = 0;
    }
    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");

    println!("{}", temp_min);
}
