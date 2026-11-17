---
description: Finance-led stages, compatibility gates, and evidence for evolving MamboUI's shared Ratatui presentation contract.
title: Development plan
order: 30
---

::page{layout="docs" width="normal" sidebar=true}

# Development plan

MamboUI is an early, deliberately small Ratatui presentation library. The current `0.2.0` source contract provides MamboColour-backed themes, shell and panel chrome, selection styling, status and help elements, empty states, responsive columns, and centered overlay placement. It is available from Git but is not yet tagged or published as `0.2.0`.

MamboFinance is the primary proving ground for this contract and the first full product plan to exercise it across simple screens, dense tables, editors, confirmations, statistics, and import review. MamboTools remains an earlier `0.1.0` consumer and can provide an independent compatibility check when it is ready to move. This plan follows the MamboFinance delivery order without turning Finance-specific behavior into library API.

The plan is dependency-ordered, not a calendar promise. Update it when consumer evidence changes the public boundary, compatibility policy, stage sequence, or release gate.

## Target outcome

MamboUI should remain the smallest shared layer that makes Project Mambo Ratatui applications visually consistent, accessible, and easy to compose:

~~~text
MamboColour --------> MamboUI theme and presentation -----> MamboFinance screens
Ratatui ------------> native widgets and layout ----------> app-owned state and actions
~~~

MamboFinance can complete its planned TUI with MamboUI's current primitives plus native Ratatui widgets. Real Finance use drives fixes and product-neutral refinements to the existing contract. A new primitive enters MamboUI only after another Project Mambo Ratatui application needs the same presentation rule; until then, Finance implements the idea locally and supplies the evidence needed to design a clean shared boundary.

MamboUI owns shared rendering and layout conventions. It does not own terminal setup, the event loop, navigation, focus or selection state, forms, commands, persistence, background work, MCP behavior, or business rules.

## Scope and deferrals

This plan includes:

- adopting the `0.2.0` source contract in MamboFinance through an exact reviewed revision;
- correcting defects and accessibility or responsive-layout gaps found through Finance use;
- validating existing components against Finance's progressively denser screens;
- promoting only independently demonstrated, app-neutral presentation rules;
- preserving Ratatui-native composition and a small crate-root API;
- deciding, documenting, and validating each compatible or breaking release.

This plan deliberately excludes a generic form framework, router, component trait, message bus, command system, state machine, background-task framework, chart abstraction, persistence layer, MCP integration, and Finance-specific widgets. Native Ratatui remains the first implementation for tabs, scrolling, forms, gauges, charts, master/detail layouts, and confirmation content. Reconsider an exclusion only when two real applications repeat the same presentation rule and neither Ratatui nor the existing MamboUI API already solves it cleanly.

## Ownership and change rule

| Boundary | Owner |
|---|---|
| MamboOrche semantic values and contrast | MamboColour |
| Ratatui version exposed to consumers | MamboUI through `mambo_ui::ratatui` |
| Shared theme mapping, chrome, selection treatment, status, help, empty states, and small layout helpers | MamboUI |
| Finance navigation, focus, forms, validation, commands, charts, screen state, and domain behavior | MamboFinance |
| Terminal setup and restoration, persistence, async work, and typed AI tools | Consuming application |

A Finance finding can change MamboUI immediately when it reveals a defect, an accessibility failure, a misleading public contract, or a product-neutral gap in an existing primitive. A new abstraction remains local to Finance until an independent consumer demonstrates that the same rule is shared. Provider changes land and pass MamboUI's gate before a consumer updates its exact revision; the consumer then runs its own regression gate before the change is considered complete.

## Stage 0: Establish the shared baseline — complete

The `0.2.0` source contract established:

- `Scheme` and `Theme` backed by the pinned MamboColour revision;
- `Shell`, `Header`, `Footer`, `Panel`, `EmptyState`, `StatusLine`, and `Help`;
- native Ratatui `List` and `Table` selection helpers with app-owned state;
- `responsive_columns` and bounds-safe `centered_rect` layout helpers;
- private implementation modules and curated crate-root exports;
- render and layout tests, an interactive demo, API guidance, and migration notes.

