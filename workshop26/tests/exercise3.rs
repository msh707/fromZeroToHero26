pub mod math {
	    pub fn multiply(a: i32, b: i32) -> i32 {
	        a * b
	    }
	}
	#[cfg(test)]
	mod tests {
        use super::math;
	    #[test]
	    fn multiply_two_numbers() {
	        assert_eq!(math::multiply(3, 4), 12);
	    }
	}
