use bevy::prelude::{Commands, Component, KeyCode, MouseButton};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Component)]
pub enum Action {
    CameraUp,
    CameraDown,
    CameraLeft,
    CameraRight,
    CameraFasterSpeed,
    CameraReset,
    CameraPan,
    CameraZoomIn,
    CameraZoomOut,
    ToggleDebugColliders,
    ToggleDebugPrints,
    ToggleInspector,
    Restart,
}

pub(crate) fn action_setup(mut commands: Commands) {
    let mut input_map = InputMap::default();
    input_map.insert_multiple([
        (KeyCode::W, Action::CameraUp),
        (KeyCode::S, Action::CameraDown),
        (KeyCode::A, Action::CameraLeft),
        (KeyCode::D, Action::CameraRight),
        (KeyCode::LShift, Action::CameraFasterSpeed),
        (KeyCode::R, Action::CameraReset),
        (KeyCode::P, Action::ToggleDebugColliders),
        (KeyCode::I, Action::ToggleInspector),
        (KeyCode::R, Action::Restart),
    ]);

    input_map.insert_multiple([(MouseButton::Left, Action::CameraPan)]);

    commands.spawn(InputManagerBundle::<Action> {
        action_state: ActionState::default(),
        input_map,
    });
}
