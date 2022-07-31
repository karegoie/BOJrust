use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Input faiure raised");
    
    let split_string: Vec<&str> = input.split(" ").collect();
    
    let a = &split_string[0];
    let b = &split_string[1];
    
    let a: i32 = a.trim().parse()
        .expect("Type casting failure");
    let b: i32 = b.trim().parse()
        .expect("Type casting failure");
    
    let tup: (i32, i32) = (a, b);
        
    println!("{}", answer(tup));
}

fn answer(ns : (i32, i32)) -> &'static str {
    if ns.0 < ns.1 {
    "<"
    }
    else if ns.0 == ns.1 {
    "=="
    }
    else {
    ">"
    }
}
