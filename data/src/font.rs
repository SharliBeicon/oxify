use crate::config::get_config;
use iced::font;
use std::borrow::Cow;
use std::sync::OnceLock;

pub static MONO: Font = Font::new(false, false);
pub static MONO_BOLD: Font = Font::new(true, false);
pub static MONO_ITALICS: Font = Font::new(false, true);
pub static MONO_BOLD_ITALICS: Font = Font::new(true, true);

#[derive(Debug, Clone)]
pub struct Font {
    bold: bool,
    italics: bool,
    inner: OnceLock<iced::Font>,
}

impl Font {
    const fn new(bold: bool, italics: bool) -> Self {
        Self {
            bold,
            italics,
            inner: OnceLock::new(),
        }
    }

    fn set(&self, name: String) {
        let name = Box::leak(name.into_boxed_str());
        let weight = if self.bold {
            font::Weight::Bold
        } else {
            font::Weight::Normal
        };
        let style = if self.italics {
            font::Style::Italic
        } else {
            font::Style::Normal
        };

        let _ = self.inner.set(iced::Font {
            weight,
            style,
            ..iced::Font::with_name(name)
        });
    }
}

impl From<Font> for iced::Font {
    fn from(value: Font) -> Self {
        value.inner.get().copied().expect("font is set on startup")
    }
}

pub fn set() {
    let family = get_config().font_family.clone();

    MONO.set(family.clone());
    MONO_BOLD.set(family.clone());
    MONO_ITALICS.set(family.clone());
    MONO_BOLD_ITALICS.set(family);
}

pub fn load() -> Vec<Cow<'static, [u8]>> {
    vec![
        include_bytes!("../../fonts/iosevka-term-regular.ttf")
            .as_slice()
            .into(),
        include_bytes!("../../fonts/iosevka-term-bold.ttf")
            .as_slice()
            .into(),
        include_bytes!("../../fonts/iosevka-term-italic.ttf")
            .as_slice()
            .into(),
    ]
}
