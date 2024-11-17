use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Widget},
    Frame,
};

#[derive(Default, Clone)]
pub struct Player {}

impl Player {
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.clone(), area);
    }
}

impl Widget for Player {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::PLAIN
        };

        Block::bordered()
            .border_set(border_set)
            .borders(Borders::BOTTOM | Borders::LEFT | Borders::RIGHT)
            .render(area, buf)
    }
}
