use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Movement {
    direction: Direction,
    distance: i32,
}

impl Movement {
    fn new(direction: Direction, distance: i32) -> Self {
        Self { direction, distance }
    }

    fn decrement_distance(&self) -> Self {
        Self { direction: self.direction.clone(), distance: self.distance - 1 }
    }
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let direction = parts.next().expect("missing direction");
        let distance = parts.next().expect("missing distance")
            .parse::<i32>().expect("couldn't parse distance");

        let direction = match direction {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        };

        direction.map(|dir| Self { direction: dir, distance })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Position{
    x: i32,
    y: i32,
}

impl Position {
    fn do_movement(&self, movement: &Movement) -> Self {
        match movement.direction {
            Direction::Up => {
                Position { x: self.x, y: self.y + movement.distance}
            },
            Direction::Down => {
                Position { x: self.x, y: self.y - movement.distance}
            },
            Direction::Left => {
                Position { x: self.x - movement.distance, y: self.y }
            },
            Direction::Right => {
                Position { x: self.x + movement.distance, y: self.y }
            },
        }
    }

    fn move_one(&self, movement: &Movement) -> Self {
        // Move one space instead of the distance specified
        self.do_movement(&Movement { direction: movement.direction.clone(), distance: 1 })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Rope {
    head: Position,
    tail: Position,
}

impl Rope {
    fn start() -> Self {
        Self {
            head: Position { x: 0, y: 0 },
            tail: Position { x: 0, y: 0 },
        }
    }

    fn new(head: Position, tail: Position) -> Self {
        Self { head, tail }
    }

    fn do_movement(&self, movement: &Movement, tail_positions: &mut BTreeSet<Position>) -> Self {
        // eprintln!("movement: {:?}", movement);
        // eprintln!("before: {:?}", self);
        // eprintln!("tail_positions: {:?}", tail_positions);
        let head_pos = self.head.move_one(movement);
        let mut rope = Self { head: head_pos, tail: self.tail.clone() };
        rope = Self { head: rope.head.clone(), tail: rope.tail_move(movement) };
        // tail_positions.push(rope.tail.clone());
        tail_positions.insert(rope.tail.clone());
        let new_movement = movement.decrement_distance();
        if new_movement.distance > 0 {
            rope = rope.do_movement(&new_movement, tail_positions);
        }
        // eprintln!("after: {:?}", rope);
        rope
    }

    fn tail_move(&self, movement: &Movement) -> Position {
        let diff_x = self.diff_x();
        let diff_y = self.diff_y();
        // eprintln!("diff_x: {}, diff_y: {}", diff_x, diff_y);
        // eprintln!("ROPE: {:?}", self);

        if self.touching_not_diagonal() || self.tail_diagonal() {
            // No tail movement
            self.tail.clone()
        // may also need to move above move_one block below this one
        // // XXX check if not same row AND not same col AND diff x or y is 2
        // } else if !self.tail_diagonal() && !self.touching_not_diagonal() {
        // } else if self.tail_diagonal() {
           } else if self.need_adjust_diag() {
            // Otherwise we need to adjust in multiple directions
            let rope = Rope::new(
                // self.head.move_one(movement),
                self.head.clone(),
                self.tail.clone()
            );
            if !rope.touching_not_diagonal() && !rope.tail_diagonal() && !rope.touching_overlap() {
                let tail = self.tail.move_one(movement);
                match movement.direction {
                    Direction::Up | Direction::Down => {
                        if diff_x > 0 {
                            // eprintln!("moving tail up and over one");
                            Position { x: tail.x + 1, y: tail.y }
                        } else if diff_x < 0 {
                            Position { x: tail.x - 1, y: tail.y }
                        } else {
                            panic!("unexpected handling");
                        }
                    },
                    Direction::Left | Direction::Right => {
                        if diff_y > 0 {
                            Position { x: tail.x, y: tail.y + 1 }
                        } else if diff_y < 0 {
                            Position { x: tail.x, y: tail.y - 1 }
                        } else {
                            panic!("unexpected handling");
                        }
                    },
                }
            } else {
                rope.tail
            }
        } else if !self.tail_diagonal() && (diff_x.abs() > 1 || diff_y.abs() > 1) {
            self.tail.move_one(movement)
        } else if self.touching_overlap() {
            self.tail.clone()
        } else {
            panic!("head and tail not touching!");
        }
    }

    fn touching_overlap(&self) -> bool {
        self.head == self.tail
    }

    fn touching_not_diagonal(&self) -> bool {
        let diff_x = self.diff_x().abs();
        let diff_y = self.diff_y().abs();
        let result = (diff_x == 1 && diff_y == 0) || (diff_x == 0 && diff_y == 1);
        // let result = (self.tail.x == self.head.x && self.tail.y != self.head.y) ||
        //     (self.tail.x != self.head.x && self.tail.y == self.head.y) ||
        //         self.tail == self.head;
        // eprintln!("touching_not_diagonal({:?}) -> {:?}", self, result);
        result
    }

    // We are diagonal if neither x nor y position are equal
    fn tail_diagonal(&self) -> bool {
        let diff_x = self.diff_x().abs();
        let diff_y = self.diff_y().abs();
        let result = self.tail.x != self.head.x && self.tail.y != self.head.y &&
            (diff_y == 1 && diff_x == 1);
        // eprintln!("tail_diagonal({:?}) -> {:?}", self, result);
        result
    }

    // check if not same row AND not same col AND diff x or y is 2
    fn need_adjust_diag(&self) -> bool {
        self.tail.x != self.head.x &&
            self.tail.y != self.head.y &&
            (self.diff_x().abs() == 2 || self.diff_y().abs() == 2)
    }

    fn diff_x(&self) -> i32 {
        self.head.x - self.tail.x
    }

    fn diff_y(&self) -> i32 {
        self.head.y - self.tail.y
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let movements: Vec<Movement> = reader
        .lines()
        .map(|l| {
            l.expect("line").parse().unwrap()
        })
        .collect();
    let mut rope = Rope::start();
    let mut tail_positions: BTreeSet<Position> = BTreeSet::new();
    // let mut tail_positions: Vec<Position> = vec![];
    // tail_positions.insert(rope.tail.clone()); // add initial position
    tail_positions.insert(rope.tail.clone()); // add initial position

    for movement in movements {
        eprintln!("!!! BEFORE MOVEMENT: {:?}", rope);
        rope = rope.do_movement(&movement, &mut tail_positions);
        eprintln!("!!! AFTER MOVEMENT: {:?}", rope);
    }

        println!("number of tail positions: {:?}", tail_positions.len());
        // println!("tail positions: {:?}", tail_positions);

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_right_initial_position() {
        let rope = Rope::start();
        let movement = Movement::new(Direction::Right, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 1, y: 0 }, tail: Position { x: 0, y: 0 } }
        )
    }

    #[test]
    fn test_move_up_initial_position() {
        let rope = Rope::start();
        let movement = Movement::new(Direction::Up, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 0, y: 1 }, tail: Position { x: 0, y: 0 } }
        )
    }

