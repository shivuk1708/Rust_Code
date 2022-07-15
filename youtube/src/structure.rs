#[derive(Debug, Clone)]
struct Color {
    red: u8,
    black: u8,
    white: u8,
}

//tuple strcut
struct Thing(u8, u8, u8);
#[derive(Debug, Clone)]
struct Person {
    firstname: String,
    lastname: String,
}

impl Person {
    fn new(firstname: &str, lastname: &str) -> Person {
        Person {
            firstname: firstname.to_string(),
            lastname: lastname.to_string(),
        }
    }

    fn get_fullname(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    fn set_last_name(&mut self, last: &str) {
        self.lastname = last.to_string();
    }

    //Name tuple
    fn tuple_name(self) -> (String, String) {
        (self.firstname, self.lastname)
    }
}
pub fn struct_fun() {
    let mut firstcolor = Color {
        red: 30,
        black: 0,
        white: 0,
    };
    println!(
        "red = {}, black = {}, white = {}",
        firstcolor.red, firstcolor.black, firstcolor.white
    );
    firstcolor.black = 200;
    println!("color = {:?}", firstcolor);

    let firstthing = Thing(20, 30, 40);
    println!(
        "Things = {} {} {}",
        firstthing.0, firstthing.1, firstthing.2
    );

    let mut student = Person::new("Shivakumar", "Nyamagoud");
    println!("{:?}", student);
    student.firstname = "shiva".to_string();
    println!("{:?}", student);
    println!("{}", student.get_fullname());
    student.set_last_name("Gouda");
    println!("{}", student.get_fullname());
    println!("{:?} ", student.tuple_name());
}
