use crate::{colored_game::ColoredGame, TileColor};
use anyhow::{anyhow, Result};
use common::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas, EventPump,
};

pub enum GuiEvent {
    Quit,
    KeyDown(Keycode),
    KeyUp(Keycode),
}

pub struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
}
impl Window {
    pub fn new() -> Result<Self> {
        let sdl_ctx = sdl2::init().map_err(|_e| anyhow!("couldn't initialize sdl context"))?;
        let video = sdl_ctx
            .video()
            .map_err(|_e| anyhow!("couldn't initialize sdl video"))?;
        let window = video
            .window("Tetris", WIDTH as u32, HEIGHT as u32)
            .allow_highdpi()
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_ctx
            .event_pump()
            .map_err(|_e| anyhow!("couldn't get sdl event pump"))?;

        let window = Window { canvas, event_pump };
        Ok(window)
    }
    pub fn poll_events(&mut self) -> Vec<GuiEvent> {
        self.event_pump
            .poll_iter()
            .filter_map(|e| match e {
                Event::Quit { .. } => Some(GuiEvent::Quit),
                Event::KeyDown {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => Some(GuiEvent::KeyDown(keycode)),
                Event::KeyUp {
                    keycode: Some(keycode),
                    repeat: false,
                    ..
                } => Some(GuiEvent::KeyUp(keycode)),
                _ => None,
            })
            .collect()
    }
}

const WHITE: Color = Color::RGB(255, 255, 255);
const LIGHT: Color = Color::RGB(191, 191, 191);
const BLACK: Color = Color::RGB(0, 0, 0);
const COLOR_O: Color = Color::RGB(227, 159, 4);
const COLOR_I: Color = Color::RGB(16, 155, 215);
const COLOR_T: Color = Color::RGB(175, 41, 138);
const COLOR_J: Color = Color::RGB(33, 65, 198);
const COLOR_L: Color = Color::RGB(227, 91, 2);
const COLOR_S: Color = Color::RGB(90, 177, 2);
const COLOR_Z: Color = Color::RGB(215, 15, 55);
const COLOR_GRAY: Color = Color::RGB(106, 106, 106);
const COLOR_GHOST: Color = Color::RGB(191, 191, 191);
const SIZE: i32 = 60;
const WIDTH: i32 = 600;
const HEIGHT: i32 = 750;

impl Window {
    fn get_tile_color(tile: TileColor) -> Option<Color> {
        match tile {
            TileColor::None => None,
            TileColor::O => Some(COLOR_O),
            TileColor::I => Some(COLOR_I),
            TileColor::T => Some(COLOR_T),
            TileColor::L => Some(COLOR_L),
            TileColor::J => Some(COLOR_J),
            TileColor::S => Some(COLOR_S),
            TileColor::Z => Some(COLOR_Z),
            TileColor::Gray => Some(COLOR_GRAY),
            TileColor::Ghost => Some(COLOR_GHOST),
        }
    }
    fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }
    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<()> {
        self.canvas
            .draw_rect(Rect::new(x, y, w as u32, h as u32))
            .map_err(|e| anyhow!("error drawing rect: {e}"))
    }
    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<()> {
        self.canvas
            .fill_rect(Rect::new(x, y, w as u32, h as u32))
            .map_err(|e| anyhow!("error drawing rect: {e}"))
    }
    fn draw_game_ui(&mut self) -> Result<()> {
        // Draw grid
        self.canvas.set_draw_color(LIGHT);
        for i in 5..15 {
            for j in 4..24 {
                self.draw_rect(i * SIZE, j * SIZE, SIZE, SIZE)?;
            }
        }

        // Draw Well
        self.set_draw_color(BLACK);
        self.fill_rect(SIZE * 4, SIZE * 4, SIZE, SIZE * 21)?;
        self.fill_rect(SIZE * 5, SIZE * 24, SIZE * 10, SIZE)?;
        self.fill_rect(SIZE * 15, SIZE * 4, SIZE, SIZE * 21)?;
        Ok(())
    }
    fn draw_board(&mut self, game: &Game) -> Result<()> {
        // Draw board
        self.set_draw_color(COLOR_GRAY);
        for i in 0..10 {
            for j in 0..24 {
                if game.board.get(i as usize, j as usize) {
                    let x = SIZE * 5 + i * SIZE;
                    let y = SIZE * 23 - j * SIZE;
                    self.fill_rect(x, y, SIZE, SIZE)?;
                }
            }
        }
        Ok(())
    }
    fn draw_colored_board(
        &mut self,
        game: &ColoredGame,
        tile_color: Option<TileColor>,
    ) -> Result<()> {
        // Draw board
        for i in 0..10 {
            for j in 0..24 {
                let tile = if let Some(tile) = tile_color {
                    tile
                } else {
                    game.get_tile(i as usize, j as usize)
                };
                if let Some(color) = Window::get_tile_color(tile) {
                    let x = SIZE * 5 + i * SIZE;
                    let y = SIZE * 23 - j * SIZE;
                    self.set_draw_color(color);
                    self.fill_rect(x, y, SIZE, SIZE)?;
                }
            }
        }
        Ok(())
    }
    fn draw_piece(
        &mut self,
        piece_type: PieceType,
        rot: i8,
        x: i32,
        y: i32,
        tile_color: Option<TileColor>,
    ) -> Result<()> {
        let tile = if let Some(tile) = tile_color {
            tile
        } else {
            TileColor::from_piece_type(piece_type)
        };
        if let Some(color) = Window::get_tile_color(tile) {
            self.canvas.set_draw_color(color);
            let grid = Piece::from_piece_type(piece_type).get_shape(Some(rot));
            for i in 0..4 {
                for j in 0..4 {
                    if grid[i as usize][j as usize] {
                        let x = x + i * SIZE;
                        let y = y - j * SIZE;
                        self.canvas
                            .fill_rect(Rect::new(x, y, SIZE as u32, SIZE as u32))
                            .map_err(|e| anyhow!("{e}"))?;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn draw_game(&mut self, game: &Game) -> Result<()> {
        self.canvas.set_draw_color(WHITE);
        self.canvas.clear();

        self.draw_game_ui()?;
        self.draw_board(game)?;

        // Draw ghost piece
        let ghost = {
            let mut game = *game;
            game.make_move(GameMove::SoftDrop);
            game.active
        };
        let x = SIZE * 5 + ghost.location.0 as i32 * SIZE;
        let y = SIZE * 23 - ghost.location.1 as i32 * SIZE;
        self.draw_piece(
            ghost.piece_type,
            ghost.rotation,
            x,
            y,
            Some(TileColor::Ghost),
        )?;

        // Draw current piece
        let active = game.active;
        let x = SIZE * 5 + active.location.0 as i32 * SIZE;
        let y = SIZE * 23 - active.location.1 as i32 * SIZE;
        self.draw_piece(active.piece_type, active.rotation, x, y, None)?;

        // Draw hold
        if let Some(hold) = game.hold {
            let x = 0;
            let y = SIZE * 4;
            self.draw_piece(hold, 0, x, y, None)?;
        }

        // Draw queue
        for (idx, &piece) in game.queue.iter().take(5).enumerate() {
            let x = SIZE * 16;
            let y = SIZE * (8 + idx as i32 * 4);
            self.draw_piece(piece, 0, x, y, None)?;
        }

        self.canvas.present();
        Ok(())
    }
    pub fn draw_colored_game(&mut self, game: &ColoredGame) -> Result<()> {
        self.canvas.set_draw_color(WHITE);
        self.canvas.clear();

        self.draw_game_ui()?;
        self.draw_colored_board(game, None)?;

        // Draw ghost piece
        let ghost = {
            let mut game = *game;
            game.make_move(GameMove::SoftDrop);
            game.game().active
        };
        let x = SIZE * 5 + ghost.location.0 as i32 * SIZE;
        let y = SIZE * 23 - ghost.location.1 as i32 * SIZE;
        self.draw_piece(
            ghost.piece_type,
            ghost.rotation,
            x,
            y,
            Some(TileColor::Ghost),
        )?;

        // Draw current piece
        let active = game.game().active;
        let x = SIZE * 5 + active.location.0 as i32 * SIZE;
        let y = SIZE * 23 - active.location.1 as i32 * SIZE;
        self.draw_piece(active.piece_type, active.rotation, x, y, None)?;

        // Draw hold
        if let Some(hold) = game.game().hold {
            let x = 0;
            let y = SIZE * 4;
            self.draw_piece(hold, 0, x, y, None)?;
        }

        // Draw queue
        for (idx, &piece) in game.game().queue.iter().take(5).enumerate() {
            let x = SIZE * 16;
            let y = SIZE * (8 + idx as i32 * 4);
            self.draw_piece(piece, 0, x, y, None)?;
        }

        self.canvas.present();
        Ok(())
    }
}
