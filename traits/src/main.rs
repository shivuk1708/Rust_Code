mod derive;
mod drop;
mod iterator;
fn main() {
    if false {
        derive::derive_main();
        drop::drop_main();
    }
    iterator::iterator_main();
}
