# Install
- Install the ![rustlang tools](https://www.rust-lang.org/tools/install) (macosx and linux).
- First compile with `$ cargo build` from the root directory, the compiled ELF file is in target/debug/globalali
```bash
$ ./target/debug/globalali -h
GlobalAli
Define: Gamma(n) = (d-e) + n*e 

USAGE:
    globalali [OPTIONS] --input-seq <input> <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dparam <d param>             (d-e) Gamma function intercept  [default: 1]
    -e, --eparam <e param>             Gamma function steep [default: 1]
    -i, --input-seq <input> <input>    input sequences
    -m, --mparam <m param>             Value for Similar elts. [default: 3]
    -s, --sparam <s param>             Different elts. [default: 2]
```

To run the program you can use either:
```bash
$ ./target/debug/globalali -i atgc agc -d 1 -e 1 -m 3 -s 2
[[0, 1, 2, 3],
 [1, 3, 2, 2],
 [2, 2, 1, 1],
 [3, 2, 5, 4],
 [4, 3, 4, 8]]
OR
$ cargo run -- -i atgc agc -d 1 -e 1 -m 3 -s 2
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/globalali -i atgc agc -d 1 -e 1 -m 3 -s 2`
[[0, 1, 2, 3],
 [1, 3, 2, 2],
 [2, 2, 1, 1],
 [3, 2, 5, 4],
 [4, 3, 4, 8]]
```
