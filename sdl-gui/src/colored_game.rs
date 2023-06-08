use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileColor {
    None,
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
    Gray,
    Ghost,
}
impl TileColor {
    pub fn from_piece_type(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::O => TileColor::O,
            PieceType::I => TileColor::I,
            PieceType::T => TileColor::T,
            PieceType::L => TileColor::L,
            PieceType::J => TileColor::J,
            PieceType::S => TileColor::S,
            PieceType::Z => TileColor::Z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColoredGame {
    game: Game,
    colors: [[TileColor; BOARD_HEIGHT]; BOARD_WIDTH],
}
impl ColoredGame {
    pub fn new(game: Game) -> Self {
        let mut colors = [[TileColor::None; BOARD_HEIGHT]; BOARD_WIDTH];

        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if game.board.get(i, j) {
                    colors[i][j] = TileColor::Gray;
                }
            }
        }

        ColoredGame { game, colors }
    }
    pub fn game(&self) -> &Game {
        &self.game
    }
    fn simulate_lock(game: Game, colors: &mut [[TileColor; BOARD_HEIGHT]; BOARD_WIDTH]) {
        let (px, py) = game.active.location;
        let shape = game.active.get_shape(None);
        for i in 0..4 {
            for j in 0..4 {
                if !shape[i as usize][j as usize] {
                    continue;
                }
                let x = px + i;
                let y = py + j;
                if x >= 0 && x < BOARD_WIDTH as i8 && y >= 0 && y <= BOARD_HEIGHT as i8 {
                    colors[x as usize][y as usize] =
                        TileColor::from_piece_type(game.active.piece_type);
                }
            }
        }
        let mut lines_cleared = 0;
        for j in 0..BOARD_HEIGHT {
            if (0..BOARD_WIDTH)
                .map(|x| colors[x][j])
                .all(|t| t != TileColor::None)
            {
                lines_cleared += 1;
            } else {
                for i in 0..BOARD_WIDTH {
                    colors[i][j - lines_cleared] = colors[i][j];
                }
            }
            for j in 0..lines_cleared {
                for i in 0..BOARD_WIDTH {
                    colors[i][BOARD_HEIGHT - lines_cleared + j] = TileColor::None;
                }
            }
        }
    }
    pub fn apply_action(&mut self, game_action: GameAction) -> ActionResult {
        match game_action {
            GameAction::Lock => {
                ColoredGame::simulate_lock(self.game, &mut self.colors);
            }
            GameAction::AddGarbage { col, height } => {
                for j in (0..BOARD_HEIGHT).rev() {
                    for i in 0..BOARD_WIDTH {
                        if j < height as usize {
                            if i == col {
                                self.colors[i][j] = TileColor::None;
                            } else {
                                self.colors[i][j] = TileColor::Gray;
                            }
                        } else {
                            self.colors[i][j] = self.colors[i][j - height as usize];
                        }
                    }
                }
            }
            _ => {}
        }
        self.game.apply_action(game_action)
    }
    pub fn make_move(&mut self, game_move: GameMove) -> ActionResult {
        if let GameMove::HardDrop = game_move {
            let mut game = self.game;
            game.apply_action(GameAction::SoftDrop);
            ColoredGame::simulate_lock(game, &mut self.colors);
        }
        self.game.make_move(game_move)
    }
    pub fn refill_queue(&mut self, bag: &mut Bag) {
        self.game.refill_queue(bag);
    }
    pub fn get_tile(&self, x: usize, y: usize) -> TileColor {
        self.colors[x][y]
    }
}
