fn thread_check() {
    let _ = match std::thread::available_parallelism() {
        Ok(n) => println!("Available parallel threads: {}", n),
        Err(e) => eprintln!("Failed to get available parallelism: {}", e),
    };
}
fn main() {
    thread_check();
}
