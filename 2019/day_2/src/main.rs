use std::fs;

fn main() {

  let _inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
  let input: Vec<_> = _inputstring.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

  part_one(input.clone());
  part_two(input.clone());

}


fn part_one(input: Vec<i32>) {
  let response = process_input(input);
  println!("part 1: {}", response.to_string());
}

fn part_two(mut input: Vec<i32>) {
  let result_val = 19690720;
  
  for i in 1..100 {
    for j in 0..100 {
      input[1] = i;
      input[2] = j;
      let result = process_input(input.clone());
      
      if result == result_val {
        let answer = 100 * i + j;
        println!("part 2: {}", answer);
      }
    }
  }
}

fn process_input(mut input: Vec<i32>) -> i32 {
  let input_size = input.len();
  
  for i in (0..input_size as i32).step_by(4) {
    let idx = i as usize;
    let opcode = input[idx as usize];

    let mut a: usize = 0;
    let mut b: usize  = 0;
    let mut result_pos: usize  = 0;

    if input.len()-1 >= idx+1 {
      a = input[(idx+1) as usize] as usize;
    }
    if input.len()-1 >= idx+2 {
      b = input[(idx+2) as usize] as usize;
    }
    if input.len()-1 >= idx+3 {
      result_pos = input[(idx+3) as usize] as usize;
    }

    if opcode == 1 {
      input[result_pos] = input[a] + input[b];
    }
    if opcode == 2 {
      input[result_pos] = input[a] * input[b];
    }
    if opcode == 99 {
      break;
    }
  }

  return input[0];
}