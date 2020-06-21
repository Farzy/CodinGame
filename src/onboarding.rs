use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * CodinGame planet is being attacked by slimy insectoid aliens.
 * <---
 * Hint:To protect the planet, you can implement the pseudo-code provided in the statement, below the player.
 **/
fn main() {
    let mut input_line = String::new();

    // game loop
    loop {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let enemy_1 = input_line.trim().to_string(); // name of enemy 1
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let dist_1 = parse_input!(input_line, i32); // distance to enemy 1
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let enemy_2 = input_line.trim().to_string(); // name of enemy 2
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let dist_2 = parse_input!(input_line, i32); // distance to enemy 2

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");


        // You have to output a correct ship name to shoot ("Buzz", enemy1, enemy2, ...)
        if dist_1 < dist_2 {
            println!("{}", enemy_1);
        } else {
            println!("{}", enemy_2);
        }
    }
}
