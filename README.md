# ttc-rs

Working through the [teenytinycompiler](http://web.eecs.utk.edu/~azh/blog/teenytinycompiler1.html) compiler tutorial, in Python.

Grammar: See [docs/grammar.md](docs/grammar.md)

## Build

A `Makefile` is provided for compiling the generated C file. The default executable name is `ttc`, but this can be changed in the `Makefile`.

```
$ cargo clean && cargo build --release && make && ./ttc
```

## Demo

```
~/dev/projects/incubator/ttc-rs:master$ cargo clean && cargo build --release
   Compiling ttc-rs v0.1.0 (/Users/z0ltan/dev/projects/incubator/ttc-rs)
    Finished release [optimized] target(s) in 1.97s

~/dev/projects/incubator/ttc-rs:master$ cargo run samples/fib.teeny
   Compiling ttc-rs v0.1.0 (/Users/z0ltan/dev/projects/incubator/ttc-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 1.07s
     Running `target/debug/ttc-rs samples/fib.teeny`
Program compiled successfully

~/dev/projects/incubator/ttc-rs:master$ make && ./ttc
gcc -Wall -std=c99 -flto -O3 -o ttc out.c
How many fibonacci numbers do you want?
10

0.00
1.00
1.00
2.00
3.00
5.00
8.00
13.00
21.00
34.00

~/dev/projects/incubator/ttc-rs:master$ make clean
   rm -f out.c ttc
```

## LICENCE

See [LICENSE.md](LICENSE.md).
