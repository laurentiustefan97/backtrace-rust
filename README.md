# Backtrace mechanism implemented in Rust

## Description
Given a binary and an address inside it, prints the function's name that is at that address.


## Usage:
With debug logging:
<pre><code>cargo run --features logging <b>binary_name</b> <b>address</b></code></pre>
Without debug logging:
<pre><code>cargo run <b>binary_name</b> <b>address</b></code></pre>

## Run tests
<pre><code>cargo test</code></pre>

## Example:
<pre><code>cargo run tests/multithreaded_server 46704</code></pre>
<pre><code>cargo run tests/basic 1545</code></pre>
