use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use itertools::Itertools;

use itertools::traits::HomogeneousTuple;

#[derive(FromPrimitive)]
#[derive(PartialEq)]
enum Opcode {
   Add = 1,
   Mul = 2,
   Halt = 99    	
}

struct IntMachine {
    memory: Vec<usize>,
    pc: usize,
}

impl IntMachine {
    fn run_program(input: &[usize], output: usize) -> usize {
	    let mut int_machine = IntMachine { memory: input.to_vec(), pc: 0 };
	    int_machine.run();
	    int_machine.read(output)
    }

	fn run(&mut self) {
        loop {
        	match self.cur_op() {
        		Opcode::Add => {
        			let (a, b, dst) = self.args();
        			self.memory[dst] = self.memory[a] + self.memory[b];
        			self.pc += 4;
        		},
        		Opcode::Mul => {
        			let (a, b, dst) = self.args();
        			self.memory[dst] = self.memory[a] * self.memory[b];
        			self.pc += 4;
        		},
        		Opcode::Halt => break,

        	}
        }
	}

	fn cur_op(&self) -> Opcode {
		FromPrimitive::from_usize(self.memory[self.pc]).unwrap()
	}

	fn args<T: HomogeneousTuple<Item = usize>>(&self) -> T {
		self.memory[self.pc + 1..].iter().cloned().next_tuple().unwrap()
	}

	// fn margs<'a, T: HomogeneousTuple<Item = usize>>(&'a self) -> &'a T {
	// 	self.memory[self.pc + 1..].iter().next_tuple().unwrap()
	// }

	fn read(&self, pos: usize) -> usize {
		self.memory[pos]
	}
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
	input.split(",").map(|l| l.parse::<usize>().unwrap()).collect_vec()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: & [usize]) -> usize {
	let mut input = input.to_vec();
	input[1] = 12;
	input[2] = 2;
    IntMachine::run_program(&input, 0)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: & [usize]) -> Option<usize> {

    (0..100).map(|noun| (0..100).map(move |verb| {
		let mut input = input.to_vec();
		input[1] = noun;
		input[2] = verb;

        let output = IntMachine::run_program(&input, 0);
        if output == 19690720 {
        	Some(100 * noun + verb)
        } else {
        	None
        }
    })).flatten().filter_map(|x| x).next()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
    	assert_eq!(IntMachine::run_program(&input_generator("1,0,0,0,99"), 0), 2);
    }

    #[test]
    fn example2() {
    	assert_eq!(IntMachine::run_program(&input_generator("2,3,0,3,99"), 3), 6);
    }

    #[test]
    fn example3() {
    	assert_eq!(IntMachine::run_program(&input_generator("2,4,4,5,99,0"), 5), 9801);
    }

    #[test]
    fn example4() {
    	assert_eq!(IntMachine::run_program(&input_generator("1,1,1,4,99,5,6,0,99"), 0), 30);
    }

    #[test]
    fn example5() {
    	assert_eq!(IntMachine::run_program(&input_generator("1,9,10,3,2,3,11,0,99,30,40,50"), 0), 3500);
    }

}