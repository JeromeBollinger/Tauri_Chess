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
                if self.white {
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
    white: bool,
    alive: bool,
}

impl Figure {
    fn new(kind: FigureType, position: Position, white: bool) -> Figure {
        Figure {
            kind,
            position,
            white,
            alive: true,
        }
    }
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

struct Board {
    figures: Vec<Figure>,
    round: i32,
}

impl Board {
    fn init() -> Board {
        let mut fig: Vec<Figure> = vec![
            Figure::new(FigureType::Rook, Position::new(0, 0), true),
            Figure::new(FigureType::Knight, Position::new(1, 0), true),
            Figure::new(FigureType::Bishop, Position::new(2, 0), true),
            Figure::new(FigureType::Queen, Position::new(3, 0), true),
            Figure::new(FigureType::King, Position::new(4, 0), true),
            Figure::new(FigureType::Bishop, Position::new(5, 0), true),
            Figure::new(FigureType::Knight, Position::new(6, 0), true),
            Figure::new(FigureType::Rook, Position::new(7, 0), true),
            Figure::new(FigureType::Rook, Position::new(0, 7), false),
            Figure::new(FigureType::Knight, Position::new(1, 7), false),
            Figure::new(FigureType::Bishop, Position::new(2, 7), false),
            Figure::new(FigureType::Queen, Position::new(3, 7), false),
            Figure::new(FigureType::King, Position::new(4, 7), false),
            Figure::new(FigureType::Bishop, Position::new(5, 7), false),
            Figure::new(FigureType::Knight, Position::new(6, 7), false),
            Figure::new(FigureType::Rook, Position::new(7, 7), false),
        ];
        for i in 0..7 {
            fig.push(Figure::new(FigureType::Pawn, Position::new(i, 1), true));
            fig.push(Figure::new(FigureType::Pawn, Position::new(i, 6), false));
        }
        Board {
            round: 0,
            figures: fig,
        }
    }
}
