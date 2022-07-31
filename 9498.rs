use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Input failure raised");
    let n: i32 = input.trim().parse()
        .expect("Type casting failure raised");
    
    println!("{}", answer(n));
}

fn answer(n: i32) -> &'static str {
    if 90<=n && n<=100 {
        "A"
    }
    else if 80<=n && n<90 {
        "B"
    }
    else if 70<=n && n<80 {
        "C"
    }
    else if 60<=n && n<70 {
        "D"
    }
    else {
    "F"
    }
}
