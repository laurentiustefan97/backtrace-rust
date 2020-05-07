
# Backtrace mechanism implemented in Rust

## Description
A backtrace implementation in Rust. This is implemented using other Rust crates such as: 
- [gimli](https://github.com/gimli-rs/gimli)
- [object](https://github.com/gimli-rs/object)
- [memmap-rs](https://github.com/danburkert/memmap-rs)

Currently working only on x86-64 architecture with different limitations.

## Usage:
Instantiate an object of type <code>backtrace::BacktraceGenerator</code> and call its method <code> unwind_stack</code>. The backtrace will be printed at standard output. 

## Tests
TODO

## Example:
- Having the <code>src/bin/test.rs</code> as in the repository
<pre><code>cargo +nightly run
0: tar
1: bar
2: foo
3: main
4: {{closure}}&lt()&gt
5: do_call&ltclosure-0,i32&gt
6: __rust_maybe_catch_panic
7: lang_start_internal
8: lang_start&lt()&gt
9: Name unknown
10: Name unknow
</code></pre>

## Limitations
* Requires the <code>Rust nightly</code> channel for running at this moment (uses inline assembly which is not a stable feature)
