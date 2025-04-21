struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");

    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("Pattern: {:?}, Path: {:?}", args.pattern, args.path);
}
