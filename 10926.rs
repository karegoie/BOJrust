use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Wrong input");

    let input: String = input.trim().parse()
        .expect("Trimming failed");

    println!("{}??!", input);
}
