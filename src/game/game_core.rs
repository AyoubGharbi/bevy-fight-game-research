use bevy::app::{App, Update};
use bevy::math::Vec2;
use bevy::prelude::{Camera2dBundle, Commands, Component, Entity, Plugin, Res, ResMut, Resource, Startup};

use crate::core::core::{GameMode, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameCameraEntity::default())
            .add_systems(Startup, spawn_game_camera)
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

#[derive(Component)]
pub struct Player;

fn spawn_game_camera(
    mut commands: Commands,
    mut game_camera_entity: ResMut<GameCameraEntity>) {
    let mut entity = commands.spawn(Camera2dBundle::default());
    entity.insert(GameCamera::new(1.0, Vec2::ZERO));
    game_camera_entity.entity = Some(entity.id());
}


fn game_state_adapter_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_camera_entity: ResMut<GameCameraEntity>, ) {
    match &game_state.mode {
        GameMode::Editor => {
            if let Some(entity) = game_camera_entity.entity.take() {
                commands.entity(entity).despawn();
            }
        }

        GameMode::Game => {
            if game_camera_entity.entity.is_none() {
                spawn_game_camera(commands, game_camera_entity);
            }
        }
    }
}