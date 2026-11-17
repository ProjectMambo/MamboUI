---
description: Shared Ratatui components and visual conventions for Project Mambo terminal applications.
title: MamboUI
order: 100
---

::page{layout="project" width="normal" sidebar=true}

# MamboUI

MamboUI is Project Mambo's shared terminal presentation layer. It maps MamboColour's semantic MamboOrche schemes into Ratatui and provides accessible, Ratatui-native components without taking ownership of application state or business logic.

::button{label="Source code" href="https://github.com/ProjectMambo/MamboUI" variant="secondary" external=true}

## Current status

Version `0.2.0` is the active source contract. It adds dark and light MamboColour-backed themes, curated crate-root exports, selectable list and table styling, shell and panel chrome, status and help elements, empty states, responsive columns, and bounds-safe overlay placement. The tagged `v0.1.0` release remains available while `0.2.0` is not yet published or tagged.

## Documentation

::children{view="list" sort="order" direction="asc" show=["title","description"]}
