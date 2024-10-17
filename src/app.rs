use crate::{
    widgets::{CustomWidget, Landing},
    InternalMessage,
};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut landing = Landing::default();
        while !self.exit {
            terminal.draw(|frame| self.draw(&landing, frame))?;
            if let Some(event) = landing.handle_events()? {
                if let InternalMessage::Exit = event {
                    self.exit = true;
                }
            }
            //match self.state.login_state {
            //    LoginState::Out => terminal.draw(|frame| self.draw(Landing, frame))?,
            //    LoginState::Loading => terminal.draw(|frame| self.draw(AwaitLogin, frame))?,
            //    LoginState::In => terminal.draw(|frame| self.draw(&*self, frame))?,
            //};
            //self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&mut self, widget: &impl CustomWidget, frame: &mut Frame) {
        frame.render_widget(*widget, frame.area());
    }
}
