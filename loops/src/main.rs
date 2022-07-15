fn main() {
    let mut count = 0;
    'counting_up:loop {
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -=1;
        }
        count +=1;

    }
    println!("End count {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 10;
        }
    };
    println!("The result is {}", result);

    println!("***WHILE LOOP***\n");
    let a = [ 1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 4{
        println!("The {} index Array Value is {}", index, a[index]);
        index += 1;
    }

    println!("***FOR LOOP***\n");
    let a = [ 1, 2, 3, 4, 5];
    for element in a{
        println!("The Array Value is {}", element);
    }

    for number in(1..10).rev() {
        println!("{}!", number);
    }
}
