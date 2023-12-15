///https://adventofcode.com/2023/day/1
use utils::Content;


fn take_while2(it : &str,p: impl Fn(char) -> bool) -> (&str,&str) {
	let mut i = 0;
	loop {
		if i >= it.len(){
			break;
		}
		let c = it.as_bytes()[i] as char;
		if p(c) {
			break;
		}
		i+=1
	}
	(&it[..i],&it[i..])
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let mut sum = 0;
	#[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);
    for l in reader {
        let itr : String = l.chars().rev().collect();
		let (ss1,ss2) = take_while2(&l,|c| c >= '0' && c <= '9');
		let (ssr1,ssr2) = take_while2(&itr,|c| c >= '0' && c <= '9');
		let num = (ss2.as_bytes()[0] as u8 - b'0') as usize * 10 + (ssr2.as_bytes()[0] as u8 - b'0') as usize;
        sum += num;
    }
    dbg!(sum);
}
