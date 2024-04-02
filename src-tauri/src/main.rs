// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_board(game: State<Game>) -> Board {
    let a = game.0.lock().unwrap().clone();
    a
}

#[tauri::command]
fn get_options(game: State<Game>, figure_id: i32) -> Vec<Position> {
    let board = game.0.lock().unwrap().clone();
    board.get_figure_from_id(figure_id).movable()
}

fn main() {
    tauri::Builder::default()
        .manage(Game::init())
        .invoke_handler(tauri::generate_handler![greet, get_board, get_options])
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
    fn movable(&self) -> Vec<Position> {
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
                return options;
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

#[derive(Serialize, Clone)]
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
struct Game(Mutex<Board>);

impl Game {
    fn init() -> Self {
        Game(Mutex::new(Board::init()))
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
                return &figure;
            }
        }
        &self.figures[0] // This case should never happen
    }
}
