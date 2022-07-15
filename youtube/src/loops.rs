pub fn loops_fun() {
    let mut count = 0;
    //Infinite loop
    loop {
        count += 1;
        println!("{}", count);
        if count == 20 {
            break;
        }
    }

    //While loop
    while count <= 40 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    //For Range loop 
    for x in 0..40{
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
