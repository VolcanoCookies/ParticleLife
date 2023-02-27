use bevy::{
    prelude::{Bundle, Component, Vec2},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

#[derive(Component)]
pub struct ParticleMarker;

#[derive(Bundle)]
pub struct Particle {
    #[bundle]
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub particle_marker: ParticleMarker,
    pub velocity: Velocity,
    pub position: Position,
    pub group_id: GroupId,
}

#[derive(Component, Copy, Clone)]
pub struct GroupId(pub usize);

#[derive(Component, Copy, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Component, Copy, Clone)]
pub struct Position(pub Vec2);
