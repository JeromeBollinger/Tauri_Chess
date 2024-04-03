// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn set_player_color(game: State<Game>, white: bool) {
    let mut p = game.player.lock().unwrap();
    p.white = white;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board(game: State<Game>) -> Board {
    let a = game.board.lock().unwrap().clone();
    a
}

#[tauri::command]
fn get_options(game: State<Game>, figure_id: i32) -> MoveOptions {
    let board = game.board.lock().unwrap().clone();
    board.get_figure_from_id(figure_id).raw_options()
}

fn main() {
    tauri::Builder::default()
        .manage(Game::init())
        .invoke_handler(tauri::generate_handler![greet, get_board, get_options, set_player_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize, Clone)]
enum FigureType {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl Figure {
    fn raw_options(&self) -> MoveOptions {
        match &self.kind {
            FigureType::Pawn => {
                let mut options: Vec<Position> = vec![];
                if self.white {
                    options.push(Position::new(self.position.x, self.position.y + 1));
                    if self.first_move {
                        options.push(Position::new(self.position.x, self.position.y + 2));
                    }
                } else {
                    options.push(Position::new(self.position.x, self.position.y - 1));
                    if self.first_move {
                        options.push(Position::new(self.position.x, self.position.y - 2));
                    }
                }
                MoveOptions {positions: options}
            }
            FigureType::King => MoveOptions {positions: vec![
                Position::new(self.position.x - 1, self.position.y),
                Position::new(self.position.x - 1, self.position.y + 1),
                Position::new(self.position.x, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y),
                Position::new(self.position.x + 1, self.position.y - 1),
                Position::new(self.position.x, self.position.y - 1),
                Position::new(self.position.x - 1, self.position.y - 1),
            ]},
            FigureType::Knight => MoveOptions {positions: vec![
                Position::new(self.position.x - 2, self.position.y + 1),
                Position::new(self.position.x - 1, self.position.y + 2),
                Position::new(self.position.x + 1, self.position.y + 2),
                Position::new(self.position.x + 2, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y - 2),
                Position::new(self.position.x + 2, self.position.y - 1),
                Position::new(self.position.x - 2, self.position.y - 1),
                Position::new(self.position.x - 1, self.position.y - 2),
            ]},
            FigureType::Rook => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..8 {
                    pos.push(Position::new(self.position.x + distance, self.position.y));
                    pos.push(Position::new(self.position.x - distance, self.position.y));
                    pos.push(Position::new(self.position.x, self.position.y + distance));
                    pos.push(Position::new(self.position.x, self.position.y - distance));
                }
                MoveOptions {positions: pos}
            }
            FigureType::Bishop => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..8 {
                    pos.push(Position::new(
                        self.position.x + distance,
                        self.position.y + distance,
                    ));
                    pos.push(Position::new(
                        self.position.x - distance,
                        self.position.y - distance,
                    ));
                    pos.push(Position::new(
                        self.position.x - distance,
                        self.position.y + distance,
                    ));
                    pos.push(Position::new(
                        self.position.x + distance,
                        self.position.y - distance,
                    ));
                }
                MoveOptions {positions: pos}
            }
            FigureType::Queen => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..8 {
                    pos.push(Position::new(
                        self.position.x + distance,
                        self.position.y + distance,
                    ));
                    pos.push(Position::new(
                        self.position.x - distance,
                        self.position.y - distance,
                    ));
                    pos.push(Position::new(
                        self.position.x - distance,
                        self.position.y + distance,
                    ));
                    pos.push(Position::new(
                        self.position.x + distance,
                        self.position.y - distance,
                    ));
                    pos.push(Position::new(self.position.x + distance, self.position.y));
                    pos.push(Position::new(self.position.x - distance, self.position.y));
                    pos.push(Position::new(self.position.x, self.position.y + distance));
                    pos.push(Position::new(self.position.x, self.position.y - distance));
                }
                MoveOptions {positions: pos}
            }
        }
    }
}

#[derive(Serialize, Clone)]
struct Figure {
    kind: FigureType,
    position: Position,
    white: bool,
    alive: bool,
    id: i32,
    first_move: bool,
}

