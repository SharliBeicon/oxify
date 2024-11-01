use std::{io, sync::mpsc};

use crossterm::event::Event as TerminalEvent;
use ratatui::DefaultTerminal;

use crate::{
    auth::{AuthState, LoginState},
    spotify,
    widgets::{await_login::AwaitLogin, landing::Landing, main_window::MainWindow, popup::Popup},
    OxifyEvent,
};

#[derive(Debug)]
pub struct App<'a> {
    exit: bool,
    auth_state: AuthState,
    active_popup: Option<Popup<'a>>,

    // Screens
    landing: Landing,
    await_login: AwaitLogin,
    main_window: MainWindow,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            exit: false,
            auth_state: AuthState::default(),
            active_popup: None,
            landing: Landing::default(),
            await_login: AwaitLogin::default(),
            main_window: MainWindow::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (auth_tx, auth_rx) = mpsc::channel::<AuthState>();
        let (event_tx, event_rx) = mpsc::channel::<OxifyEvent>();

        self.landing.auth_tx = Some(auth_tx.clone());
        self.landing.event_tx = Some(event_tx.clone());
        self.await_login.event_tx = Some(event_tx.clone());
        self.main_window.set_event_sender(event_tx.clone());

        terminal.draw(|frame| self.landing.draw(frame))?;
        while !self.exit {
            // Try to update auth state
            if let Ok(auth_state) = auth_rx.try_recv() {
                self.auth_state = auth_state;
            }

            // Check for terminal input events
            let terminal_event: Option<TerminalEvent> =
                crossterm::event::poll(std::time::Duration::new(0, 0))?
                    .then(|| crossterm::event::read())
                    .transpose()?;

            // Check for oxify app events
            let oxify_event: Option<OxifyEvent> = event_rx.try_recv().ok();

            // Common oxify events
            oxify_event.as_ref().map(|e| {
                if let OxifyEvent::Exit = e {
                    self.exit = true
                }
                if let OxifyEvent::LoginAttempt = e {
                    self.auth_state.login_state = LoginState::Loading
                }
                if let OxifyEvent::ClosePopup = e {
                    self.active_popup = None
                }
                if let OxifyEvent::Popup(popup) = e {
                    self.active_popup = Some(popup.clone())
                }
            });

            // Handle events depending of the auth state
            match self.auth_state.login_state {
                LoginState::Out => {
                    self.landing.handle_events(&terminal_event);
                    terminal.draw(|frame| self.landing.draw(frame))?;
                }
                LoginState::Loading => {
                    self.await_login.handle_events(&terminal_event);
                    terminal.draw(|frame| self.await_login.draw(frame))?;
                }
                LoginState::In => {
                    // Load user profile on login
                    if self.main_window.user_profile.is_none() {
                        self.main_window.user_profile = Some(spotify::api::get_user_profile(
                            self.auth_state
                                .access_token
                                .as_ref()
                                .expect("Token not found somehow")
                                .to_string(),
                        )?);
                    }

                    self.main_window
                        .handle_events(&terminal_event, &oxify_event);
                    terminal.draw(|frame| {
                        self.main_window.draw(frame);
                        //Handle possible help popup event
                        match &self.active_popup {
                            Some(popup) => {
                                popup.draw(frame);
                                popup.handle_events(&event_tx, &terminal_event);
                            }
                            None => Popup::handle_toggle_popup(&event_tx, &terminal_event),
                        }
                    })?;
                }
            }
        }

        Ok(())
    }
}
