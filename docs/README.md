# MamboUI

[![CI](https://github.com/ProjectMambo/MamboUI/actions/workflows/ci.yml/badge.svg)](https://github.com/ProjectMambo/MamboUI/actions/workflows/ci.yml)
[![Latest release v0.1.0](https://img.shields.io/badge/release-v0.1.0-7a5fff.svg?style=flat-square)](https://github.com/ProjectMambo/MamboUI/releases/tag/v0.1.0)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Ratatui 0.29](https://img.shields.io/badge/ratatui-0.29-blue.svg?style=flat-square)](https://ratatui.rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)](https://github.com/ProjectMambo/MamboUI/blob/main/LICENSE)

MamboUI is the small, shared [Ratatui](https://ratatui.rs) presentation layer for Project Mambo terminal applications. It maps MamboColour's MamboOrche schemes into Ratatui and provides consistent app chrome, panels, feedback, keyboard help, selectable collections, and responsive layouts while leaving product state and workflows in each application.

Version `0.2.0` establishes the MamboColour-backed source contract. MamboUI is usable from Git but is not yet published to crates.io or tagged as a `0.2.0` release.

## Motivation

Project Mambo applications should feel related without rebuilding basic terminal UI decisions in every repository. MamboUI centralizes the small set of shared primitives that improve clarity, accessibility, and consistency while keeping each product in control of its screens.

## Status

MamboUI `0.2.0` is active source for Project Mambo Ratatui applications. The tagged `v0.1.0` release remains available for existing consumers; new `0.2.0` consumers should pin a reviewed commit until a release is published. MamboFinance is the primary proving ground: its real screens drive fixes and refinements to the existing contract, while new abstractions remain local until another Ratatui application demonstrates the same presentation rule.

## User stories

- As a Mambo application user, I can recognize navigation, status, and keyboard help across products.
- As a Mambo developer, I can assemble a clear terminal screen from tested shared components.
- As a maintainer, I can improve a common interaction once and adopt it consistently across applications.

## Getting started

### Install

Until the crate is published, pin a reviewed Git commit:

```toml
[dependencies]
mambo-ui = { git = "https://github.com/ProjectMambo/MamboUI.git", rev = "<MAMBOUI_COMMIT>" }
```

For local development next to this repository:

```toml
[dependencies]
mambo-ui = { path = "../MamboUI" }
```

MamboUI re-exports its exact Ratatui dependency as `mambo_ui::ratatui`, which prevents widget-type mismatches between Ratatui versions.

### Quick start

```rust
use mambo_ui::{Panel, Scheme, Shell, Theme};
use mambo_ui::ratatui::{Frame, widgets::Paragraph};

fn draw(frame: &mut Frame<'_>) {
    let theme = Theme::new(Scheme::Dark);
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

## Dependencies

| Dependency | Classification | Purpose | Provider, version pin, or source | Scope | Update path |
|---|---|---|---|---|---|
| Rust toolchain | Tool | Build and document the crate | Rust `1.85` or newer from `Cargo.toml` | Build/test | Update `rust-version`, compile on the minimum version, and run the full gate |
| Ratatui | Package | Widget, text, style, layout, frame, and test-backend types | crates.io `0.29`, resolved by `Cargo.lock` | Public API and runtime | Update `Cargo.toml`, the re-export contract, examples, and render tests together |
| MamboColour | Sibling package | Authoritative MamboOrche light and dark semantic colours | Git revision `c703092a619b3ee9b05503eaf2a553ae52595d35` | Build/runtime | Update the exact revision and role-mapping tests together |
| Crossterm | Package | Run the interactive example | crates.io `0.28`, resolved by `Cargo.lock` | Development only | Update `Cargo.toml`, the lockfile, and the demo smoke test together |

MamboUI deliberately exposes Ratatui types because it is a Ratatui presentation library. It does not expose MamboColour types: `Theme` is the adapter from MamboColour's semantic roles into Ratatui colours and styles.

## API

### Design principles

- Clarity first: every state has a text label, not colour alone.
- Ratatui-native: components compose with standard `Frame`, `Rect`, and widget types instead of introducing an application framework.
- Responsive by default: the shell preserves space on narrow terminals and two-column views stack when they no longer fit.
- Small public API: add a primitive only when more than one Mambo product needs the same behaviour.

## Components

| API | Purpose |
|---|---|
| `Scheme`, `Theme` | MamboOrche dark/light selection and shared semantic styles |
| `Shell`, `Header`, `Footer` | Responsive application chrome |
| `Panel` | Consistent titled and focused borders |
| `selection_list`, `selection_table` | Standard list and table selection treatment; callers own state and content |
| `EmptyState` | Centred empty view with a next step |
| `Status`, `StatusLine` | Labelled operation feedback |
| `KeyHint`, `Help` | Discoverable keyboard shortcuts |
| `responsive_columns` | Equal panes that sit side by side or stack on narrow screens |
| `centered_rect` | Bounds-safe rectangle for overlays and dialogs |

The crate root is the complete supported MamboUI API. Internal modules are private. All components accept a `Theme`; its fields are public for focused product customization, but `Theme` is non-exhaustive and must be created with `Theme::default()` or `Theme::new(Scheme)`. Prefer the bundled schemes so Mambo applications remain recognizable.

## Documentation

| Goal | Guide |
|---|---|
| Add MamboUI to a Rust application | [Install](#install) |
| Render the shared shell and panels | [Quick start](#quick-start) |
| Browse every public item and migration note | [API reference](API.md) |
| Choose or extend a component | [Design and developer guide](Design%20and%20Developer%20Guide.md) |
| Follow the Finance-led delivery order and compatibility gates | [Development plan](Development%20Plan.md) |
| Browse the public API | [Local rustdoc](#validation) |
| Read the public Wiki page | [projectmambo.org/mamboui/](https://projectmambo.org/mamboui/) |

## Project structure

```text
src/theme.rs          palette and semantic styles
src/components.rs     reusable Ratatui widgets
src/layout.rs         responsive layout helpers
examples/demo.rs      interactive component showcase
docs/                 synchronized user and developer guides
.github/workflows/    formatting, test, Clippy, and rustdoc checks
```

## Validation

Run the interactive component demo:

```bash
cargo run --locked --example demo
```

Run the same checks used by CI:

```bash
cargo fmt --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
```

## Development

Keep components product-neutral, keyboard-friendly, and understandable without colour. Use MamboFinance as the primary proving ground, but keep Finance-owned state and workflows in Finance. MamboUI owns shared presentation only; consuming applications own terminal setup, event loops, navigation, focus and selection state, commands, persistence, async work, and business rules. Include a focused render test for new behaviour and update the demo when a new public primitive is added. Author documentation in `notes/Docs/Projects/MamboUI/`, then synchronize it with `notes/Scripts/sync_docs.js`.

## License

MamboUI is available under the [MIT License](https://github.com/ProjectMambo/MamboUI/blob/main/LICENSE).
