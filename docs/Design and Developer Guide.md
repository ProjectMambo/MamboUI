---
description: Apply and extend MamboUI components without turning the crate into an application framework.
title: Design and developer guide
order: 10
---

::page{layout="docs" width="normal" sidebar=true}

# Design and developer guide

MamboUI owns presentation patterns that should look and behave the same in more than one Project Mambo terminal application. Product state, business logic, routing, persistence, and async work remain in the consuming application.

## Compose a screen

1. Render `Shell` across the complete frame and retain the returned content rectangle.
2. Split the content with Ratatui layouts or `responsive_columns`.
3. Wrap major regions with `Panel`; mark only the active region as focused.
4. Use `selection_list` with the application's `ListState` for navigable collections.
5. Reserve `StatusLine` for operation feedback and `Help` for the currently available keys.
6. Use `EmptyState` when a collection has no rows and include the next useful action.

The consuming application owns focus, selection, key handling, commands, and error text. Components receive state and render it; they do not start their own event loop.

## Accessibility contract

- Every success, warning, and failure includes a visible text label.
- Keyboard actions appear in `Help` whenever they are available but not obvious.
- Focus uses both border treatment and layout context; never rely on hue alone.
- Copy remains useful in a monochrome terminal.
- Narrow layouts preserve every action and stack content rather than clipping it.

RGB values define the preferred palette, but semantic meaning comes from the component and label. Applications may override individual public `Theme` fields for a concrete product need; avoid creating parallel theme systems.

## Add a shared primitive

Add a component only after a second real consumer needs the same presentation rule. Keep it Ratatui-native, accept borrowed data where practical, and leave state transitions with the caller.

A public change includes:

1. the smallest implementation in `src/components.rs`, `src/layout.rs`, or `src/theme.rs`;
2. one render or unit test that proves the shared behaviour;
3. an example update when users need to see composition;
4. API documentation and this guide when the contract changes;
5. a SemVer decision before release.

Run the full validation sequence from the README. Treat a removed item, renamed public type, changed method signature, or materially different rendering contract as a breaking change once `1.0.0` is released.
