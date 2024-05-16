use bevy::app::App;
use bevy_egui::{egui, EguiContexts};

use crate::core::*;
use crate::core::core_core::*;

pub struct CoreGuiPlugin;

impl Plugin for CoreGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CoreGuiState {
                show_hit_boxes: false,
                show_hurt_boxes: false,
            })
            .add_systems(Update, display_core_information);
    }
}

#[derive(Default, Resource)]
pub struct CoreGuiState {
    pub show_hit_boxes: bool,
    pub show_hurt_boxes: bool,
}

fn display_core_information(
    mut egui_contexts: EguiContexts,
    mut game_state: ResMut<GameState>,
    mut gui_state: ResMut<CoreGuiState>) {
    let ctx = egui_contexts.ctx_mut();

    egui::Window::new("Core").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let button_text: String = if game_state.mode == GameMode::Editor {
                "Play".to_string()
            } else {
                "Stop".to_string()
            };

            if ui.button(button_text).clicked() {
                game_state.mode = if game_state.mode == GameMode::Editor {
                    GameMode::Game
                } else {
                    GameMode::Editor
                };
            };

            ui.label(format!("Mode: {}", game_state.mode));
        });

        if game_state.mode == GameMode::Game {
            ui.checkbox(&mut gui_state.show_hit_boxes, "Show Hit Boxes");
            ui.checkbox(&mut gui_state.show_hurt_boxes, "Show Hurt Boxes");
        }
    });
}