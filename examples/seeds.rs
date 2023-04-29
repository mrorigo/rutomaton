use rutomaton::transitions;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;

pub fn main() {
    let colors = vec![
        sdl2::pixels::Color::RGB(25, 25, 25),
        sdl2::pixels::Color::RGB(25, 25, 255),
    ];

    let sd = Automaton::new(vec![transitions![0, 62 => 1], transitions![0,]], colors);

    let mut i = 0;
    let mut board = Board::new(sd.clone(), 100, 100);

    visualize(&mut board, 8, &mut |board| {
        if i % 200 == 0 {
            for item in &mut board.states[board.curr_state] {
                *item = 0;
            }
        }
        i += 1;
        board.fill_random(2, 0.2, Some(sd.rules.len() as u8 - 1));
    });
}
