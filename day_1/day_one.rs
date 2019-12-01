use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main () {
  let file = File::open("input.txt").unwrap();
  let reader = BufReader::new(file);
  let mut fuel_requirements = 0;

  for (_, line) in reader.lines().enumerate() {
    let module_mass = line.unwrap().parse::<i32>().unwrap();

    let module_req = get_fuel_req(module_mass);
    fuel_requirements += module_req;
    
    let mut fuel_fuel = module_req;

    while fuel_fuel > 0 {
      let fuel_fuel_req = get_fuel_req(fuel_fuel);
      if fuel_fuel_req >= 0 {
        fuel_requirements += fuel_fuel_req;
      }
      fuel_fuel = fuel_fuel_req;
    }

  }

  println!("spaseship requirements with fuel fuel 4822435: {}", fuel_requirements);

}

fn get_fuel_req(module_mass: i32) -> i32 {
  return module_mass / 3 - 2;
}