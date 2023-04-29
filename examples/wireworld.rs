use rutomaton::transitions;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;

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

    let ww = Automaton::new(
        vec![
            // empty (black),
            transitions![0,],
            // electron head (blue),
            transitions![2,],
            // electron tail (red),
            transitions![3,],
            // conductor (green).
            transitions![3,
                    710 => 1,
                    611 => 1,
                    512 => 1,
                    413 => 1,
                    314 => 1,
                    215 => 1,
                    116 => 1,
                    017 => 1,
                    026 => 1,
                    125 => 1,
                    224 => 1,
                    323 => 1,
                    422 => 1,
                    521 => 1,
                    620 => 1],
        ],
        colors,
    );
    let mut i = 0;
    let mut board = Board::new(ww.clone(), 20, 20);

    visualize(&mut board, 32, &mut |board| {
        board.fill_random(5, 1.0, Some(ww.rules.len() as u8 - 1));
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
