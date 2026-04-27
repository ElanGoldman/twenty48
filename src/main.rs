mod game;
mod history;
mod leaderboard;

use eframe::egui;
use game::{Direction, Game};
use leaderboard::Leaderboard;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([560.0, 760.0])
            .with_min_inner_size([440.0, 540.0])
            .with_title("2048 in Rust"),
        ..Default::default()
    };

    eframe::run_native(
        "rust_2048",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

struct App {
    game: Game,
    board_size_input: String,
    status_message: String,
    leaderboard: Leaderboard,
    score_saved: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        let leaderboard = cc
            .storage
            .map(|s| Leaderboard::load(s))
            .unwrap_or_default();
        let size = 4;
        Self {
            game: Game::new(size),
            board_size_input: size.to_string(),
            status_message: String::new(),
            leaderboard,
            score_saved: false,
        }
    }

    fn try_start_new_game(&mut self) {
        if !self.score_saved && self.game.score > 0 {
            self.leaderboard.add_score(self.game.score, self.game.size);
        }
        match self.board_size_input.trim().parse::<usize>() {
            Ok(size) if size >= 2 => {
                self.game = Game::new(size);
                self.status_message = format!("Started new {}x{} game.", size, size);
                self.score_saved = false;
            }
            _ => {
                self.status_message = "Board size must be an integer >= 2.".to_string();
            }
        }
    }

    fn handle_move(&mut self, direction: Direction) {
        if self.game.make_move(direction) {
            self.status_message.clear();
        }
    }

    fn tile_colors(value: u32) -> (egui::Color32, egui::Color32) {
        match value {
            0 => (
                egui::Color32::from_rgb(205, 193, 180),
                egui::Color32::TRANSPARENT,
            ),
            2 => (
                egui::Color32::from_rgb(238, 228, 218),
                egui::Color32::from_rgb(119, 110, 101),
            ),
            4 => (
                egui::Color32::from_rgb(237, 224, 200),
                egui::Color32::from_rgb(119, 110, 101),
            ),
            8 => (
                egui::Color32::from_rgb(242, 177, 121),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            16 => (
                egui::Color32::from_rgb(245, 149, 99),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            32 => (
                egui::Color32::from_rgb(246, 124, 95),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            64 => (
                egui::Color32::from_rgb(246, 94, 59),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            128 => (
                egui::Color32::from_rgb(237, 207, 114),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            256 => (
                egui::Color32::from_rgb(237, 204, 97),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            512 => (
                egui::Color32::from_rgb(237, 200, 80),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            1024 => (
                egui::Color32::from_rgb(237, 197, 63),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            2048 => (
                egui::Color32::from_rgb(237, 194, 46),
                egui::Color32::from_rgb(249, 246, 242),
            ),
            _ => (
                egui::Color32::from_rgb(60, 58, 50),
                egui::Color32::from_rgb(249, 246, 242),
            ),
        }
    }

    fn draw_board(&self, ui: &mut egui::Ui, board_px: f32) {
        let gap = 8.0;
        let margin = 12.0;
        let n = self.game.size as f32;
        let cell_size = ((board_px - 2.0 * margin - gap * (n - 1.0)) / n).max(24.0);

        egui::Frame::new()
            .fill(egui::Color32::from_rgb(187, 173, 160))
            .corner_radius(12.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                egui::Grid::new("board_grid")
                    .spacing([gap, gap])
                    .show(ui, |ui| {
                        for r in 0..self.game.size {
                            for c in 0..self.game.size {
                                let cell = self.game.board[(r, c)];
                                let (bg, fg) = Self::tile_colors(cell);
                                let text = if cell == 0 {
                                    "".to_string()
                                } else {
                                    cell.to_string()
                                };

                                egui::Frame::new()
                                    .fill(bg)
                                    .corner_radius(8.0)
                                    .show(ui, |ui| {
                                        ui.set_min_size(egui::vec2(cell_size, cell_size));
                                        ui.set_max_size(egui::vec2(cell_size, cell_size));
                                        ui.vertical_centered_justified(|ui| {
                                            let font_size = if cell >= 1024 {
                                                cell_size * 0.28
                                            } else if cell >= 128 {
                                                cell_size * 0.34
                                            } else {
                                                cell_size * 0.40
                                            };

                                            ui.label(
                                                egui::RichText::new(text)
                                                    .size(font_size)
                                                    .strong()
                                                    .color(fg),
                                            );
                                        });
                                    });
                            }
                            ui.end_row();
                        }
                    });
            });
    }

    fn draw_leaderboard_popup(&mut self, ctx: &egui::Context) {
        if !self.leaderboard.show {
            return;
        }

        let mut show = self.leaderboard.show;
        egui::Window::new("Leaderboard")
            .open(&mut show)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.set_min_width(280.0);

                if self.leaderboard.entries.is_empty() {
                    ui.label("No scores yet. Play a game to get on the board!");
                } else {
                    egui::Grid::new("leaderboard_grid")
                        .striped(true)
                        .spacing([24.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong("Rank");
                            ui.strong("Score");
                            ui.strong("Board");
                            ui.end_row();

                            for (i, entry) in self.leaderboard.entries.iter().enumerate() {
                                ui.label(format!("#{}", i + 1));
                                ui.label(entry.score.to_string());
                                ui.label(format!("{}x{}", entry.board_size, entry.board_size));
                                ui.end_row();
                            }
                        });
                }
            });
        self.leaderboard.show = show;
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A)) {
            self.handle_move(Direction::Left);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D)) {
            self.handle_move(Direction::Right);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W)) {
            self.handle_move(Direction::Up);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S)) {
            self.handle_move(Direction::Down);
        }

        if !self.score_saved && !self.game.can_make_any_move() {
            self.leaderboard.add_score(self.game.score, self.game.size);
            self.score_saved = true;
            self.leaderboard.show = true;
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("2048");
            ui.label(format!("Score: {}", self.game.score));
            ui.label(format!("Board: {}x{}", self.game.size, self.game.size));

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Undo").clicked() {
                    if !self.game.undo() {
                        self.status_message = "Nothing to undo.".to_string();
                    }
                }

                if ui.button("Redo").clicked() {
                    if !self.game.redo() {
                        self.status_message = "Nothing to redo.".to_string();
                    }
                }

                if ui.button("New Game").clicked() {
                    self.try_start_new_game();
                }

                if ui.button("Leaderboard").clicked() {
                    self.leaderboard.show = !self.leaderboard.show;
                }
            });

            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Board size:");
                let te = ui.text_edit_singleline(&mut self.board_size_input);
                if ui.button("Apply").clicked()
                    || (te.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    self.try_start_new_game();
                }
            });

            ui.add_space(16.0);

            let bottom_padding = 80.0;
            let available = ui.available_size();
            let board_px = available
                .x
                .min((available.y - bottom_padding).max(0.0))
                .max(100.0);

            ui.vertical_centered(|ui| {
                self.draw_board(ui, board_px);
            });

            ui.add_space(14.0);

            if self.game.won {
                ui.label(
                    egui::RichText::new("You reached 2048!")
                        .strong()
                        .size(20.0),
                );
            } else if !self.game.can_make_any_move() {
                ui.label(
                    egui::RichText::new("Game over.")
                        .strong()
                        .size(20.0),
                );
            }

            if !self.status_message.is_empty() {
                ui.label(&self.status_message);
            }

            ui.add_space(8.0);
            ui.label("Controls: Arrow keys or WASD");
        });

        self.draw_leaderboard_popup(&ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.leaderboard.save(storage);
    }
}
