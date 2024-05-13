use bevy::app::App;
use bevy::prelude::{Plugin, Res, Update};
use bevy_egui::{egui, EguiContexts};
use crate::core::core::GameState;

pub struct CoreGuiPlugin;

impl Plugin for CoreGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_core_information);
    }
}

fn display_core_information(mut egui_contexts: EguiContexts, game_state: Res<GameState>) {
    let ctx = egui_contexts.ctx_mut();

    egui::Window::new("Core").show(ctx, |ui| {
        ui.label(format!("Current mode: {}", game_state.mode));
    });
}