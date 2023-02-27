use bevy::prelude::{Query, Res, Transform, With};

use crate::{
    entity::particle::{ParticleMarker, Position, Velocity},
    resources::settings::Settings,
};

pub fn apply_velocity(
    settings: Res<Settings>,
    mut particle_query: Query<(&mut Velocity, &mut Position, &mut Transform), With<ParticleMarker>>,
) {
    particle_query.par_for_each_mut(64, |(mut vel, mut pos, mut trans)| {
        vel.0 = vel.0.clamp_length(0.0, settings.max_velocity);
		pos.0 += vel.0;
        vel.0 *= settings.drag_coef;

        trans.translation.x = pos.0.x;
        trans.translation.y = pos.0.y;
    });
}
