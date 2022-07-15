pub fn pointer_ref_fun() {
    let array1 = [1, 2, 3, 4];
    let array2 = array1;

    println!("array {:?}", (array1, array2));

    let vector1 = vec![1, 2, 3, 4];
    let vector2 = &vector1;

    println!("{:?}", (&vector1, vector2));
}
