use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{game_assets::GameAssets, lifetime::Lifetime};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
}

pub fn shooting(
    mut commands: Commands,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
    mut towers: Query<&mut Tower>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());

        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn_bundle(SceneBundle {
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, false),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

pub fn bullet_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
