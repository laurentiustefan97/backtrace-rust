use std::env;
use backtrace::get_function_name;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./test BINARY_NAME ADDRESS");
        return;
    }

    // Binary name
    let binary_name = &args[1];
    // The address wanted for the lookup
    let address: u64 = args[2].parse::<u64>().expect("Please introduce a number for the address!");
    // Getting the function name
    let function_name = get_function_name(binary_name, address).expect("No function was found at that address!");

    println!("The function name is {}!", function_name);
}
