use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Peekable,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part 1 answer: {}", part1::run("input.txt")?);
    println!("Part 2 answer: \n{}", part2::run("input.txt")?);
    Ok(())
}

mod part1 {
    use super::*;
    pub fn run(filename: &str) -> Result<i32, Box<dyn Error>> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);

        let mut signal_strength = 0;
        let mut machine = Machine {
            x_reg: 1,
            cycle: 1,
            instr: None,
        };
        let mut lines = reader.lines().peekable();
        loop {
            machine.execute_cycle(&mut lines);
            if matches!(machine.cycle, 20 | 60 | 100 | 140 | 180 | 220) {
                signal_strength += machine.cycle * machine.x_reg;
            }
            // If next line is not instruction and not currently executing instruction don't continue execution.
            if machine.instr.is_none() && lines.peek().is_none() {
                break;
            }
        }

        Ok(signal_strength)
    }
}

mod part2 {
    use super::*;
    pub fn run(filename: &str) -> Result<String, Box<dyn Error>> {
        let f = File::open(filename)?;
        let reader = BufReader::new(f);

        let mut machine = Machine {
            x_reg: 1,
            cycle: 1,
            instr: None,
        };
        let mut lines = reader.lines().peekable();
        let mut crt = CRT::default();
        loop {
            crt.draw_next_pixel(machine.x_reg);
            machine.execute_cycle(&mut lines);

            // If next line is not instruction and not currently executing instruction don't continue execution.
            if machine.instr.is_none() && lines.peek().is_none() {
                break;
            }
        }

        let result = {
            crt.data.insert(240, '\n');
            crt.data.insert(200, '\n');
            crt.data.insert(160, '\n');
            crt.data.insert(120, '\n');
            crt.data.insert(80, '\n');
            crt.data.insert(40, '\n');
            crt.data
        };
        Ok(result)
    }
}

enum Instruction {
    Noop,
    Addx(i32, u8),
}
struct Machine {
    x_reg: i32,
    cycle: i32,
    instr: Option<Instruction>,
}
impl Machine {
    fn execute_cycle(&mut self, lines: &mut Peekable<Lines<BufReader<File>>>) {
        if self.instr.is_none() {
            self.instr = Machine::fetch_decode(lines);
        }
        self.execute();
        self.cycle += 1;
    }

    fn fetch_decode(lines: &mut Peekable<Lines<BufReader<File>>>) -> Option<Instruction> {
        let line = lines.next();
        if line.is_none() {
            return None;
        }
        let line = line.unwrap().unwrap();
        match &line[0..4] {
            "noop" => Some(Instruction::Noop),
            "addx" => {
                let num = i32::from_str_radix(&line[5..], 10).unwrap();
                Some(Instruction::Addx(num, 0))
            }
            _ => unimplemented!(),
        }
    }

    fn execute(&mut self) {
        match self
            .instr
            .as_mut()
            .expect("Shouldn't try executing if there is no instruction")
        {
            Instruction::Noop => self.instr = None,
            Instruction::Addx(x, 0) => self.instr = Some(Instruction::Addx(*x, 1)),
            Instruction::Addx(x, 1) => {
                self.x_reg += *x;
                self.instr = None;
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Default)]
struct CRT {
    data: String,
}

impl CRT {
    fn draw_next_pixel(&mut self, x_reg: i32) {
        if (x_reg - 1..=x_reg + 1).contains(&(self.data.len() as i32 % 40)) {
            self.data.push('#');
        } else {
            self.data.push('.');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1::run("inputtest.txt").unwrap(), 13140);
    }

    #[test]
    fn test_part1_ans() {
        assert_eq!(part1::run("input.txt").unwrap(), 14520);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2::run("inputtest.txt").unwrap(),
"\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }

    #[test]
    fn test_part2_ans() {
        assert_eq!(
            part2::run("input.txt").unwrap(),
"\
###..####.###...##..####.####...##.###..
#..#....#.#..#.#..#....#.#.......#.#..#.
#..#...#..###..#......#..###.....#.###..
###...#...#..#.#.##..#...#.......#.#..#.
#....#....#..#.#..#.#....#....#..#.#..#.
#....####.###...###.####.####..##..###..
"
        );
    }
}
