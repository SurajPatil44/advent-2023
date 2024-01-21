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

    fn is_valid<F : Fn(u8) -> bool>(&self, num: &Number,P : F) -> bool {
        let start = num.start;
        let end = num.end;
        // up
        let st = start.0.saturating_sub(1);
        let first = start.1.saturating_sub(1);
        let second = bound_check_gt(end.1 + 1, self.wd);
        //dbg!(st, first, second);
        if st > 0 {
            let bd = bound_check_gt(second + 1, self.wd);
            for c in self.data[st][first..bd].iter() {
                if P(*c) {
                    //num.valid = true;
                    return true;
                }
            }
        }
        // down
        let ed = start.0 + 1;
        if ed < self.ht {
            let bd = bound_check_gt(second + 1, self.wd);
            for c in self.data[ed][first..bd].iter() {
                if P(*c) {
                    //num.valid = true;
                    return true;
                }
            }
        }

        // sides
        let sl = start.1.saturating_sub(1);
        if sl > 0 {
            if P(self.data[start.0][sl]) {
                return true;
            }
        }

        let sr = end.1 + 1;

        if sr < self.wd {
            if P(self.data[start.0][sr]) {
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
}
fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let grid = Grid::from_file_path(&fname);
    let nums = grid.get_nums();
    let mut sum = 0;
    for num in nums {
        if grid.is_valid(&num,|x| {x != b'.'}) {
            sum += num.Num.parse::<usize>().unwrap();
        } else {
            //dbg!(&num);
        }
    }
    dbg!(&sum);
}
