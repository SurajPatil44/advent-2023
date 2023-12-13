use std::fs::File;
use std::io::BufReader;
use std::io::*;
use std::path::Path;

pub struct Content {
    reader: BufReader<File>,
}

impl Content {
    pub fn read_from_file<P: AsRef<Path>>(fname: P) -> Self {
        let f = File::open(fname).unwrap();
        Content {
            reader: BufReader::new(f),
        }
    }
}

pub fn partition<P>(haystack: &P, pin: char) -> (&str, &str)
where
    P: AsRef<str>,
{
    let posm = haystack.as_ref().find(pin);
    match posm {
        Some(pos) => (&haystack.as_ref()[..pos], &haystack.as_ref()[pos + 1..]),
        None => (&haystack.as_ref(), ""),
    }
}

impl Iterator for Content {
    type Item = String; // neew to use cow
    fn next(&mut self) -> Option<String> {
        let mut line = String::new();
        let n = self.reader.read_line(&mut line).unwrap();
        if n == 0 {
            return None;
        } else {
            Some(line[..n - 1].to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iteration_test() {
        let mut content = Content::read_from_file("input.txt");
        assert_eq!(content.next(), Some(String::from("line 1")));
        assert_eq!(content.next(), Some(String::from("line 2")));
    }

    #[test]
    fn take_test() {
        let content = Content::read_from_file("input.txt");
        let mut lines = content.take(2);
        assert_eq!(lines.next(), Some(String::from("line 1")));
        assert_eq!(lines.next(), Some(String::from("line 2")));
    }
    #[test]
    fn partition_test() {
        let string = String::from("a-b,c-d");
        let parts = partition(&string, ',');
        assert_eq!(parts, ("a-b", "c-d"));
    }
}
