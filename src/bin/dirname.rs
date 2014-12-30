use std::vec::Vec;

//fn basename(pathname: &str, suffix: &str) -> Result<String, &'static str> {
//    let path = try!(Path::new_opt(pathname).ok_or("contains null"));
//    let base = try!(path.filename_str().ok_or("no basename"));
//
//    // strip extension suffix
//    if base.len() >= suffix.len() {
//        let base_start_len: uint = base.len() - suffix.len();
//        let base_end: &str = base.slice_from_or_fail(&base_start_len);
//        if base_end == suffix {
//            return Ok(String::from_str(base.slice_to_or_fail(&base_start_len)));
//        }
//    }
//
//    return Ok(String::from_str(base));
//}
//
//fn basename_main(args: Vec<String>) {
//    let result = match args.len() {
//        1 => Err("too few arguments"),
//        2 => basename(args[1].as_slice(), ""),
//        3 => basename(args[1].as_slice(), args[2].as_slice()),
//        _ => Err("too many arguments"),
//    };
//    
//    match result {
//        Ok(base) => println!("{}", base),
//        Err(e) => println!("usage: basename string [suffix]\n\n{}", e),
//    }
//}


fn dirname(pathname: &str) -> Result<String, &'static str> {
    let path = try!(Path::new_opt(pathname).ok_or("contains null"));
    let dir = try!(path.dirname_str().ok_or("no dirname"));
    return Ok(String::from_str(dir));
}

fn dirname_main(args: Vec<String>) {
    let result = match args.len() {
        1 => Err("too few arguments"),
        2 => dirname(args[1].as_slice()),
        _ => Err("too many arguments"),
    };
    
    match result {
        Ok(dir) => println!("{}", dir),
        Err(e) => println!("usage: dirname string\n\n{}", e),
    }
}

fn main() {
    dirname_main(std::os::args());
}
