#![feature(macro_rules)]
extern crate getopts;
use std::io::File;
use std::io::IoResult;
use std::io::Writer;
use std::path::Path;

// general

fn with_stderr(f: |&mut Writer| -> IoResult<()>) {
    let result = f(&mut std::io::stderr());
    match result {
        Ok(()) => {}
        Err(e) => panic!("failed printing to stderr: {}", e),
    }
}

macro_rules! stderr_println {
    ($($arg: tt)*) => (
        with_stderr(|io| writeln!(io, $($arg)*))
    )
}

// wc-specific

#[deriving(Copy)]
pub struct Total {
    lines: uint,
    words: uint,
    chars: uint,
    bytes: uint
}

#[deriving(Copy)]
pub struct Shown {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
    filename: bool
}
 
fn empty() -> Total {
    Total{
        lines: 0, 
        words: 0, 
        chars: 0, 
        bytes: 0
    }
}

fn count(filename: &str) -> Total {
    let mut total = empty();
    let s = File::open(&Path::new(filename)).read_to_string().unwrap();
 
    for ch in s.chars() {
        if ch == '\n' {
            total.lines += 1;
        }
        if ch.is_whitespace() {
            total.words += 1;
        }
    }
 
    total.chars = s.char_len();
    total.bytes = s.len();
 
    return total;
}
 
fn print_count(filename: &str, show: &Shown) -> Total {
    let total = count(filename);
    print_total(filename, show, &total);
    return total;
}
 
fn print_total(filename: &str, show: &Shown, total: &Total) {
    if show.lines {
        print!("{} ", total.lines as int);
    }
    if show.words {
        print!("{} ", total.words as int);
    }
    if show.chars {
        print!("{} ", total.chars as int);
    } else if show.bytes {
        print!("{} ", total.bytes as int);
    }
 
    if show.filename {
        println!("{}", filename);
    } else {
        println!("");
    }
}

fn wc_main(arguments: &[String]) {
    let program = arguments[0].as_slice();
    let args = arguments.tail();

    // options for getopt
    let opts = &[
        getopts::optflag("c", "bytes", "Write to stdout the number of bytes in each input file."),
        getopts::optflag("l", "lines", "Write to stdout the number of <newline> characters in each input file."),
        getopts::optflag("m", "chars", "Write to stdout the number of characters in each input file."),
        getopts::optflag("w", "words", "Write to stdout the number of words in each input file."),
        getopts::optflag("v", "version" , "This program's version number."),
        getopts::optflag("h", "help" , "This help.")
    ];
 
    // options for us to use
    let mut show = Shown{
        lines: false, 
        words: false, 
        chars: false, 
        bytes: false,
        filename: true
    };

    // counts for output
    let mut total = Total{
        lines: 0, 
        words: 0, 
        chars: 0, 
        bytes: 0
    };
 
    let matches = getopts::getopts(args, opts).unwrap();
 
    if matches.opt_present("h") {
        let brief = "word, line, and byte or character count";
        stderr_println!("{} {}\n\n{}", getopts::short_usage(program, opts),
                        "[FILE ...]", getopts::usage(brief, opts));
        std::os::set_exit_status(1);
        return;
    }
 
    // HACK: getopt should provide this
    let optcount = args.len() - matches.free.len();
    if optcount == 0 {
        show.lines = true; 
        show.words = true;
        show.bytes = true;
    }
 
    if matches.opt_present("l") {
        show.lines = true;
    }
    if matches.opt_present("w") {
        show.words = true;
    }

    if matches.opt_present("m") {
        show.chars = true;
    } else if matches.opt_present("c") {
        show.bytes = true;
    }
 
    if matches.free.len() == 0 {
        show.filename = false;
        print_count("/dev/stdin", &show);
        return;
    }
 
    for arg in matches.free.iter() {
        let subtotal;
        let filename = arg.as_slice();
        if filename == "-" {
            subtotal = print_count("/dev/stdin", &show);
        } else {
            subtotal = print_count(filename, &show);
        }
        total.lines += subtotal.lines;
        total.words += subtotal.words;
        total.chars += subtotal.chars;
        total.bytes += subtotal.bytes;
    }
 
    show.filename = false;
    print_total("total", &show, &total);
}

fn main() {
    wc_main(std::os::args().as_slice());
}
