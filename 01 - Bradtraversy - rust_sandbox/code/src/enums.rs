enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    move_avatar(Movement::Left);
    move_avatar(Movement::Up);
    move_avatar(Movement::Right);
    move_avatar(Movement::Down);
}