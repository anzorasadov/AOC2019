fn main() {
	let min = 138241;
	let max = 674034;
	let mut possible_combinations_one: Vec<i32> = Vec::new();
	let mut possible_combinations_two: Vec<i32> = Vec::new();


	for num in min..max {

		let adjacent_exists = find_adjacent(num.clone());
		let is_asc = get_is_asc(num.clone());
		let two_adjacent_exists = find_two_adjacent(num.clone());
		
		if adjacent_exists && is_asc {
			possible_combinations_one.push(num);
		}

		if two_adjacent_exists && is_asc {
			possible_combinations_two.push(num);
		}

	}

	println!("part one: {}", possible_combinations_one.len());
	println!("part two: {}", possible_combinations_two.len());
}

fn find_adjacent(num: i32) -> bool {
	let digits: Vec<char> = num.to_string().chars().collect();

	for (idx, digit) in digits.iter().enumerate() {
		let next_adjacent = digits.len()-1 > idx && digits[idx+1] == *digit;
		if next_adjacent {
			return true;
		}
	}
	return false;
}

fn find_two_adjacent(num: i32) -> bool {
	let digits: Vec<char> = num.to_string().chars().collect();
	let mut streak = 1;
	
	for (idx, digit) in digits.iter().enumerate() {
		let next_adjacent = digits.len()-1 > idx && digits[idx+1] == *digit;
		
		if next_adjacent {
			streak = streak+1;
		} else if streak == 2 {
			return true;
		} else {
			streak = 1;
		}
	}
	return false;
}

fn get_is_asc(num: i32) -> bool {
	let digits: Vec<i32> = num.to_string().chars().map(|d| d as i32).collect();
	let mut prev = 0;
	for digit in digits {
		if digit < prev {
			return false;
		} else {
			prev = digit;
		}
	}
	return true;
}