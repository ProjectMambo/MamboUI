use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};

/// A MamboOrche colour scheme.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Scheme {
    /// Dark background with light text.
    #[default]
    Dark,
    /// Light background with dark text.
    Light,
}

/// The shared Project Mambo terminal color palette.
///
/// [`Theme::new`] maps MamboColour's semantic MamboOrche roles into Ratatui
/// colours. UI components also include text labels and symbols, so colour is
/// never the only way they communicate state. The fields stay public for small
/// product-specific adjustments, but applications should start from a bundled
/// scheme rather than construct a theme from scratch.
#[non_exhaustive]
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
    /// Tertiary text color.
    pub subtle: Color,
    /// Default border color.
    pub border: Color,
    /// Project Mambo brand color.
    pub brand: Color,
    /// Text color placed on a brand background.
    pub on_brand: Color,
    /// Selection indicator color.
    pub selection: Color,
    /// Keyboard focus indicator.
    pub focus: Color,
    /// General interactive indicator.
    pub interactive: Color,
    /// Successful operation color.
    pub success: Color,
    /// Caution color.
    pub warning: Color,
    /// Failure color.
    pub error: Color,
}

impl Theme {
    /// Builds a theme from the matching MamboColour scheme.
    #[must_use]
    pub fn new(scheme: Scheme) -> Self {
        let palette = mambocolour::theme(match scheme {
            Scheme::Dark => mambocolour::Scheme::Dark,
            Scheme::Light => mambocolour::Scheme::Light,
        })
        .ui();

        Self {
            background: color(palette.bg()),
            surface: color(palette.bg_surface()),
            text: color(palette.fg()),
            muted: color(palette.fg_muted()),
            subtle: color(palette.fg_subtle()),
            border: color(palette.border()),
            brand: color(palette.brand()),
            on_brand: color(palette.on_brand()),
            selection: color(palette.selection()),
            focus: color(palette.focus()),
            interactive: color(palette.interactive()),
            success: color(palette.success()),
            warning: color(palette.warning()),
            error: color(palette.error()),
        }
    }

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
            .fg(self.brand)
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
        self.base().fg(self.focus).add_modifier(Modifier::BOLD)
    }

    /// Returns the selected list-row style.
    #[must_use]
    pub fn selected(&self) -> Style {
        self.base()
            .fg(self.on_brand)
            .bg(self.brand)
            .add_modifier(Modifier::BOLD)
    }

    /// Styles a title line consistently with the theme.
    #[must_use]
    pub fn title_line<'a>(&self, title: impl Into<Line<'a>>) -> Line<'a> {
        title.into().style(self.title())
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::new(Scheme::Dark)
    }
}

fn color(colour: mambocolour::Colour) -> Color {
    let [red, green, blue] = colour.rgb();
    Color::Rgb(red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schemes_map_mambocolour_roles() {
        let dark = Theme::new(Scheme::Dark);
        let light = Theme::new(Scheme::Light);

        assert_eq!(dark.background, Color::Rgb(24, 22, 21));
        assert_eq!(dark.brand, Color::Rgb(255, 107, 87));
        assert_eq!(light.background, Color::Rgb(250, 247, 242));
        assert_eq!(light.text, Color::Rgb(28, 17, 17));
        assert_eq!(dark.selected().fg, Some(dark.on_brand));
        assert_eq!(dark.selected().bg, Some(dark.brand));
        assert_eq!(light.selected().fg, Some(light.on_brand));
        assert_eq!(light.selected().bg, Some(light.brand));
    }
}
