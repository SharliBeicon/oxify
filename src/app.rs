use crate::{
    auth::{AuthState, LoginState},
    spotify::{self, backend},
    widgets::{await_login::AwaitLogin, landing::Landing, main_window::MainWindow, popup::Popup},
    OxifyEvent, OxifyPlayerEvent,
};
use crossterm::event::Event as TerminalEvent;
use ratatui::DefaultTerminal;
use std::{io, sync::mpsc};

pub struct App<'a> {
    exit: bool,
    active_backend: bool,
    auth_state: AuthState,
    active_popup: Option<Popup<'a>>,

    // Screens
    landing: Landing,
    await_login: AwaitLogin,
    main_window: MainWindow,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            exit: false,
            auth_state: AuthState::default(),
            active_popup: None,
            active_backend: false,
            landing: Landing::default(),
            await_login: AwaitLogin::default(),
            main_window: MainWindow::default(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (auth_tx, auth_rx) = mpsc::channel::<AuthState>();
        let (event_tx, event_rx) = mpsc::channel::<OxifyEvent>();
        let (ope_tx, _) = tokio::sync::broadcast::channel::<OxifyPlayerEvent>(2048);

        self.landing.auth_tx = Some(auth_tx.clone());
        self.landing.event_tx = Some(event_tx.clone());
        self.await_login.event_tx = Some(event_tx.clone());
        self.main_window
            .set_senders(event_tx.clone(), ope_tx.clone());
        terminal.draw(|frame| self.landing.draw(frame))?;
        while !self.exit {
            // Try to update auth state
            if let Ok(auth_state) = auth_rx.try_recv() {
                self.auth_state = auth_state;
            }

            // Check for terminal input events
            let terminal_event: Option<TerminalEvent> =
                crossterm::event::poll(std::time::Duration::new(0, 0))?
                    .then(crossterm::event::read)
                    .transpose()?;

            // Check for oxify app events
            let oxify_event: Option<OxifyEvent> = event_rx.try_recv().ok();

            // Common oxify events
            if let Some(ref oxify_event) = oxify_event {
                match oxify_event {
                    OxifyEvent::Exit => self.exit = true,
                    OxifyEvent::LoginAttempt => self.auth_state.login_state = LoginState::Loading,
                    OxifyEvent::ClosePopup => self.active_popup = None,
                    OxifyEvent::Popup(popup) => self.active_popup = Some(popup.clone()),
                    OxifyEvent::SearchRequest(query) => {
                        let token = self
                            .auth_state
                            .access_token
                            .as_ref()
                            .expect("Token not found somehow");
                        if let Ok(response) =
                            spotify::api::search(token.to_string(), query.to_string())
                        {
                            OxifyEvent::send(
                                &event_tx,
                                OxifyEvent::SearchResponse(Box::new(response)),
                            );
                        }
                    }
                    OxifyEvent::ActiveBackend(is_active) => self.active_backend = *is_active,
                    _ => (),
                }
            }

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
                    let access_token = self
                        .auth_state
                        .access_token
                        .as_ref()
                        .expect("Token not found somehow")
                        .to_string();
                    // Load user profile on login
                    if self.main_window.user_profile.is_none() {
                        self.main_window.user_profile =
                            Some(spotify::api::get_user_profile(access_token.clone())?);
                    }

                    //Load backend session on login
                    if !self.active_backend {
                        let ope_tx = ope_tx.clone();
                        let event_tx = event_tx.clone();
                        tokio::spawn(backend::run(access_token.clone(), ope_tx, event_tx));
                        self.active_backend = true;
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
