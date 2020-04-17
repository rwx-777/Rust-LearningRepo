//Enums are types which have a few deficitinve values

enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_player(m: Movement) {
    //perform Action depending on info
    match m {
        Movement::Up => println!("Player moving up"),
        Movement::Down => println!("Player moving down"),
        Movement::Left => println!("Player moving left"),
        Movement::Right => println!("Player moving right")
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Right;

    move_player(avatar1);
    move_player(avatar2);
    move_player(avatar3);
    move_player(avatar4);

}