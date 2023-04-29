extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::collections::HashMap;
use std::time::Duration;

type State = u64;
type AutomatonRules = Vec<HashMap<i64, State>>;

#[macro_export]
macro_rules! transitions {
    ($default: expr, $( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map.insert(-1, $default);
         map
    }}
}

#[derive(Clone)]
pub struct Automaton {
    pub colors: Vec<Color>,
    pub rules: AutomatonRules,
}

impl Automaton {
    pub fn new(rules: AutomatonRules, colors: Vec<Color>) -> Self {
        Automaton { rules, colors }
    }
}

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub curr_state: usize,
    pub next_state: usize,
    pub automaton: Automaton,
    pub states: Vec<Vec<State>>,
}

impl Board {
    pub fn new(automaton: Automaton, width: i32, height: i32) -> Self {
        let len = (width * height) as usize;
        let curr_state = vec![0; len];
        let next_state = vec![0; len];
        Board {
            width,
            height,
            curr_state: 0,
            next_state: 1,
            automaton,
            states: vec![curr_state, next_state],
        }
    }

    pub fn fill_random(&mut self, count: usize, spread: f64, val: Option<u8>) {
        let mut rng = rand::thread_rng();
        let hw = ((self.width / 2) as f64 * spread) as usize;
        let hh = ((self.height / 2) as f64 * spread) as usize;

        for _i in 0..count {
            let mut xx = (rng.gen_range(0..hw) as i64) as i32;
            let mut yy = (rng.gen_range(0..hh) as i64) as i32;

            xx += (self.width / 2) - hw as i32 / 2;
            yy += (self.height / 2) - hh as i32 / 2;

            let idx = (yy * self.width + xx) as usize;
            self.states[self.curr_state][idx] = if val.is_some() {
                *val.as_ref().unwrap() as State
            } else {
                rng.gen_range(0..self.automaton.rules.len()) as State
            };
        }
    }

    pub fn compute_next(&mut self) {
        let mut idx: usize = 0;
        let curr_state = self.curr_state;
        let next_state = self.next_state;

        for y in 0..self.height {
            for x in 0..self.width {
                let neighb = self.get_nbors(x, y) as i64;
                let curr = self.get_curr_cell(x, y);
                let next = self.automaton.rules[curr as usize]
                    .get(&neighb)
                    .unwrap_or_else(|| {
                        self.automaton.rules[curr as usize]
                            .get(&-1)
                            .expect("must have -1 key")
                    });
                self.states[next_state][idx] = *next;
                idx += 1;
            }
        }

        self.curr_state = next_state;
        self.next_state = curr_state;
    }

    pub fn set_curr_cell(&mut self, xx: i32, yy: i32, val: State) {
        self.states[self.curr_state][(yy * self.width + xx) as usize] = val;
    }

    pub fn get_curr_cell(&self, xx: i32, yy: i32) -> State {
        let xr = if xx < 0 {
            xx + self.width
        } else if xx >= self.width {
            xx - self.width
        } else {
            xx
        };
        let yr = if yy < 0 {
            yy + self.height
        } else if yy >= self.height {
            yy - self.height
        } else {
            yy
        };
        let idx = (yr * self.width) + xr;
        self.states[self.curr_state][idx as usize]
    }

    // Returns integer-encoded form for the neighbors, one digit per state
    fn get_nbors(&mut self, x: i32, y: i32) -> i32 {
        let mut val: i32 = 0;
        for yy in y - 1..=y + 1 {
            for xx in x - 1..=x + 1 {
                if xx - x == 0 && yy - y == 0 {
                    continue;
                }
                let state = self.get_curr_cell(xx, yy);
                val += (10 as u32).pow(state as u32) as i32;
            }
        }
        val
    }
}

pub fn visualize(board: &mut Board, cell_size: u32, cb: &mut dyn FnMut(&mut Board)) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "rutomaton",
            board.width as u32 * cell_size,
            board.height as u32 * cell_size,
        )
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        (cb)(board);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for cy in 0..board.height {
            for cx in 0..board.width {
                let color = board.automaton.colors[board.get_curr_cell(cx, cy) as usize];

                let x = cx * cell_size as i32;
                let y = cy * cell_size as i32;
                canvas.set_draw_color(color);
                canvas
                    .fill_rect(Rect::new(x, y, cell_size, cell_size))
                    .unwrap();
            }
        }

        canvas.present();
        board.compute_next();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
