use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    let filepath: &str = &args[1];

    println!("Arg1: {}", filepath);
    dbg!(args);
    println!("Hello, world!");
}
