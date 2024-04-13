// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
fn position_interaction(game: State<Game>, x: i32, y: i32) {
    let mut board = game.board.lock().unwrap();
    let board_clone = &board.clone();
    let last_interacted_position = &board.last_interacted_position.clone();
    match last_interacted_position {
        Some(last_pos) => {
            let figure_option = board
                .get_figure_from_position(last_pos.clone())
                .unwrap()
                .get_move_options(board_clone);
            if figure_option.movable.contains(&Position::new(x, y)) {
                board
                    .get_figure_from_position_mut(*last_pos)
                    .unwrap()
                    .set_position(x, y);
                board.round += 1;
            } else if figure_option.killable.contains(&Position::new(x, y)) {
                board
                    .get_figure_from_position_mut(Position::new(x, y))
                    .unwrap()
                    .alive = false;
                board
                    .get_figure_from_position_mut(*last_pos)
                    .unwrap()
                    .set_position(x, y);
                board.round += 1;
            }
            board.last_interacted_position = None;
        }
        None => {
            if let Some(figure_color) = board.get_figure_from_position_mut(Position::new(x, y)) {
                if is_figures_turn(figure_color.white, board.round) {
                    board.last_interacted_position = Some(Position::new(x, y))
                }
            }
        }
    }
}

#[tauri::command]
fn set_player_color(game: State<Game>, white: bool) {
    let mut p = game.player.lock().unwrap();
    p.white = white;
}

#[tauri::command]
fn get_board(game: State<Game>) -> Board {
    let a = game.board.lock().unwrap().clone();
    a
}

#[tauri::command]
fn get_options(game: State<Game>, x: i32, y: i32) -> Option<MoveOptions> {
    let board = game.board.lock().unwrap();
    if let Some(figure) = board.get_figure_from_position(Position::new(x, y)) {
        if is_figures_turn(figure.white, board.round) {
            return Some(
                figure
                    .get_move_options(&board)
                    .remove_out_of_bounds_options(),
            );
        }
    }
    None
}

