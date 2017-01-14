use std::io::BufRead;
use std::io;
use std::str::SplitWhitespace;

fn vsplit(s: &String) -> Vec<&str> {
    let mut v: Vec<&str> = vec![];
    for word in s.split_whitespace() {
        v.push(word);
    }
    return v;
}

// das geht doch besser oder?
// python:
// def isplit(s):
//     for w in s.split():
//         yield w
// oder sogar:
// def isplit(s):
//     yield from s.split()

// das 'a is voll bescheuert!!!!!
struct Isplit<'a> {
    sw: SplitWhitespace<'a>
}

//   +-- warum muss das 'a dahin?
//   v           v- und nicht hier hin?
impl<'a> Iterator for Isplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        self.sw.next()
    }
}

fn isplit(s: &String) -> Isplit {
    Isplit { sw: s.split_whitespace() }
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ln = line.unwrap();
        for word in isplit(&ln) {
            println!("{}", word);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
    }

}
