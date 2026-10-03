use std::borrow::Cow;

use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget, Wrap},
};

use crate::Theme;

/// A full-screen Project Mambo application shell.
///
/// [`Shell::render`] paints the background, header, and footer, then returns
/// the rectangle available to application content.
#[derive(Clone, Debug)]
pub struct Shell<'a> {
    title: Cow<'a, str>,
    subtitle: Option<Cow<'a, str>>,
    footer: Cow<'a, str>,
}

impl<'a> Shell<'a> {
    /// Creates a shell with the given product title.
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            footer: Cow::Borrowed(""),
        }
    }

    /// Adds brief product context to the header.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<Cow<'a, str>>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Sets the persistent footer text, normally navigation help.
    #[must_use]
    pub fn footer(mut self, footer: impl Into<Cow<'a, str>>) -> Self {
        self.footer = footer.into();
        self
    }

    /// Renders app chrome and returns the content rectangle.
    ///
    /// Terminals at least 40 columns wide receive one cell of horizontal
    /// breathing room. Smaller areas retain every available column.
    #[must_use]
    pub fn render(&self, frame: &mut Frame<'_>, theme: &Theme) -> Rect {
        let area = frame.area();
        frame.render_widget(Block::new().style(theme.base()), area);

        let (header_area, body_area, footer_area) = shell_areas(area);
        frame.render_widget(
            Header::new(self.title.as_ref(), theme)
                .subtitle(self.subtitle.as_deref().unwrap_or_default()),
            header_area,
        );
        frame.render_widget(Footer::new(self.footer.as_ref(), theme), footer_area);

        if body_area.width >= 40 {
            Rect::new(
                body_area.x.saturating_add(1),
                body_area.y,
                body_area.width.saturating_sub(2),
                body_area.height,
            )
        } else {
            body_area
        }
    }
}

/// A one-line application header.
#[derive(Clone, Debug)]
pub struct Header<'a> {
    title: Cow<'a, str>,
    subtitle: Option<Cow<'a, str>>,
    theme: Theme,
}

impl<'a> Header<'a> {
    /// Creates a header with the shared theme.
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>, theme: &Theme) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            theme: *theme,
        }
    }

    /// Adds secondary text after the title.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<Cow<'a, str>>) -> Self {
        let subtitle = subtitle.into();
        if !subtitle.is_empty() {
            self.subtitle = Some(subtitle);
        }
        self
    }
}

impl Widget for Header<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let mut spans = vec![Span::styled(self.title, self.theme.title())];
        if let Some(subtitle) = self.subtitle {
            spans.extend([
                Span::styled("  ", self.theme.surface_style()),
                Span::styled(subtitle, self.theme.surface_style().fg(self.theme.muted)),
            ]);
        }

        Paragraph::new(Line::from(spans))
            .style(self.theme.surface_style())
            .render(area, buffer);
    }
}

/// A one-line footer for persistent context or navigation help.
#[derive(Clone, Debug)]
pub struct Footer<'a> {
    text: Cow<'a, str>,
    theme: Theme,
}

impl<'a> Footer<'a> {
    /// Creates a footer with the shared theme.
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>, theme: &Theme) -> Self {
        Self {
            text: text.into(),
            theme: *theme,
        }
    }
}

impl Widget for Footer<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.text)
            .style(self.theme.surface_style().fg(self.theme.muted))
            .render(area, buffer);
    }
}

/// A consistently styled bordered surface.
#[derive(Clone, Debug)]
pub struct Panel<'a> {
    title: Cow<'a, str>,
    focused: bool,
}

impl<'a> Panel<'a> {
    /// Creates a panel with a title.
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            focused: false,
        }
    }

    /// Marks the panel as focused, using the accent border.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Builds the ratatui block used to wrap application content.
    #[must_use]
    pub fn block(self, theme: &Theme) -> Block<'a> {
        let border_style = if self.focused {
            theme.focused_border()
        } else {
            theme.border()
        };

        Block::new()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(Line::styled(self.title, theme.title()))
            .style(theme.base())
    }
}

