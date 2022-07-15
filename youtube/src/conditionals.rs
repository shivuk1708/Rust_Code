pub fn conditionals_fun() {
    let age = 18;

    let check_id: bool = false;
    let knows_person = true;
    if age >= 21 && check_id || knows_person {
        println!("Bartender : What do you like to drink");
    } else if age < 21 && check_id {
        println!("Batender : Sorry you have to leave");
    } else {
        println!("Bartender : Can you please show your id")
    }

    //Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
