fn main() {
    //let mut str1 = String::new();
    //str1 = "hello".to_string();
    //println!("{}", str1);

    {
        let str = String::from("Hello Hai");
        //println!("{}", str);

        //str.push_str(", How are you");
        //println!("{}", str);

        //let str2 = str.clone();
        //println!("{} {}", str, str2);

        makes_mov(str);
        //println!("{}", str);

        let str1 = String::from("which fun");
        let str2 = makes_move_gives_back(str1);
        println!(" the str2 is {}", str2);

        let value = 5;
        makes_copy(value);
        println!("{}", value);

        let (mut str2, len) = calculte_length(str2);
        println!(" {} {} ", str2, len);

        //Taking as reference
        {
            let len_refer = calculte_length_refer(&mut str2);
            println!("str2 {}, len_refer {}", str2, len_refer);
        }

        //Multiple reference
        {
            let mut s = String::from("Hello");
            let r1 = & s;
            let r2 = & s;
            //let r3 = &mut s;
            println!("{} {}", r1, r2);
        }
    }
}

fn makes_copy(x: u32) {
    println!("The makes_copy value is {}", x);
}
fn makes_mov(str: String) {
    println!("The string moved value is {}", str);
}
fn makes_move_gives_back(mut str: String) -> String {
    str.push_str(" = makes_move_gives_back");
    str
}

fn calculte_length(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}
fn calculte_length_refer(str: &mut String) -> usize {
    str.push_str(", hello");
    str.len()
}
