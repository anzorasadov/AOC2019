use std::fs;

fn main() {
  let _inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
  let input: Vec<String> = _inputstring.split("\n").map(|s| s.to_string()).collect();
  dbg!(input);
}
