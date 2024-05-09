use std::fs;
use std::path::{Path, PathBuf};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::{HitBox, HurtBox, Player};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<EditorSpace>()
            .init_resource::<EditorExplorer>()
            .add_systems(PostUpdate, setup_editor);
    }
}


#[derive(Default, Resource)]
struct EditorSpace {
    left: f32,
}


#[derive(Resource)]
struct EditorExplorer {
    path: String,
}

impl Default for EditorExplorer {
    fn default() -> Self {
        EditorExplorer {
            path: "./assets".to_string(),
        }
    }
}

fn setup_editor(
    mut contexts: EguiContexts,
    mut editor_space: ResMut<EditorSpace>,
    mut editor_explorer: ResMut<EditorExplorer>,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut Player, &mut TextureAtlas, &mut Handle<Image>, &mut HitBox, &mut HurtBox)>) {
    let ctx = contexts.ctx_mut();

    editor_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            for (entity,
                mut player,
                mut texture_atlas,
                mut texture,
                mut hitbox,
                mut hurtbox) in query.iter_mut() {
                if let Ok(entries) = fs::read_dir(&editor_explorer.path) {
                    ui.vertical(|ui| {
                        if ui.button("..").clicked() {
                            if let Some(parent) = Path::new(&editor_explorer.path).parent() {
                                editor_explorer.path = parent.to_str().unwrap().to_string();
                            }
                        }

                        for entry in entries {
                            if let Ok(entry) = entry {
                                let path = entry.path();
                                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                                    if ui.button(name).clicked() {
                                        let path_str = path.to_str().unwrap_or_default();
                                        if path.is_dir() {
                                            editor_explorer.path = path_str.to_owned();
                                        } else {
                                            let asset_path = normalize_path(path_str);
                                            texture_atlas.index = if texture_atlas.index == player.animation_indices.last {
                                                player.animation_indices.first
                                            } else {
                                                texture_atlas.index + 1
                                            };
                                            *texture = asset_server.load(asset_path);
                                        }
                                    }
                                }
                            }
                        }
                    });
                }

                ui.horizontal(|ui| {
                    ui.label(format!("Entity: {}", entity.index()));
                });

                ui.collapsing("HitBox Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Size");
                        ui.add(egui::DragValue::new(&mut hitbox.size.x));
                        ui.add(egui::DragValue::new(&mut hitbox.size.y));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Offset");
                        ui.add(egui::DragValue::new(&mut hitbox.offset.x));
                        ui.add(egui::DragValue::new(&mut hitbox.offset.y));
                    });
                });

                ui.collapsing("HurtBox Settings", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Size");
                        ui.add(egui::DragValue::new(&mut hurtbox.size.x));
                        ui.add(egui::DragValue::new(&mut hurtbox.size.y));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Offset");
                        ui.add(egui::DragValue::new(&mut hurtbox.offset.x));
                        ui.add(egui::DragValue::new(&mut hurtbox.offset.y));
                    });
                });

                ui.horizontal(|ui| {
                    if ui.button("Previous Sprite").clicked() {
                        let num_sprites = player.animation_indices.size;

                        texture_atlas.index = if texture_atlas.index == player.animation_indices.first {
                            player.animation_indices.last
                        } else {
                            texture_atlas.index - 1
                        };
                    }

                    if ui.button("Next Sprite").clicked() {
                        texture_atlas.index = if texture_atlas.index == player.animation_indices.last {
                            player.animation_indices.first
                        } else {
                            texture_atlas.index + 1
                        };
                    }
                });
            };

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
}

fn normalize_path(full_path: &str) -> String {
    // Convert to PathBuf for easier manipulation
    let path = PathBuf::from(full_path);

    // Find the 'assets' segment and return the subpath from that point onwards
    path.iter()
        .skip_while(|&component| component != "assets")
        .skip(1)  // Skip the 'assets' component itself
        .collect::<PathBuf>()
        .display()
        .to_string()
}