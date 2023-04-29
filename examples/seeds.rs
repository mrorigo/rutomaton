use phf::phf_map;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;
use rutomaton::State;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(25, 25, 255),
    ];

    let mut i = 0;

    static GOL_0: phf::Map<i64, State> = phf_map! {
        26i64 => 1,
        -1i64 => 0
    };
    static GOL_1: phf::Map<i64, State> = phf_map! {
        -1i64 => 0
    };

    let gol = Automaton::new(vec![&GOL_0, &GOL_1], colors);
    let mut board = Board::new(gol, 100, 100);

    visualize(&mut board, 8, &mut |board| {
        if i % 200 == 0 {
            for item in &mut board.states[board.curr_state] {
                *item = 0;
            }
        }
        i += 1;
        board.fill_random(20, 0.3, Some(board.automaton.rules.len() as u8 - 1));
    });
}
