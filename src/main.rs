mod editor;

use std::fs;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::editor::{editor::EditorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        // .add_systems(Update, animate_player)
        .add_systems(Update, draw_collision_boxes)
        .run();
}

#[derive(Component)]
struct Player {
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
    size: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player-punch-jab/player-jab-48x48.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(48.0, 48.0), 10, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let anim_indices = AnimationIndices { first: 1, last: 9, size: 10 };

    // Load settings
    let saved_hitbox: HitBox = load_settings_from_file("hitbox_settings.json");
    let saved_hurtbox: HurtBox = load_settings_from_file("hurtbox_settings.json");

    commands.spawn(SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            index: anim_indices.first,
        },
        transform: Transform::from_scale(Vec3::splat(6.0)),
        ..default()
    })
        .insert(
            Player {
                animation_indices: anim_indices,
                animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            }
        )
        .insert(saved_hitbox)
        .insert(saved_hurtbox);
}


fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut TextureAtlas)>,
) {
    for (mut player, mut texture_atlas) in &mut query {
        player.animation_timer.tick(time.delta());
        if player.animation_timer.just_finished() {
            texture_atlas.index = if texture_atlas.index == player.animation_indices.last {
                player.animation_indices.first
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
struct HitBox {
    size: Vec2,
    offset: Vec2,
}

#[derive(Component, Serialize, Deserialize, Clone)]
struct HurtBox {
    size: Vec2,
    offset: Vec2,
}

fn draw_collision_boxes(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &HitBox, &HurtBox)>,
) {
    for (transform, _hitbox, _hurtbox) in query.iter() {
        let scale = transform.scale.truncate();

        let saved_hitbox: HitBox = load_settings_from_file("hitbox_settings.json");
        let saved_hurtbox: HurtBox = load_settings_from_file("hurtbox_settings.json");

        let hitbox_size_scaled = _hitbox.size * scale;
        let hitbox_offset_scaled = _hitbox.offset * scale;

        let hurtbox_size_scaled = _hurtbox.size * scale;
        let hurtbox_offset_scaled = _hurtbox.offset * scale;

        gizmos.rect_2d(
            transform.translation.truncate() + hitbox_offset_scaled,
            0.0,
            hitbox_size_scaled,
            Color::RED,
        );

        gizmos.rect_2d(
            transform.translation.truncate() + hurtbox_offset_scaled,
            0.0,
            hurtbox_size_scaled,
            Color::GREEN,
        );
    }
}

fn load_settings_from_file<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
}
