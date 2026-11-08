<p align="center">
  <img src="assets/fexo_256x256.webp" alt="Fexo, FeXun mascot">
</p>

<div>&nbsp;</div>

FeXun is a visual novel engine written in Rust and powered by [Bevy](https://github.com/bevyengine/bevy). Its long-term goal is a no-code authoring tool with optional Lua scripting for complex logic.

## Workspace

Engine crates live under `crates/fexun-*`. [`crates/fexun-core`](crates/fexun-core) is the `fexun` package and exposes `fexun::FexunBevyPlugin` directly. FeXun enables Bevy's 2D, UI, and audio features and registers `DefaultPlugins`. The [test harness](bin/main.rs) lives in `bin/`.

In your own Bevy app, register FeXun once to set up the engine and Bevy's default plugins.

```rust
use bevy::app::App;
use fexun::FexunBevyPlugin;

fn main() {
    App::new().add_plugins(FexunBevyPlugin).run();
}
```

## Development and release

Use [rustup](https://rustup.rs/) and follow the [Bevy platform setup guide](https://bevy.org/learn/quick-start/getting-started/setup/) for native system dependencies. The workspace selects stable Rust and declares a minimum of Rust 1.99.1.

| Command | Purpose |
| --- | --- |
| `cargo dev` | Run the native app with Bevy dynamically linked for faster iterative links. |
| `cargo run` | Run the app with Bevy statically linked in the dev profile. |
| `cargo check -p fexun` | Check the engine crate independently of the test executable. |
| `cargo test --workspace --locked` | Test the library and app with the committed lockfile. |
| `cargo build --release --locked -p fexun-app` | Build the native executable in `target/release/` with Bevy statically linked. |

For an IDE Cargo run configuration equivalent to `cargo dev`, use `run -p fexun-app --features dev-dylib`. Cargo supplies the library search paths when it launches the app. The first dynamic build has to compile and link the Bevy dylib. Subsequent edits to FeXun can reuse it.

Cargo profiles control compiler settings independently of features. Dev builds keep debug information and optimize dependencies. Release builds use thin LTO and a single codegen unit. For distribution, use the release command above so `dev-dylib` stays off. Adding `--all-features` to an app release build would turn it on. The `wasm-release` profile inherits the release settings and optimizes for size.

## Bevy plugin guides and docs

1. [Creating your first plugin](https://bevy.org/learn/quick-start/getting-started/plugins/#creating-your-first-plugin) walks through moving systems into a plugin and registering it in a game.
2. [Official Bevy 0.20.1 plugin example](https://github.com/bevyengine/bevy/blob/v0.20.1/examples/app/plugin.rs) shows a plugin with configuration, a resource, and an update system.
3. [`Plugin` trait](https://docs.rs/bevy_app/0.20.1/bevy_app/trait.Plugin.html) documents `build`, lifecycle hooks, and plugin uniqueness.
4. [`App` API](https://docs.rs/bevy_app/0.20.1/bevy_app/struct.App.html) covers registering plugins, systems, resources, and messages.
5. [ECS quick start](https://bevy.org/learn/quick-start/getting-started/ecs/) and [Resources](https://bevy.org/learn/quick-start/getting-started/resources/) explain how plugin systems access game data.
6. [Building Bevy's ecosystem](https://bevy.org/learn/quick-start/plugin-development/) covers dependency features, compatibility, and publishing a reusable plugin.

## Publishing the crate

The public package is `fexun` in `crates/fexun-core`. The test executable has `publish = false`. The current release version is `0.1.0`.

Before release, review the public README and version, synchronize the crate's license copies with the root licenses, and commit the intended release contents. Run these checks from the workspace root.

```sh
cargo fmt --all -- --check
cargo package -p fexun --list
cargo publish -p fexun --dry-run --locked
```

Use `--allow-dirty` when verifying an uncommitted working tree. Review the archive's normalized manifest and contents under `target/package/`. It should contain the library, public README, both licenses, and Cargo-generated metadata.

Once the release is approved and the crates.io account is authenticated, publish with `cargo publish -p fexun --locked`. Registry name availability, ownership, and acceptance require an online crates.io check.
