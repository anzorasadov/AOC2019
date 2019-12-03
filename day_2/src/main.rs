use std::fs;

fn main() {

  let _inputstring = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

  let mut input: Vec<_> = _inputstring.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
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

  println!("part 1: {}", input[0]);

}
