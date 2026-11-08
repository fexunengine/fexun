//! FeXun's initial visual novel engine integration with Bevy.
//!
//! Add [`FexunBevyPlugin`] to a Bevy app to install Bevy's default plugins.
//! FeXun enables Bevy's 2D, UI, and audio features. Story execution and
//! visual novel authoring tools are planned features.
//!
//! ```no_run
//! use bevy::app::App;
//! use fexun::FexunBevyPlugin;
//!
//! App::new().add_plugins(FexunBevyPlugin).run();
//! ```
//!
//! Register FeXun once. Adding Bevy's default plugins separately would
//! duplicate their registration.

use bevy::DefaultPlugins;
use bevy::app::{App, Plugin};

/// Registers FeXun and Bevy's default plugins with a Bevy app.
pub struct FexunBevyPlugin;

impl Plugin for FexunBevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
