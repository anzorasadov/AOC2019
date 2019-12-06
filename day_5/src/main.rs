use std::fs;

fn main() {
	let _inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
	let input: Vec<_> = _inputstring.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
	
	process_input(input);
}

fn process_input(mut input: Vec<i32>) -> i32 {
		let input_size = input.len();
		let output: Vec<i32> = Vec::new();
    let mut pointer = 0;

    while pointer < input_size {
			let idx = pointer as usize;
			let raw_opcode = input[idx as usize];
			let mut opcode: Vec<i32>;

			if raw_opcode <= 99 {
				opcode = vec!(raw_opcode);
				// opcode = [raw_opcode].to_vec();
			} else {
				opcode = input[idx as usize].to_string().chars().rev().map(|x| x.to_digit(10).unwrap() as i32).collect();
				// reverse first two
			}
			pointer = pointer + 1;

			// dbg!(input[idx as usize], opcode);
  
      // let mut a: usize = 0;
      // let mut b: usize  = 0;
      // let mut result_pos: usize  = 0;
  
      // if input.len()-1 >= idx+1 {
      //   a = input[(idx+1) as usize] as usize;
      // }
      // if input.len()-1 >= idx+2 {
      //   b = input[(idx+2) as usize] as usize;
      // }
      // if input.len()-1 >= idx+3 {
      //   result_pos = input[(idx+3) as usize] as usize;
      // }
  
      // if opcode == 1 {
			// 	input[result_pos] = input[a] + input[b];
			// 	pointer = pointer + 4;
      // }
      // if opcode == 2 {
			// 	input[result_pos] = input[a] * input[b];
			// 	pointer = pointer + 4;
			// }
			// if opcode == 3 {
			// 	input[a] = a as i32;
			// 	pointer = pointer + 1;
			// }
			// if opcode == 4 {
			// 	output.push(a as i32);
			// 	pointer = pointer + 1;
			// }
      // if opcode == 99 {
      //   break;
      // }
    }
  
    return input[0];
  }