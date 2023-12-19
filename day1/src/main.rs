use std::cmp::Reverse;
use std::collections::BinaryHeap;
///https://adventofcode.com/2023/day/1
use utils::Content;

const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const NUMBERSR: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn take_while2(it: &str, p: impl Fn(char) -> bool) -> (&str, &str) {
    let mut i = 0;
    loop {
        if i >= it.len() {
            break;
        }
        let c = it.as_bytes()[i] as char;
        if p(c) {
            break;
        }
        i += 1
    }
    (&it[..i], &it[i..])
}

fn find_first_alpha_num(haystack: &str, num: &mut usize) {
    if haystack.len() == 0 {
        return;
    }
    let mut pos = BinaryHeap::<Reverse<(usize, usize)>>::new();
    for (ind, num) in NUMBERS.iter().enumerate() {
        let p = haystack.find(num);
        if p.is_some() {
            pos.push(Reverse((p.unwrap(), ind)));
        }
    }
    let an = pos.pop();
    //dbg!(haystack, an);
    if an.is_some() {
        // dbg!(&haystack[an.unwrap()..]);
        let (_, fnum) = an.unwrap().0;
        *num = fnum;
    }
}

fn find_last_alpha_num(haystack: &str, num: &mut usize) {
    if haystack.len() == 0 {
        return;
    }
    let mut pos = BinaryHeap::<Reverse<(usize, usize)>>::new();
    for (ind, num) in NUMBERSR.iter().enumerate() {
        let p = haystack.find(num);
        if p.is_some() {
            pos.push(Reverse((p.unwrap(), ind)));
        }
    }
    let an = pos.pop();
    //dbg!(haystack, an);
    if an.is_some() {
        // dbg!(&haystack[an.unwrap()..]);
        let (_, fnum) = an.unwrap().0;
        *num = fnum;
    }
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let mut sum = 0;
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);
    for l in reader {
        let itr: String = l.chars().rev().collect();
        let (mut first, mut second) = (0, 0);
        let (ss1, ss2) = take_while2(&l, |c| c >= '0' && c <= '9');
        let (ssr1, ssr2) = take_while2(&itr, |c| c >= '0' && c <= '9');
        if ss2.len() > 0 {
            first = (ss2.as_bytes()[0] as u8 - b'0') as usize;
        }
        if ssr2.len() > 0 {
            second = (ssr2.as_bytes()[0] as u8 - b'0') as usize;
        }
        find_first_alpha_num(ss1, &mut first);
        find_last_alpha_num(ssr1, &mut second);
        //dbg!(first, second);
        let num = first * 10 + second;
        sum += num;
    }
    dbg!(sum);
}
