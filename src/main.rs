use std::io::{self, Write};

fn read (input: &mut String) {
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(input)
        .expect("Failed to read from stdin");
}

fn main() {
    let mut rows_num = String::new();
    let mut character = String::from("*");

    print!("How many rows do you want? ");
    read(&mut rows_num);

    let rows_num:u8 = rows_num.trim().parse().expect("Not a valid number");
    println!("{} is the number of rows to print\n", rows_num);

    for _ in 1..=rows_num {
        println!("{}", character);
        character.push_str("*");
    }
}