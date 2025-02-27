enum Direction {
    North,
    South,
    East,
    West
}

fn main() {
    
    let mut direction = Direction::North;
    print_direction(direction);

    direction = Direction::South;
    print_direction(direction);

    direction = Direction::West;
    print_direction(direction);

    direction = Direction::East;
    print_direction(direction);

}


fn print_direction (dir: Direction) {
    match dir {
        Direction::East => println!("\nEast direction\n"),
        Direction::West => println!("\nWest direction\n"),
        Direction::South => println!("\nSouth direction\n"),
        Direction::North => println!("\nNorth direction\n"),
    }

}

