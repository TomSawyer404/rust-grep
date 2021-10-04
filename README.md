# rust-grep

The next utility you will build is called rust-grep, a variant of the UNIX tool grep. This tool looks through a file, line by line, trying to find a user-specified search term in the line. If a line has the word within it, the line is printed out, otherwise it is not.  

Here is how a user would look for the term `to` in the file poem.txt:

```bash
WSL rust-grep on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯ cargo r std ./src/main.rs
   Compiling rust-grep v0.1.0 (/home/banana/Workshop/Rust/rust-grep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.72s
     Running `target/debug/rust-grep std ./src/main.rs`
use std::env;
use std::fs::*;
use std::io::*;
        std::process::exit(1);
    // Read file from stdin
            let read_bytes = std::io::stdin()
                .expect("Cannot read from stdin");

WSL rust-grep on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
❯ cat src/main.rs| cargo r std
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-grep std`
use std::env;
use std::fs::*;
use std::io::*;
        std::process::exit(1);
    // Read file from stdin
            let read_bytes = std::io::stdin()
                .expect("Cannot read from stdin");

WSL rust-grep on  main [?] is 📦 v0.1.0 via 🦀 v1.55.0
[✔️ ]❯

```

Details  
- Your program rust-grep is always passed a search term and zero or more files to grep through (thus, more than one is possible). It should go through each line and see if the search term is in it; if so, the line should be printed, and if not, the line should be skipped.
- The matching is case sensitive. Thus, if searching for foo, lines with Foo will not match.
- Lines can be arbitrarily long (that is, you may see many many characters before you encounter a newline character, \n). rust-grep should work as expected even with very long lines. 
- If rust-grep is passed no command-line arguments, it should print "rust-grep: searchterm [file ...]" (followed by a newline) and exit with status 1.
- If rust-grep encounters a file that it cannot open, it should print "rust-grep: cannot open file" (followed by a newline) and exit with status 1.
- In all other cases, rust-grep should exit with return code 0.
- For simplicity, if passed the empty string as a search string, rust-grep can either match NO lines or match ALL lines, both are acceptable.

