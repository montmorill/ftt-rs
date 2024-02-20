use clap::builder::styling::{AnsiColor, Color, Style};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type Map = Vec<Vec<Option<Block>>>;
pub type Position = (usize, usize);

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Block {
    Wall,
    Piston,
    Sand,
    Cobweb,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash, Debug)]
pub struct FttMap {
    pub map: Map,
    pub player: Position,
    pub target: Position,
}

impl Display for Block {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (color, text) = match self {
            Block::Wall => (AnsiColor::BrightWhite, "[#]"),
            Block::Piston => (AnsiColor::Green, "[*]"),
            Block::Sand => (AnsiColor::BrightYellow, ":::"),
            Block::Cobweb => (AnsiColor::Magenta, ">|<"),
        };
        let style = Style::new().fg_color(Some(Color::Ansi(color)));
        write!(f, "{style}{text}{style:#}")
    }
}

impl FttMap {
    pub fn new(map: Map, player: Position, target: Position) -> Self {
        Self {
            map,
            player,
            target,
        }
    }
}

impl Display for FttMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        for (row, series) in self.map.iter().enumerate() {
            for (column, block) in series.iter().enumerate() {
                let style = Style::new()
                    .fg_color(Some(Color::Ansi(AnsiColor::Cyan)))
                    .bg_color(Some(Color::Ansi(AnsiColor::Blue)))
                    .bold()
                    .underline();
                string += (if (row, column).eq(&self.player) {
                    format!("{style}YOU{style:#}")
                } else if (row, column).eq(&self.target) {
                    format!("{style}END{style:#}")
                } else {
                    match block {
                        None => "   ".to_string(),
                        Some(block) => block.to_string(),
                    }
                })
                .as_str();
            }
            string += "\n";
        }
        string.pop();
        write!(f, "{string}")
    }
}
