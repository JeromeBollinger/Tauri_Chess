// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
                if self.color.white {
                    vec![Position::new(self.position.x, self.position.y + 1)]
                } else {
                    vec![Position::new(self.position.x, self.position.y + 1)]
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
                for distance in 1..7 {
                    pos.push(Position::new(self.position.x + distance, self.position.y));
                    pos.push(Position::new(self.position.x - distance, self.position.y));
                    pos.push(Position::new(self.position.x, self.position.y + distance));
                    pos.push(Position::new(self.position.x, self.position.y - distance));
                }
                pos
            },
            FigureType::Bishop => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..7 {
                    pos.push(Position::new(self.position.x + distance, self.position.y + distance));
                    pos.push(Position::new(self.position.x - distance, self.position.y - distance));
                    pos.push(Position::new(self.position.x - distance, self.position.y + distance));
                    pos.push(Position::new(self.position.x + distance, self.position.y - distance));
                }
                pos
            },
            FigureType::Queen => {
                let mut pos: Vec<Position> = vec![];
                for distance in 1..7 {
                    pos.push(Position::new(self.position.x + distance, self.position.y + distance));
                    pos.push(Position::new(self.position.x - distance, self.position.y - distance));
                    pos.push(Position::new(self.position.x - distance, self.position.y + distance));
                    pos.push(Position::new(self.position.x + distance, self.position.y - distance));
                    pos.push(Position::new(self.position.x + distance, self.position.y));
                    pos.push(Position::new(self.position.x - distance, self.position.y));
                    pos.push(Position::new(self.position.x, self.position.y + distance));
                    pos.push(Position::new(self.position.x, self.position.y - distance));
                }
                pos
            },
        }
    }
}

struct Figure {
    kind: FigureType,
    position: Position,
    color: Color,
    alive: bool,
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Color {
    white: bool,
}