Evidence is the `0.2.0` manifest, public API reference, demo, and passing format, test, Clippy, rustdoc, doctest, and strict repository checks. The source is merged, but the version is not yet a tagged release.

Exit gate: complete. A consumer can pin the reviewed source revision, compose native Ratatui widgets with the re-exported version, and keep all persistent interaction state outside MamboUI.

Not included: application navigation, forms, commands, domain widgets, or a stable `1.0` compatibility promise.

## Stage 1: Prove the baseline in MamboFinance

Prerequisite: MamboFinance has an executable TUI spine and can pin an exact MamboUI revision.

Use the current API to render Finance's empty shell and first-run path. Finance owns tab selection, Manage navigation, focus, account and category forms, validation, and actions. MamboUI changes only when this integration exposes a defect or a product-neutral problem in an existing component.

Affected surfaces are the MamboUI public API only when required, focused render tests, the demo when composition changes, synchronized API or developer guidance, the Finance dependency pin, and Finance render or interaction tests. No Finance data model, storage, or MCP surface crosses into MamboUI.

Focused checks:

- build Finance against `mambo_ui::ratatui` without a conflicting Ratatui type boundary;
- render the shell, focused panels, empty state, validation status, and contextual help in dark and light schemes;
- verify the first-run screen at 80 by 24 and a narrower reduced layout;
- verify that labels and focus remain understandable without colour.

Exit gate: the Finance first-account journey renders and navigates with the pinned MamboUI revision, rejected input remains Finance-owned, terminal restoration remains Finance-owned, and both repositories pass their documented gates.

Not included: a shared tabs widget, form state, validation model, or navigation framework.

## Stage 2: Pressure-test tables and responsive detail

Prerequisite: Finance accounts, categories, and first-run behavior have passed Stage 1.

Exercise the existing selection, panel, layout, status, and help contracts on Finance transaction, budget, overview, and insight screens. Finance implements filtering, sorting, scrolling, table state, editors, confirmations, budget progress, and charts with native Ratatui and local state. Fix MamboUI when its existing presentation behavior clips, loses focus or selection meaning, breaks theme semantics, or prevents clean native composition.

Affected MamboUI surfaces are limited to existing primitives, their render tests, demo examples, public docs, and a SemVer decision for any observable change. Finance owns representative screen fixtures and regression coverage.

Focused checks:

- wide, 80-by-24, and reduced single-pane layouts preserve every available action;
- selected and focused states remain distinct in light, dark, and monochrome inspection;
- tables with no rows and filtered empty results show a useful next step;
- detail panes and overlays stay within frame bounds;
- status and help text describe state without relying on colour.

Exit gate: Finance can complete the transaction, budget, overview, and insight journeys using the current shared boundary; any MamboUI change has a focused regression test and a reviewed Finance pin update.

Not included: shared finance formatters, filters, table models, gauges, charts, editors, or confirmation state.

## Stage 3: Pressure-test dense review workflows

Prerequisite: Finance's ordinary ledger screens satisfy Stage 2.

Use Finance import staging and review to test dense tables, long labels, warning and error states, expanded help, overlays, and slow-operation feedback. Keep import decisions, row state, progress state, issue codes, and commit confirmations in Finance. Prefer local Ratatui composition before changing MamboUI.

If an independent Project Mambo Ratatui consumer needs the same presentation rule, promote the smallest stateless primitive. Promotion must include a render test, demo or example, public documentation, a SemVer decision, migration notes when needed, a provider-first commit, an exact Finance pin update, and Finance regression checks.

Focused checks:

- long and untrusted report text remains data and does not corrupt the layout;
- warning, error, busy, and success states include visible labels;
- overlays fit the frame or degrade to a reduced layout;
- keyboard-only review exposes the currently available actions;
- no shared component retains Finance state or invokes a Finance command.

Exit gate: the Finance stage-review-commit journey uses MamboUI without leaking Finance concepts into the public API, and every promoted primitive is already exercised by an independent consumer.

Not included: import-specific widgets, progress orchestration, background tasks, or a generic confirmation workflow.

## Stage 4: Stabilize releases and independent adoption

Prerequisite: the relevant Finance stages pass with an exact MamboUI revision and no unresolved boundary defect.

