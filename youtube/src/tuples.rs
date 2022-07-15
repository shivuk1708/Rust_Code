//Max 12 elements we can add in tuples
pub fn tuple_fun (){
    let person : (&str, &str, u32 ) = ("Shivakumar", "Nyamagoud", 25);
    println!("{} {} {}", person.0, person.1, person.2);

}