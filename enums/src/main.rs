fn main() {
    println!("Hello, world!");
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?} {:?}",home, loopback);
    
    #[derive(Debug, Clone, Hash, PartialEq, Eq)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msgq = Message::Quit;
    let msgmove = Message::Move{x:20, y:30};
    let msgwrite = Message::Write("HEllo".to_string());
    let msgcolor = Message:: ChangeColor(1, 2, 3);

    let str1 : String = String::from("h-e-l-l-o");
    println!("{}", str1.replace("-", "_"));
    println!("{:?} {:?} {:?} {:?}",msgq, msgmove,msgwrite, msgcolor );
}
