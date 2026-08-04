enum Direction {
    East,
    North,
    South,
    West,
}

//only inside the enums direciton can be used. if any type like east,west will raise error.

fn main() {
    let direction = Direction::East;
    steer(direction);
}

fn steer(dir: Direction) {
    match dir {
        Direction::North => print!("North Direction"),
        Direction::South => print!("South Direction"),
        _ => print!("Horizontal Direction"),
    }
}
