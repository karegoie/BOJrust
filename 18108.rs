use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Input failure raised");

    let input: i32 = input.trim().parse().expect("Type converting failed");

    let bulgi: i32 = input - 543;

    println!("{}", bulgi);
}
