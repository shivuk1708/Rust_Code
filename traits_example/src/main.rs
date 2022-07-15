pub trait Draw {
    fn draw(&self);
}
/*
pub struct Screen {
    pub components : Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run (&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
*/
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{} {} {}", self.width, self.height, self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{} {} {:?}", self.width, self.height, self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 85,
                options: vec![
                    String::from("Yes"),
                    String::from("May be"),
                    String::from("No"),
                ],
            }),
            Box::new(SelectBox {
                width: 50,
                height: 25,
                options: vec![
                    String::from("Yes"),
                    String::from("May be"),
                    String::from("No"),
                ],
            }),
        ],
    };
    screen.run();
}
