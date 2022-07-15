use std::collections::HashMap;

fn main() {
    let mut gfg = HashMap::new();
    gfg.insert("Data Structures", "90");
    gfg.insert("Algorithms", "99");
    gfg.insert("Interview Preparations", "100");
    gfg.insert("FAANG", "97");
    gfg.insert("CP", "99");
    let value = gfg.get(&"shiva");
    println!("value={:?}", value);
}
