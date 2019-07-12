fn main() {
    // This works
    print!("{}", include_str!("file1.txt"));
    // This doesn't work
    print!("{}", include_str!("file2.rs"));
}
