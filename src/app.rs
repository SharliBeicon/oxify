use crate::{
    auth::{self, AuthState, LoginState},
    model::user_profile::UserProfile,
    spotify,
    widgets::{
        login::AwaitLogin,
        main_window::MainWindow,
        popup::{help_popup, Popup},
        CustomWidget, Landing,
    },
    Focus, OxifyEvent,
};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::Stylize,
    text::{Line, Text},
    DefaultTerminal, Frame,
};
use std::{
    io,
    sync::{
        mpsc::{channel, Sender},
        Arc, Mutex,
    },
    thread,
};

#[derive(Debug)]
pub struct App<'a> {
    exit: bool,
    auth_state: Arc<Mutex<AuthState>>,
    active_popup: Option<Popup<'a>>,
    user_profile: Option<UserProfile>,
    focus: Focus,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            exit: false,
            auth_state: Arc::new(Mutex::new(AuthState::default())),
            active_popup: None,
            user_profile: None,
            focus: Focus::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let (tx, rx) = channel::<OxifyEvent>();

        let mut landing = Landing::default();
        let mut await_login = AwaitLogin::default();
        let mut main_window = MainWindow::new();

        terminal.draw(|frame| self.draw(&Landing::default(), frame, None))?;
        while !self.exit {
            if let Ok(received) = rx.try_recv() {
                log::info!("Oxify event received: {:?}", received);
                match received {
                    OxifyEvent::Popup(popup_content) => {
                        self.active_popup = Some(Popup {
                            title: Line::from(popup_content.title.bold()),
                            content: Text::from(popup_content.content),
                            kind: popup_content.kind,
                        })
                    }
                    OxifyEvent::AuthInfo(auth_state) => {
                        *self.auth_state.lock().unwrap() = auth_state
                    }
                    _ => (),
                }
            }

            self.handle_state(
                tx.clone(),
                terminal,
                &mut landing,
                &mut await_login,
                &mut main_window,
            )?;
        }

        Ok(())
    }

    fn handle_state(
        &mut self,
        tx: Sender<OxifyEvent>,
        terminal: &mut DefaultTerminal,
        landing: &mut Landing,
        await_login: &mut AwaitLogin,
        main_window: &mut MainWindow,
    ) -> io::Result<()> {
        let login_state = self.auth_state.lock().unwrap().login_state.clone();
        let event: Option<Event> = if crossterm::event::poll(std::time::Duration::new(0, 0))? {
            Some(crossterm::event::read()?)
        } else {
            None
        };
        match login_state {
            LoginState::Out => {
                terminal.draw(|frame| self.draw(landing, frame, None))?;
                self.handle_events(landing, &event, tx)?;
            }
            LoginState::Loading => {
                terminal.draw(|frame| self.draw(await_login, frame, None))?;
                self.handle_events(await_login, &event, tx)?;
            }
            LoginState::In => {
                if self.user_profile.is_none() {
                    let auth_state_lock = self.auth_state.lock().unwrap();
                    let token = match auth_state_lock.access_token.clone() {
                        Some(token) => token,
                        None => {
                            return Err(io::Error::new(
                                io::ErrorKind::PermissionDenied,
                                "Token not found",
                            ))
                        }
                    };
                    drop(auth_state_lock);
                    self.user_profile = Some(spotify::api::get_user_profile(token)?);
                    main_window.player.username =
                        self.user_profile.as_ref().unwrap().display_name.clone();
                }
                terminal.draw(|frame| {
                    let (library_panel, main_panel) = main_window.layout(frame.area());
                    if self.focus == Focus::Search {
                        self.draw_input(main_window, frame, main_panel[0]);
                    }
                    self.draw(&main_window.search, frame, Some(main_panel[0]));

                    self.draw(&main_window.library, frame, Some(library_panel));
                    self.draw(&main_window.player, frame, Some(main_panel[1]));
                })?;
                main_window.set_focus(&self.focus);
                match self.focus {
                    Focus::Search => {
                        self.handle_events(&mut main_window.search, &event, tx.clone())?
                    }
                    Focus::Library => {
                        self.handle_events(&mut main_window.library, &event, tx.clone())?
                    }
                    Focus::Player => {
                        self.handle_events(&mut main_window.player, &event, tx.clone())?
                    }
                    Focus::None => {
                        self.handle_events(&mut main_window.player, &event, tx.clone())?;
                        self.handle_events(&mut main_window.library, &event, tx.clone())?;
                        self.handle_events(&mut main_window.search, &event, tx.clone())?;
                    }
                }
                if let Some(event) = &event {
                    if let crossterm::event::Event::Key(key_event) = event {
                        if key_event.kind == KeyEventKind::Press
                            && key_event.code == KeyCode::Char('?')
                        {
                            self.active_popup = Some(help_popup());
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn draw_input(&self, widget: &mut MainWindow, frame: &mut Frame, area: Rect) {
        #[allow(clippy::cast_possible_truncation)]
        frame.set_cursor_position(Position::new(
            area.x + widget.search.character_index as u16 + 1,
            area.y + 1,
        ));
    }
    fn draw(&self, widget: &impl CustomWidget, frame: &mut Frame, area: Option<Rect>) {
        let drawing_area = match area {
            Some(area) => area,
            None => frame.area(),
        };
        let popup_area = match frame.area().height {
            0..20 => resize_area(frame.area(), 50, 46),
            20..30 => resize_area(frame.area(), 40, 37),
            30.. => resize_area(frame.area(), 30, 28),
        };
        frame.render_widget(widget.clone(), drawing_area);

        self.active_popup.as_ref().map(|popup| {
            frame.render_widget(popup.clone(), popup_area);
        });
    }

    fn handle_events(
        &mut self,
        custom_widget: &mut impl CustomWidget,
        event: &Option<Event>,
        tx: Sender<OxifyEvent>,
    ) -> io::Result<()> {
        if let Some(event) = event {
            match event {
                crossterm::event::Event::Key(key_event)
                    if key_event.kind == KeyEventKind::Press =>
                {
                    if let Some(popup) = self.active_popup.as_mut() {
                        if let Some(event) = popup.handle_key_event(*key_event) {
                            match event {
                                OxifyEvent::Exit => {
                                    self.active_popup = None;
                                }
                                _ => {}
                            }
                        }
                    }
                    let tx_clone = tx.clone();
                    custom_widget
                        .handle_key_event(*key_event)
                        .map(|key_event| match key_event {
                            OxifyEvent::Exit => self.exit = true,
                            OxifyEvent::LoginAttempt => {
                                self.auth_state.lock().unwrap().login_state = LoginState::Loading;
                                thread::spawn(|| auth::api::login(tx_clone));
                            }
                            OxifyEvent::Focus(focus) => self.focus = focus,
                            _ => (),
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
