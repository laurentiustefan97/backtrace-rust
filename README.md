
# Backtrace mechanism implemented in Rust

## Description
A backtrace implementation fully in Rust. This is implemented using other Rust crates such as:
- [gimli](https://github.com/gimli-rs/gimli)
- [object](https://github.com/gimli-rs/object)
- [memmap-rs](https://github.com/danburkert/memmap-rs)
- [addr2line](https://github.com/gimli-rs/addr2line)
- [rustc-demangle](https://github.com/alexcrichton/rustc-demangle)

Currently working only on **x86-64**, **x86** architectures and on **libc** and **musl-libc** platforms.

## Usage:
Instantiate an object of type <code>backtrace::Backtrace</code> and print it with the <code> {:?}</code> format. Example:

<pre><code>let bt = Backtrace::new();
println!("{:?}", bt);
</code></pre>

## Tests
TODO

## Example:
- Having the <code>src/bin/test.rs</code> as in the repository
<pre><code>cargo +nightly run
   0: test::tazz
             at /backtrace/src/bin/test.rs:24
   1: test::taz
             at /backtrace/src/bin/test.rs:30
      &lttest::MyStruct as test::MyTrait&gt::test
             at /backtrace/src/bin/test.rs:19
   2: test::tar
             at /backtrace/src/bin/test.rs:36
      test::bar
             at /backtrace/src/bin/test.rs:41
      test::foo
             at /backtrace/src/bin/test.rs:46
      test::main
             at /backtrace/src/bin/test.rs:54
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

- Using release profile:
<pre><code>cargo +nightly run --release
   0: test::main
   1: std::rt::lang_start::{{closure}}
   2: std::rt::lang_start_internal::{{closure}}
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:52
      std::panicking::try::do_call
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panicking.rs:303
   3: __rust_maybe_catch_panic
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libpanic_unwind/lib.rs:86
   4: std::panicking::try
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panicking.rs:281
      std::panic::catch_unwind
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/panic.rs:394
      std::rt::lang_start_internal
             at /rustc/c20d7eecbc0928b57da8fe30b2ef8528e2bdd5be/src/libstd/rt.rs:51
   5: main
</code></pre>

## Limitations
* Requires the <code>Rust nightly</code> channel for running at this moment (uses inline assembly which is not a stable feature)
