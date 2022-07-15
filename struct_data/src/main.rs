//use std::io;
struct Entrydata {
    activate: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let shiva = Entrydata {
        email: String::from("shivuk1708@gamil.com"),
        username: String::from("shivakumar"),
        activate: false,
        sign_in_count: 1000,
    };
    /*println!(
        "{} {} {} {} ",
        shiva.email, shiva.username, shiva.activate, shiva.sign_in_count
    );

    let mut user1_username = String::new();
    let mut user1_email = String::new();
    println!("enter the username");
    io::stdin()
        .read_line(&mut user1_username)
        .expect("invalid data");
    println!("enter the email");
    io::stdin()
        .read_line(&mut user1_email)
        .expect("invalid data");*/

    let user1_username = String::from("NewUser");
    let user1_email = String::from("NewUser@email.com");

    let user1 = build_user(user1_email, user1_username);
    println!("***The user1 data is***");
    println!(
        "{} {} {} {} ",
        user1.email, user1.username, user1.activate, user1.sign_in_count
    );
    println!("\n***The user2 data is***");

    let user2 = Entrydata {
        activate: !user1.activate,
        username: user1.username,
        email: String::from("user2@email.com"),
        sign_in_count: user1.sign_in_count + 3,
    };
    println!(
        "{} {} {} {} ",
        user2.email, user2.username, user2.activate, user2.sign_in_count
    );

    println!("\n***The user3 data is***");
    let user3 = Entrydata {
        email: String::from("user3@email.com"),
        ..shiva
    };
    println!(
        "{} {} {} {} ",
        user3.email, user3.username, user3.activate, user3.sign_in_count
    );

    //Below line gives error because user3 barrowed data from shiva
   /* println!(
        "{} {} {} {} ",
        shiva.email, shiva.username, shiva.activate, shiva.sign_in_count
    );*/
}
fn build_user(email: String, username: String) -> Entrydata {
    Entrydata {
        activate: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
