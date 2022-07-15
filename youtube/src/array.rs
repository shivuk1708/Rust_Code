pub fn array_fun() {
    let numbers: [u32; 6] = [1, 2, 3, 4, 5, 6];
    for elements in numbers {
        println!("{}", elements);
    }

    println!("{}", std::mem::size_of_val(&numbers));
    println!("{}", numbers.len());

    //Get Slice
    let slice : &[u32] = &numbers[2..5];
    println!("{:?}", slice);
    
    //Without slice it give error 
    //println!("{:?}", numbers[3..5]);

}
