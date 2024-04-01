// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board() -> Board {
    let board = Board::init();
    board
}

#[tauri::command]
fn get_options(figure_id: i32) -> Vec<Position> {
    let board = Board::init();
    board.get_figure_from_id(figure_id).movable()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_board, get_options])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
enum FigureType {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl Figure {
    fn movable(&self) -> Vec<Position> {
        match &self.kind {
            FigureType::Pawn => {
                if self.white {
                    vec![Position::new(self.position.x, self.position.y + 1)]
                } else {
                    vec![Position::new(self.position.x, self.position.y - 1)]
                }
            }
            FigureType::King => vec![
                Position::new(self.position.x - 1, self.position.y),
                Position::new(self.position.x - 1, self.position.y + 1),
                Position::new(self.position.x, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y),
                Position::new(self.position.x + 1, self.position.y - 1),
                Position::new(self.position.x, self.position.y - 1),
                Position::new(self.position.x - 1, self.position.y - 1),
            ],
            FigureType::Knight => vec![
                Position::new(self.position.x - 2, self.position.y + 1),
                Position::new(self.position.x - 1, self.position.y + 2),
                Position::new(self.position.x + 1, self.position.y + 2),
                Position::new(self.position.x + 2, self.position.y + 1),
                Position::new(self.position.x + 1, self.position.y - 2),
                Position::new(self.position.x + 2, self.position.y - 1),
                Position::new(self.position.x - 2, self.position.y - 1),
                Position::new(self.position.x - 1, self.position.y - 2),
            ],
            FigureType::Rook => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..8 {
                    pos.push(Position::new(self.position.x + distance, self.position.y));
                    pos.push(Position::new(self.position.x - distance, self.position.y));
                    pos.push(Position::new(self.position.x, self.position.y + distance));
                    pos.push(Position::new(self.position.x, self.position.y - distance));
                }
                pos
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
                pos
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
                pos
            }
        }
    }
}

#[derive(Serialize)]
struct Figure {
    kind: FigureType,
    position: Position,
    white: bool,
    alive: bool,
    id: i32,
}

impl Figure {
    fn new(kind: FigureType, position: Position, white: bool, id: i32) -> Figure {
        Figure {
            kind,
            position,
            white,
            alive: true,
            id,
        }
    }
}

#[derive(Serialize)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Serialize)]
struct Board {
    figures: Vec<Figure>,
    round: i32,
}

impl Board {
    fn init() -> Board {
        let mut fig: Vec<Figure> = vec![
            Figure::new(FigureType::Rook, Position::new(0, 0), true, 1),
            Figure::new(FigureType::Knight, Position::new(1, 0), true, 2),
            Figure::new(FigureType::Bishop, Position::new(2, 0), true, 3),
            Figure::new(FigureType::Queen, Position::new(3, 0), true, 4),
            Figure::new(FigureType::King, Position::new(4, 0), true, 5),
            Figure::new(FigureType::Bishop, Position::new(5, 0), true, 6),
            Figure::new(FigureType::Knight, Position::new(6, 0), true, 7),
            Figure::new(FigureType::Rook, Position::new(7, 0), true, 8),
            Figure::new(FigureType::Rook, Position::new(0, 7), false, 9),
            Figure::new(FigureType::Knight, Position::new(1, 7), false, 10),
            Figure::new(FigureType::Bishop, Position::new(2, 7), false, 11),
            Figure::new(FigureType::Queen, Position::new(3, 7), false, 12),
            Figure::new(FigureType::King, Position::new(4, 7), false, 13),
            Figure::new(FigureType::Bishop, Position::new(5, 7), false, 14),
            Figure::new(FigureType::Knight, Position::new(6, 7), false, 15),
            Figure::new(FigureType::Rook, Position::new(7, 7), false, 16),
        ];
        for i in 0..8 {
            fig.push(Figure::new(FigureType::Pawn, Position::new(i, 1), true, i + 17));
            fig.push(Figure::new(FigureType::Pawn, Position::new(i, 6), false, i + 25));
        }
        Board {
            round: 0,
            figures: fig,
        }
    }

    fn get_figure_from_id(&self, id: i32) -> &Figure {
        for figure in &self.figures {
            if figure.id == id {
                return &figure;
            }
        }
        &self.figures[0] // This case should never happen
    }
}
