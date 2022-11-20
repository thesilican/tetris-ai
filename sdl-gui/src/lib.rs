use common::{Ai, Bag, Game, GameAction, GameMove, GenericErr, GenericResult};
use sdl2::{
    event::{Event, EventPollIterator},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    EventPump, Sdl,
};
use std::{collections::VecDeque, thread::sleep, time::Duration};

const WIDTH: i32 = 600;
const HEIGHT: i32 = 750;

pub struct Gui {
    game: Game,
    bag: Bag,
    window: Window,
}
impl Gui {
    pub fn new() -> Self {
        let bag = Bag::new(0);
        let game = Game::from_bag(&bag);
        let window = Window::new().unwrap();
        Gui { bag, game, window }
    }
    pub fn play(&mut self) {
        'main: for tick in 0.. {
            self.game.refill_queue_shuffled(&mut self.bag);
            for event in self.window.poll_events() {
                match event {
                    GuiEvent::Quit => break 'main,
                    GuiEvent::GameMove(game_move) => {
                        self.game.make_move(game_move);
                    }
                }
                sleep(Duration::from_nanos(1_000_000_000 / 60));
            }
            self.window.render(&self.game).unwrap();
        }
    }
    pub fn watch<A: Ai>(&mut self, mut ai: A) {
        let mut timer = 0;
        let mut queue = VecDeque::new();
        'main: loop {
            for event in self.window.poll_events() {
                match event {
                    GuiEvent::Quit => break 'main,
                    _ => {}
                }
            }
            self.game.refill_queue_shuffled(&mut self.bag);
            if timer == 0 {
                match queue.pop_front() {
                    Some(game_move) => {
                        self.game.make_move(game_move);
                        // timer = 1;
                    }
                    None => match ai.evaluate(&self.game) {
                        common::AiRes::Success { moves, score } => {
                            queue.extend(moves);
                            // timer = 1;
                        }
                        common::AiRes::Fail { reason } => {}
                    },
                }
            } else {
                timer -= 1;
            }
            self.window.render(&self.game).unwrap();
            sleep(Duration::from_nanos(1_000_000_000 / 60));
        }
    }
}

#[derive(Debug)]
enum GuiEvent {
    Quit,
    GameMove(GameMove),
}

struct Window {
    canvas: WindowCanvas,
    event_pump: EventPump,
}
impl Window {
    fn new() -> GenericResult<Self> {
        let sdl_ctx = sdl2::init()?;
        let video = sdl_ctx.video()?;
        let window = video
            .window("SDL", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()?;
        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_ctx.event_pump()?;

        let window = Window { canvas, event_pump };
        Ok(window)
    }
    fn render(&mut self, game: &Game) -> GenericResult<()> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        // Draw debug grid
        self.canvas.set_draw_color(Color::RGB(223, 223, 223));
        for i in 0..20 {
            for j in 0..25 {
                self.canvas.draw_rect(Rect::new(i * 30, j * 30, 30, 30))?;
            }
        }

        // Draw Well
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rects(&[
            Rect::new(120, 120, 30, 630),
            Rect::new(150, 720, 300, 30),
            Rect::new(450, 120, 30, 630),
        ])?;

        // Draw board
        self.canvas.set_draw_color(Color::RGB(63, 63, 63));
        for i in 0..10 {
            for j in 0..24 {
                if game.board.get(i as usize, j as usize) {
                    let x = 150 + i * 30;
                    let y = 690 - j * 30;
                    self.canvas.fill_rect(Rect::new(x, y, 30, 30))?;
                }
            }
        }

        // Draw ghost piece
        self.canvas.set_draw_color(Color::RGB(127, 127, 127));
        let ghost_piece = {
            let mut game = *game;
            game.make_move(GameMove::SoftDrop);
            game.current_piece
        };
        let px = ghost_piece.location.0 as i32;
        let py = ghost_piece.location.1 as i32;
        let grid = ghost_piece.get_shape(None);
        for i in 0..4 {
            for j in 0..4 {
                if grid[i as usize][j as usize] {
                    let x = 150 + (i + px) * 30;
                    let y = 690 - (j + py) * 30;
                    self.canvas.fill_rect(Rect::new(x, y, 30, 30))?;
                }
            }
        }

        // Draw current piece
        self.canvas.set_draw_color(Color::RGB(63, 63, 63));
        let px = game.current_piece.location.0 as i32;
        let py = game.current_piece.location.1 as i32;
        let grid = game.current_piece.get_shape(None);
        for i in 0..4 {
            for j in 0..4 {
                if grid[i as usize][j as usize] {
                    let x = 150 + (i + px) * 30;
                    let y = 690 - (j + py) * 30;
                    self.canvas.fill_rect(Rect::new(x, y, 30, 30))?;
                }
            }
        }

        self.canvas.present();
        Ok(())
    }
    fn poll_events<'a>(&'a mut self) -> impl Iterator<Item = GuiEvent> + 'a {
        self.event_pump.poll_iter().filter_map(|e| match e {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => Some(GuiEvent::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => Some(GuiEvent::GameMove(GameMove::SoftDrop)),
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => Some(GuiEvent::GameMove(GameMove::ShiftLeft)),
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => Some(GuiEvent::GameMove(GameMove::ShiftRight)),
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                repeat: false,
                ..
            } => Some(GuiEvent::GameMove(GameMove::HardDrop)),
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                repeat: false,
                ..
            } => Some(GuiEvent::GameMove(GameMove::RotateCCW)),
            Event::KeyDown {
                keycode: Some(Keycode::X),
                repeat: false,
                ..
            } => Some(GuiEvent::GameMove(GameMove::RotateCW)),
            Event::KeyDown {
                keycode: Some(Keycode::C),
                repeat: false,
                ..
            } => Some(GuiEvent::GameMove(GameMove::Hold)),
            _ => None,
        })
    }
}
