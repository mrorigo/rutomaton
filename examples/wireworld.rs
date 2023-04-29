use phf::phf_map;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;
use rutomaton::State;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(25, 25, 255),
        sdl2::pixels::Color::RGB(255, 25, 25),
        sdl2::pixels::Color::RGB(25, 255, 25),
    ];
    // empty → empty,
    // electron head → electron tail,
    // electron tail → conductor,
    // conductor → electron head if exactly one or two of the neighbouring cells are electron heads, otherwise remains conductor.

    const WW0: phf::Map<i64, State> = phf_map!(-1i64 => 0);
    const WW1: phf::Map<i64, State> = phf_map!(-1i64 => 2);
    const WW2: phf::Map<i64, State> = phf_map!(-1i64 => 3);
    const WW3: phf::Map<i64, State> = phf_map!(
        7100i64 => 1,
        6110i64 => 1,
        5120i64 => 1,
        4130i64 => 1,
        3140i64 => 1,
        2150i64 => 1,
        1160i64 => 1,
        0170i64 => 1,
        0260i64 => 1,
        1250i64 => 1,
        2240i64 => 1,
        3230i64 => 1,
        4220i64 => 1,
        5210i64 => 1,
        6200i64 => 1,
        -1i64 => 3);

    let ww = Automaton::new(vec![&WW0, &WW1, &WW2, &WW3], colors);
    let mut i = 0;
    let mut board = Board::new(ww, 20, 20);

    visualize(&mut board, 32, &mut |board| {
        board.fill_random(5, 1.0, Some(board.automaton.rules.len() as u8 - 1));
        board.fill_random(1, 1.0, Some(0));
        let offy = board.height / 2;
        let offx = board.width / 2 - 4;
        for ii in 0..7 {
            let v = i & (1 << ii);
            if v != 0 {
                board.set_curr_cell(offx + ii, offy, 1);
            }
        }
        i += 1;
    });
}
