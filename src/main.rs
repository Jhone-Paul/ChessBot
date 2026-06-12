use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "1"{
        println!("tui mode!");
    } else if args[1] == "0"{
        println!("api mode!");
    } else if args[1]== null
}
