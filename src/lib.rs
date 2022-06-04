use std::cmp;

use rand::Rng;

#[derive(Clone)]
pub enum Tile {
    Empty(u8),
    Bomb,
}

impl From<&Tile> for &'static str {
    fn from(t: &Tile) -> Self {
        const CHARS: [&str; 9] = ["⬜", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];
        match t {
            Tile::Empty(v) => CHARS[*v as usize],
            Tile::Bomb => "💣",
        }
    }
}

#[derive(Clone)]
pub struct Grid<E> {
    inner: Vec<Vec<E>>,
    reveal: Vec<Vec<bool>>,
    height: usize,
    width: usize,
}

impl std::fmt::Display for Grid<Tile> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiles = self
            .view()
            .into_iter()
            .map(|row| row.join(" "))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", tiles)
    }
}

impl Grid<Tile> {
    pub fn new(height: usize, width: usize) -> Grid<Tile> {
        let inner = vec![vec![Tile::Empty(0); width]; height];
        let reveal = vec![vec![false; width]; height];

        Grid {
            inner,
            reveal,
            height,
            width,
        }
    }

    /// Place a bunch of bombs randomly
    pub fn bombify(&mut self, diff: usize) {
        let bombs = cmp::max(1, (self.height * self.width * diff) / 100);

        let mut rng = rand::thread_rng();
        for _ in 0..bombs {
            // TODO fix dupe bombs on top of each other
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            self.bomb(y, x);
        }
    }

    /// Place a bomb
    fn bomb(&mut self, y: usize, x: usize) {
        let lower = |v: usize| {
            if v > 0 {
                v - 1
            } else {
                v
            }
        };

        if let Tile::Bomb = self.inner[y][x] {
            return;
        }
        self.inner[y][x] = Tile::Bomb;

        for i in lower(y)..=y + 1 {
            for j in lower(x)..=x + 1 {
                if i < self.height && j < self.width {
                    if let Tile::Empty(v) = self.inner[i][j] {
                        self.inner[i][j] = Tile::Empty(v + 1);
                    }
                }
            }
        }
    }

    /// Reveal a random piece
    pub fn reveal_rnd(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);

        // Ugly hack to only reveal empty pieces
        if self.reveal(y, x).is_err() {
            self.reveal_rnd(); // try again
        }
    }

    /// Try to reveal a piece, making eligible neighbouring ones visible too
    fn reveal(&mut self, y: usize, x: usize) -> Result<(), ()> {
        match self.inner[y][x] {
            Tile::Bomb => {
                return Err(());
            }
            Tile::Empty(v) => {
                if v != 0 {
                    return Err(());
                }
            }
        }

        let mut q = std::collections::vec_deque::VecDeque::new();
        let mut been = vec![vec![false; self.width]; self.height];

        q.push_back((y, x));
        been[y][x] = true;

        while !q.is_empty() {
            let top = q.pop_back().unwrap();
            let (y, x) = top;

            let tile = &self.inner[y][x];
            match tile {
                Tile::Bomb => {
                    // shouldnt happen
                    panic!();
                }
                Tile::Empty(v) => {
                    self.reveal[y][x] = true;
                    if *v != 0 {
                        continue;
                    }

                    let lower = |v: usize| {
                        if v > 0 {
                            v - 1
                        } else {
                            v
                        }
                    };

                    for i in lower(y)..=y + 1 {
                        for j in lower(x)..=x + 1 {
                            if i >= self.height || j >= self.width {
                                continue;
                            }

                            if been[i][j] {
                                continue;
                            }
                            been[i][j] = true;

                            q.push_back((i, j));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Player POV
    fn view(&self) -> Vec<Vec<String>> {
        let mut out = vec![vec![String::new(); self.width]; self.height];

        for i in 0..self.height {
            for j in 0..self.width {
                let encoded: &'static str = (&self.inner[i][j]).into();

                let mut s = String::new();
                if self.reveal[i][j] {
                    s.push_str(encoded);
                } else {
                    s.push_str("||");
                    s.push_str(encoded);
                    s.push_str("||");
                }

                out[i][j].push_str(&s);
            }
        }

        out
    }
}
