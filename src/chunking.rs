use bevy::prelude::{ Query, ResMut,  With};

use crate::{
    entity::particle::{GroupId, ParticleMarker, Position},
    resources::chunks::Chunks,
};

pub fn sort_into_chunks(
    mut chunks: ResMut<Chunks>,
    particle_query: Query<(&Position, &GroupId), With<ParticleMarker>>,
) {
    chunks.clear();
    for (pos, id) in particle_query.iter() {
        chunks.insert_particle(*pos, *id);
    }
}
