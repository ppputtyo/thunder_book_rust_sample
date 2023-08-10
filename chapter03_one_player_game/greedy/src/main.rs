use rand::Rng;
use rand_mt::{Mt, Mt19937GenRand32};

/// 座標を保持する
struct Coord {
    x: usize,
    y: usize,
}

/// 迷路の高さ
const H: usize = 3;
/// 迷路の幅
const W: usize = 4;
/// ゲーム終了ターン
const END_TURN: usize = 4;

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// 一人ゲームの例
/// 1ターンに上下左右四方向のいずれかに1マスずつ進む。
/// 床にあるポイントを踏むと自身のスコアとなり、床のポイントが消える。
/// END_TURNの時点のスコアを高くすることが目的
struct MazeState {
    /// 床のポイントを1~9で表現する
    points: Vec<Vec<usize>>,
    /// 現在のターン
    turn: usize,
    character: Coord,
    /// ゲーム上で実際に得たスコア
    game_score: usize,
}

impl MazeState {
    fn new(seed: u32) -> Self {
        let mut mt_from_construct = Mt::new(seed); // 盤面構築用の乱数生成器を初期化

        let character_y = mt_from_construct.gen::<u32>() as usize % H;
        let character_x = mt_from_construct.gen::<u32>() as usize % W;

        let character = Coord::new(character_x, character_y);

        let mut points = vec![vec![0; W]; H];

        for y in 0..H {
            for x in 0..W {
                if y == character.y && x == character.x {
                    continue;
                }
                points[y][x] = mt_from_construct.gen::<u32>() as usize % 10;
            }
        }

        Self {
            points,
            turn: 0,
            character,
            game_score: 0,
        }
    }

    /// [どのゲームでも実装する] : ゲームの終了判定
    fn is_done(&self) -> bool {
        self.turn == END_TURN
    }

    /// 右、左、下、上への移動方向のx成分
    const DX: [i32; 4] = [1, -1, 0, 0];
    /// 右、左、下、上への移動方向のy成分
    const DY: [i32; 4] = [0, 0, 1, -1];

    /// [どのゲームでも実装する] : 指定したactionでゲームを1ターン進める
    fn advance(&mut self, action: usize) {
        self.character.x = (self.character.x as i32 + Self::DX[action]) as usize;
        self.character.y = (self.character.y as i32 + Self::DY[action]) as usize;

        let point = &mut self.points[self.character.y][self.character.x];

        self.game_score += *point;
        *point = 0;

        self.turn += 1;
    }

    /// [どのゲームでも実装する] : 現在の状況でプレイヤーが可能な行動を全て取得する
    fn legal_actions(&self) -> Vec<usize> {
        let mut actions = vec![];

        for action in 0..4 {
            let ty = (self.character.y as i32 + Self::DY[action]) as usize;
            let tx = (self.character.x as i32 + Self::DX[action]) as usize;

            if ty < H && tx < W {
                actions.push(action);
            }
        }

        actions
    }

    /// [実装しなくてもよいが実装すると便利] : 現在のゲーム状況を文字列にする
    fn to_string(&self) -> String {
        let mut res = String::new();

        res.push_str(&format!("turn:\t{}\n", self.turn));
        res.push_str(&format!("score:\t{}\n", self.game_score));

        for h in 0..H {
            for w in 0..W {
                if self.character.y == h && self.character.x == w {
                    res.push('@');
                } else if self.points[h][w] > 0 {
                    res.push_str(&self.points[h][w].to_string());
                } else {
                    res.push('.')
                }
            }
            res.push('\n');
        }

        res
    }
}

type State = MazeState;

/// ランダムに行動を決定する
fn random_action(state: &State, mt_for_action: &mut Mt19937GenRand32) -> usize {
    let legal_actions = state.legal_actions();
    legal_actions[mt_for_action.gen::<u32>() as usize % legal_actions.len()]
}

/// シードを指定してゲーム状況を表示しながらAIにプレイさせる。
fn play_game(seed: u32) {
    let mut state = MazeState::new(seed);
    let mut mt_for_action = Mt::new(0); // 行動選択用の乱数生成器を初期化

    println!("{}", state.to_string());

    while !state.is_done() {
        state.advance(random_action(&state, &mut mt_for_action));
        println!("{}", state.to_string());
    }
}

fn main() {
    play_game(/*盤面初期化のシード*/ 121321);
}
