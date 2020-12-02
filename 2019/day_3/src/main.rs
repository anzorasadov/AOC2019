use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let _inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
  let inputs: Vec<String> = _inputstring.split("\n").map(|s| s.to_string()).collect();
  let input_a: Vec<String> = inputs[0].split(",").map(|s| s.to_string()).collect();
  let input_b: Vec<String> = inputs[1].split(",").map(|s| s.to_string()).collect();
  
  let path_a: Vec<_> = tag_path(input_a);
  let path_b: Vec<_> = tag_path(input_b);

  let path_a_hash: HashSet<_> = path_a.clone().into_iter().collect();
  let path_b_hash: HashSet<_> = path_b.clone().into_iter().collect();

  let intersections: HashSet<_> = path_a_hash.intersection(&path_b_hash).collect();
  
  part_one(intersections.clone());
  part_two(path_a.clone(), path_b.clone(), intersections.clone());

}

fn part_two(path_a: Vec<Point>, path_b: Vec<Point>, intersections: HashSet<&Point>) {
  let mut shortest_combined_steps = 100000;
  
  for int in intersections {
    let a_steps = traverse_path(path_a.clone(), int);
    let b_steps = traverse_path(path_b.clone(), int);
    let combined_steps = a_steps + b_steps;

    if combined_steps != 0 && combined_steps < shortest_combined_steps {
      shortest_combined_steps = combined_steps;
    }
  }

  println!("part two: {}", shortest_combined_steps);
}

fn traverse_path(path: Vec<Point>, destination: &Point) -> i32  {
  for (idx, pointer) in path.iter().enumerate() {
    if pointer == destination {
      return idx as i32;
      // return idx.parse::<i32>();
    }
  }
  return 0;
}

fn part_one(intersections: HashSet<&Point>) {
  let mut shortest_manhattan_distance = 10000;
  for i in intersections {
    let manhattan_distance = i.x.abs() + i.y.abs();
    if manhattan_distance > 0 && manhattan_distance < shortest_manhattan_distance {
      shortest_manhattan_distance = manhattan_distance;
    }
  }
  println!("part one: {}", shortest_manhattan_distance);
}

fn tag_path(input: Vec<String>) -> Vec<Point> {
  let mut pointer = Point{ x: 0, y: 0 };
  let mut path: Vec<Point> = Vec::new();
  
  for step in input {
    let dir = step.chars().nth(0).unwrap();
    let step_count = &step[1..step.len()].parse::<i32>().unwrap();

    if dir == 'U' {
      for i in 0..*step_count {
        let t = Point { x: pointer.x, y: pointer.y + i };
        path.push(t);
      }
      pointer.y += step_count;
    }
    if dir == 'D' {
      for i in 0..*step_count {
        let t = Point { x: pointer.x, y: pointer.y - i };
        path.push(t);
      }
      pointer.y -= step_count;
    }
    if dir == 'R' {
      for i in 0..*step_count {
        let t = Point { x: pointer.x + i, y: pointer.y };
        path.push(t);
      }
      pointer.x += step_count;
    }
    if dir == 'L' {
      for i in 0..*step_count {
        let t = Point { x: pointer.x - i, y: pointer.y };
        path.push(t);
      }
      pointer.x -= step_count;
    }
  }

  return path
}
