use std::env;
use backtrace::print_function_information;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ./test BINARY_NAME ADDRESS");
        return;
    }

    // Binary name
    let binary_name = &args[1];
    // TODO get the function name corresponding to an address
    let _address = &args[2];

    print_function_information(binary_name).unwrap();
}