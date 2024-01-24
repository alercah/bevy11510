use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, |mut commands: Commands, asset_server: Res<AssetServer>| {
            let handle: Handle<Image> = asset_server.load("Shimla_night.jpg");
            commands.spawn(SpriteBundle{
                texture: handle,
                ..default()
            });
            commands.spawn(Camera2dBundle::default());
        })
        .run();
}
