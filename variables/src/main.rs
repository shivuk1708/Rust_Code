//mod fun;

fn main() {
    println!("\n****Mutable****");
    {
        let mut x = 15;
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!(" the x value is = {} {}", x, THREE_HOURS_IN_SECONDS);
        x = 20;

        println!(" the x value is = {}", x);
    }

    println!("\n****SHADOWING****");
    {
        let x = 5;
        let x = x + 5;

        {
            let x = x * 5;
            println!("The x value in block {}", x);
        }
        println!("The x value in out {}", x);
    }

    println!("\n****different Var****");
    {
        let spaces_str = "     ";
        let spaces_num = spaces_str.len();

        println!(" string = {} and lentgh = {}", spaces_str, spaces_num);
    }
    
    println!("**** CONVERT ****\n");
    {
        let num: u32 = "123456789".parse().expect("Not a number");
	println!("the converted number is {}", num);
    }
    
}
