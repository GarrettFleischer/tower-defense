use bevy::prelude::*;

pub struct GameAssets {
    pub bullet_scene: Handle<Scene>,
}

pub fn load(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bullet_scene: assets.load("Bullet.glb#Scene0"),
    })
}
