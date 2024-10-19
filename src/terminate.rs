pub fn terminate<D: std::fmt::Display>(msg: D, code: i32) -> ! {
    println!("{}", msg);
    std::process::exit(code);
}
