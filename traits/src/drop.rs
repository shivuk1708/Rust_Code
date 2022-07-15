struct Droppable {
    name: &'static str,
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}
pub fn drop_main() {
    //Block A
    let _a = Droppable { name: "a" };
    {
        //Block B
        let _b = Droppable { name: "b" };
        {
            //Block C
            let _c = Droppable { name: "C" };
            {
                let _d = Droppable { name: "d" };
                println!("Exiting block C");
            }
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(_a);
    println!("End of the main Fun");
}
