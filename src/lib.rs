#![no_std]
#![no_main]

use defmt::info;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_nrf::Peripherals;

pub const NUM_COLS: usize = 5;
pub const NUM_ROWS: usize = 5;

#[derive(Clone, Copy)]
pub enum Button {
    A,
    B,
}

pub struct Buttons<'a> {
    pub button_a: Input<'a>,
    pub button_b: Input<'a>,
}

pub struct DisplayPins<'a> {
    pub col1: Output<'a>,
    pub col2: Output<'a>,
    pub col3: Output<'a>,
    pub col4: Output<'a>,
    pub col5: Output<'a>,
    pub row1: Output<'a>,
    pub row2: Output<'a>,
    pub row3: Output<'a>,
    pub row4: Output<'a>,
    pub row5: Output<'a>,
}

pub struct App<'a> {
    pub display_pins: DisplayPins<'a>,
    pub buttons: Buttons<'a>,
    pub active_led: (usize, usize),
}

impl<'a> App<'a> {
    pub fn new(p: Peripherals) -> Self {
        Self {
            active_led: (0, 0),
            display_pins: DisplayPins {
                col1: Output::new(p.P0_28.degrade(), Level::High, OutputDrive::Standard),
                col2: Output::new(p.P0_11.degrade(), Level::High, OutputDrive::Standard),
                col3: Output::new(p.P0_31.degrade(), Level::High, OutputDrive::Standard),
                col4: Output::new(p.P1_05.degrade(), Level::High, OutputDrive::Standard),
                col5: Output::new(p.P0_30.degrade(), Level::High, OutputDrive::Standard),
                row1: Output::new(p.P0_21.degrade(), Level::High, OutputDrive::Standard),
                row2: Output::new(p.P0_22.degrade(), Level::High, OutputDrive::Standard),
                row3: Output::new(p.P0_15.degrade(), Level::High, OutputDrive::Standard),
                row4: Output::new(p.P0_24.degrade(), Level::High, OutputDrive::Standard),
                row5: Output::new(p.P0_19.degrade(), Level::High, OutputDrive::Standard),
            },
            buttons: Buttons {
                button_a: Input::new(p.P0_14.degrade(), Pull::None),
                button_b: Input::new(p.P0_23.degrade(), Pull::None),
            },
        }
    }

    pub fn toggle(&mut self) {
        info!(
            "Blinking LED ({}, {})",
            self.active_led.0, self.active_led.1
        );

        let display_pin_col = match self.active_led.1 {
            0 => &mut self.display_pins.col1,
            1 => &mut self.display_pins.col2,
            2 => &mut self.display_pins.col3,
            3 => &mut self.display_pins.col4,
            4 => &mut self.display_pins.col5,
            _ => panic!("Invalid column"),
        };

        display_pin_col.toggle();
    }

    pub fn shift(&mut self, button: Button) {
        let (mut row, mut col) = self.active_led;
        self.set_led(row, col, false);
        match button {
            Button::A => {
                let (new_row, new_col) = match (row, col) {
                    (0, 0) => (4, 4),
                    (0, _) => (4, col - 1),
                    (_, 0) => (row - 1, 4),
                    (_, _) => (row, col - 1),
                };
                row = new_row;
                col = new_col;
            }
            Button::B => {
                let (new_row, new_col) = match (row, col) {
                    (4, 4) => (0, 0),
                    (_, 4) => (row + 1, 0),
                    (4, _) => (0, col + 1),
                    (_, _) => (row + 1, col + 1),
                };
                row = new_row;
                col = new_col;
            }
        }
        self.active_led = (row, col);
        self.set_led(row, col, true);
    }

    fn set_led(&mut self, row: usize, col: usize, state: bool) {
        let display_pin_row = match row {
            0 => &mut self.display_pins.row1,
            1 => &mut self.display_pins.row2,
            2 => &mut self.display_pins.row3,
            3 => &mut self.display_pins.row4,
            4 => &mut self.display_pins.row5,
            _ => panic!("Invalid row"),
        };

        let display_pin_col = match col {
            0 => &mut self.display_pins.col1,
            1 => &mut self.display_pins.col2,
            2 => &mut self.display_pins.col3,
            3 => &mut self.display_pins.col4,
            4 => &mut self.display_pins.col5,
            _ => panic!("Invalid column"),
        };

        if state {
            display_pin_row.set_low();
            display_pin_col.set_low();
        } else {
            display_pin_row.set_high();
            display_pin_col.set_high();
        }
    }
}
