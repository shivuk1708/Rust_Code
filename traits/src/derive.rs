#[derive(Debug, Clone)]
struct Sheep {
    color: bool,
    name: String,
}

struct Dog {}
struct Cow {}
trait Pet {
    // Instance method signature
    fn noise(&self) -> String;
}
trait Animal {
    fn new(name: String) -> Self;

    fn name(&self) -> String;
    fn noise(&self) -> String;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_white(&self) -> bool {
        self.color
    }

    fn shear(&mut self) {
        if self.is_white() {
            println!("{}  is already white", self.name);
        } else {
            println!("{}  gets bath", self.name);
            self.color = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: String) -> Sheep {
        Sheep {
            color: false,
            name: name,
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_white() {
            return "Baaaaaaah".to_string();
        } else {
            return "Bla Bla".to_string();
        }
    }
    fn talk(&self) {
        println!("{} Pauses briefly {}", self.name, self.noise());
    }
}
impl Pet for Dog {
    fn noise(&self) -> String {
        return "Bow Bow".to_string();
    }
}

impl Pet for Cow {
    fn noise(&self) -> String {
        return "Aweemba".to_string();
    }
}
fn random_animal(ramdom_number: f64) -> Box<dyn Pet> {
    if ramdom_number < 0.5 {
        Box::new(Dog {})
    } else {
        Box::new(Cow {})
    }
}
pub fn derive_main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly".to_string());
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let ramdom_number = 0.51;
    let animal = random_animal(ramdom_number);
    println!("The random animal make noise {}", animal.noise());
}
