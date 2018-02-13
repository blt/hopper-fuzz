A fuzz project for postmates/hopper

## How To Run

Install afl.rs: https://rust-fuzz.github.io/book/afl/setup.html

```
> cargo afl build --release
> cargo afl fuzz -m1000 -t1000+ -i in -o out -M 001 target/release/hopper-fuzz
> cargo afl fuzz -m1000 -t1000+ -i in -o out -S 002 target/release/hopper-fuzz
> cargo afl fuzz -m1000 -t1000+ -i in -o out -S 003 target/release/hopper-fuzz
> cargo afl fuzz -m1000 -t1000+ -i in -o out -S 004 target/release/hopper-fuzz
```
