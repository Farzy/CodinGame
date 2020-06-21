use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 * ---
 * Hint: You can use the debug stream to print initialTX and initialTY, if Thor seems not follow your orders.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let light_x = parse_input!(inputs[0], i32); // the X position of the light of power
    let light_y = parse_input!(inputs[1], i32); // the Y position of the light of power
    let mut initial_tx = parse_input!(inputs[2], i32); // Thor's starting X position
    let mut initial_ty = parse_input!(inputs[3], i32); // Thor's starting Y position

    let dir_x = vec!["W", "", "E"];
    let dir_y = vec!["N", "", "S"];

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let remaining_turns = parse_input!(input_line, i32); // The remaining amount of turns Thor can move. Do not remove this line.

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        let dx = (light_x - initial_tx).signum();
        let dy = (light_y - initial_ty).signum();
        if dx != 0 || dy != 0 {
            println!("{}{}", dir_y[(dy + 1) as usize], dir_x[(dx + 1) as usize]);
            initial_tx += dx;
            initial_ty += dy;
        }
    }
}
