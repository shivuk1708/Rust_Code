fn main() {
    println!("Hello, world!");
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{1} {2} {0}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    println!("My name is {0}, {1} {0}", "Bond", "Hello");
}
