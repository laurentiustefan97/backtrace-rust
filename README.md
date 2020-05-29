# Backtrace mechanism implemented in Rust

## Description
A backtrace implementation in Rust. This is implemented using other Rust crates such as:
- [gimli](https://github.com/gimli-rs/gimli)
- [object](https://github.com/gimli-rs/object)
- [memmap-rs](https://github.com/danburkert/memmap-rs)
- [addr2line](https://github.com/gimli-rs/addr2line)
- [rustc-demangle](https://github.com/alexcrichton/rustc-demangle)

Currently working only on **x86-64**, **x86** architectures and on **libc** and **musl-libc** platforms.

## Install
Add the crate dependency in the <code>Cargo.toml</code> file.
<pre><code>[dependencies]
backtrace-rust = "0.1"
</code></pre>

## Usage
Instantiate an object of type <code>backtrace::Backtrace</code> and print it with the <code> {:?}</code> format. Example:

<pre><code>use backtrace_rust::backtrace::Backtrace;

fn main() {
	let bt = Backtrace::new();
	// other code
	println!("{:?}", bt);
}
</code></pre>

## Examples
There are 3 examples in the <code>examples/</code> directory. Example of running:
<pre><code>$ cargo +nightly run --example complex_inline
   0: complex_inline::tazz
             at /backtrace-rust/examples/complex_inline.rs:22
   1: complex_inline::taz
             at /backtrace-rust/examples/complex_inline.rs:28
      &ltcomplex_inline::MyStruct as complex_inline::MyTrait&gt::test
             at /backtrace-rust/examples/complex_inline.rs:17
   2: complex_inline::tar
             at /backtrace-rust/examples/complex_inline.rs:34
      complex_inline::bar
             at /backtrace-rust/examples/complex_inline.rs:39
      complex_inline::foo
             at /backtrace-rust/examples/complex_inline.rs:44
      complex_inline::main
             at /backtrace-rust/examples/complex_inline.rs:48
   3: std::rt::lang_start::{{closure}}
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:67
   4: std::rt::lang_start_internal::{{closure}}
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:52
      std::panicking::try::do_call
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panicking.rs:303
   5: __rust_maybe_catch_panic
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libpanic_unwind/lib.rs:86
   6: std::panicking::try
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panicking.rs:281
      std::panic::catch_unwind
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panic.rs:394
      std::rt::lang_start_internal
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:51
   7: std::rt::lang_start
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:67
   8: main
</code></pre>


## Limitations
* Requires the <code>Rust nightly</code> channel for running at this moment (uses inline assembly which is not a stable feature)
* Does not support yet using the <code>.debug_frame</code> debug section when <code> eh_frame</code> is not present
* Can not evaluate a more complex <code>eh_frame</code> register restoring rule

