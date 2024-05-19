use bevy::app::{App, Update};
use bevy::math::Vec3;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_math::primitives::Rectangle;

use crate::core::core_core::*;
use crate::core::core_gui::*;
use crate::editor::editor_core::*;
use crate::editor::inspector::inspector_core::SelectedFrame;
use crate::game::*;
use crate::game::game_gui::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameGuiPlugin)
            .insert_resource(GameCameraEntity::default())
            .insert_resource(HitboxMeshAndMaterial::default())
            .insert_resource(HurtboxMeshAndMaterial::default())
            .add_systems(Update, animate_sprite)
            .add_systems(Update, game_state_adapter_system)
            .add_systems(Update, gizmos_selected_sprite)
            .add_systems(Update, update_lifetimes);
    }
}

#[derive(Component)]
struct Player;

#[derive(Default, Component)]
struct GameCamera;

#[derive(Default, Resource)]
struct HitboxMeshAndMaterial {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Default, Resource)]
struct HurtboxMeshAndMaterial {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

#[derive(Default, Resource)]
pub struct GameCameraEntity {
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
    config_store: ResMut<GizmoConfigStore>,
    game_state: Res<GameState>,
    sprite_sheets: ResMut<EditorSpriteSheets>,
    selected_frame: ResMut<SelectedFrame>,
    mut game_camera_entity: ResMut<GameCameraEntity>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    hit_box_mesh_and_material: ResMut<HitboxMeshAndMaterial>,
    hurt_box_mesh_and_material: ResMut<HurtboxMeshAndMaterial>,
    mut query: Query<Entity, With<Player>>) {
    match &game_state.mode {
        GameMode::Editor => {
            if let Some(entity) = game_camera_entity.entity.take() {
                commands.entity(entity).despawn();
            }

            for entity in query.iter_mut() {
                commands.entity(entity).despawn();
            }
        }

        GameMode::Game => {
            if game_camera_entity.entity.is_none() {
                setup(
                    commands,
                    config_store,
                    sprite_sheets,
                    selected_frame,
                    game_camera_entity,
                    meshes, materials, hit_box_mesh_and_material, hurt_box_mesh_and_material);
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
    mut sprite_sheets: ResMut<EditorSpriteSheets>,
    selected_frame: ResMut<SelectedFrame>,
    mut game_camera_entity: ResMut<GameCameraEntity>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hit_box_mesh_and_material: ResMut<HitboxMeshAndMaterial>,
    mut hurtbox_mesh_and_material: ResMut<HurtboxMeshAndMaterial>,
) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 5.;

    // hitbox gizmo
    hit_box_mesh_and_material.mesh = meshes.add(Mesh::from(Rectangle::default()));
    hit_box_mesh_and_material.material = materials.add(ColorMaterial::from(Color::rgb_u8(243, 139, 168)));

    // hurtbox gizmo
    hurtbox_mesh_and_material.mesh = meshes.add(Mesh::from(Rectangle::default()));
    hurtbox_mesh_and_material.material = materials.add(ColorMaterial::from(Color::rgb_u8(166, 227, 161)));

    // camera
    let mut entity = commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb_u8(17, 17, 27)),
            ..default()
        },
        ..default()
    });
    entity.insert(GameCamera);
    game_camera_entity.entity = Some(entity.id());

    if let Some(id) = &selected_frame.sprite_sheet_id {
        if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get_mut(id) {
            let animation_indices = AnimationIndices { first: 1, last: sprite_sheet_atlas.sprite_sheet_info.columns - 1 };
            let texture_handle = sprite_sheet_atlas.texture_handle.clone();
            let mut player_entity = commands.spawn(
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

            player_entity.insert(Player);
        }
    }
}

fn gizmos_selected_sprite(
    mut commands: Commands,
    mut gizmos: Gizmos,
    hitbox_mesh_and_material: Res<HitboxMeshAndMaterial>,
    hurtbox_mesh_and_material: Res<HurtboxMeshAndMaterial>,
    selected_frame: ResMut<SelectedFrame>,
    mut sprite_sheets: ResMut<EditorSpriteSheets>,
    mut query: Query<(&Transform, &AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
    game_state: Res<GameState>,
    gui_state: ResMut<CoreGuiState>,
) {
    if game_state.mode != GameMode::Game {
        return;
    }

    if let Some(id) = &selected_frame.sprite_sheet_id {
        if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get_mut(id) {
            for (transform, _indices, _timer, atlas) in &mut query {
                let scale = transform.scale.truncate();

                if let Some(frame_data) = sprite_sheet_atlas.sprite_sheet_info.frames.get_mut(atlas.index) {
                    if gui_state.show_hit_boxes {
                        for hit_box in &frame_data.hit_boxes {
                            let hit_box_size_scaled = hit_box.size * scale;
                            let hit_box_offset_scaled = hit_box.offset * scale;

                            commands.spawn((MaterialMesh2dBundle {
                                mesh: hitbox_mesh_and_material.mesh.clone().into(),
                                material: hitbox_mesh_and_material.material.clone(),
                                transform: Transform::from_translation(hit_box_offset_scaled.extend(100.))
                                    .with_scale(hit_box_size_scaled.extend(0.)),
                                ..default()
                            }, Lifetime {
                                timer: Timer::from_seconds(0.01, TimerMode::Once),
                            }));

                            gizmos.rect_2d(
                                transform.translation.truncate() + hit_box_offset_scaled,
                                0.0,
                                hit_box_size_scaled,
                                Color::rgba(1f32, 0f32, 0f32, 0.3f32),
                            );
                        }
                    }

                    if gui_state.show_hurt_boxes {
                        for hurt_box in &frame_data.hurt_boxes {
                            let hurt_box_size_scaled = hurt_box.size * scale;
                            let hurt_box_offset_scaled = hurt_box.offset * scale;

                            commands.spawn((MaterialMesh2dBundle {
                                mesh: hurtbox_mesh_and_material.mesh.clone().into(),
                                material: hurtbox_mesh_and_material.material.clone(),
                                transform: Transform::from_translation(hurt_box_offset_scaled.extend(100.))
                                    .with_scale(hurt_box_size_scaled.extend(0.)),
                                ..default()
                            }, Lifetime {
                                timer: Timer::from_seconds(0.01, TimerMode::Once),
                            }));

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
}

fn update_lifetimes(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());

        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}