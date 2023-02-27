use bevy::{
    core_pipeline::{bloom::BloomSettings, clear_color::ClearColorConfig},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{
        default,
        shape::{self, Box},
        App, AssetServer, Assets, Camera, Camera2d, Camera2dBundle, Camera3dBundle, ClearColor,
        Color, Commands, ImagePlugin, IntoSystemDescriptor, Mesh, PluginGroup, Res, ResMut,
        Transform, Vec2, Vec3,
    },
    render::texture::ImageSampler,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    window::{CursorGrabMode, WindowDescriptor, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, Ccd, Damping, ExternalForce, NoUserData, RapierConfiguration,
    RapierPhysicsPlugin, RigidBody, Sleeping, Vect, Velocity,
};
use camera::{camera_movement, controls, CameraZoom, MousePosition};
use chunking::sort_into_chunks;
use entity::particle::{Particle, ParticleMarker};
use iyes_loopless::prelude::IntoConditionalSystem;
use leafwing_input_manager::prelude::InputManagerPlugin;
use physics::apply_velocity;
use rand::{random, Rng};
use resources::{
    actions::{action_setup, Action},
    chunks::Chunks,
    rules::Rules,
    settings::Settings,
};
use simulation::{configure, update_edge, update_rules};

pub mod camera;
pub mod chunking;
pub mod entity;
pub mod physics;
pub mod resources;
pub mod simulation;

const WORLD_WIDTH: usize = 800;
const WORLD_HEIGHT: usize = 600;

const SIZE: usize = 4;

fn main() {
    App::new()
        //.insert_resource(RapierConfiguration {
        //    gravity: Vect::ZERO,
        //    ..default()
        //})
        .insert_resource(Rules::<SIZE>::random())
        .insert_resource(Settings::default())
        .insert_resource(MousePosition::default())
        .insert_resource(CameraZoom::default())
        .insert_resource(ClearColor(Color::hex("2596be").unwrap()))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Particle Life".into(),
                        resizable: true,
                        cursor_visible: true,
                        cursor_grab_mode: CursorGrabMode::None,
                        ..default()
                    },
                    exit_on_all_closed: true,
                    ..default()
                })
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_startup_system(action_setup)
        .add_system(camera_movement)
        .add_system(controls)
        .add_system(configure::<SIZE>)
        .add_system(
            update_rules::<SIZE>
                .run_if_resource_exists::<Chunks>()
                .label("update"),
        )
        .add_system(update_edge.run_if_resource_exists::<Chunks>().label("edge"))
        .add_system(
            apply_velocity
                .run_if_resource_exists::<Chunks>()
                .label("apply_velocity")
                .after("update")
                .after("edge"),
        )
        .add_system(
            sort_into_chunks
                .run_if_resource_exists::<Chunks>()
                .after("apply_velocity"),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn main camera
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 25.0,
                },
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
                ..default()
            },
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings {
            threshold: 0.5,
            ..default()
        });
}
