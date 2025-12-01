advent_of_code::solution!(1);

struct Move {
    direction: Direction,
    clicks: u64,
}
impl Move {
    fn parse(line: &str) -> Option<Self> {
        let dir = match Direction::parse(line) {
            Some(dir) => dir,
            None => panic!("Error parsing direction for line {}", line),
        };

        let clicks = line
            .split_at(1)
            .1
            .parse()
            .unwrap_or_else(|e| panic!("Error parsing number: {}", e));

        Some(Move {
            direction: dir,
            clicks,
        })
    }
}
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(line: &str) -> Option<Self> {
        if line.starts_with("R") {
            Some(Direction::Right)
        } else if line.starts_with("L") {
            Some(Direction::Left)
        } else {
            None
        }
    }
}

struct Dial {
    point: u64,
    passwd: u64,
}

impl Dial {
    pub fn new() -> Self {
        Dial {
            point: 50,
            passwd: 0,
        }
    }

    pub fn adjust(&mut self, mv: Move) {
        for _ in 0..mv.clicks {
            if self.point == 0 {
                self.passwd += 1
            }
            match mv.direction {
                Direction::Left => {
                    if self.point == 0 {
                        self.point = 99;
                    } else {
                        self.point -= 1;
                    }
                }
                Direction::Right => {
                    if self.point == 99 {
                        self.point = 0;
                    } else {
                        self.point += 1;
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new();
    let mut passwd = 0;
    for line in input.lines() {
        if let Some(mv) = Move::parse(line) {
            dial.adjust(mv);
        } else {
            println!("Error on line: {}", line)
        }
        if dial.point == 0 {
            passwd += 1;
        }
    }

    Some(passwd)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new();
    for line in input.lines() {
        if let Some(mv) = Move::parse(line) {
            dial.adjust(mv);
        } else {
            println!("Error on line: {}", line)
        }
    }

    Some(dial.passwd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
