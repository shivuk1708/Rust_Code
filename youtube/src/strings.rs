pub fn string_data() {
    let mut helloworld = String::from("Hello ");
    println!("{} {}", helloworld, helloworld.len());
    helloworld.push('w');
    println!("{} {}", helloworld, helloworld.len());
    helloworld.push_str("orld");
    println!("{} {}", helloworld, helloworld.len());

    println!("capacity {}", helloworld.capacity());
    println!("Is empty {}", helloworld.is_empty());
    helloworld.clear();
    println!("Is empty {}", helloworld.is_empty());
    println!("capacity {}", helloworld.capacity());

    helloworld.push_str("Shivakumar Giramallappa nyamagoud");
    println!("{} {}", helloworld, helloworld.len());
    println!("capacity {}", helloworld.capacity());

    println!(
        "hello = {} nyamagoud = {}",
        helloworld.contains("hello"),
        helloworld.contains("nyamagoud")
    );

    println!("Replace {}", helloworld.replace("nyamagoud", "Nyamagoud"));
    println!("{} {}", helloworld, helloworld.len());
    println!(
        "hello = {} nyamagoud = {}",
        helloworld.contains("hello"),
        helloworld.contains("nyamagoud")
    );

    for word in helloworld.split_whitespace() {
        println!("{}", word);
    }

    let mut str1 = String::with_capacity(10);
    str1.push('a');
    str1.push('b');
    println!("{} {}", str1, str1.capacity());
    str1.push_str("cdefghij");
    println!("{} {}", str1, str1.capacity());

    assert_eq!(10, str1.len());
    assert_eq!(10, str1.capacity());
}
