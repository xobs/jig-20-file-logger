use std::io;
use std::env;

use std::io::Write;
use std::fs::OpenOptions;

// Stolen from http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr#27590832
macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();

    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println_stderr!("Error: no logfile specified.  Usage: {} [file-to-log-to]", args[0]);
        return;
    }
    let ref file_name = args[1];
    println_stderr!("Logging to file {}", file_name);

    let mut file = OpenOptions::new().create(true)
                                     .truncate(false)
                                     .append(true)
                                     .open(file_name).unwrap();
    loop {
        match stdin.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = buf.trim();
                let r = writeln!(&mut file, "{}", trimmed);
                r.expect("failed writing to logfile");
            },
            Err(e) => { panic!(e); }
        }
        buf.truncate(0);
    }
}