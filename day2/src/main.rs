use utils::*;

#[derive(Debug)]
struct Game {
    red: u32,
    blue: u32,
    green: u32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Game {
    fn from_str(line: &str) -> Self {
        let mut g = Game::default();
        let parts = line.split(',');
        for part in parts {
            let p = rstrip(part);
            let (n, c) = partition(&p, ' ');
            match c {
                "red" => {
                    g.red = n.parse().unwrap();
                }
                "blue" => {
                    g.blue = n.parse().unwrap();
                }
                "green" => {
                    g.green = n.parse().unwrap();
                }
                _ => {
                    dbg!(c);
                    unreachable!();
                }
            }
        }
        g
    }
}

fn check(me: &Game, other: &Game) -> bool {
    me.red <= other.red && me.blue <= other.blue && me.green <= other.green
}

fn main() {
    let mut args = std::env::args();
    let _pgm_name = args.next();
    let fname = args.next().unwrap_or("sample.txt".to_string());
    let other = Game {
        red: 12,
        blue: 14,
        green: 13,
    };
    let mut ids = Vec::new();
    let mut max_runs: Vec<(u32, u32, u32)> = Vec::new();
    #[allow(unused_mut)]
    let mut reader = Content::read_from_file(&fname);
    for line in reader {
        let mut found = false;
        let (mut rd, mut gr, mut bl) = (0, 0, 0);
        let (game_id, line) = partition(&line, ':');
        let (_, game_id) = partition(&game_id, ' ');
        let game_id: u32 = game_id.parse().unwrap();
        for part in line.split(";") {
            let g = Game::from_str(&part);
            if !check(&g, &other) {
                found = true;
            }
            if g.red > rd {
                rd = g.red;
            }
            if g.blue > bl {
                bl = g.blue;
            }
            if g.green > gr {
                gr = g.green;
            }
        }
        if !found {
            ids.push(game_id);
        }
        max_runs.push((rd, gr, bl));
    }
    let sum: u32 = ids.iter().sum();
    dbg!(sum);
    let mut pow = 0;
    for (r, g, b) in &max_runs {
        pow += r * g * b;
    }
    dbg!(pow);
}
