fn main() {
    if let Err(message) = ops::run() {
        eprintln!("Failed: {}", message);
        std::process::exit(1);
    }
}

mod ops;
