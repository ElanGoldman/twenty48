use rand::prelude::*;

use crate::history::GameSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Vec<Vec<u32>>,
    pub size: usize,
    pub score: u32,
    pub undo_stack: Vec<GameSnapshot>,
    pub redo_stack: Vec<GameSnapshot>,
    pub won: bool,
}

impl Game {
    pub fn new(size: usize) -> Self {
        assert!(size >= 2, "Board size must be at least 2");

        let mut game = Self {
            board: vec![vec![0; size]; size],
            size,
            score: 0,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            won: false,
        };

        game.spawn_tile();
        game.spawn_tile();
        game
    }

    pub fn make_move(&mut self, direction: Direction) -> bool {
        let snapshot = GameSnapshot::from_game(self);
        let original_board = self.board.clone();
        let original_score = self.score;

        match direction {
            Direction::Left => {
                self.move_left();
            }
            Direction::Right => {
                self.reverse_rows();
                self.move_left();
                self.reverse_rows();
            }
            Direction::Up => {
                self.transpose();
                self.move_left();
                self.transpose();
            }
            Direction::Down => {
                self.transpose();
                self.reverse_rows();
                self.move_left();
                self.reverse_rows();
                self.transpose();
            }
        }

        let changed = self.board != original_board || self.score != original_score;

        if changed {
            self.undo_stack.push(snapshot);
            self.redo_stack.clear();
            self.spawn_tile();
            self.won = self.has_tile(2048);
        }

        changed
    }

    pub fn undo(&mut self) -> bool {
        let Some(previous) = self.undo_stack.pop() else {
            return false;
        };

        let current = GameSnapshot::from_game(self);
        self.redo_stack.push(current);

        self.board = previous.board;
        self.score = previous.score;
        self.won = self.has_tile(2048);
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(next) = self.redo_stack.pop() else {
            return false;
        };

        let current = GameSnapshot::from_game(self);
        self.undo_stack.push(current);

        self.board = next.board;
        self.score = next.score;
        self.won = self.has_tile(2048);
        true
    }

    pub fn can_make_any_move(&self) -> bool {
        if self.board.iter().flatten().any(|&x| x == 0) {
            return true;
        }

        for r in 0..self.size {
            for c in 0..self.size {
                let val = self.board[r][c];

                if r + 1 < self.size && self.board[r + 1][c] == val {
                    return true;
                }
                if c + 1 < self.size && self.board[r][c + 1] == val {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_tile(&self, target: u32) -> bool {
        self.board.iter().flatten().any(|&x| x == target)
    }

    fn move_left(&mut self) {
        for r in 0..self.size {
            let (new_row, gained_score) = Self::compress_and_merge_row(&self.board[r]);
            self.board[r] = new_row;
            self.score += gained_score;
        }
    }

    fn compress_and_merge_row(row: &[u32]) -> (Vec<u32>, u32) {
        let tiles: Vec<u32> = row.iter().copied().filter(|&x| x != 0).collect();
        let mut merged: Vec<u32> = Vec::with_capacity(row.len());
        let mut score_gained = 0;
        let mut i = 0;

        while i < tiles.len() {
            if i + 1 < tiles.len() && tiles[i] == tiles[i + 1] {
                let new_val = tiles[i] * 2;
                merged.push(new_val);
                score_gained += new_val;
                i += 2;
            } else {
                merged.push(tiles[i]);
                i += 1;
            }
        }

        while merged.len() < row.len() {
            merged.push(0);
        }

        (merged, score_gained)
    }

    fn spawn_tile(&mut self) {
        let mut empty_cells = Vec::new();

        for r in 0..self.size {
            for c in 0..self.size {
                if self.board[r][c] == 0 {
                    empty_cells.push((r, c));
                }
            }
        }

        if empty_cells.is_empty() {
            return;
        }

        let mut rng = rand::rng();
        let &(r, c) = empty_cells.choose(&mut rng).unwrap();
        let value = if rng.random_bool(0.9) { 2 } else { 4 };

        self.board[r][c] = value;
    }

    fn reverse_rows(&mut self) {
        for row in &mut self.board {
            row.reverse();
        }
    }

    fn transpose(&mut self) {
        for r in 0..self.size {
            for c in (r + 1)..self.size {
                let temp = self.board[r][c];
                self.board[r][c] = self.board[c][r];
                self.board[c][r] = temp;
            }
        }
    }
}