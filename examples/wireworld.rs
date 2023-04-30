use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use phf::phf_map;
use rutomaton::visualize;
use rutomaton::Automaton;
use rutomaton::Board;
use rutomaton::State;

const WW0: phf::Map<i64, State> = phf_map!(-1i64 => 0);
const WW1: phf::Map<i64, State> = phf_map!(-1i64 => 2);
const WW2: phf::Map<i64, State> = phf_map!(-1i64 => 3);
const WW3: phf::Map<i64, State> = phf_map!(
            0017i64 => 1,
            0026i64 => 1,
            0116i64 => 1,
            0125i64 => 1,
            0215i64 => 1,
            0224i64 => 1,
            0314i64 => 1,
            0323i64 => 1,
            0413i64 => 1,
            0422i64 => 1,
            0512i64 => 1,
            0521i64 => 1,
            0611i64 => 1,
            0620i64 => 1,
            0710i64 => 1,
            1016i64 => 1,
            1025i64 => 1,
            1115i64 => 1,
            1124i64 => 1,
            1214i64 => 1,
            1223i64 => 1,
            1313i64 => 1,
            1322i64 => 1,
            1412i64 => 1,
            1421i64 => 1,
            1511i64 => 1,
            1520i64 => 1,
            1610i64 => 1,
            2015i64 => 1,
            2024i64 => 1,
            2114i64 => 1,
            2123i64 => 1,
            2213i64 => 1,
            2222i64 => 1,
            2312i64 => 1,
            2321i64 => 1,
            2411i64 => 1,
            2420i64 => 1,
            2510i64 => 1,
            3014i64 => 1,
            3023i64 => 1,
            3113i64 => 1,
            3122i64 => 1,
            3212i64 => 1,
            3221i64 => 1,
            3311i64 => 1,
            3320i64 => 1,
            3410i64 => 1,
            4013i64 => 1,
            4022i64 => 1,
            4112i64 => 1,
            4121i64 => 1,
            4211i64 => 1,
            4220i64 => 1,
            4310i64 => 1,
            5012i64 => 1,
            5021i64 => 1,
            5111i64 => 1,
            5120i64 => 1,
            5210i64 => 1,
            6011i64 => 1,
            6020i64 => 1,
            6110i64 => 1,
            7010i64 => 1,
            -1i64 => 3);

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

    let ww = Automaton::new(vec![&WW0, &WW1, &WW2, &WW3], colors);
    let mut i = 0;
    let mut board = read_board("examples/wireworld-primes.wi", ww); //Board::new(ww, 20, 20);

    //place_diode(&mut board, 5, 5);

    visualize(&mut board, 1, &mut |board| {
        if i % 16 == 0 {
            //board.set_curr_cell(7, 1, 1);
        } else if i % 8 == 0 {
            board.set_curr_cell(5, 6, 1);
        }
        i += 1;
    });
}

fn read_board(file: &str, automaton: Automaton) -> Board {
    let reader = BufReader::new(File::open(file).expect("Cannot open file"));

    let mut lines_iter = reader.lines();
    let mut board = if let Some(header) = lines_iter.next() {
        let size = header
            .unwrap()
            .split_whitespace()
            .map(|x| u32::from_str_radix(x, 10).expect("should be an int"))
            .collect::<Vec<u32>>();
        Board::new(automaton, size[0] as i32, size[1] as i32)
    } else {
        panic!()
    };

    let mut row = 0;
    for line in lines_iter {
        line.as_ref()
            .unwrap()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, c)| match *c {
                b'#' => board.set_curr_cell(i as i32, row, 3),
                b'@' => board.set_curr_cell(i as i32, row, 1),
                b'~' => board.set_curr_cell(i as i32, row, 2),
                _ => {}
            });
        row += 1;
    }
    board
}
