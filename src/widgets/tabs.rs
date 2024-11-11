use ratatui::text::Line;
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Tracks")]
    Tracks,
    #[strum(to_string = "Albums")]
    Albums,
    #[strum(to_string = "Artists")]
    Artists,
    #[strum(to_string = "Playlists")]
    Playlists,
}

impl SelectedTab {
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);

        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);

        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn title(self) -> Line<'static> {
        format!(" {self} ").into()
    }
}
