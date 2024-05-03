use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

fn main() {
    let mut state = MazeState::new(100);
    while !state.is_done() {
        state.show();
        let action = state.random_action();
        state.advance(action);
    }
    state.show();
}


const H: i64 = 3;
const W: i64 = 4;
const END_TURN: u64 = 4;


static DX:  [i64; 4] = [1, -1, 0, 0];
static DY: [i64; 4] = [0, 0, 1, -1];

#[derive(Debug)]
struct MazeState {
    points: Vec<Vec<u64>>,
    tern: u64,
    rng: StdRng,
    pub coord: Coord,
    pub game_score: u64,
}

impl MazeState {
    fn new(seed: u64) -> MazeState {
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        let coord = Coord {
            x: TryInto::<i64>::try_into(rng.next_u64() & 0xffff).unwrap() % W,
            y: TryInto::<i64>::try_into(rng.next_u64() & 0xffff).unwrap() % H,
        };

        let mut points: Vec<Vec<u64>> = (0..W).map(|_| (0..H).map(|_| 0).collect()).collect();
        for i in 0..W {
            for j in 0..H {
                if i == coord.x  && j == coord.y {
                    continue;
                }
                points[i as usize][j as usize] = rng.next_u64() % 10;
            }
        }

        MazeState {
            points,
            coord,
            tern: 0,
            game_score: 0,
            rng,
        }
    }

    fn is_done(&self) -> bool {
        self.tern == END_TURN
    }

    fn advance(&mut self, action: usize) {
        self.coord.x += DX[action];
        self.coord.y += DY[action];
        let point = self.points[TryInto::<usize>::try_into(self.coord.x.abs()).unwrap()][TryInto::<usize>::try_into(self.coord.y.abs()).unwrap()];
        if point > 0 {
            self.game_score += point;
            self.points[TryInto::<usize>::try_into(self.coord.x.abs()).unwrap()][TryInto::<usize>::try_into(self.coord.y.abs()).unwrap()] = 0;
        }
        self.tern += 1;
    }

    fn legal_actions(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        for action in 0..4 {
            let tx = self.coord.x + DX[action];
            let ty = self.coord.y + DY[action];
            if tx >= 0 && tx < W && ty >= 0 && ty < H {
                ret.push(action);
            }
        }
        ret
    }

    fn random_action(&mut self) -> usize {
        let actions = self.legal_actions();
        actions[TryInto::<usize>::try_into(self.rng.next_u64()).unwrap() % actions.len()]
    }

    fn show(&self) {
        println!("tern: {}", self.tern);
        println!("score: {}", self.game_score);
        for i in 0..W {
            for j in 0..H {
                let w = TryInto::<usize>::try_into(i).unwrap();
                let h = TryInto::<usize>::try_into(j).unwrap();
                if self.coord.x == i && self.coord.y == j {
                    print!("@");
                } else if self.points[w][h] > 0 {
                    print!("{}", self.points[w][h]);
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

