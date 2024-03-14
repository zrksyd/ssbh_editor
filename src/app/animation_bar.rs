use egui::{Button, DragValue, Ui};

use crate::AnimationState;

pub fn display_animation_bar(
    ui: &mut Ui,
    animation_state: &mut AnimationState,
    final_frame_index: f32,
) {
    // TODO: Find a better layout for this.
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label("Speed");
            ui.add(
                DragValue::new(&mut animation_state.playback_speed)
                    .min_decimals(2)
                    .speed(0.01)
                    .clamp_range(0.25..=2.0),
            );

            // TODO: Custom checkbox widget so label is on the left side.
            ui.checkbox(&mut animation_state.should_loop, "Loop");
        });
        ui.horizontal_centered(|ui| {
            // TODO: How to fill available space?
            // TODO: Get the space that would normally be taken up by the central panel?
            ui.spacing_mut().slider_width = (ui.available_width() - 520.0).max(0.0);
            let response = ui.add(
                // TODO: Show ticks?
                egui::Slider::new(&mut animation_state.current_frame, 0.0..=final_frame_index)
                    .step_by(1.0)
                    .show_value(false),
            );
            if response.hovered() {
                ui.ctx().input_mut(|i| {
                    if i.consume_key(egui::Modifiers::default(), egui::Key::ArrowRight) {
                        animation_state.current_frame += 1.0;
                        animation_state.should_update_animations = true;
                    } else if i.consume_key(egui::Modifiers::default(), egui::Key::ArrowLeft) {
                        animation_state.current_frame -= 1.0;
                        animation_state.should_update_animations = true;
                    }
                })
            };

            if response.changed() {
                // Manually trigger an update in case the playback is paused.
                animation_state.should_update_animations = true;
            }

            // Use a separate widget from the slider value to force the size.
            // This reduces the chances of the widget resizing during animations.

            let size = [60.0, 30.0];
            if animation_state.is_playing {
                // Nest these conditions to avoid displaying both "Pause" and "Play" at once.
                if ui.add_sized(size, Button::new("Pause")).clicked() {
                    animation_state.is_playing = false;
                }
            } else if ui.add_sized(size, Button::new("Play")).clicked() {
                animation_state.is_playing = true;
            }

            if ui
                .add_sized(
                    [60.0, 20.0],
                    egui::DragValue::new(&mut animation_state.current_frame)
                        .clamp_range(0.0..=final_frame_index),
                )
                .changed()
            {
                // Manually trigger an update in case the playback is paused.
                animation_state.should_update_animations = true;
            }
            ui.label(&format!("/ {final_frame_index}"));
        });
    });
}
