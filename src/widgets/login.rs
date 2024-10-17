use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Padding, Paragraph, Widget,
    },
};

use super::centered_height;

pub struct AwaitLogin;

impl Widget for AwaitLogin {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = Text::from(" Please, follow the instructions on the browser. ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));

        let block = Block::bordered()
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .padding(Padding::top(centered_height(
                content.height() as u16,
                &area,
            )))
            .border_set(border::THICK);

        Paragraph::new(content)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
