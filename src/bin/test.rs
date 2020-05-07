use backtrace::backtrace::BacktraceGenerator;

fn tar() {
    let backtrace_generator = BacktraceGenerator::new();
    backtrace_generator.unwind_stack();
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
