use utils::*;

#[derive(Debug)]
struct Grid {
    ht: usize,
    wd: usize,
    data: Vec<Vec<u8>>,
}

#[derive(PartialEq)]
enum State {
    Num,
    Sym,
}

#[inline]
fn bound_check_ls(f: usize, b: usize) -> usize {
    if f < b {
        return b;
    } else {
        return f;
    }
}
#[inline]
fn bound_check_gt(f: usize, b: usize) -> usize {
    if f > b {
        return b;
    } else {
        return f;
    }
}

impl Grid {
    fn from_file_path(fname: &str) -> Grid {
        let mut data: Vec<Vec<u8>> = Vec::new();
        #[allow(unused_mut)]
        let mut reader = Content::read_from_file(fname);
        for line in reader {
            let mut temp: Vec<u8> = Vec::new();
            for b in line.as_bytes() {
                temp.push(*b);
            }
            data.push(temp);
            // temp.clear();
        }
        Grid {
            ht: data.len(),
            wd: data[0].len(),
            data,
        }
    }

    fn get_nums(&self) -> Vec<Number> {
        let mut nums: Vec<Number> = Vec::new();
        let mut cur_state = State::Sym;
        for (i, v) in self.data.clone().into_iter().enumerate() {
            let mut number = Number::default();
            cur_state = State::Sym;
            for (j, b) in v.iter().enumerate() {
                if *b >= b'0' && *b <= b'9' {
                    if cur_state == State::Sym {
                        cur_state = State::Num;
                        number.start = (i, j);
                        number.Num.push(*b as char);
                    } else {
                        number.Num.push(*b as char);
                    }
                } else {
                    if cur_state == State::Num {
                        cur_state = State::Sym;
                        number.end = (i, j - 1);
                        nums.push(number);
                        number = Number::default();
                    }
                }
            }
            if number.Num != "" {
                //dbg!(&number);
                cur_state = State::Num;
                number.end = (i, self.wd);
                // dbg!(&number);
                nums.push(number);
            }
        }
        nums
    }

    fn is_valid<F: Fn(u8) -> bool>(&self, num: &mut Number, P: F) -> bool {
        //
        //     (-1,-1)................................(   -1,end.1+1)
        //     ( 0,-1)(0,0)..............(end.0,end.1)(end.0,end.1,+1)
        //     (+1,-1)................................(   +1,end.1,+1)
        //

        //  up => [num.start.0 - 1][num.start.1 - 1..end.1+1]
        //  down => [num.start.0 + 1][num.start.1 - 1..end.1+1]
        //  left => [num.start.0][num.start.1-1]
        //  right => [num.start.0][num.end+1]

        let start = num.start;
        let end = num.end;
        // up
        let st = start.0.saturating_sub(1);
        let first = start.1.saturating_sub(1);
        let second = bound_check_gt(end.1 + 1, self.wd);
        if st > 0 {
            let bd = bound_check_gt(second + 1, self.wd);
            for (ind, c) in self.data[st][first..bd].iter().enumerate() {
                if P(*c) {
                    num.sLocation = (st, ind + first);
                    return true;
                }
            }
        }
        // down
        let ed = start.0 + 1;
        if ed < self.ht {
            let bd = bound_check_gt(second + 1, self.wd);
            for (ind, c) in self.data[ed][first..bd].iter().enumerate() {
                if P(*c) {
                    num.sLocation = (ed, ind + first);
                    return true;
                }
            }
        }

        // sides
        let sl = start.1.saturating_sub(1);
        if sl > 0 {
            if P(self.data[start.0][sl]) {
                num.sLocation = (start.0, sl);
                return true;
            }
        }

        let sr = end.1 + 1;

        if sr < self.wd {
            if P(self.data[start.0][sr]) {
                num.sLocation = (start.0, sr);
                return true;
            }
        }
        false
    }
}

#[derive(Default, Debug, Clone)]
struct Number {
    Num: String, //change it later
    start: (usize, usize),
    end: (usize, usize),
    sLocation: (usize, usize),
}

impl Number {
    fn overlapping(&self, other: &Number) -> bool {
        self.sLocation.0 == other.sLocation.0 && self.sLocation.1 == other.sLocation.1
    }
}
fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let grid = Grid::from_file_path(&fname);
    let mut nums = grid.get_nums();
    let mut fnums: Vec<_> = vec![];
    for num in nums.iter_mut() {
        if grid.is_valid(num, |c| c == b'*') {
            fnums.push(num);
        }
    }
    let mut overlapping_nums: Vec<(usize, usize)> = vec![];
    for (ind, num1) in fnums.iter().enumerate() {
        for num2 in &fnums[ind + 1..] {
            if num1.overlapping(num2) {
                overlapping_nums.push((
                    num1.Num.parse::<usize>().unwrap(),
                    num2.Num.parse::<usize>().unwrap(),
                ));
            }
        }
    }
    let sum: usize = overlapping_nums.iter().map(|x| x.0 * x.1).sum();
    dbg!(sum);
}
