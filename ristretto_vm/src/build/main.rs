mod intrinsics;

fn main() {
    if let Err(error) = intrinsics::build() {
        eprintln!("Error building intrinsics: {error}");
        std::process::exit(1);
    }
}
