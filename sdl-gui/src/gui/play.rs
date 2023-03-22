use std::{thread::sleep, time::Duration};

use crate::*;
use anyhow::Result;
use common::*;
use sdl2::keyboard::Keycode;

enum ShiftDirection {
    None,
    Left,
    Right,
}

const SHIFT_DAS: i32 = 10;
const SHIFT_ARR: i32 = 1;
const DROP_ARR: i32 = 1;
const GRAVITY: i32 = 60;
const LOCK: i32 = 60;

pub struct PlayGui {
    bag: Bag,
    game: ColoredGame,
    window: Window,
    shift: ShiftDirection,
    shift_das: i32,
    shift_arr: i32,
    drop: bool,
    drop_arr: i32,
    gravity: i32,
    lock: i32,
    game_over: bool,
}
impl PlayGui {
    pub fn new() -> Result<Self> {
        let mut bag = Bag::new_rng7(2);
        let game = ColoredGame::new(Game::from_bag(&mut bag));
        let window = Window::new()?;
        Ok(PlayGui {
            bag,
            game,
            window,
            game_over: true,
            shift: ShiftDirection::None,
            drop: false,
            drop_arr: 0,
            shift_das: 0,
            shift_arr: 0,
            gravity: 0,
            lock: 0,
        })
    }
    pub fn run(&mut self) -> Result<()> {
        self.game_over = false;
        self.gravity = GRAVITY;
        self.lock = LOCK;
        while !self.game_over {
            self.tick()?;
            sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
        Ok(())
    }
    fn tick(&mut self) -> Result<()> {
        self.read_input();

        if self.drop {
            if self.drop_arr <= 1 {
                self.game.apply_action(GameAction::ShiftDown);
                self.drop_arr = DROP_ARR;
            } else {
                self.drop_arr -= 1;
            }
        }
        match self.shift {
            ShiftDirection::None => {}
            ShiftDirection::Left | ShiftDirection::Right => {
                if self.shift_das <= 1 {
                    if self.shift_arr <= 1 {
                        if let ShiftDirection::Left = self.shift {
                            self.game.make_move(GameMove::ShiftLeft);
                        } else {
                            self.game.make_move(GameMove::ShiftRight);
                        }
                        self.shift_arr = SHIFT_ARR;
                    } else {
                        self.shift_arr -= 1;
                    }
                } else {
                    self.shift_das -= 1;
                }
            }
        }
        if self.gravity <= 1 {
            self.game.apply_action(GameAction::ShiftDown);
            self.gravity = GRAVITY;
        } else {
            self.gravity -= 1;
        }

        let should_lock = {
            let mut game = *self.game.game();
            game.apply_action(GameAction::ShiftDown) == GameActionRes::Fail
        };
        if should_lock {
            if self.lock <= 1 {
                self.game.apply_action(GameAction::Lock);
                self.lock = LOCK;
                self.gravity = GRAVITY;
            } else {
                self.lock -= 1;
            }
        }

        self.game.refill_queue(&mut self.bag);
        self.window.draw_colored_game(&self.game)?;
        Ok(())
    }
    fn read_input(&mut self) {
        for event in self.window.poll_events() {
            match event {
                GuiEvent::Quit | GuiEvent::KeyDown(Keycode::Q) => {
                    self.game_over = true;
                }
                GuiEvent::KeyDown(Keycode::Left) => {
                    self.game.make_move(GameMove::ShiftLeft);
                    self.shift = ShiftDirection::Left;
                    self.shift_das = SHIFT_DAS;
                    self.shift_arr = 0;
                    self.lock = LOCK;
                }
                GuiEvent::KeyDown(Keycode::Right) => {
                    self.game.make_move(GameMove::ShiftRight);
                    self.shift = ShiftDirection::Right;
                    self.shift_das = SHIFT_DAS;
                    self.shift_arr = 0;
                    self.lock = LOCK;
                }
                GuiEvent::KeyUp(Keycode::Left | Keycode::Right) => {
                    self.shift = ShiftDirection::None;
                }
                GuiEvent::KeyDown(Keycode::Down) => {
                    self.game.apply_action(GameAction::ShiftDown);
                    self.drop = true;
                    self.drop_arr = DROP_ARR;
                    self.lock = LOCK;
                }
                GuiEvent::KeyUp(Keycode::Down) => {
                    self.drop = false;
                }
                GuiEvent::KeyDown(Keycode::Space) => {
                    self.game.make_move(GameMove::HardDrop);
                    self.lock = LOCK;
                    self.gravity = GRAVITY;
                }
                GuiEvent::KeyDown(Keycode::Z) => {
                    self.game.make_move(GameMove::RotateCCW);
                    self.lock = LOCK;
                }
                GuiEvent::KeyDown(Keycode::X) => {
                    self.game.make_move(GameMove::RotateCW);
                    self.lock = LOCK;
                }
                GuiEvent::KeyDown(Keycode::A) => {
                    self.game.make_move(GameMove::Rotate180);
                    self.lock = LOCK;
                }
                GuiEvent::KeyDown(Keycode::C) => {
                    self.game.make_move(GameMove::Hold);
                    self.lock = LOCK;
                }
                _ => {}
            }
        }
    }
}
