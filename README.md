# Sodium Clump for Horizon

A native mod for **Horizon** (Minecraft Java 26.3 rewritten in Rust) that aims to bring the useful bits of:

- **Sodium** – performance-oriented rendering mindset (Horizon already does most of this natively)
- **Sodium Extra** – extra visual toggles (animations, particles, fog, clouds, entity details, etc.)
- **Reese's Sodium Options** – cleaner, searchable, tabbed options UI

into one package.

> **Important reality check**  
> Horizon already replaces the core of what Sodium does (native multi-draw terrain, better culling, worker-thread meshing, distant LOD, etc.). A full 1:1 Sodium renderer port is neither possible nor useful.  
> This mod focuses on the **extra options + better settings UX** that players actually miss.

## Status

Expanded scaffold based on the official `horizon-native-mod-template`.

Already done:
- Full config system with all common Sodium Extra style toggles
- JSON load/save (`config/sodium_clump.json`)
- Global config access
- `frame` + `hud` + example `break_block` injections
- Option category list ready for a searchable UI
- Unit tests for the pure logic

Still TODO (needs real Horizon Game bindings after you stage):
- Actually force the visual flags / draw the HUD overlays
- Native searchable options screen

## Requirements

- Linux (template tooling uses Linux file locks)
- Rust 1.95.0 (see `rust-toolchain.toml`)
- Python 3.12+
- Git + Cargo
- A clone of Horizon and horizon-mixins at the pinned revisions (see `compatibility.json`)

## Quick start

```bash
# 1. Clone the tools next to this folder
git clone https://github.com/horizon-rust/horizon ../horizon
git clone https://github.com/horizon-rust/horizon-mixins ../horizon-mixins
git -C ../horizon-mixins checkout "$(python3 -c 'import json; print(json.load(open("compatibility.json"))["horizon_mixins_revision"])')"

# 2. Fetch
cargo fetch --locked
cargo test --workspace --locked --offline

# 3. Stage, check, test, build
python3 tools/horizon.py stage --horizon ../horizon
python3 tools/horizon.py check --horizon ../horizon
python3 tools/horizon.py test  --horizon ../horizon
python3 tools/horizon.py build --horizon ../horizon
```

You will need prepared game resources for the pinned Horizon commit (use the official launcher or `horizon-prepare`).

## Project layout

```
sodium_clump/
├── Cargo.toml
├── compatibility.json          # pinned Horizon + mixins revisions
├── mixins/sodium_clump/
│   ├── horizon-mixin.toml      # mod id, version, injections
│   ├── break.rs                # example typed hook adapter
│   ├── frame.rs                # per-frame adapter
│   ├── hud.rs                  # HUD adapter
│   └── runtime.rs              # main logic + unit tests
└── tools/
    └── horizon.py              # stage / plan / apply / check / build (replace with real one)
```

## What to implement next

1. **Config** – expand `Config` in `runtime.rs` with the toggles you want (particles, animations, fog distance, cloud height, entity shadows, etc.).
2. **Options screen** – use Horizon’s native screen / settings hooks (see current `native-mods.md` and `native-hooks.json` in the Horizon source) to draw a searchable tabbed UI.
3. **Apply the options** – on frame / HUD / render hooks, read the config and change the relevant game state (or call into Horizon’s existing option system when possible).
4. **Package** – `python3 tools/package.py` → `.hnmod` ready for the registry (or local `horizon-native-mods/` folder).

## Licence

MIT (same as the official template and most Horizon community mods).

## Credits

- Scaffold based on https://github.com/horizon-rust/horizon-native-mod-template
- Inspired by Sodium (JellySquid / CaffeineMC), Sodium Extra & Reese’s Sodium Options (FlashyReese)
- Horizon project: https://playhorizon.dev