    #[test]
    fn test_move_up_then_right_initial_position() {
        let rope = Rope::start();
        let mut movement = Movement::new(Direction::Up, 1);

        let rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            rope,
            Rope { head: Position { x: 0, y: 1 }, tail: Position { x: 0, y: 0 } }
        );

        movement = Movement::new(Direction::Right, 1);
        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 1, y: 1 }, tail: Position { x: 0, y: 0 } }
        );
    }

    #[test]
    fn test_follow_move_down() {
        let rope = Rope { head: Position { x: 0, y: 2 }, tail: Position { x: 0, y: 3 } };
        let movement = Movement::new(Direction::Down, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 0, y: 1 }, tail: Position { x: 0, y: 2 } }
        );
    }

    #[test]
    fn test_tail_no_movement_when_head_lands_on_tail() {
        let rope = Rope { head: Position { x: 0, y: 2 }, tail: Position { x: 0, y: 1 } };
        let movement = Movement::new(Direction::Down, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 0, y: 1 }, tail: Position { x: 0, y: 1 } }
        );
    }

    /*
    Rope { head: Position { x: 2, y: 4 }, tail: Position { x: 3, y: 3 } }
    diff_x: 2, diff_y: -1
    touching_not_diagonal(Rope { head: Position { x: 1, y: 4 }, tail: Position { x: 3, y: 3 } }) -> false
    tail_diagonal(Rope { head: Position { x: 1, y: 4 }, tail: Position { x: 3, y: 3 } }) -> false
    tail_diagonal(Rope { head: Position { x: 1, y: 4 }, tail: Position { x: 3, y: 3 } }) -> false
    Rope { head: Position { x: 1, y: 4 }, tail: Position { x: 2, y: 3 } }

    Before:
    ===========
    ..H.
    ...T
    ....
    ....
    ....

    Head Move:
    ===========
    .H..
    ...T
    ....
    ....
    ....

    After:
    ==========
    .H..
    ..T.
    ....
    ....
    ....

    Expected:
    ==========
    .HT.
    ....
    ....
    ....
    ....
    */
    #[test]
    fn test_move_left_tail_diagonal_down_right() {
        let rope = Rope { head: Position { x: 2, y: 4 }, tail: Position { x: 3, y: 3 } };
        let movement = Movement::new(Direction::Left, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 1, y: 4 }, tail: Position { x: 2, y: 4 } }
        )
    }

    #[test]
    fn test_move_up_tail_diagonal_up_right() {
        let rope = Rope { head: Position { x: 4, y: 1 }, tail: Position { x: 3, y: 0 } };
        let movement = Movement::new(Direction::Up, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 4, y: 2 }, tail: Position { x: 4, y: 1 } }
        )
    }

    #[test]
    fn test_move_up_tail_diagonal_up_up() {
        let rope = Rope { head: Position { x: 4, y: 0 }, tail: Position { x: 3, y: 0 } };
        let movement = Movement::new(Direction::Up, 2);
        let mut tail_positions = BTreeSet::new();

        let new_rope = rope.do_movement(&movement, &mut tail_positions);

        assert_eq!(tail_positions, BTreeSet::from([ Position { x: 3, y: 0 }, Position {x: 4, y: 1} ]));
        assert_eq!(
            new_rope,
            Rope { head: Position { x: 4, y: 2 }, tail: Position { x: 4, y: 2 } }
        );
    }

    #[test]
    fn test_move_down_tail_diagonal_left_down() {
        let rope = Rope { head: Position { x: 4, y: 1 }, tail: Position { x: 5, y: 2 } };
        let movement = Movement::new(Direction::Down, 1);

        let new_rope = rope.do_movement(&movement, &mut BTreeSet::new());

        assert_eq!(
            new_rope,
            Rope { head: Position { x: 4, y: 0 }, tail: Position { x: 4, y: 1 } }
        )
    }
}
