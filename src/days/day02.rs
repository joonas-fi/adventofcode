use std::fs;

pub fn a() {
    let command_texts = fs::read_to_string("day02_input.txt").unwrap();

    let mut position = position_zero();

    for command_text in command_texts.lines() {
        let command = command_parse(command_text.to_string()).unwrap();

        position = command_apply_a(command, position)
    }

    println!(
        "horizontal={} depth={} multiplied={}",
        position.horizontal,
        position.depth,
        position_multiply(&position),
    );
}

pub fn b() {
    let command_texts = fs::read_to_string("day02_input.txt").unwrap();

    let mut position = position_zero();

    for command_text in command_texts.lines() {
        let command = command_parse(command_text.to_string()).unwrap();

        position = command_apply_b(command, position)
    }

    println!(
        "horizontal={} depth={} multiplied={}",
        position.horizontal,
        position.depth,
        position_multiply(&position),
    );
}

enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn position_zero() -> Position {
    Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    }
}

fn position_multiply(position: &Position) -> i32 {
    position.horizontal * position.depth
}

fn command_parse(comm_str: String) -> Result<Command, String> {
    match comm_str {
        i if i.starts_with("forward ") => Ok(Command::Forward(
            i["forward ".len()..].parse::<i32>().unwrap(),
        )),
        i if i.starts_with("down ") => {
            Ok(Command::Down(i["down ".len()..].parse::<i32>().unwrap()))
        }
        i if i.starts_with("up ") => Ok(Command::Up(i["up ".len()..].parse::<i32>().unwrap())),
        _ => Err(format!("invalid command: {}", comm_str)),
    }
}

fn command_apply_a(command: Command, current: Position) -> Position {
    match command {
        Command::Up(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth - amount,
            aim: current.aim, // unchanged in first part
        },
        Command::Down(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth + amount,
            aim: current.aim, // unchanged in first part
        },
        Command::Forward(amount) => Position {
            horizontal: current.horizontal + amount,
            depth: current.depth,
            aim: current.aim, // unchanged in first part
        },
    }
}

fn command_apply_b(command: Command, current: Position) -> Position {
    match command {
        Command::Down(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth,
            aim: current.aim + amount,
        },
        Command::Up(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth,
            aim: current.aim - amount,
        },
        Command::Forward(amount) => Position {
            horizontal: current.horizontal + amount,
            depth: current.depth + (current.aim * amount),
            aim: current.aim,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let test_input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

        let mut position = position_zero();

        for line in test_input.lines() {
            let command = command_parse(line.to_string()).unwrap();

            position = command_apply_a(command, position)
        }

        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);

        assert_eq!(position_multiply(&position), 150);
    }

    #[test]
    fn test_b() {
        let test_input = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

        let mut position = position_zero();

        for line in test_input.lines() {
            let command = command_parse(line.to_string()).unwrap();

            position = command_apply_b(command, position)
        }

        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 60);

        assert_eq!(position_multiply(&position), 900);
    }
}
