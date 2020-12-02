use std::fs;

fn main() {
	let _inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
	let input: Vec<_> = _inputstring.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
	process_input(input);
}

fn process_input(mut input: Vec<i32>) -> i32 {
	let input_size = input.len();
	let mut output: Vec<i32> = Vec::new();
	let mut pointer = 0;
	const input_param: i32 = 1;

	while pointer < input_size {
		let idx = pointer as usize;
		let raw_instruction = input[idx];

		let instruction: [i32; 4] = parse_instruction(raw_instruction);

		let a: i32 = parse_param(&input, idx+1, instruction[1]);
		let b: i32 = parse_param(&input, idx+2, instruction[2]);
		// let result_pos: i32 = parse_param(&input, idx+3, instruction[3]);
		let result_pos: i32 = input[idx+3];

		// // dbg!(&instruction, a, input[a as usize]);

		// println!(" ");
		// 	dbg!(raw_instruction, a, b, result_pos);

		// 	if raw_instruction == 1100 { break }

		if instruction[0] == 1 {
			input[result_pos as usize] = a + b;
			pointer = pointer + 4;
		}
		if instruction[0] == 2 {
			input[result_pos as usize] = a * b;
			pointer = pointer + 4;
		}
		if instruction[0] == 3 {
			let pos = input[idx+1]; 
			input[pos as usize] = input_param;
			pointer = pointer + 2;
		}
		if instruction[0] == 4 {
			output.push(input[idx+1]);
			pointer = pointer + 2;
		}
		if instruction[0] == 99 {
		  break;
		}
	
		let x = input[0..10].to_vec();
		dbg!(pointer);
		
		// if input.len()-1 >= idx+2 {
		//   b = input[(idx+2) as usize] as usize;
		// }
		// if input.len()-1 >= idx+3 {
		//   result_pos = input[(idx+3) as usize] as usize;
		// }

		
	}

	dbg!(output);

	return input[0];
}

fn parse_param(input: &Vec<i32>, idx: usize, is_immediate: i32) -> i32 {
	let mut param = 0;

	if is_immediate == 0 {
		param = input[input[idx] as usize];
	} else {
		param = input[idx];
	}

	return param;
}

fn parse_instruction(raw_instruction: i32) -> [i32; 4] {
	let mut instruction = [0,0,0,0];

	if raw_instruction < 99 {
		instruction = [raw_instruction,0,0,0];
	} else {
		let mut pre_inst: Vec<String> = raw_instruction.to_string().chars().map(|c| c.to_string()).collect();

		let last = pre_inst.pop().unwrap();
		let mut pre_last = pre_inst.pop().unwrap();
		pre_last.push_str(&last);
		pre_inst.push(pre_last);
		pre_inst.reverse();

		for (idx, d) in pre_inst.iter().enumerate() {
			instruction[idx as usize] = d.parse::<i32>().unwrap();
		}
	}
	
	return instruction;
}