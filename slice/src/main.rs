fn main() {
    let str1 = String::from("Hell oHai");
    let len = firstword_len(&str1);
    println!("{}", len);
    let str2 = firstword_data(&str1);
    println!("{}", str2);

    //str1.clear();
    {
        let str1 = String::from("Hello Shivakumar");

        let word = first_word(&str1[0..5]);
        let word = first_word(&str1[..]);

        let my_string_literal = "hello wolrd";

        let word = first_word(&my_string_literal[0..5]);
        let word = first_word(&my_string_literal[..]);


        let word = first_word(my_string_literal);
    }
}
fn firstword_len(str: &String) -> usize {
    let byte = str.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}
fn firstword_data(str1 : &String) -> & str{
    let byte = str1.as_bytes();
    for( i , &item) in byte.iter().enumerate() {
        if item == b' '{
            return &str1[0..i];
        }
    }
    &str1[..]
}
fn first_word(s: &str) -> &str {
    println!("in first_word fun {}", s);
    &s[..]
}
