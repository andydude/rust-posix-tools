extern crate getopts;

fn xargs_main(arguments: &[String]) {
    let program = arguments[0].as_slice();
    let args = arguments.tail();
    
    let opts = &[
        getopts::optopt("E", "eof" , "Use STR as the logical end-of-file string.", "STR"),
        getopts::optopt("I", "replace" , "Insert mode: CMD is executed for each logical line from stdin. The resulting argument shall be inserted in arguments in place of each occurrence of STR.", "STR"),
        getopts::optopt("L", "max-lines" , "This help.", "NUM"),
        getopts::optopt("n", "max-args" , "Invoke CMD using as many stdin arguments as possible, up to NUM (a positive decimal integer) arguments maximum.", "NUM"),
        getopts::optopt("s", "max-chars" , "Invoke CMD using as many stdin arguments as possible yielding a command line length less than NUM (a positive decimal integer) bytes.", "NUM"),
        getopts::optflag("p", "interactive" , "Prompt mode: the user is asked whether to execute CMD at each invocation."),
        getopts::optflag("t", "verbose" , "Trace mode. Each generated command line shall be written to standard error just prior to invocation."),
        getopts::optflag("x", "exit" , "Terminate if a constructed command line will not fit in the implied or specified max-chars (see above)."),
        getopts::optflag("v", "version" , "This program's version number."),
        getopts::optflag("h", "help" , "This help.")
    ];

    
    let matches = getopts::getopts(args, opts).unwrap();
    
    if matches.opt_present("h") {
        let brief = "construct argument lists and invoke CMD";
        println!("{} {}\n\n{}", getopts::short_usage(program, opts),
                 "[CMD [ARG ...]]", getopts::usage(brief, opts));
        std::os::set_exit_status(1);
        return;
    }
 }

fn main() {
    xargs_main(std::os::args().as_slice());
}
