---
description: Apply and extend MamboUI components without turning the crate into an application framework.
title: Design and developer guide
order: 10
---

::page{layout="docs" width="normal" sidebar=true}

# Design and developer guide

MamboUI owns presentation patterns that should look and behave the same in more than one Project Mambo terminal application. It is a view library, not an application framework.

| Boundary | Owner |
|---|---|
| Semantic MamboOrche values and contrast | MamboColour |
| Mapping semantic colours into Ratatui, shared visual styles, chrome, and small layout helpers | MamboUI |
| Terminal setup and restore, event loop, input mapping, focus and selection state, navigation, commands, persistence, async work, and business rules | Consuming application |

Ratatui types deliberately cross the public boundary because consumers compose MamboUI with native widgets. MamboColour types do not cross it; `Theme` is the adapter between the palette provider and Ratatui.

## Compose a screen

1. Render `Shell` across the complete frame and retain the returned content rectangle.
2. Split the content with Ratatui layouts or `responsive_columns`.
3. Wrap major regions with `Panel`; mark only the active region as focused.
4. Use `selection_list` or `selection_table` with the application's `ListState` or `TableState` for navigable collections.
5. Reserve `StatusLine` for operation feedback and `Help` for the currently available keys.
6. Use `EmptyState` when a collection has no rows and include the next useful action.
7. Use `centered_rect` with Ratatui's `Clear` for a bounds-safe overlay; the application still owns whether the overlay is open and what actions it performs.

Create one `Theme` per draw or application and pass it to every shared component. `Theme::default()` is the dark scheme; use `Theme::new(Scheme::Light)` for the light scheme. Start with a bundled scheme and change individual public fields only for a concrete product requirement.

## Public API rules

- Import supported items from the crate root. The source modules are private so implementation file names cannot become consumer dependencies.
- Treat the `mambo_ui::ratatui` re-export as the compatible Ratatui boundary. Do not add a second Ratatui version to the same consumer unless its types never meet MamboUI.
- Keep `Theme` non-exhaustive. Consumers may tune fields after construction but cannot construct or exhaustively destructure it.
- Keep persistent state with the application. MamboUI builders create Ratatui widgets for the current render and never retain selection, focus, form data, or commands.
- Add no generic component trait, event type, message bus, runtime, or plugin interface until multiple applications require the same behavior and Ratatui itself does not already provide it.

## Accessibility contract

- Every success, warning, and failure includes a visible text label.
- Keyboard actions appear in `Help` whenever they are available but not obvious.
- Selection uses both a leading symbol and a background treatment; focus uses a distinct border and layout context.
- Copy remains useful in a monochrome terminal.
- Narrow layouts preserve every action and stack content rather than clipping it.

RGB values define the preferred palette, but semantic meaning comes from the component and label. Applications may override individual public `Theme` fields for a concrete product need; avoid creating parallel theme systems.

## Add a shared primitive

Add a component only after a second real consumer needs the same presentation rule. Keep it Ratatui-native, accept borrowed data where practical, and leave state transitions with the caller. Prefer a function returning a standard Ratatui widget over a new wrapper type when no shared state or rendering behavior is required.

A public change includes:

1. the smallest implementation in `src/components.rs`, `src/layout.rs`, or `src/theme.rs`;
2. one render or unit test that proves the shared behaviour;
3. an example update when users need to see composition;
4. API documentation and this guide when the contract changes;
5. a SemVer decision and migration note before release.

Run the full validation sequence from the README. During `0.x`, use a new minor version for a breaking public change. After `1.0.0`, use a new major version for a removed item, renamed public type, changed method signature, or materially different rendering contract.
