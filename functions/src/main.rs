use std::io;
fn main() {
    println!("In Main Fun\n");
    loop {
        let mut op = String::new();
        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("Enter the num1 and num2 value");
        io::stdin().read_line(&mut num1).expect("Invalid Input");
        io::stdin().read_line(&mut num2).expect("Invalid Input");

        println!("Enter the operation need to be perform");
        io::stdin().read_line(&mut op).expect("Invalid Input");

        let op: char = op.trim().parse().expect("Invalid Parsing");
        let num1: u32 = num1.trim().parse().expect("Invalid Parsing");
        let num2: u32 = num2.trim().parse().expect("Invalid Parsing");
        if op == '0' {
            println!("**** EXIT ****");
            break;
        }
        if ext_fun(num1, num2, op) {
            break;
        }
    }
}
fn ext_fun(x: u32, y: u32, op: char) -> bool {
    let mut ret: bool = false;
    let result:u32 ;
    
    if op == '+' {
        result = x + y;
    } else if op == '-' {
        result = x - y;
    } else if op == '*' {
        result = x * y;
    } else if op == '/' {
        result = x / y;
    } else if op == '%' {
        result = x % y;
    } else {
        println!("***** Invalid operation *****");
        ret = true;
        return ret;
    }

    println!("The {} result is = {}\n", op, result);
    ret
}
