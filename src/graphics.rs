use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::AnimationTimer;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animation_system);
    }
}

fn animation_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &Velocity,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, velocity) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }

        //This may be a naive approach but it might jsut work for what we need, and it would be tedious to create 2 copies of every sprite
        if velocity.linvel.x < 0.0 {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}
