///https://adventofcode.com/2023/day/1
use std::env;
use utils::Content;

fn take_while<'a, I>(it: &mut I, P: impl Fn(char) -> bool) -> (usize, usize)
where
    I: Iterator<Item = char>,
{
    let mut i = 0;
    loop {
        let c = it.next().unwrap();
        if P(c) {
            break;
        }
        i += 1;
    }
    (i, i + 1)
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let mut sum = 0;
    let mut reader = Content::read_from_file(&fname);
    for l in reader {
        let sz = l.len();
        let mut it = l.chars();
        let mut itr = l.chars().rev();
        let (n1, _) = take_while(&mut it, |c| c >= '0' && c <= '9');
        let (n2, _) = take_while(&mut itr, |c| c >= '0' && c <= '9');
        let (first, second) = (l.as_bytes()[n1], l.as_bytes()[sz - 1 - n2]);
        let num = (first as u8 - b'0') as usize * 10 + (second as u8 - b'0') as usize;
        sum += num;
    }
    dbg!(sum);
}