fn main() {
    tauri::Builder::default()
        .manage(Game::init())
        .invoke_handler(tauri::generate_handler![
            get_board,
            get_options,
            set_player_color,
            position_interaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn is_figures_turn(white: bool, round: i32) -> bool {
    white == (round % 2 == 0)
}

#[derive(Serialize, Clone, PartialEq)]
enum FigureType {
    Pawn,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
}

impl Figure {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
        self.first_move = false;
    }
    fn get_move_options(&self, board: &Board) -> MoveOptions {
        match &self.kind {
            FigureType::Pawn => {
                let mut movable: Vec<Position> = vec![];
                let mut killable: Vec<Position> = vec![];
                let direction = if self.white { 1 } else { -1 };
                // Movable position if free in front
                let p = Position::new(self.position.x, self.position.y + direction);
                if board.occupied_by(p).is_none() {
                    movable.push(p);
                    // Movable position if free in front and first move
                    if self.first_move {
                        let p = Position::new(self.position.x, self.position.y + 2 * direction);
                        if board.occupied_by(p).is_none() {
                            movable.push(p);
                        }
                    }
                }
                // Killable positions diagonal left
                let p = Position::new(self.position.x + 1, self.position.y + direction);
                if let Some(f) = board.occupied_by(p) {
                    if f.white != self.white {
                        killable.push(p);
                    }
                }
                // Killable positions diagonal right
                let p = Position::new(self.position.x - 1, self.position.y + direction);
                if let Some(f) = board.occupied_by(p) {
                    if f.white != self.white {
                        killable.push(p);
                    }
                }
                // TODO: En passant
                MoveOptions { movable, killable }
            }
            FigureType::King => {
                let mut movable = vec![];
                let mut killable = vec![];
                let moves = vec![
                    Position::new(self.position.x - 1, self.position.y),
                    Position::new(self.position.x - 1, self.position.y + 1),
                    Position::new(self.position.x, self.position.y + 1),
                    Position::new(self.position.x + 1, self.position.y + 1),
                    Position::new(self.position.x + 1, self.position.y),
                    Position::new(self.position.x + 1, self.position.y - 1),
                    Position::new(self.position.x, self.position.y - 1),
                    Position::new(self.position.x - 1, self.position.y - 1),
                ];
                for p in moves {
                    match board.occupied_by(p) {
                        Some(f) => {
                            if f.white != self.white {
                                killable.push(p)
                            }
                        }
                        None => movable.push(p),
                    }
                }
                MoveOptions { movable, killable }
            }
            FigureType::Knight => {
                let mut movable = vec![];
                let mut killable = vec![];
                let moves = vec![
                    Position::new(self.position.x - 2, self.position.y + 1),
                    Position::new(self.position.x - 1, self.position.y + 2),
                    Position::new(self.position.x + 1, self.position.y + 2),
                    Position::new(self.position.x + 2, self.position.y + 1),
                    Position::new(self.position.x + 1, self.position.y - 2),
                    Position::new(self.position.x + 2, self.position.y - 1),
                    Position::new(self.position.x - 2, self.position.y - 1),
                    Position::new(self.position.x - 1, self.position.y - 2),
                ];
                for p in moves {
                    match board.occupied_by(p) {
                        Some(f) => {
                            if f.white != self.white {
                                killable.push(p)
                            }
                        }
                        None => movable.push(p),
                    }
                }
                MoveOptions { movable, killable }
            }
            FigureType::Rook => {
                let mut movable = vec![];
                let mut killable = vec![];
                let directions = vec![-1, 1];

                // horizontal left and right
                for direction in &directions {
                    for distance in 1..8 {
                        let p =
                            Position::new(self.position.x + distance * direction, self.position.y);
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }
                // vertical up and down
                for direction in &directions {
                    for distance in 1..8 {
                        let p =
                            Position::new(self.position.x, self.position.y + distance * direction);
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }
                MoveOptions { movable, killable }
            }
            FigureType::Bishop => {
                let mut movable = vec![];
                let mut killable = vec![];
                let directions = vec![-1, 1];

                // diagonal /
                for direction in directions {
                    for distance in 1..8 {
                        let p = Position::new(
                            self.position.x + distance * direction,
                            self.position.y + distance * direction,
                        );
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                    // diagonal \
                    for distance in 1..8 {
                        let p = Position::new(
                            self.position.x + -distance * direction,
                            self.position.y + distance * direction,
                        );
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }
                MoveOptions { movable, killable }
            }
            FigureType::Queen => {
                let mut movable = vec![];
                let mut killable = vec![];
                let directions = vec![-1, 1];

                // horizontal left and right
                for direction in &directions {
                    for distance in 1..8 {
                        let p =
                            Position::new(self.position.x + distance * direction, self.position.y);
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }
                // vertical up and down
                for direction in &directions {
                    for distance in 1..8 {
                        let p =
                            Position::new(self.position.x, self.position.y + distance * direction);
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }

                // diagonal /
                for direction in directions {
                    for distance in 1..8 {
                        let p = Position::new(
                            self.position.x + distance * direction,
                            self.position.y + distance * direction,
                        );
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                    // diagonal \
                    for distance in 1..8 {
                        let p = Position::new(
                            self.position.x + -distance * direction,
                            self.position.y + distance * direction,
                        );
                        if let Some(f) = board.occupied_by(p) {
                            if f.white != self.white {
                                killable.push(p)
                            }
                            break; // Figure is blocked and cannot move further
                        } else {
                            movable.push(p)
                        }
                    }
                }
                MoveOptions { movable, killable }
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
    first_move: bool,
}

impl Figure {
    fn new(kind: FigureType, position: Position, white: bool, first_move: bool) -> Figure {
        Figure {
            kind,
            position,
            white,
            alive: true,
            first_move,
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq)]
struct MoveOptions {
    movable: Vec<Position>,
    killable: Vec<Position>,
}

impl MoveOptions {
    fn remove_out_of_bounds_options(mut self) -> Self {
        let allowed_range = 0..8;
        self.movable.retain(|position| {
            allowed_range.contains(&position.x) && allowed_range.contains(&position.y)
        });
        self
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

#[derive(Serialize, Default, Debug)]
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
    last_interacted_position: Option<Position>,
}

impl Board {
    fn position_threatened_from(&self, figure: &Figure) -> Vec<Position> {
        self.figures
            .iter()
            .filter(|pt| pt.white != figure.white && pt.alive)
            .filter_map(|pt| {
                pt.get_move_options(self)
                    .killable
                    .iter()
                    .find(|&&k_pos| k_pos == figure.position)
                    .map(|_| pt.position)
            })
            .collect()
    }

    fn occupied_by(&self, position: Position) -> Option<&Figure> {
        self.figures
            .iter()
            .find(|&figure| figure.position == position && figure.alive)
    }
    fn init() -> Board {
        let mut fig: Vec<Figure> = vec![
            Figure::new(FigureType::Rook, Position::new(0, 0), true, true),
            Figure::new(FigureType::Knight, Position::new(1, 0), true, true),
            Figure::new(FigureType::Bishop, Position::new(2, 0), true, true),
            Figure::new(FigureType::Queen, Position::new(3, 0), true, true),
            Figure::new(FigureType::King, Position::new(4, 0), true, true),
            Figure::new(FigureType::Bishop, Position::new(5, 0), true, true),
            Figure::new(FigureType::Knight, Position::new(6, 0), true, true),
            Figure::new(FigureType::Rook, Position::new(7, 0), true, true),
            Figure::new(FigureType::Rook, Position::new(0, 7), false, true),
            Figure::new(FigureType::Knight, Position::new(1, 7), false, true),
            Figure::new(FigureType::Bishop, Position::new(2, 7), false, true),
            Figure::new(FigureType::Queen, Position::new(3, 7), false, true),
            Figure::new(FigureType::King, Position::new(4, 7), false, true),
            Figure::new(FigureType::Bishop, Position::new(5, 7), false, true),
            Figure::new(FigureType::Knight, Position::new(6, 7), false, true),
            Figure::new(FigureType::Rook, Position::new(7, 7), false, true),
        ];
        for i in 0..8 {
            fig.push(Figure::new(
                FigureType::Pawn,
                Position::new(i, 1),
                true,
                true,
            ));
            fig.push(Figure::new(
                FigureType::Pawn,
                Position::new(i, 6),
                false,
                true,
            ));
        }
        Board {
            round: 0,
            figures: fig,
            last_interacted_position: None,
        }
    }

    fn get_figure_from_position_mut(&mut self, position: Position) -> Option<&mut Figure> {
        self.figures
            .iter_mut()
            .find(|figure| figure.position == position && figure.alive)
    }

    fn get_figure_from_position(&self, position: Position) -> Option<&Figure> {
        self.figures
            .iter()
            .find(|figure| figure.position == position && figure.alive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_check() {
        let queen = Figure::new(FigureType::Queen, Position::new(4, 4), true, true);
        let king = Figure::new(FigureType::King, Position::new(6, 4), false, true);
        let board = Board {
            figures: vec![queen.clone(), king.clone()],
            round: 0,
            last_interacted_position: None,
        };
        assert_eq!(board.position_threatened_from(&king), vec![queen.position]);

        let board = Board {
            figures: vec![king.clone()],
            round: 0,
            last_interacted_position: None,
        };
        assert_eq!(board.position_threatened_from(&king), vec![]);

        let queen = Figure::new(FigureType::Queen, Position::new(4, 4), false, true);
        let king = Figure::new(FigureType::King, Position::new(6, 4), false, true);
        let board = Board {
            figures: vec![queen.clone(), king.clone()],
            round: 0,
            last_interacted_position: None,
        };
        assert_eq!(board.position_threatened_from(&king), vec![]);
    }

    #[test]
    fn pawn_test() {
        let pawn = Figure::new(FigureType::Pawn, Position::new(4, 4), true, true);
        let raw_options = MoveOptions {
            movable: vec![Position::new(4, 5), Position::new(4, 6)],
            killable: vec![],
        };
        assert_eq!(
            pawn.get_move_options(&Board {
                round: 0,
                figures: vec![],
                last_interacted_position: None,
            }),
            raw_options
        );
        let pawn = Figure::new(FigureType::Pawn, Position::new(4, 4), true, false);
        let raw_options = MoveOptions {
            movable: vec![Position::new(4, 5)],
            killable: vec![],
        };
        assert_eq!(
            pawn.get_move_options(&Board {
                figures: vec![],
                round: 1,
                last_interacted_position: None,
            }),
            raw_options
        );
    }

    #[test]
    fn remove_out_of_bounds_position() {
        let raw_options = MoveOptions {
            movable: vec![
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
            killable: vec![],
        };
        let inbound_options = MoveOptions {
            movable: vec![
                Position::new(4, 5),
                Position::new(0, 5),
                Position::new(7, 5),
                Position::new(5, 7),
                Position::new(7, 7),
            ],
            killable: vec![],
        };
        assert_eq!(raw_options.remove_out_of_bounds_options(), inbound_options);
    }

    #[test]
    fn is_turn_of() {
        let white = true;
        let black = false;
        assert_eq!(is_figures_turn(white, 0), true);
        assert_eq!(is_figures_turn(black, 0), false);
        assert_eq!(is_figures_turn(white, 1), false);
        assert_eq!(is_figures_turn(black, 1), true);
    }
}
