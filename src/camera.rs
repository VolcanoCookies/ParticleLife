use bevy::{input::mouse::MouseWheel, prelude::*};
use leafwing_input_manager::{action_state, prelude::ActionState};

use crate::{
    resources::{actions::Action, rules::Rules},
    SIZE,
};

#[derive(Resource, Default)]
pub struct MousePosition {
    pub world: Vec2,
    pub screen: Vec2,
}

#[derive(Resource)]
pub struct CameraZoom(pub f32);

impl Default for CameraZoom {
    fn default() -> Self {
        Self(1f32)
    }
}

pub fn window_to_world_position(
    mouse_position: Vec2,
    window: &Window,
    camera_transform: &Transform,
) -> Vec2 {
    let win_dim = Vec2::new(window.width(), window.height());

    let pos = (mouse_position - (win_dim / 2.)) * camera_transform.scale.truncate()
        + camera_transform.translation.truncate();

    return pos.round();
}

// Slightly simpler constant rate zooming!
//zoom = zoom*pow(targetZoom/zoom, deltaTime/duration)
//
//// ..or as a function:
//float logerp(float a, float b, float t){
//    return a*pow(b/a, t);
//}

pub fn camera_movement(
    mut move_events: EventReader<CursorMoved>,
    mut scroll_events: EventReader<MouseWheel>,
    mut mouse_position: ResMut<MousePosition>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut windows: Res<Windows>,
    mut zoom: ResMut<CameraZoom>,
    action_query: Query<&ActionState<Action>>,
) {
    let action_state = action_query.single();

    // Calculate camera zoom
    let mut zoom_delta = 0.;
    for event in scroll_events.iter() {
        zoom_delta += event.y as f32;
    }
    zoom.0 = zoom.0 * (0.125 / zoom.0).powf((zoom_delta as f32) / 20.);

    let mut delta = Vec2::ZERO;

    let camera_pan = action_state.pressed(Action::CameraPan);

    let (mut camera_transform, mut camera_projection) = camera_query.single_mut();

    // Calculate new mouse positions
    for event in move_events.iter() {
        if camera_pan {
            delta = event.position - mouse_position.screen;
        }
        mouse_position.screen = event.position;
    }

    mouse_position.world =
        window_to_world_position(mouse_position.screen, windows.primary(), &camera_transform);

    // Move camera with keyboard
    if !camera_pan {
        if action_state.pressed(Action::CameraRight) {
            delta.x -= 1.;
        }
        if action_state.pressed(Action::CameraLeft) {
            delta.x += 1.;
        }
        if action_state.pressed(Action::CameraUp) {
            delta.y -= 1.;
        }
        if action_state.pressed(Action::CameraDown) {
            delta.y += 1.;
        }

        // Speed modifier
        delta *= 16.;

        if action_state.pressed(Action::CameraFasterSpeed) {
            delta *= 2.;
        }
    }

    // Apply changes to camera
    delta *= zoom.0;
    camera_transform.translation -= Vec3::new(delta.x, delta.y, 0.0);
    camera_transform.scale = Vec3::splat(zoom.0);
    camera_projection.far = 1000. / zoom.0;
}

pub fn controls(mut commands: Commands, action_query: Query<&ActionState<Action>>) {
    let action_state = action_query.single();

    if action_state.just_pressed(Action::Restart) {
        commands.insert_resource(Rules::<SIZE>::random());
    }
}
