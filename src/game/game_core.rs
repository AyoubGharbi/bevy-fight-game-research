use bevy::app::{App, Update};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera2dBundle, Color, Commands, Component, default, DefaultGizmoConfigGroup, Deref, DerefMut, Entity, GizmoConfigStore, Gizmos, Plugin, Query, Res, ResMut, Resource, SpriteSheetBundle, TextureAtlas, Time, Timer, TimerMode, Transform};

use crate::core::core::{GameMode, GameState};
use crate::editor::editor_core::SpriteSheets;
use crate::editor::editor_gui::EditorSelectedSpriteSheet;
use crate::game::game_gui::GameGuiPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameGuiPlugin)
            .insert_resource(GameCameraEntity::default())
            .insert_resource(Player::default())
            .add_systems(Update, animate_sprite)
            .add_systems(Update, game_state_adapter_system)
            .add_systems(Update, gizmos_selected_sprite);
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

#[derive(Resource)]
pub struct GameSelectedSpriteSheet {
    pub id: Option<String>,
    pub frame_index: Option<usize>,
}

impl Default for GameSelectedSpriteSheet {
    fn default() -> Self {
        GameSelectedSpriteSheet {
            id: None,
            frame_index: Some(0),
        }
    }
}

fn game_state_adapter_system(
    mut commands: Commands,
    mut config_store: ResMut<GizmoConfigStore>,
    game_state: Res<GameState>,
    sprite_sheets: ResMut<SpriteSheets>,
    selected_sprite_sheet: ResMut<EditorSelectedSpriteSheet>,
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
                setup(commands, config_store, sprite_sheets, selected_sprite_sheet, game_camera_entity, player);
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
    mut config_store: ResMut<GizmoConfigStore>,
    mut sprite_sheets: ResMut<SpriteSheets>,
    selected_sprite_sheet: ResMut<EditorSelectedSpriteSheet>,
    mut game_camera_entity: ResMut<GameCameraEntity>,
    mut player: ResMut<Player>,
) {

    // gizmos

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 5.;

    // camera
    let mut entity = commands.spawn(Camera2dBundle::default());
    entity.insert(GameCamera::new(1.0, Vec2::ZERO));
    game_camera_entity.entity = Some(entity.id());

    if let Some(id) = &selected_sprite_sheet.id {
        if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get_mut(id) {
            let animation_indices = AnimationIndices { first: 1, last: sprite_sheet_atlas.sprite_sheet_info.columns - 1 };
            let texture_handle = sprite_sheet_atlas.texture_handle.clone();
            let entity = commands.spawn(
                (SpriteSheetBundle {
                    texture: texture_handle,
                    atlas: TextureAtlas {
                        layout: sprite_sheet_atlas.handle.clone(),
                        index: animation_indices.first,
                    },
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                },
                 animation_indices,
                 AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
                ));


            player.entity = Some(entity.id());
        }
    }
}

fn gizmos_selected_sprite(
    mut gizmos: Gizmos,
    selected_sprite_sheet: ResMut<EditorSelectedSpriteSheet>,
    mut sprite_sheets: ResMut<SpriteSheets>,
    mut query: Query<(&Transform, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Game {
        return;
    }

    if let Some(id) = &selected_sprite_sheet.id {
        if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get_mut(id) {
            for (transform, _indices, _timer, atlas) in &mut query {
                let scale = transform.scale.truncate();


                if let Some(frame_data) = sprite_sheet_atlas.sprite_sheet_info.frames.get_mut(atlas.index) {
                    for hit_box in &frame_data.hit_boxes {
                        let hit_box_size_scaled = hit_box.size * scale;
                        let hit_box_offset_scaled = hit_box.offset * scale;
                        gizmos.rect_2d(
                            transform.translation.truncate() + hit_box_offset_scaled,
                            0.0,
                            hit_box_size_scaled,
                            Color::rgba(1f32, 0f32, 0f32, 0.3f32),
                        );
                    }

                    for hurt_box in &frame_data.hurt_boxes {
                        let hurt_box_size_scaled = hurt_box.size * scale;
                        let hurt_box_offset_scaled = hurt_box.offset * scale;
                        gizmos.rect_2d(
                            transform.translation.truncate() + hurt_box_offset_scaled,
                            0.0,
                            hurt_box_size_scaled,
                            Color::rgba(0f32, 1f32, 0f32, 0.3f32),
                        );
                    }
                }
            }
        }
    }
}