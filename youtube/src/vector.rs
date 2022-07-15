pub fn vector_fun() {
    let mut numbers: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    for elements in numbers.iter() {
        println!("{}", elements);
    }
    println!("{:?}", numbers);
    println!(
        "size_of_val = {} len = {} ",
        std::mem::size_of_val(&numbers),
        numbers.len()
    );

    numbers.push(20);
    numbers.push(19);
    numbers.push(18);

    println!("{:?}", numbers);
    println!(
        "size_of_val = {} len = {} ",
        std::mem::size_of_val(&numbers),
        numbers.len()
    );

    //Get Slice
    numbers[5] = 100;
    let slice: &[u32] = &numbers[4..6];
    println!("{:?}", slice);

    numbers.pop();
    numbers.pop();
    println!("{:?}", numbers);
    println!(
        "size_of_val = {} len = {} ",
        std::mem::size_of_val(&numbers),
        numbers.len()
    );

    //Mutate
    for elements in numbers.iter_mut() {
        *elements *= *elements;
    }
    for elements in numbers.iter() {
        println!("{}", elements);
    }
}
