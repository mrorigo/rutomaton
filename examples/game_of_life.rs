use phf::phf_map;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;
use rutomaton::State;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(255, 255, 255),
    ];

    static GOL_0: phf::Map<i64, State> = phf_map! {
        35i64 => 1,
        -1i64 => 0
    };
    static GOL_1: phf::Map<i64, State> = phf_map! {
        35i64 => 1,
        26i64 => 1,
        -1i64 => 0
    };

    let gol = Automaton::new(vec![&GOL_0, &GOL_1], colors);
    let mut board = Board::new(gol, 100, 100);

    visualize(&mut board, 4, &mut |board| {
        board.fill_random(5, 0.1, Some(board.automaton.rules.len() as u8 - 1));
        board.fill_random(1, 0.99, Some(0));
    });
}
