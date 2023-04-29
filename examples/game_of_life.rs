use rutomaton::transitions;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(255, 255, 255),
    ];

    let gol = Automaton::new(
        vec![transitions![0, 35 => 1], transitions![0, 35 => 1, 26 => 1]],
        colors,
    );
    let mut board = Board::new(gol.clone(), 100, 100);

    visualize(&mut board, 4, &mut |board| {
        board.fill_random(5, 0.1, Some(gol.rules.len() as u8 - 1));
        board.fill_random(1, 0.99, Some(0));
    });
}
