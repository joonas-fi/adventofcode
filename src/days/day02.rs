use std::fs;

pub fn a() {
    let command_texts = fs::read_to_string("day02_input.txt").unwrap();

    let mut position = position_zero();

    for command_text in command_texts.lines() {
        let command = command_parse(command_text.to_string()).unwrap();

        position = command_apply(command, position)
    }

    let multiplied = position.horizontal * position.depth;

    println!(
        "horizontal={} depth={} multiplied={}",
        position.horizontal, position.depth, multiplied,
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
}

fn position_zero() -> Position {
    Position {
        horizontal: 0,
        depth: 0,
    }
}

fn mul(position: Position) -> i32 {
    position.horizontal * position.depth
}

fn command_parse(commStr: String) -> Result<Command, String> {
    match commStr {
        i if i.starts_with("forward ") => Ok(Command::Forward(
            i["forward ".len()..].parse::<i32>().unwrap(),
        )),
        i if i.starts_with("down ") => {
            Ok(Command::Down(i["down ".len()..].parse::<i32>().unwrap()))
        }
        i if i.starts_with("up ") => Ok(Command::Up(i["up ".len()..].parse::<i32>().unwrap())),
        _ => Err(format!("invalid command: {}", commStr)),
    }
}

fn command_apply(command: Command, current: Position) -> Position {
    match command {
        Command::Up(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth - amount,
        },
        Command::Down(amount) => Position {
            horizontal: current.horizontal,
            depth: current.depth + amount,
        },
        Command::Forward(amount) => Position {
            horizontal: current.horizontal + amount,
            depth: current.depth,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
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

            position = command_apply(command, position)
        }

        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);

        assert_eq!(mul(position), 150);
    }
}
