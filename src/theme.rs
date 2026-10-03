use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};

/// The shared Project Mambo terminal color palette.
///
/// The default palette uses high-contrast foreground/background pairs. UI
/// components also include text labels and symbols, so color is never the
/// only way they communicate state. Every field is public so an application
/// can make a small product-specific adjustment without defining a new theme
/// abstraction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Theme {
    /// Main application background.
    pub background: Color,
    /// Raised surface background, used by application chrome.
    pub surface: Color,
    /// Primary text color.
    pub text: Color,
    /// Secondary text color.
    pub muted: Color,
    /// Default border color.
    pub border: Color,
    /// Focus and interaction color.
    pub accent: Color,
    /// Selected row background.
    pub selection: Color,
    /// Successful operation color.
    pub success: Color,
    /// Caution color.
    pub warning: Color,
    /// Failure color.
    pub error: Color,
    /// Informational color.
    pub info: Color,
}

impl Theme {
    /// Returns the base style used for application content.
    #[must_use]
    pub fn base(&self) -> Style {
        Style::new().fg(self.text).bg(self.background)
    }

    /// Returns the style used for app chrome and raised surfaces.
    #[must_use]
    pub fn surface_style(&self) -> Style {
        Style::new().fg(self.text).bg(self.surface)
    }

    /// Returns the style used for primary headings.
    #[must_use]
    pub fn title(&self) -> Style {
        self.surface_style()
            .fg(self.accent)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the style used for secondary copy.
    #[must_use]
    pub fn muted(&self) -> Style {
        self.base().fg(self.muted)
    }

    /// Returns the normal border style.
    #[must_use]
    pub fn border(&self) -> Style {
        self.base().fg(self.border)
    }

    /// Returns the focused border style.
    #[must_use]
    pub fn focused_border(&self) -> Style {
        self.base().fg(self.accent).add_modifier(Modifier::BOLD)
    }

    /// Returns the selected list-row style.
    #[must_use]
    pub fn selected(&self) -> Style {
        self.base().bg(self.selection).add_modifier(Modifier::BOLD)
    }

    /// Styles a title line consistently with the theme.
    #[must_use]
    pub fn title_line<'a>(&self, title: impl Into<Line<'a>>) -> Line<'a> {
        title.into().style(self.title())
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Rgb(13, 17, 23),
            surface: Color::Rgb(22, 27, 34),
            text: Color::Rgb(240, 246, 252),
            muted: Color::Rgb(139, 148, 158),
            border: Color::Rgb(139, 148, 158),
            accent: Color::Rgb(88, 166, 255),
            selection: Color::Rgb(31, 74, 125),
            success: Color::Rgb(63, 185, 80),
            warning: Color::Rgb(210, 153, 34),
            error: Color::Rgb(248, 81, 73),
            info: Color::Rgb(88, 166, 255),
        }
    }
}
