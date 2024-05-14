use bevy::app::{App, Update};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Assets, AssetServer, Camera, Camera2dBundle, Commands, Component, default, Deref, DerefMut, Entity, Plugin, Query, Res, ResMut, Resource, SpriteSheetBundle, Startup, TextureAtlas, TextureAtlasLayout, Time, Timer, TimerMode, Transform};

use crate::core::core::{GameMode, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameCameraEntity::default())
            .insert_resource(Player::default())
            .add_systems(Update, animate_sprite)
            .add_systems(Update, game_state_adapter_system);
    }
}

#[derive(Component)]
struct GameCamera {
    zoom: f32,
    target: Vec2,
}

impl GameCamera {
    fn new(zoom: f32, target: Vec2) -> Self {
        Self { zoom, target }
    }
}

#[derive(Default, Resource)]
pub struct GameCameraEntity {
    pub entity: Option<Entity>,
}

#[derive(Default, Resource)]
pub struct Player {
    pub entity: Option<Entity>,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn game_state_adapter_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_camera_entity: ResMut<GameCameraEntity>,
    mut player: ResMut<Player>) {
    match &game_state.mode {
        GameMode::Editor => {
            if let Some(entity) = game_camera_entity.entity.take() {
                commands.entity(entity).despawn();
            }

            if let Some(entity) = player.entity.take() {
                commands.entity(entity).despawn();
            }
        }

        GameMode::Game => {
            if game_camera_entity.entity.is_none() {
                setup(commands, asset_server, texture_atlas_layouts, game_camera_entity, player);
            }
        }
    }
}


fn animate_sprite(
    time: Res<Time>,
    game_state: Res<GameState>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    if game_state.mode != GameMode::Game {
        return;
    }

    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game_camera_entity: ResMut<GameCameraEntity>,
    mut player: ResMut<Player>,
) {
    let mut entity = commands.spawn(Camera2dBundle::default());
    entity.insert(GameCamera::new(1.0, Vec2::ZERO));
    game_camera_entity.entity = Some(entity.id());


    let texture = asset_server.load("player-punch-cross/player-punch cross-64x64.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    let entity = commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));

    player.entity = Some(entity.id());
}