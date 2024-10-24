use super::{library::Library, player::Player, search::Search};
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

    pub fn layout(&self, area: Rect) -> (Rc<[Rect]>, Rc<[Rect]>) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
            .split(outer_layout[1]);

        (outer_layout, right_layout)
    }
}
