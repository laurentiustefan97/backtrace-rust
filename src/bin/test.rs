use backtrace::backtrace::Backtrace;

trait MyTrait {
    fn test(&self);
}

struct MyStruct { }

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {

        }
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

#[inline(always)]
fn taz() {
    tazz();
}

#[inline(always)]
fn tar() {
    let my_struct = MyStruct::new();
    my_struct.test();
}

#[inline(always)]
fn bar() {
    tar();
}

#[inline(always)]
fn foo() {
    bar();
}

fn main() {
    let mut v = [1, 2, 3, 4, 5];
    v[0] = 5;
    v[1] = 2;
    v[2] = 3;
    foo();
}
