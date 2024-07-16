use std::fmt;
use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Field {
    s: Vec<Vec<bool>>,
    w: usize,
    h: usize,
}

impl Field {
    fn new(w: usize, h: usize) -> Field {
        let s = vec![vec![false; w]; h];
        Field { s, w, h }
    }

    fn set(&mut self, x: usize, y: usize, b: bool) {
        self.s[y][x] = b;
    }

    fn alive(&self, x: isize, y: isize) -> bool {
        let x = ((x % self.w as isize) + self.w as isize) as usize % self.w;
        let y = ((y % self.h as isize) + self.h as isize) as usize % self.h;
        self.s[y][x]
    }

    fn next(&self, x: usize, y: usize) -> bool {
        let mut alive = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if (j != 0 || i != 0) && self.alive(x as isize + i, y as isize + j) {
                    alive += 1;
                }
            }
        }
        alive == 3 || alive == 2 && self.alive(x as isize, y as isize)
    }
}

struct Life {
    a: Field,
    b: Field,
    w: usize,
    h: usize,
}

impl Life {
    fn new(w: usize, h: usize) -> Life {
        let mut a = Field::new(w, h);
        let mut rng = rand::thread_rng();
        for _ in 0..(w * h / 4) {
            a.set(rng.gen_range(0..w), rng.gen_range(0..h), true);
        }
        Life {
            a,
            b: Field::new(w, h),
            w,
            h,
        }
    }

    fn step(&mut self) {
        for y in 0..self.h {
            for x in 0..self.w {
                self.b.set(x, y, self.a.next(x, y));
            }
        }
        std::mem::swap(&mut self.a, &mut self.b);
    }
}

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let c = if self.a.alive(x as isize, y as isize) { '*' } else { ' ' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn run() {
    let mut l = Life::new(40, 15);
    for _ in 0..300 {
        l.step();
        print!("{}[2J", 27 as char); // Clear screen
        println!("{}", l);
        thread::sleep(Duration::from_millis(33));
    }
}