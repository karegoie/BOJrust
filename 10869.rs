use std::io;

fn main(){
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Input error raised");
    
    let split_string: Vec<&str> = input.split(" ").collect();
    let input1 = &split_string[0];
    let input2 = &split_string[1];
    
    let input1: i32 = input1.trim().parse()
        .expect("Failed to convert to integar");
    let input2: i32 = input2.trim().parse()
        .expect("Failed to convert to integar");
        
    let sum: i32 = input1 + input2;
    let diff: i32 = input1 - input2;
    let mul: i32 = input1 * input2;
    let div: i32 = (input1 / input2).into();
    let q: i32 = (input1 % input2).into();
    
    println!("{}\n{}\n{}\n{}\n{}", sum, diff, mul, div, q);
}
