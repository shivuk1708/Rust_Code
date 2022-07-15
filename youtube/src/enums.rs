enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn mov_avatar(m: Movement) {
    //perform action defending on info
    match m {
        Movement::Up => println!("Avatar moving Up"),
        Movement::Down => println!("Avatar moving Down"),
        Movement::Left => println!("Avatar moving Left"),
        Movement::Right => println!("Avatar moving Right"),
    }
}
pub fn enums_fun() {
    let avatar1 = Movement::Up;
    let avatar4 = Movement::Left;
    let avatar3 = Movement::Down;
    let avatar2 = Movement::Right;

    mov_avatar(avatar1);
    mov_avatar(avatar2);
    mov_avatar(avatar3);
    mov_avatar(avatar4);
}
