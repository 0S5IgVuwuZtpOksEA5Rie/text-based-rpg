use std::io;

fn intro_scene() {
    println!("Hello player, welcome to the game");

    let user_name: String = input("What is your name?");
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut output: String = String::new();

    io::stdin().read_line(&mut output).expect("read line error");

    return output;
}

fn main() {
    intro_scene();
}
