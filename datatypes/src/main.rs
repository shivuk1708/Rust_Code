use std::io;

fn main() {
        //addition
        let sum = 5 + 500;
        println!("the sum value is {}", sum);

        //substraction
        let sub = sum - 5;
        println!("the sub value is {}", sub);

        //Multiplication
        let mul = sub * 5;
        println!("the mul value is {}", mul);

        //Division
        let div = mul / 5;
        println!("the div value is {}", div);

        //Mod
        let rem = div % 5;
        println!("the rem value is {}", rem);

        //For float
        let fl: f32 = 5.5 + 5.5555001001001;
        println!("the fl value is {}", fl);

        //char type
        let c = 'C';
        let z = 'ಕ';
        let smile_face = '😻';
        println!("c = {},  z = {},  smile_face = {}", c, z, smile_face);

        //Compound Types
        let tup: (i32, f32, u8) = (500, 4.6, 32);
        println!("tup values 1 = {} 2 = {} 3 = {}", tup.0, tup.1, tup.2);
        let (x, y, z) = tup;
        println!("Values x = {} y = {} z = {}", x, y, z);

        //Array
        //let a: [u8; 5] = [1, 2, 3, 4];     Gives error Because the assigning elements should be same as the mentioned size
        let a = [5; 5];

        println!(
            " array value = {} {} {} {} {}",
            a[0], a[1], a[2], a[3], a[4]
        );

        //Array access over command line
        let b: [u32; 5] = [9, 8, 7, 6, 5];

        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("failed to read");

        let index: usize = index.trim().parse().expect("Index enter value is wrong");

        let element = b[index];
        println!(" the index is {} and the vale is {}", index, element);
    
}
