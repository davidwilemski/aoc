use std::io::prelude::*;
use std::io::BufReader;

struct Command {
    count: u32,
    from: usize,
    to: usize,
}

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    args.next();
    let num_stacks_arg = args.next()
        .expect("num_stacks missing");
    let num_stacks = num_stacks_arg.parse::<usize>()
        .expect("failed to parse arg");
    let move_multiple = args.next().is_some();

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.reserve(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    let mut commands: Vec<Command> = vec![];

    for line in lines {
        if line.contains("move") {
            let quantity_end = line.find("f").unwrap() - 1;
            let quantity = &line[5..quantity_end].parse::<u32>().unwrap();
            let from_start = quantity_end + 6;
            let from_end = line.find('t').unwrap() - 1;
            let from = &line[from_start..from_end].parse::<usize>().unwrap();
            let to_start = from_end + 4;
            let to = &line[to_start..].parse::<usize>().unwrap();
            commands.push(Command { count: *quantity, from: *from - 1, to: *to - 1 });
            continue;
        }

        if line == "\n" {
            // We have reached the end of the stack definition
            continue;
        }

        let stack_vals = line.chars().enumerate().filter(|(_, c)| c.is_alphabetic());
        // assume we will fine a crate on some stack
        for (idx, crate_val) in stack_vals {
            let stack_idx = //(idx / 3) - 1;
                if (idx / 4) == num_stacks { // I don't think this is needed?
                    // XXX bad hack for handling last stack
                    num_stacks - 1
                } else {
                    idx / 4
                };
            println!("found crate val! idx: {:?}, stack idx: {:?} val: {:?}", idx, stack_idx, crate_val);
            stacks[stack_idx].push(crate_val);
        }
    }
    // once we push all crates on each stack we need to reverse each one because the crates are
    // on a stack (lifo) and we will be inserting the bottom item last.
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    let mesg = stacks.iter().map(|s| s.last()).flatten().collect::<String>();
    println!("msg at start: {}", mesg);
    println!("stacks at start: {:?}", stacks);

    for command in commands {
        if move_multiple {
            let mut moved_stack = vec![];
            for _ in 0..command.count {
                if let Some(c) = stacks[command.from].pop() {
                    moved_stack.push(c);
                }
            }
            moved_stack.reverse();
            stacks[command.to].extend(moved_stack);
        } else {
            for _ in 0..command.count {
                if let Some(c) = stacks[command.from].pop() {
                    stacks[command.to].push(c);
                }
            }
        }
    }

    println!("stacks at end: {:?}", stacks);

    let mesg = stacks.iter().map(|s| s.last()).flatten().collect::<String>();
    println!("message: {:?}", mesg);

    Ok(())
}
