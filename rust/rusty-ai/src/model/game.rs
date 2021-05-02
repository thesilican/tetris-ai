use crate::model::board::Board;
use crate::model::board::BoardDropOptions;
use crate::model::board::BoardDropResult;
use crate::model::board::BoardUndoInfo;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::GAME_MAX_QUEUE_LEN;
use crate::model::piece::Piece;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Clone)]
pub struct Game {
    pub score: i32,
    pub board: Board,
    pub hold: Piece,
    pub queue: [Piece; GAME_MAX_QUEUE_LEN as usize],
    pub queue_ptr: i32,
    pub queue_len: i32,
}
impl Game {
    fn drop_score(res: &GameDropResult) -> i32 {
        // Temporary, to encourage a down-stacking bot
        (match res.lines_cleared {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            _ => panic!("Unable to get drop score for {} lines", res.lines_cleared),
        }) + (match res.perfect_clear {
            true => 10,
            false => 0,
        })
    }
    pub fn new() -> Self {
        Game {
            score: 0,
            board: Board::new(),
            hold: Piece::O,
            queue: [
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
                Piece::O,
            ],
            queue_ptr: 0,
            queue_len: 0,
        }
    }
    pub fn get_curr_piece(&self) -> Result<&Piece, ()> {
        if self.queue_len == 0 {
            return Err(());
        }
        Ok(&self.queue[self.queue_ptr as usize])
    }
    pub fn get_queue_piece(&self, index: i32) -> Result<&Piece, ()> {
        if index >= self.queue_len {
            return Err(());
        }
        let index = (self.queue_ptr + index) % GAME_MAX_QUEUE_LEN;
        let piece = &self.queue[index as usize];
        Ok(piece)
    }

    pub fn drop(
        &mut self,
        options: &GameDropOptions,
    ) -> Result<(GameDropResult, GameUndoInfo), ()> {
        if self.queue_len <= 1 {
            return Err(());
        }
        if options.hold {
            self.swap_hold();
        }
        let piece = &mut self.queue[self.queue_ptr as usize];
        let drop_res = self.board.drop(
            &piece,
            &BoardDropOptions {
                rotation: options.rotation,
                shift: options.shift,
            },
        );
        if let Err(_) = drop_res {
            if options.hold {
                // Undo hold
                self.swap_hold();
            }
            return Err(());
        }
        let (drop_res, board_undo_info) = drop_res.unwrap();
        let game_res = GameDropResult::from_board_drop_result(&drop_res);
        let score = Game::drop_score(&game_res);
        self.score += score;
        self.queue_ptr = (self.queue_ptr + 1) % GAME_MAX_QUEUE_LEN;
        self.queue_len -= 1;
        let game_undo_info = GameUndoInfo {
            hold: options.hold,
            score,
            board: board_undo_info,
        };
        Ok((game_res, game_undo_info))
    }

    pub fn undo(&mut self, undo_info: &GameUndoInfo) {
        self.board.undo(&undo_info.board);
        self.queue_ptr = (self.queue_ptr - 1 + GAME_MAX_QUEUE_LEN) % GAME_MAX_QUEUE_LEN;
        self.queue_len += 1;
        if undo_info.hold {
            self.swap_hold();
        }
        self.score -= undo_info.score;
    }
    fn swap_hold(&mut self) {
        let tmp = self.queue[self.queue_ptr as usize].clone();
        self.queue[self.queue_ptr as usize] = self.hold.clone();
        self.hold = tmp;
    }
    pub fn append_queue(&mut self, piece: Piece) -> Result<(), ()> {
        if self.queue_len == GAME_MAX_QUEUE_LEN {
            return Err(());
        }
        let index = (self.queue_ptr + self.queue_len) % GAME_MAX_QUEUE_LEN;
        self.queue[index as usize] = piece;
        self.queue_len += 1;
        Ok(())
    }
    pub fn extend_queue(&mut self, pieces: Vec<Piece>) {
        for piece in pieces {
            let res = self.append_queue(piece);
            if let Err(_) = res {
                break;
            }
        }
    }
    pub fn clear_queue(&mut self) {
        self.queue_ptr = 0;
        self.queue_len = 0;
    }
    pub fn set_queue(&mut self, pieces: Vec<Piece>) {
        self.clear_queue();
        for piece in pieces {
            let res = self.append_queue(piece);
            if let Err(_) = res {
                break;
            }
        }
    }
    pub fn set_hold(&mut self, piece: Piece) {
        self.hold = piece;
    }
}

#[derive(Clone)]
pub struct GameDropOptions {
    pub rotation: i32,
    pub shift: i32,
    pub hold: bool,
}
pub struct GameDropResult {
    pub lines_cleared: i32,
    pub perfect_clear: bool,
}
impl GameDropResult {
    fn from_board_drop_result(res: &BoardDropResult) -> Self {
        GameDropResult {
            lines_cleared: res.lines_cleared,
            perfect_clear: res.perfect_clear,
        }
    }
}
pub struct GameUndoInfo {
    pub hold: bool,
    pub score: i32,
    pub board: BoardUndoInfo,
}

pub struct GameRNGGenerator {
    rng: StdRng,
}
impl GameRNGGenerator {
    pub fn new(seed: Option<u64>) -> Self {
        GameRNGGenerator {
            rng: StdRng::seed_from_u64(seed.unwrap_or(1)),
        }
    }
    pub fn gen_7bag(&mut self) -> Vec<Piece> {
        let mut pieces = Piece::get_all_pieces();
        let mut i = pieces.len() as u64;
        // Fisher-Yates
        while i > 1 {
            i -= 1;
            let k = Uniform::new(0, i + 1).sample(&mut self.rng);
            let tmp = pieces[k as usize].clone();
            pieces[k as usize] = pieces[i as usize].clone();
            pieces[i as usize] = tmp;
        }
        pieces
    }
    pub fn gen_garbage_line(&mut self) -> u16 {
        let index = Uniform::new(0, BOARD_WIDTH).sample(&mut self.rng);
        let mut line = (1u16 << BOARD_WIDTH) - 1;
        line &= !(1 << index);
        line
    }
}
