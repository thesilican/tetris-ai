use libtetris::*;

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

    fn paint_active_piece(&mut self) {
        let game = &self.game;
        let colors = &mut self.colors;
        let px = game.active.position_x;
        let py = game.active.position_y;
        let shape = PieceInfo::shape(game.active.piece_type, game.active.rotation);
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

    pub fn apply(&mut self, action: Action) -> ActionInfo {
        match action {
            Action::Lock => {
                self.paint_active_piece();
                self.game.apply(Action::Lock)
            }
            Action::HardDrop => {
                self.game.apply(Action::SoftDrop);
                self.paint_active_piece();
                self.game.apply(Action::Lock)
            }
            action => self.game.apply(action),
        }
    }

    pub fn refill_queue(&mut self, bag: &mut Bag) {
        self.game.refill_queue(bag);
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileColor {
        self.colors[x][y]
    }
}
