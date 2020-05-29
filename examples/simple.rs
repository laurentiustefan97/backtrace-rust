use backtrace_rust::backtrace::Backtrace;

fn tar() {
    println!("{:?}", Backtrace::new());
}

fn bar() {
    tar();
}

fn foo() {
    bar();
}

fn main() {
    foo();
}
