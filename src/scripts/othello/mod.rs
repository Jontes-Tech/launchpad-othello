use crate::controllers::Controller;

use tracing::info;

use super::Script;

use magpie::othello::{Game, Position};

pub struct PingScript {
    pub game: Game,
}

impl Script for PingScript {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn on_control_press(&mut self, index: u8, _controller: &dyn Controller) {
        if index == 7 {
            let _ = self.game.pass_turn();
            self.initialize(_controller);
        }
        if index == 5 {
            let original_string = self.game
                .display()
                .with_format(magpie::othello::Format::Compact)
                .to_string();
            let mut black: u8 = 0;
            let mut white: u8 = 0;

            for line in original_string.lines().skip(2).take(8) {
                let new_line = line.replace("|", "");
                for c in new_line.chars().skip(2).take(9) {
                    if c.to_string() != "|" {
                        if c.to_string() == "W" {
                            white += 1;
                        } else if c.to_string() == "B" {
                            black += 1;
                        }
                        info!("Black: {}, White: {}", black, white);
                    }
                }
            }

        }
    }

    fn on_press(&mut self, x: u8, y: u8, _controller: &dyn Controller) {
        let x_in_letters = match x {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => "z",
        };

        let pos = format!("{}{}", x_in_letters, y + 1);

        if let Ok(pos) = Position::try_from(pos) {
            if self
                .game
                .board()
                .is_legal_move(self.game.current_turn(), pos)
            {
                println!("Legal move");
                let _ = self.game.play(pos);
                self.initialize(_controller);
            } else {
                info!("Illegal move");
            }
        }
    }

    fn new() -> Self {
        PingScript { game: Game::new() }
    }

    fn initialize(&mut self, controller: &dyn Controller) {
        let _ = controller.set_control_button_color(4, 1);
        let game = &self.game;
        if self.game.current_turn() == magpie::othello::Stone::White {
            let _ = controller.set_control_button_color(7, 2);
        } else {
            let _ = controller.set_control_button_color(7, 3);
        }
        let original_string = game
            .display()
            .with_format(magpie::othello::Format::Compact)
            .to_string();
        let mut index = 0;

        for line in original_string.lines().skip(2).take(8) {
            let new_line = line.replace("|", "");
            for c in new_line.chars().skip(2).take(9) {
                if c.to_string() != "|" {
                    let (x, y) = get_coordinates(index);
                    if c.to_string() == "W" {
                        let _ = controller.set_button_color(
                            x.try_into().unwrap(),
                            y.try_into().unwrap(),
                            2,
                        );
                    } else if c.to_string() == "B" {
                        let _ = controller.set_button_color(
                            x.try_into().unwrap(),
                            y.try_into().unwrap(),
                            3,
                        );
                    } else {
                        let _ = controller.set_button_color(
                            x.try_into().unwrap(),
                            y.try_into().unwrap(),
                            0,
                        );
                    }
                }
                index += 1;
            }
        }
        info!("Redrawing board");
    }
}
fn get_coordinates(index: usize) -> (usize, usize) {
    let x = index % 8;
    let y = index / 8;
    (x, y)
}
