fn parse_positive_number(s: &str) -> Result<u32, String> {    
 let n: i32 = s.parse().map_err(|_| "Not a number".to_string())?;    
 	if n > 0 {        
 	Ok(n as u32)   
 	} else {        
		Err("Not positive".to_string())   
 	}
}
	#[cfg(test)]
	mod tests {
	    use super::*;
	
	    #[test]
	    fn parses_positive_number() {
	        let result = parse_positive_number("42");
	        assert!(result.is_ok());
	        assert_eq!(result.unwrap(), 42);
	    }

	#[test]
	    fn rejects_negative_number() {
	        let result = parse_positive_number("-1");
	        assert!(result.is_err());
	    }
	}
