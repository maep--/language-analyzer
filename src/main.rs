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


#[cfg(test)]
mod tests {
   
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

}
