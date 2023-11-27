use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".").min_depth(1).max_depth(1) {
        println!("{}", entry.unwrap().path().display());
    }
}
