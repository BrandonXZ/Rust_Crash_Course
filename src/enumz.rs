//ENUM - enums are types which have a few definite values **enum is a keyword in rust, like other languages 

enum Movement {
    //variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m:Movement){
    //Perform action depending on info
    match m{  //match is similar to a switch
        Movement::Up =>println!("Avatar moving: UP"),
        Movement::Down =>println!("Avatar moving: Down"),
        Movement::Left =>println!("Avatar moving: Left"),
        Movement::Right =>println!("Avatar moving: Right")
    }                  
}

pub fn run(){
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}