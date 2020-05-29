use backtrace_rust::backtrace::Backtrace;

trait MyTrait {
    fn test(&self);
}

struct MyStruct { }

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { }
    }
}

impl MyTrait for MyStruct {
    fn test(&self) {
        taz();
    }
}

fn tazz() {
    let bt = Backtrace::new();
    println!("{:?}", bt);
}

fn taz() {
    tazz();
}

fn tar() {
    let my_struct = MyStruct::new();
    my_struct.test();
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
