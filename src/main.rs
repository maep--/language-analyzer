
use std::io::BufRead;
use std::io;

fn main() {
    let stdin = io::stdin();
    //For each line from stdin
    for line in stdin.lock().lines() {
        //Check if the line was correctly read
        if let Ok(line) = line {
            //Split the line at every whitespace
            //Then Print the terms
            //Iterators in rust are lazy. So the count at the end is only to consume the iterator!
            line.split_whitespace().inspect(|term| println!("{}", term)).count();
        }
    }
    
}
//function for returning a vector of whitespace-splitted words for a given string
fn split_string(buf: &str) -> (Vec<&str>) {
	return buf.split_whitespace().collect();
}

// struct for iterator
struct Stringsplitter {
	strings: Vec<String>,
}

//return a vector for each string of the given vector
impl Iterator for Stringsplitter {
	type Item = Vec<String>;
	
	fn next(&mut self) -> Option<Vec<String>> {
		let len = self.strings.len();
		if len <= 0 {
			None
		}else {
		let vec = self.strings.last().unwrap().split_whitespace().map(ToOwned::to_owned).collect();
		self.strings.remove(len - 1);
		Some(vec)
		}
	}
}



#[cfg(test)]
mod tests {
	use super::split_string;
	use super::Stringsplitter;
	
   
    #[test]
    fn basic_test() {
        //use asserts to check your results
        //Use assert to check for a true value
        assert!(true);
        //Or with an expression
        assert!(1 == 1);
        //Or use assert_eq! for a check of equality
        assert_eq!(1, 1);
    }

	#[test]
	fn split_test() {
		let test: Vec<&str> = split_string("Das ist ein Haus");
		assert_eq!(test, ["Das", "ist", "ein", "Haus"]);
	}
	
	#[test]
	fn split_iterator_test() {
		let mut test = vec!["Das ist ein Haus".to_owned(), "Das ist ein Boot".to_owned(), "Das ist ein Auto".to_owned()];
		let mut iter = Stringsplitter {strings: test};
		assert_eq!(iter.next().unwrap(), vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Auto".to_owned(), ]);
		assert_eq!(iter.next().unwrap(), vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Boot".to_owned(), ]);
		assert_eq!(iter.next().unwrap(), vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Haus".to_owned(), ]);
		assert_eq!(iter.next(), None);
	}
}
