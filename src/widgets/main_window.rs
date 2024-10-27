use crate::Focus;

use super::{library::Library, player::Player, search::Search, InputMode};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use std::rc::Rc;

pub struct MainWindow {
    pub player: Player,
    pub search: Search,
    pub library: Library,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            player: Player::default(),
            search: Search::default(),
            library: Library::default(),
        }
    }

    pub fn set_focus(&mut self, focus: &Focus) {
        match focus {
            Focus::Search => {
                self.search.input_mode = InputMode::Focus;
                self.player.input_mode = InputMode::Normal;
                self.library.input_mode = InputMode::Normal;
            }
            Focus::Library => {
                self.search.input_mode = InputMode::Normal;
                self.player.input_mode = InputMode::Normal;
                self.library.input_mode = InputMode::Focus;
            }
            Focus::Player => {
                self.search.input_mode = InputMode::Normal;
                self.player.input_mode = InputMode::Focus;
                self.library.input_mode = InputMode::Normal;
            }
            Focus::None => {
                self.search.input_mode = InputMode::Normal;
                self.player.input_mode = InputMode::Normal;
                self.library.input_mode = InputMode::Normal;
            }
        }
    }

    pub fn layout(&self, area: Rect) -> (Rect, Rc<[Rect]>) {
        let search_bar_percentage = match area.height {
            0..20 => 14,
            20..30 => 12,
            30.. => 8,
        };
        let library_percentage = match area.width {
            0..150 => 30,
            150.. => 25,
        };
        let left_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(library_percentage),
                Constraint::Percentage(100 - library_percentage),
            ])
            .split(area);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(search_bar_percentage),
                Constraint::Percentage(100 - search_bar_percentage),
            ])
            .split(left_layout[1]);

        (left_layout[0], right_layout)
    }
}
