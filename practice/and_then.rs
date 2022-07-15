fn get_alias(name :&str) -> Option<&str>
{
    match name {
        "shiva" => Some("kumar"),
        "nyamagoud" => Some("Gouda"),
        _ => None,
    }
}
fn main ()
{
    let name1 :Option<&str> = Some("shiva");
    let nickname1 = name1.as_ref().and_then(|m| get_alias (&m));
    println!("nick name is {}", nickname1.unwrap_or("Not Present"));

    let name2 :Option<&str> = Some("kumar");
    let nickname2 = name2.as_ref().and_then(|m| get_alias (&m));
    println!("nick name is {}", nickname2.unwrap_or("Not Present"));
}