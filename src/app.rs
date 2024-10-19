use crate::{
    auth::{api, LoginState},
    widgets::{login::AwaitLogin, CustomWidget, Landing, Popup},
    OxifyEvent,
};
use crossterm::event::KeyEventKind;
use ratatui::{
    style::Stylize,
    text::{Line, Text},
    DefaultTerminal, Frame,
};
use std::io;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct App<'a> {
    exit: bool,
    login_state: LoginState,
    active_popup: Option<Popup<'a>>,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            exit: false,
            login_state: LoginState::Out,
            active_popup: None,
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (tx, mut rx) = mpsc::channel::<OxifyEvent>(256);

        terminal.draw(|frame| draw(&Landing::default(), frame))?;
        while !self.exit {
            if let Ok(received) = rx.try_recv() {
                log::info!("Oxify event received: {:?}", received);
                if let OxifyEvent::Popup(popup_content) = received {
                    self.active_popup = Some(Popup {
                        title: Line::from(popup_content.title.bold()),
                        content: Text::from(popup_content.content),
                        kind: popup_content.kind,
                    })
                }
            }

            self.handle_state(tx.clone(), terminal)?;

            if self.active_popup.is_some() {
                terminal.draw(|frame| draw(self.active_popup.as_ref().unwrap(), frame))?;
            }
        }

        Ok(())
    }

    fn handle_state(
        &mut self,
        tx: mpsc::Sender<OxifyEvent>,
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        match self.login_state {
            LoginState::Out => {
                let landing = &Landing::default();
                terminal.draw(|frame| draw(landing, frame))?;
                if let Some(event) = handle_events(landing)? {
                    match event {
                        OxifyEvent::Exit => self.exit = true,
                        OxifyEvent::LoginAttempt => {
                            self.login_state = LoginState::Loading;
                            tokio::spawn(api::init_login(tx.clone()));
                        }
                        _ => (),
                    }
                }
            }
            LoginState::Loading => {
                let await_login = &AwaitLogin::default();
                terminal.draw(|frame| draw(await_login, frame))?;
                if let Some(event) = handle_events(await_login)? {
                    match event {
                        OxifyEvent::Exit => self.exit = true,
                        _ => (),
                    }
                }
            }
            LoginState::In => todo!(),
        }
        Ok(())
    }
}

fn draw(widget: &impl CustomWidget, frame: &mut Frame) {
    frame.render_widget(widget.clone(), frame.area());
}

fn handle_events(custom_widget: &impl CustomWidget) -> io::Result<Option<OxifyEvent>> {
    if crossterm::event::poll(std::time::Duration::new(0, 0))? {
        match crossterm::event::read()? {
            crossterm::event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                Ok(custom_widget.handle_key_event(key_event))
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
