use crate::{
    auth::{api, LoginState},
    widgets::{login::AwaitLogin, CustomWidget, Landing, Popup},
    OxifyEvent,
};
use crossterm::event::KeyEventKind;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
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

        terminal.draw(|frame| self.draw(&Landing::default(), frame))?;
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
                terminal.draw(|frame| self.draw(landing, frame))?;
                self.handle_events(landing, tx.clone())?;
            }
            LoginState::Loading => {
                let await_login = &AwaitLogin::default();
                terminal.draw(|frame| self.draw(await_login, frame))?;
                self.handle_events(await_login, tx.clone())?;
            }
            LoginState::In => todo!(),
        }
        Ok(())
    }

    fn draw(&self, widget: &impl CustomWidget, frame: &mut Frame) {
        let popup_area = resize_area(frame.area(), 60, 20);
        frame.render_widget(widget.clone(), frame.area());

        if self.active_popup.as_ref().is_some() {
            frame.render_widget(self.active_popup.as_ref().unwrap().clone(), popup_area);
        }
    }

    fn handle_events(
        &mut self,
        custom_widget: &impl CustomWidget,
        tx: mpsc::Sender<OxifyEvent>,
    ) -> io::Result<()> {
        if crossterm::event::poll(std::time::Duration::new(0, 0))? {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    if let Some(popup) = self.active_popup.as_mut() {
                        if let Some(event) = popup.handle_key_event(key_event) {
                            match event {
                                OxifyEvent::Exit => {
                                    self.active_popup = None;
                                }
                                _ => {}
                            }
                        }
                    }
                    custom_widget
                        .handle_key_event(key_event)
                        .map(|key_event| match key_event {
                            OxifyEvent::Exit => self.exit = true,
                            OxifyEvent::LoginAttempt => {
                                self.login_state = LoginState::Loading;
                                tokio::spawn(api::init_login(tx.clone()));
                            }
                            OxifyEvent::Popup(_) => (),
                        });
                }
                _ => (),
            }
        }
        Ok(())
    }
}

fn resize_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
