#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::display::blocking::Display;
use microbit::hal::timer::Timer;
use microbit::Board;
use panic_halt as _;

const PIXELS: &[(usize, usize)] = &[
    (0, 0),
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (4, 3),
    (4, 2),
    (4, 1),
    (4, 0),
    (3, 0),
    (2, 0),
    (1, 0),
];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [[0; 5]; 5];
    let (mut last_col, mut last_row) = (0, 0);

    loop {
        for &(col, row) in PIXELS {
            leds[last_col][last_row] = 0;
            leds[col][row] = 1;
            display.show(&mut timer, leds, 50);
            (last_col, last_row) = (col, row);
        }
    }
}
