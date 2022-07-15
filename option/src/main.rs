use std::collections::HashMap;
//pub type Data = Option<HashMap<String, u32>>;
fn main() {
    let data : Option<HashMap<String, u32>> = Some(HashMap::from([("hello".to_owned(), 25)]));
    println!("{:?}", data.as_ref().unwrap().get("hello")) ;
    println!("{:?}", data.unwrap().get("Hai")) ;
}
/*
fn setdaata(str1 : String, id_num : u32) -> Option<HashMap<String, u32>>{

    (str1, id_num)
}*/