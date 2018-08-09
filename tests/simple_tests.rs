extern crate thanot;

use thanot::{add_one, add_two, add_three};

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_add_one_integration_test() {
	    assert_eq!(3, add_one(2));
	}

	#[test]
	fn test_add_one() {
	    assert_eq!(5, add_one(4));
	}

	#[test]
	fn test_add_two() {
	    assert_eq!(5, add_two(3));
	}

	#[test]
	fn test_add_three() {
	    assert_eq!(5, add_three(2));
	}

}