Choose the next release from evidence rather than a preset feature list. A compatible fix or addition increments the `0.x` minor version when required by Cargo SemVer expectations; a breaking public change receives a new minor version with migration guidance. Tagging or publishing remains a separate authorized release action.

Use MamboTools or the next Project Mambo Ratatui application as the independent adoption check. Migrate only when that consumer has a real need; do not add adapters solely to claim another consumer. Keep the last working exact revision available so a consumer can roll back code without changing user data.

Focused checks:

- compile every adopting consumer against the exact Ratatui boundary;
- run MamboUI's full gate and each changed consumer's regression gate;
- review public exports, rustdoc, migration notes, and synchronized documentation;
- verify the demo in dark and light schemes and at representative terminal sizes;
- confirm the release does not claim ownership of consumer state or data.

Exit gate: the chosen revision has a documented compatibility decision, passes MamboUI and consumer gates, is usable by at least two real applications where a new abstraction is shared, and has an explicit tag or publication decision.

Not included: a promise to reach `1.0` on a schedule. Consider `1.0` only after the boundary survives multiple consumers and ordinary upgrades without repeated breaking redesign.

## Data, compatibility, and recovery

MamboUI stores no user data, credentials, or application state, so it has no data migration or backup format. Its risks are source compatibility, Ratatui type compatibility, and changed rendering behavior.

Consumers pin an exact reviewed Git revision until a release is tagged. MamboUI re-exports its Ratatui version; types that meet MamboUI must use that path. MamboColour stays behind `Theme` and remains pinned to an exact provider revision. Update either provider before consumers, run the full provider gate, then update and test each consumer pin.

During `0.x`, breaking public changes use a new minor version and include migration notes. A consumer recovers by returning to its last reviewed MamboUI revision. Because MamboUI owns no persistent state, rollback must not require or perform user-data changes.

## Security, privacy, and accessibility

MamboUI performs no networking, persistence, authentication, logging, terminal initialization, or command execution. It must not retain or log consumer-provided text. Consumers own authorization, sensitive-data handling, input validation, terminal restoration, and destructive confirmations.

Shared rendering must treat caller text as display data, stay within the supplied frame, preserve useful labels without colour, distinguish focus from selection, and keep keyboard actions discoverable. Any new primitive must state what the caller owns and must not hide a security- or domain-sensitive transition inside presentation code.

## Validation

Every MamboUI change runs the narrow test for the affected primitive followed by:

~~~bash
cargo fmt --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
cargo test --locked --doc
../MamboDocs/script/check-repository.sh --strict .
git diff --check
~~~

Public rendering changes also update or exercise the demo and inspect representative light, dark, monochrome, wide, 80-by-24, and reduced layouts as appropriate. After a provider change, every adopting consumer updates its exact revision and runs its own full gate. Documentation changes are authored in the canonical notes tree, synchronized to MamboUI and MamboWiki, and validated in both outputs.

## Milestones and risks

| Milestone | Included stages | Outcome |
|---|---|---|
| Shared baseline | Stage 0 | Small documented `0.2.0` source contract |
| Finance adoption | Stage 1 | First full product path on the shared boundary |
| Product pressure test | Stages 2 and 3 | Existing primitives proven across ordinary and dense Finance workflows |
| Independent release evidence | Stage 4 | Versioned contract validated by another real Ratatui consumer |

No milestone advances while an earlier exit gate has an unresolved MamboUI boundary defect.

Known risks and evidence triggers:

- **Finance bias:** a pattern that looks generic in one product may encode Finance workflow assumptions. Keep new abstractions local until an independent consumer repeats the need.
- **Framework creep:** stateful forms, routers, commands, or task orchestration would blur ownership. Add none unless multiple consumers prove a smaller stateless boundary cannot solve the problem.
- **Version coupling:** Ratatui types are public. Any Ratatui upgrade requires provider-first compilation and consumer regression evidence.
- **Visual regressions:** terminal size, colour capability, and long content vary. Add focused render coverage when Finance exposes a real failure rather than constructing a speculative matrix.
- **Release drift:** source version `0.2.0` is not the latest tag. Do not advertise it as released until an authorized tag and publication step succeeds.
