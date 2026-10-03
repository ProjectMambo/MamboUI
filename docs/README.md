# MamboUI

[![CI](https://github.com/ProjectMambo/MamboUI/actions/workflows/ci.yml/badge.svg)](https://github.com/ProjectMambo/MamboUI/actions/workflows/ci.yml)
[![Version 0.1.0](https://img.shields.io/badge/version-0.1.0-7a5fff.svg)](https://github.com/ProjectMambo/MamboUI)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)
[![Ratatui 0.29](https://img.shields.io/badge/ratatui-0.29-blue.svg)](https://ratatui.rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/ProjectMambo/MamboUI/blob/main/LICENSE)

MamboUI is the small, shared [Ratatui](https://ratatui.rs) design layer for Project Mambo terminal applications. It gives Mambo products a consistent app shell, accessible palette, feedback states, keyboard help, panels, lists, and responsive layouts while leaving product-specific screens in the product repository.

Version `0.1.0` establishes the initial component contract. MamboUI is usable from Git but is not yet published to crates.io.

## Start here

| Goal | Guide |
|---|---|
| Add MamboUI to a Rust application | [Install](#install) |
| Render the shared shell and panels | [Quick start](#quick-start) |
| Choose or extend a component | [Design and developer guide](Design%20and%20Developer%20Guide.md) |
| Browse the public API | [Local rustdoc](#demo-and-checks) |
| Read the public Wiki page | [projectmambo.org/mamboui/](https://projectmambo.org/mamboui/) |

## Design principles

- Clarity first: every state has a text label, not colour alone.
- Ratatui-native: components compose with standard `Frame`, `Rect`, and widget types instead of introducing an application framework.
- Responsive by default: the shell preserves space on narrow terminals and two-column views stack when they no longer fit.
- Small public API: add a primitive only when more than one Mambo product needs the same behaviour.

## Install

Until the crate is published, use the Git repository:

```toml
[dependencies]
mambo-ui = { git = "https://github.com/ProjectMambo/MamboUI.git", tag = "v0.1.0" }
```

For local development next to this repository:

```toml
[dependencies]
mambo-ui = { path = "../MamboUI" }
```

MamboUI re-exports its exact Ratatui dependency as `mambo_ui::ratatui`, which prevents widget-type mismatches between Ratatui versions.

## Quick start

```rust
use mambo_ui::{Panel, Shell, Theme};
use mambo_ui::ratatui::{Frame, widgets::Paragraph};

fn draw(frame: &mut Frame<'_>) {
    let theme = Theme::default();
    let content = Shell::new("MamboTools")
        .subtitle("Repository manager")
        .footer("[?] help  [q] quit")
        .render(frame, &theme);

    frame.render_widget(
        Paragraph::new("Choose a repository")
            .block(Panel::new("Repositories").focused(true).block(&theme)),
        content,
    );
}
```

## Components

| API | Purpose |
|---|---|
| `Theme` | High-contrast Project Mambo palette and shared styles |
| `Shell`, `Header`, `Footer` | Responsive application chrome |
| `Panel` | Consistent titled and focused borders |
| `selection_list` | Standard list selection treatment |
| `EmptyState` | Centred empty view with a next step |
| `Status`, `StatusLine` | Labelled operation feedback |
| `KeyHint`, `Help` | Discoverable keyboard shortcuts |
| `responsive_columns` | Side-by-side panes that stack on narrow screens |

All components accept a `Theme`; its fields are public for focused product customization. Prefer the default palette so Mambo applications remain recognizable.

## Repository layout

```text
src/theme.rs          palette and semantic styles
src/components.rs     reusable Ratatui widgets
src/layout.rs         responsive layout helpers
examples/demo.rs      interactive component showcase
docs/                 synchronized user and developer guides
.github/workflows/    formatting, test, Clippy, and rustdoc checks
```

## Demo and checks

Run the interactive component demo:

```bash
cargo run --example demo
```

Run the same checks used by CI:

```bash
cargo fmt --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Contributing

Keep components product-neutral, keyboard-friendly, and understandable without colour. Include a focused render test for new behaviour and update the demo when a new public primitive is added. Author documentation in `notes/Docs/Projects/MamboUI/`, then synchronize it with `notes/Scripts/sync_docs.js`.

## License

MamboUI is available under the [MIT License](https://github.com/ProjectMambo/MamboUI/blob/main/LICENSE).
