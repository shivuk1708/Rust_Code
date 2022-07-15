pub fn functions_fun() {
    functions_greeting("hello", "shivakumar");

    println!("Additon result {}", functions_add(10, 32));

    //closure to add fun
    let z :u32 = 10;
    let add_ret = |x: u32, y: u32| x + y + z;
    println!("Add_ret = {}", add_ret(32, 10));
}
pub fn functions_greeting(greet: &str, name: &str) {
    println!("{} {} nice to meet you", greet, name);
}
pub fn functions_add(x: u32, y: u32) -> u32 {
    x + y
}
