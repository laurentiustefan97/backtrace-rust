use backtrace::backtrace::BacktraceGenerator;

fn main() {
    let backtrace_generator = BacktraceGenerator::new();
    backtrace_generator.unwind_stack();
}
