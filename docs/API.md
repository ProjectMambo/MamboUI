---
description: Consume MamboUI's curated theme, component, layout, and Ratatui boundaries.
title: API reference
order: 20
---

::page{layout="docs" width="normal" sidebar=true}

# API reference

MamboUI exports its complete supported contract from the crate root. Source modules are private. The API stays close to Ratatui: components accept or return native `Frame`, `Rect`, `Block`, `List`, `Table`, and state types rather than wrapping the application in a framework.

## Theme

```rust
use mambo_ui::{Scheme, Theme};

let dark = Theme::default();
let light = Theme::new(Scheme::Light);
```

`Theme::default()` is equivalent to `Theme::new(Scheme::Dark)`. Both schemes come from MamboColour's pinned MamboOrche provider revision and are converted to Ratatui RGB values. MamboColour types remain private to this adapter.

`Theme` is non-exhaustive. An application may change a public field after construction for a product-specific need, but it cannot construct or exhaustively destructure the type. The fields are semantic:

| Field | Meaning |
|---|---|
| `background`, `surface` | Main canvas and raised panel or chrome surface |
| `text`, `muted`, `subtle` | Primary, secondary, and tertiary text |
| `border`, `focus` | Normal boundary and active keyboard focus |
| `brand`, `on_brand` | Project Mambo identity and its contrast-checked text colour; shared selected rows use this pair |
| `selection`, `interactive` | Selection and general information or interaction indicators for application-specific composition |
| `success`, `warning`, `error` | Semantic operation outcomes |

The style methods `base`, `surface_style`, `title`, `muted`, `border`, `focused_border`, `selected`, and `title_line` apply those fields consistently. Use the methods before assembling a parallel set of product styles.

## Components

| Item | Input and output | Application-owned state |
|---|---|---|
| `Shell` | Renders the full background, one-line `Header`, and one-line `Footer`; returns the remaining content `Rect` | Current screen, navigation, footer text, and draw timing |
| `Header`, `Footer` | Stateless Ratatui widgets for standalone chrome composition | Content and placement |
| `Panel` | Builds a themed Ratatui `Block` with normal or focused treatment | Which panel is focused and the widget rendered inside |
| `selection_list` | Builds a themed Ratatui `List` from caller rows | `ListState`, selection movement, empty behavior, and actions |
| `selection_table` | Builds a themed Ratatui `Table` from caller rows and widths | `TableState`, headings, sorting, filtering, selection movement, and actions |
| `EmptyState` | Renders a centered title and optional next step | Whether the view is empty and what action is useful |
| `Status`, `StatusLine` | Renders a labelled idle, information, success, warning, or error message | Operation lifecycle, message text, retry, and dismissal |
| `KeyHint`, `Help` | Renders the currently available key/action pairs | Input mapping and which actions are active |

Components render only. They do not read terminal events, start tasks, mutate application data, confirm destructive operations, or persist state.

## Layout helpers

`responsive_columns(area, minimum_width)` returns two equal panes separated by one column when both can meet the minimum width. Otherwise it stacks two panes and preserves every row.

`centered_rect(area, width, height)` caps the requested size to its parent and centers the result. It performs no drawing. Render Ratatui's `Clear` before the returned rectangle when an overlay must hide underlying cells.

## Ratatui compatibility

MamboUI re-exports its exact Ratatui dependency as `mambo_ui::ratatui`. Use that path for types which meet MamboUI widgets so one application does not accidentally combine incompatible Ratatui versions. Ratatui is deliberately part of the public contract; Crossterm is not.

## Moving from 0.1

Version `0.2.0` is a breaking `0.x` source revision:

- import all MamboUI items from the crate root; `components`, `layout`, and `theme` are now private implementation modules;
- the default colours are the MamboOrche dark scheme instead of the earlier generic dark palette;
- construct a theme with `Theme::default()` or `Theme::new(Scheme)` because `Theme` is non-exhaustive;
- replace direct `theme.accent` access with `theme.brand` and direct `theme.info` access with `theme.interactive`;
- use `selection_table` and `centered_rect` where an application previously repeated the same presentation helpers.

The existing `Shell`, `Panel`, `selection_list`, `EmptyState`, `StatusLine`, and `Help` constructors remain available from the crate root.