/// A centered message shown when a view has no content.
#[derive(Clone, Debug)]
pub struct EmptyState<'a> {
    title: Cow<'a, str>,
    message: Option<Cow<'a, str>>,
    theme: Theme,
}

impl<'a> EmptyState<'a> {
    /// Creates an empty state with a clear title.
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>, theme: &Theme) -> Self {
        Self {
            title: title.into(),
            message: None,
            theme: *theme,
        }
    }

    /// Adds a concise next step.
    #[must_use]
    pub fn message(mut self, message: impl Into<Cow<'a, str>>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl Widget for EmptyState<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if area.is_empty() {
            return;
        }

        let mut lines = vec![Line::styled(
            self.title,
            self.theme.base().add_modifier(Modifier::BOLD),
        )];
        if let Some(message) = self.message {
            lines.push(Line::styled(message, self.theme.muted()));
        }
        let height = u16::try_from(lines.len())
            .unwrap_or(u16::MAX)
            .min(area.height);
        let centered = Rect::new(
            area.x,
            area.y
                .saturating_add(area.height.saturating_sub(height) / 2),
            area.width,
            height,
        );

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .render(centered, buffer);
    }
}

/// Semantic operation states used by [`StatusLine`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Status {
    /// No operation is active.
    #[default]
    Idle,
    /// Informational context.
    Info,
    /// An operation completed successfully.
    Success,
    /// An operation needs attention.
    Warning,
    /// An operation failed.
    Error,
}

impl Status {
    fn label(self) -> &'static str {
        match self {
            Self::Idle => "IDLE",
            Self::Info => "INFO",
            Self::Success => "OK",
            Self::Warning => "WARN",
            Self::Error => "ERROR",
        }
    }

    fn color(self, theme: &Theme) -> Color {
        match self {
            Self::Idle => theme.muted,
            Self::Info => theme.info,
            Self::Success => theme.success,
            Self::Warning => theme.warning,
            Self::Error => theme.error,
        }
    }
}

/// A labeled status message that does not rely on color alone.
#[derive(Clone, Debug)]
pub struct StatusLine<'a> {
    status: Status,
    message: Cow<'a, str>,
    theme: Theme,
}

impl<'a> StatusLine<'a> {
    /// Creates a status line.
    #[must_use]
    pub fn new(status: Status, message: impl Into<Cow<'a, str>>, theme: &Theme) -> Self {
        Self {
            status,
            message: message.into(),
            theme: *theme,
        }
    }
}

impl Widget for StatusLine<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let badge = format!("[{}]", self.status.label());
        let badge_style = Style::new()
            .fg(self.status.color(&self.theme))
            .bg(self.theme.background)
            .add_modifier(Modifier::BOLD);

        Paragraph::new(Line::from(vec![
            Span::styled(badge, badge_style),
            Span::raw(" "),
            Span::styled(self.message, self.theme.base()),
        ]))
        .style(self.theme.base())
        .render(area, buffer);
    }
}

/// A keyboard shortcut displayed by [`Help`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyHint<'a> {
    key: Cow<'a, str>,
    label: Cow<'a, str>,
}

impl<'a> KeyHint<'a> {
    /// Creates a key and action pair.
    #[must_use]
    pub fn new(key: impl Into<Cow<'a, str>>, label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
        }
    }
}

/// A one-line collection of keyboard shortcuts.
#[derive(Clone, Debug)]
pub struct Help<'a> {
    hints: Vec<KeyHint<'a>>,
    theme: Theme,
}

impl<'a> Help<'a> {
    /// Creates help from key/action pairs.
    #[must_use]
    pub fn new(hints: impl IntoIterator<Item = KeyHint<'a>>, theme: &Theme) -> Self {
        Self {
            hints: hints.into_iter().collect(),
            theme: *theme,
        }
    }
}

