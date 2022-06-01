use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

pub fn error(msg: &str) {
    println!("Error: {}", msg);
    pause();
    std::process::exit(1);
}