impl Figure {
    fn new(kind: FigureType, position: Position, white: bool, id: i32, first_move: bool) -> Figure {
        Figure {
            kind,
            position,
            white,
            alive: true,
            id,
            first_move,
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
struct MoveOptions {
    positions: Vec<Position>
impl MoveOptions {
    fn remove_out_of_bounds_options(mut self) -> Self{
        let allowed_range = 0..8;
        self.positions.retain(|position| {
            allowed_range.contains(&position.x) && allowed_range.contains(&position.y)
        });
        self
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Serialize, Default)]
struct Player {
    white: bool,
}

#[derive(Serialize)]
struct Game {
    board: Mutex<Board>,
    player: Mutex<Player>,
}

impl Game {
    fn init() -> Self {
        Game {
            board: Mutex::new(Board::init()),
            player: Mutex::new(Player::default()),
        }
    }
}

#[derive(Serialize, Clone)]
struct Board {
    figures: Vec<Figure>,
    round: i32,
}

impl Board {
    fn init() -> Board {
        let mut fig: Vec<Figure> = vec![
            Figure::new(FigureType::Rook, Position::new(0, 0), true, 1, true),
            Figure::new(FigureType::Knight, Position::new(1, 0), true, 2, true),
            Figure::new(FigureType::Bishop, Position::new(2, 0), true, 3, true),
            Figure::new(FigureType::Queen, Position::new(3, 0), true, 4, true),
            Figure::new(FigureType::King, Position::new(4, 0), true, 5, true),
            Figure::new(FigureType::Bishop, Position::new(5, 0), true, 6, true),
            Figure::new(FigureType::Knight, Position::new(6, 0), true, 7, true),
            Figure::new(FigureType::Rook, Position::new(7, 0), true, 8, true),
            Figure::new(FigureType::Rook, Position::new(0, 7), false, 9, true),
            Figure::new(FigureType::Knight, Position::new(1, 7), false, 10, true),
            Figure::new(FigureType::Bishop, Position::new(2, 7), false, 11, true),
            Figure::new(FigureType::Queen, Position::new(3, 7), false, 12, true),
            Figure::new(FigureType::King, Position::new(4, 7), false, 13, true),
            Figure::new(FigureType::Bishop, Position::new(5, 7), false, 14, true),
            Figure::new(FigureType::Knight, Position::new(6, 7), false, 15, true),
            Figure::new(FigureType::Rook, Position::new(7, 7), false, 16, true),
        ];
        for i in 0..8 {
            fig.push(Figure::new(
                FigureType::Pawn,
                Position::new(i, 1),
                true,
                i + 17,
                true,
            ));
            fig.push(Figure::new(
                FigureType::Pawn,
                Position::new(i, 6),
                false,
                i + 25,
                true,
            ));
        }
        Board {
            round: 0,
            figures: fig,
        }
    }

    fn get_figure_from_id(&self, id: i32) -> &Figure {
        for figure in &self.figures {
            if figure.id == id {
                return figure;
            }
        }
        &self.figures[0] // This case should never happen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_test() {
        let pawn = Figure::new(FigureType::Pawn, Position::new(4, 4), true, 1, true);
        let raw_options = MoveOptions{positions: vec![Position::new(4, 5), Position::new(4, 6)]};
        assert_eq!(pawn.raw_options(), raw_options);
        let pawn = Figure::new(FigureType::Pawn, Position::new(4, 4), true, 1, false);
        let raw_options = MoveOptions{positions: vec![Position::new(4, 5)]};
        assert_eq!(pawn.raw_options(), raw_options);
    }

    #[test]
    fn remove_out_of_bounds_position() {
        let raw_options = MoveOptions {
            positions: vec![
                Position::new(-4, 5),
                Position::new(-4, -5),
                Position::new(4, -5),
                Position::new(8, 5),
                Position::new(5, 8),
                Position::new(-4, 8),
                Position::new(4, 5),
                Position::new(0, 5),
                Position::new(7, 5),
                Position::new(5, 7),
                Position::new(7, 7),

            ],
        };
        let inbound_options = MoveOptions {
            positions: vec![
                Position::new(4, 5),
                Position::new(0, 5),
                Position::new(7, 5),
                Position::new(5, 7),
                Position::new(7, 7),

            ],
        };
        assert_eq!(raw_options.remove_out_of_bounds_options()
                   , inbound_options);
    }
}
