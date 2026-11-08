use bevy::DefaultPlugins;
use bevy::app::{App, Plugin};

pub struct FexunBevyPlugin;

impl Plugin for FexunBevyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins);
    }
}
