use bevy::app::App;
use bevy_egui::{egui, EguiContexts};

use crate::core::*;
use crate::core::core_core::*;

pub struct CoreGuiPlugin;

impl Plugin for CoreGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GuiState {
                show_hit_boxes: false,
                show_hurt_boxes: false,
            })
            .add_systems(Update, display_core_information);
    }
}

#[derive(Default, Resource)]
pub struct GuiState {
    pub show_hit_boxes: bool,
    pub show_hurt_boxes: bool,
}

fn display_core_information(
    mut egui_contexts: EguiContexts,
    game_state: Res<GameState>,
    mut gui_state: ResMut<GuiState>) {
    let ctx = egui_contexts.ctx_mut();

    egui::Window::new("Core").show(ctx, |ui| {
        ui.label(format!("Mode: {}", game_state.mode));
        if game_state.mode == GameMode::Game {
            ui.checkbox(&mut gui_state.show_hit_boxes, "Show Hit Boxes");
            ui.checkbox(&mut gui_state.show_hurt_boxes, "Show Hurt Boxes");
        }
    });
}