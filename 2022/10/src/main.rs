use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

enum Instr {
    Noop,
    AddX(i32),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let instr_name = parts.next().expect("missing instruction");
        match instr_name {
            "noop" => Ok(Instr::Noop),
            "addx" => {
                let val = parts.next().expect("addx value")
                    .parse::<i32>().expect("number");
                Ok(Instr::AddX(val))
            },
            _ => Err(())
        }
    }
}

struct Cpu {
    cycle: usize,
    register_x: i32,
    interesting_signals: Vec<i32>,
}

impl Cpu {
    fn handle_instr(&mut self, instr: &Instr) {
        match instr {
            Instr::Noop => {
                self.cycles(1);
            },
            Instr::AddX(v) => {
                self.cycles(2);
                self.register_x += v;
            },
        }
    }

    fn cycles(&mut self, instr_count: usize) {
        self.cycle += 1;

        if self.cycle == 1 {
            print!("cycle {:03} -> ", self.cycle);
        }

        if self.cycle == 20 {
            // eprintln!("cycle: {:?}, interesting signal: {:?}", self.cycle, self.register_x.clone() * self.cycle as i32);
            self.interesting_signals.push(self.register_x.clone() * self.cycle as i32);
        } else if self.cycle > 20 && (self.cycle - 20) % 40 == 0 {
            // eprintln!("cycle: {:?}, interesting signal: {:?}", self.cycle, self.register_x.clone() * self.cycle as i32);
            self.interesting_signals.push(self.register_x.clone() * self.cycle as i32);
        }

        let sprite_pos = self.register_x;
        let crt_pos = (self.cycle % 40) as i32 - 1;
        // eprintln!("sprite_pos: {:?}, crt_pos: {:?}", sprite_pos, crt_pos);
        if sprite_pos >= crt_pos - 1 && sprite_pos <= crt_pos + 1 {
            print!("#");
        } else {
            print!(".");
        }

        if self.cycle % 40 == 0 {
            print!("\n");
            print!("cycle {:03} -> ", self.cycle);
        }

        if instr_count - 1 != 0 {
            return self.cycles(instr_count - 1);
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let mut cpu = Cpu { cycle: 0, register_x: 1, interesting_signals: vec![] };

    for line in reader.lines() {
        let instr: Instr = line?.parse().expect("valid instr");
        cpu.handle_instr(&instr);
    }

    println!("sum of interesting signals: {:?}", cpu.interesting_signals.iter().sum::<i32>());

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_small_program() {
//         let mut cpu = Cpu { cycle: 0, register_x: 0, interesting_signals: vec![] };
//     }
// }
