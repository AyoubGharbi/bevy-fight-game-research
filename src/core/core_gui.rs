use bevy::app::App;
use bevy::prelude::{Plugin, Res, ResMut, Resource, Update};
use bevy_egui::{egui, EguiContexts};
use crate::core::core::{GameMode, GameState};

#[derive(Default, Resource)]
pub struct GuiState {
    pub show_hitboxes: bool,
    pub show_hurtboxes: bool,
}

pub struct CoreGuiPlugin;

impl Plugin for CoreGuiPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GuiState {
                show_hitboxes: false,
                show_hurtboxes: false,
            })
            .add_systems(Update, display_core_information);
    }
}

fn display_core_information(
    mut egui_contexts: EguiContexts,
    game_state: Res<GameState>,
    mut gui_state: ResMut<GuiState>) {
    let ctx = egui_contexts.ctx_mut();

    egui::Window::new("Core").show(ctx, |ui| {
        ui.label(format!("Current mode: {}", game_state.mode));
        if game_state.mode == GameMode::Game {
            ui.checkbox(&mut gui_state.show_hitboxes, "Show Hitboxes");
            ui.checkbox(&mut gui_state.show_hurtboxes, "Show Hurtboxes");
        }
    });
}