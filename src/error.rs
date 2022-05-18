pub fn error(msg: &str) {
    println!("Error: {}", msg);
    std::process::exit(1);
}