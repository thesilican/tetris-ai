use std::{collections::VecDeque, thread::sleep, time::Duration};

use crate::*;
use anyhow::Result;
use libtetris::*;
use sdl2::keyboard::Keycode;

pub struct AiGui<A: Ai> {
    bag: Bag,
    game: ColoredGame,
    window: Window,
    ai: A,
}

impl<A: Ai> AiGui<A> {
    pub fn new(ai: A) -> Result<Self> {
        let mut bag = Bag::new_rng7(1);
        let game = ColoredGame::new(Game::from_bag(&mut bag));
        let window = Window::new()?;
        Ok(AiGui {
            bag,
            game,
            window,
            ai,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let bag = &mut self.bag;
        let game = &mut self.game;
        let window = &mut self.window;
        let ai = &mut self.ai;

        let mut moves = VecDeque::<Action>::new();
        let mut game_over = false;
        const COOLDOWN: i32 = 1;
        let mut cooldown = COOLDOWN;

        'main: for _ in 0.. {
            for event in window.poll_events() {
                match event {
                    GuiEvent::Quit | GuiEvent::KeyDown(Keycode::Q) => break 'main,
                    _ => {}
                }
            }

            if !game_over {
                if !moves.is_empty() {
                    game.apply(moves.pop_front().unwrap());
                    if game.game().board.topped_out() {
                        game_over = true;
                    }
                } else if cooldown <= 1 {
                    let res = ai.evaluate(game.game());
                    match res {
                        Evaluation::Success { actions, .. } => moves.extend(actions),
                        Evaluation::Fail { .. } => {}
                    }
                    cooldown = COOLDOWN;
                }
                if cooldown > 1 {
                    cooldown -= 1;
                }
            }
            game.refill_queue(bag);
            window.draw_colored_game(game)?;
            sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
        Ok(())
    }
}
