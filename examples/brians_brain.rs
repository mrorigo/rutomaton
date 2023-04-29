use phf::phf_map;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;
use rutomaton::State;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(255, 25, 25),
        sdl2::pixels::Color::RGB(25, 255, 25),
    ];
    const BB0: phf::Map<i64, State> = phf_map!(
             26i64 => 1,
            125i64 => 1,
            224i64 => 1,
            323i64 => 1,
            422i64 => 1,
            521i64 => 1,
            620i64 => 1,
            -1i64 => 0);
    const BB1: phf::Map<i64, State> = phf_map!(
        -1i64 => 2
    );
    const BB2: phf::Map<i64, State> = phf_map!(
        -1i64 => 0
    );

    let bb = Automaton::new(vec![&BB0, &BB1, &BB2], colors);
    let mut board = Board::new(bb, 100, 100);

    visualize(&mut board, 4, &mut |board| {
        board.fill_random(2, 0.13, None);
    });
}
