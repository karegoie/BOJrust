use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Input failure raised");

    let split_string: Vec<&str> = input.split(" ").collect();
    let n1 = &split_string[0];
    let n2 = &split_string[1];
    let n3 = &split_string[2];
    
    let n1: i32 = n1.trim().parse()
        .expect("Type casting failed");
    let n2: i32 = n2.trim().parse()
        .expect("Type casting failed");
    let n3: i32 = n3.trim().parse()
        .expect("Type casting failed");
    
    let o1 = (n1+n2)%n3;
    let o2 = ((n1%n3) + (n2%n3))%n3;
    let o3 = (n1*n2)%n3;
    let o4 = ((n1%n3) * (n2%n3))%n3;

    println!("{}", o1);
    println!("{}", o2);
    println!("{}", o3);
    println!("{}", o4);
}
