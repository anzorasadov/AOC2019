fn main() {
	let min = 138241;
	let max = 674034;
	let mut possible_combinations: Vec<i32> = Vec::new();

	for num in min..max {

		let adjacent_exists = find_adjacent(num.clone());
		let is_asc = get_is_asc(num.clone());
		
		if adjacent_exists && is_asc {
			possible_combinations.push(num);
		}

	}

	println!("part one: {}", possible_combinations.len());
}

fn find_adjacent(num: i32) -> bool {
	let digits: Vec<char> = num.to_string().chars().collect();
	for (idx, digit) in digits.iter().enumerate() {
		if digits.len()-1 > idx && digits[idx+1] == *digit {
			return true;
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