use crate::{
    auth::{api, LoginState},
    widgets::{login::AwaitLogin, CustomWidget, Landing},
    Event,
};
use crossterm::event::KeyEventKind;
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    login_state: LoginState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut landing = Landing::default();
        let mut await_login = AwaitLogin::default();

        terminal.draw(|frame| self.draw(&landing, frame))?;

        while !self.exit {
            match self.login_state {
                LoginState::Out => {
                    terminal.draw(|frame| self.draw(&landing, frame))?;
                    if let Some(event) = handle_events(&mut landing)? {
                        match event {
                            Event::Exit => self.exit = true,
                            Event::LoginAttempt => {
                                self.login_state = LoginState::Loading;
                                tokio::spawn(api::init_login());
                            }
                            _ => (),
                        }
                    }
                }
                LoginState::Loading => {
                    terminal.draw(|frame| self.draw(&await_login, frame))?;
                    if let Some(event) = handle_events(&mut await_login)? {
                        match event {
                            Event::Exit => self.exit = true,
                            _ => (),
                        }
                    }
                }
                LoginState::In => todo!(),
            }
        }

        Ok(())
    }

    fn draw(&mut self, widget: &impl CustomWidget, frame: &mut Frame) {
        frame.render_widget(*widget, frame.area());
    }
}

fn handle_events<T: CustomWidget>(custom_widget: &mut T) -> io::Result<Option<Event>> {
    match crossterm::event::read()? {
        crossterm::event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            Ok(custom_widget.handle_key_event(key_event))
        }
        _ => Ok(None),
    }
}