impl Widget for Help<'_> {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let mut spans = Vec::with_capacity(self.hints.len().saturating_mul(4));
        for (index, hint) in self.hints.into_iter().enumerate() {
            if index > 0 {
                spans.push(Span::styled("  ", self.theme.muted()));
            }
            spans.extend([
                Span::styled("[", self.theme.muted()),
                Span::styled(hint.key, self.theme.base().fg(self.theme.accent).bold()),
                Span::styled("] ", self.theme.muted()),
                Span::styled(hint.label, self.theme.muted()),
            ]);
        }

        Paragraph::new(Line::from(spans))
            .style(self.theme.base())
            .render(area, buffer);
    }
}

/// Builds a stateful ratatui list using Project Mambo selection styling.
///
/// Render the result with [`Frame::render_stateful_widget`] and a ratatui
/// [`ListState`](ratatui::widgets::ListState).
pub fn selection_list<'a, I, T>(items: I, theme: &Theme) -> List<'a>
where
    I: IntoIterator<Item = T>,
    T: Into<Line<'a>>,
{
    List::new(items.into_iter().map(|item| ListItem::new(item.into())))
        .style(theme.base())
        .highlight_style(theme.selected())
        .highlight_symbol("› ")
        .repeat_highlight_symbol(true)
}

fn shell_areas(area: Rect) -> (Rect, Rect, Rect) {
    let header_height = u16::from(area.height > 0);
    let footer_height = u16::from(area.height > 1);
    let body_height = area
        .height
        .saturating_sub(header_height)
        .saturating_sub(footer_height);

    (
        Rect::new(area.x, area.y, area.width, header_height),
        Rect::new(
            area.x,
            area.y.saturating_add(header_height),
            area.width,
            body_height,
        ),
        Rect::new(
            area.x,
            area.y
                .saturating_add(area.height.saturating_sub(footer_height)),
            area.width,
            footer_height,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend, widgets::ListState};

    #[test]
    fn shell_renders_chrome_and_returns_padded_content() {
        let backend = TestBackend::new(40, 6);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        let theme = Theme::default();
        let mut content = Rect::default();

        terminal
            .draw(|frame| {
                content = Shell::new("MamboUI")
                    .subtitle("demo")
                    .footer("[q] quit")
                    .render(frame, &theme);
            })
            .expect("render succeeds");

        assert_eq!(content, Rect::new(1, 1, 38, 4));
        let buffer = terminal.backend().buffer();
        assert_eq!(row(buffer, 0).trim_end(), "MamboUI  demo");
        assert_eq!(row(buffer, 5).trim_end(), "[q] quit");
    }

    #[test]
    fn semantic_widgets_keep_text_labels() {
        let backend = TestBackend::new(32, 3);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        let theme = Theme::default();

        terminal
            .draw(|frame| {
                frame.render_widget(
                    StatusLine::new(Status::Error, "Network unavailable", &theme),
                    Rect::new(0, 0, 32, 1),
                );
                frame.render_widget(
                    Help::new([KeyHint::new("r", "retry")], &theme),
                    Rect::new(0, 1, 32, 1),
                );
            })
            .expect("render succeeds");

        let buffer = terminal.backend().buffer();
        assert_eq!(row(buffer, 0).trim_end(), "[ERROR] Network unavailable");
        assert_eq!(row(buffer, 1).trim_end(), "[r] retry");
    }

    #[test]
    fn selection_list_marks_the_selected_item() {
        let backend = TestBackend::new(16, 2);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        let theme = Theme::default();
        let mut state = ListState::default().with_selected(Some(1));

        terminal
            .draw(|frame| {
                frame.render_stateful_widget(
                    selection_list(["First", "Second"], &theme),
                    frame.area(),
                    &mut state,
                );
            })
            .expect("render succeeds");

        assert_eq!(row(terminal.backend().buffer(), 1).trim_end(), "› Second");
    }

    fn row(buffer: &Buffer, y: u16) -> String {
        (0..buffer.area.width)
            .filter_map(|x| buffer.cell((x, y)))
            .map(|cell| cell.symbol())
            .collect()
    }
}
