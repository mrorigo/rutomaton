use rutomaton::transitions;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(255, 25, 25),
        sdl2::pixels::Color::RGB(25, 255, 25),
    ];
    let bb = Automaton::new(
        vec![
            // Dead
            transitions![0, 26 => 1,
            125 => 1,
            224 => 1,
            323 => 1,
            422 => 1,
            521 => 1,
            620 => 1],
            // Alive
            transitions![2,],
            // Dying
            transitions![0,],
        ],
        colors,
    );

    let mut board = Board::new(bb.clone(), 100, 100);
    board.fill_random(1000, 0.99, None);

    visualize(&mut board, 4, &mut |board| {
        board.fill_random(78, 0.13, None);
    });
}
