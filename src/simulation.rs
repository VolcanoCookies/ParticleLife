use bevy::{
    prelude::{
        default, shape, Assets, Commands, Entity, Mesh, Query, Res, ResMut, Transform, Vec2, With,
        World,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use bevy_rapier2d::prelude::{AdditionalMassProperties, Damping, ExternalForce, RigidBody};
use rand::Rng;

use crate::{
    entity::particle::{GroupId, Particle, ParticleMarker, Position, Velocity},
    resources::{chunks::Chunks, rules::Rules, settings::Settings},
    WORLD_HEIGHT, WORLD_WIDTH,
};

pub fn configure<const S: usize>(
    mut commands: Commands,
    rules: Res<Rules<S>>,
    settings: Res<Settings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_query: Query<Entity, With<ParticleMarker>>,
) {
    if rules.is_changed() {
        let chunk_size = settings.max_dist;
        let chunks_x = (WORLD_WIDTH as f32 * 2. / chunk_size).ceil();
        let chunks_y = (WORLD_HEIGHT as f32 * 2. / chunk_size).ceil();
        let chunks = Chunks::new(chunks_x as usize, chunks_y as usize, chunk_size as usize);

        commands.insert_resource(chunks);

        particle_query.for_each(|particle| commands.entity(particle).despawn());

        let mut rng = rand::thread_rng();

        for group_id in 0..S {
            let mesh = meshes.add(Mesh::from(shape::Circle {
                radius: 2.0,
                vertices: 32usize,
            }));
            let material = materials.add(ColorMaterial::from(rules.colors[group_id]));

            for _ in 0..rules.amount[group_id] {
                let x = rng.gen_range(-(WORLD_WIDTH as f32)..(WORLD_WIDTH as f32));
                let y = rng.gen_range(-(WORLD_HEIGHT as f32)..(WORLD_HEIGHT as f32));

                commands
                    .spawn(Particle {
                        mesh_bundle: MaterialMesh2dBundle {
                            mesh: mesh.clone().into(),
                            material: material.clone().into(),
                            transform: Transform::from_xyz(x, y, 0.0),
                            ..default()
                        },
                        particle_marker: ParticleMarker,
                        velocity: Velocity(Vec2::ZERO),
                        position: Position(Vec2::new(x, y)),
                        group_id: GroupId(group_id),
                    })
                    .insert(AdditionalMassProperties::Mass(settings.mass))
                    .insert(Damping {
                        linear_damping: settings.drag_coef,
                        angular_damping: settings.drag_coef,
                    })
                    .insert(GroupId(group_id));
            }
        }
    }
}

/*pub fn update_rules_chunked<const S: usize>(
    chunks: Res<Chunks>,
    rules: Res<Rules<S>>,
    settings: Res<Settings>,
    mut particle_query: Query<(&mut Velocity, &Transform, &GroupId), With<ParticleMarker>>,
    transform_query: Query<(&Transform, &GroupId), With<ParticleMarker>>,
) {
    particle_query.par_for_each_mut(64, |(mut velocity, transform, group_id)| {
        let mut combined = Vec2::ZERO;

        let chunk_neighbours =
            chunks.get_chunks_around(transform.translation.x, transform.translation.y);

        for chunk in chunk_neighbours {
            for other_particle in &chunk.particles {
                if let Ok((other_transform, other_group_id)) = transform_query.get(*other_particle)
                {
                    let vec = transform.translation - other_transform.translation;
                    let dist = vec.length();
                    if dist == 0.0 || dist > settings.max_dist {
                        continue;
                    }

                    let modifier = if dist <= rules.rep_range[group_id.0] {
                        rules.rep_force[group_id.0] * dist / rules.rep_range[group_id.0]
                    } else {
                        let attraction = rules.attractions[group_id.0][other_group_id.0];
                        let half_dist = settings.max_dist / 2.;
                        if dist < half_dist {
                            attraction * dist / half_dist
                        } else {
                            attraction * (dist - half_dist) / half_dist
                        }
                    };

                    let dir = vec.normalize_or_zero();
                    let strength = settings.g * modifier;
                    combined -= Vec2::new(dir.x, dir.y) * strength;
                }
            }

            //velocity.linvel += combined.clamp_length(0.0, settings.max_force);
        }
    });
}

pub fn update_rules_old<const S: usize>(
    rules: Res<Rules<S>>,
    settings: Res<Settings>,
    mut particle_query: Query<(&mut Velocity, &Transform, &GroupId), With<ParticleMarker>>,
    transform_query: Query<(&Transform, &GroupId), With<ParticleMarker>>,
) {
    particle_query.par_for_each_mut(64, |(mut velocity, transform, group_id)| {
        let mut combined = Vec2::ZERO;

        for (other_transform, other_group_id) in transform_query.iter() {
            let vec = transform.translation - other_transform.translation;
            let dist = vec.length();
            if dist == 0.0 || dist > settings.max_dist {
                continue;
            }

            let modifier = if dist <= rules.rep_range[group_id.0] {
                rules.rep_force[group_id.0] * dist / rules.rep_range[group_id.0]
            } else {
                let attraction = rules.attractions[group_id.0][other_group_id.0];
                let half_dist = settings.max_dist / 2.;
                if dist < half_dist {
                    attraction * dist / half_dist
                } else {
                    attraction * (dist - half_dist) / half_dist
                }
            };

            let dir = vec.normalize_or_zero();
            let strength = settings.g * modifier;
            combined -= Vec2::new(dir.x, dir.y) * strength;
        }

        //velocity.linvel += combined.clamp_length(0.0, settings.max_force);
    });
}*/

pub fn update_rules<const S: usize>(
    chunks: Res<Chunks>,
    rules: Res<Rules<S>>,
    settings: Res<Settings>,
    mut particle_query: Query<(&mut Velocity, &Position, &GroupId), With<ParticleMarker>>,
) {
    particle_query.par_for_each_mut(16, |(mut vel, pos, id)| {
        let mut combined = Vec2::ZERO;

        for chunk in chunks.get_chunks_around(pos.0.x, pos.0.y) {
            for (other_pos, other_id) in &chunk.particles {
                let vec = pos.0 - other_pos.0;
                let dist = vec.length();
                if dist == 0.0 || dist > settings.max_dist {
                    continue;
                }

                let modifier = if dist <= rules.rep_range[id.0] {
                    rules.rep_force[id.0] * dist / rules.rep_range[id.0]
                } else {
                    let attraction = rules.attractions[id.0][other_id.0];
                    let half_dist = settings.max_dist / 2.;
                    if dist < half_dist {
                        attraction * dist / half_dist
                    } else {
                        attraction * (dist - half_dist) / half_dist
                    }
                };

                let dir = vec.normalize_or_zero();
                let strength = settings.g * modifier;
                combined -= Vec2::new(dir.x, dir.y) * strength;
            }
        }

        vel.0 += combined.clamp_length(0.0, settings.max_velocity);
    });
}

pub fn update_repulsion(
    settings: Res<Settings>,
    mut query: Query<(&mut Velocity, &mut Transform), With<ParticleMarker>>,
) {
}

/*pub fn update_edge(
    settings: Res<Settings>,
    mut query: Query<(&mut Velocity, &mut Transform), With<ParticleMarker>>,
) {
    let x_bound = WORLD_WIDTH as f32;
    let y_bound = WORLD_HEIGHT as f32;

    for (mut velocity, mut transform) in query.iter_mut() {
        if transform.translation.x.abs() > x_bound {
            match settings.edge_mode {
                crate::resources::settings::EdgeMode::WRAP => {
                    transform.translation.x = -(transform.translation.x.signum() * 2.0 * x_bound
                        - transform.translation.x);
                }
                crate::resources::settings::EdgeMode::BOUNCE => {
                    transform.translation.x =
                        transform.translation.x.signum() * 2.0 * x_bound - transform.translation.x;
                    velocity.linvel.x *= -1.0;
                }
                crate::resources::settings::EdgeMode::STOP => {
                    transform.translation.x = transform.translation.x.signum() * x_bound;
                    velocity.linvel.x = 0.0;
                }
            }
        }

        if transform.translation.y.abs() > y_bound {
            match settings.edge_mode {
                crate::resources::settings::EdgeMode::WRAP => {
                    transform.translation.y = -(transform.translation.y.signum() * 2.0 * y_bound
                        - transform.translation.y);
                }
                crate::resources::settings::EdgeMode::BOUNCE => {
                    transform.translation.y =
                        transform.translation.y.signum() * 2.0 * y_bound - transform.translation.y;
                    velocity.linvel.y *= -1.0;
                }
                crate::resources::settings::EdgeMode::STOP => {
                    transform.translation.y = transform.translation.y.signum() * y_bound;
                    velocity.linvel.y = 0.0;
                }
            }
        }
    }
}*/

pub fn update_edge(
    settings: Res<Settings>,
    mut query: Query<(&mut Velocity, &mut Position), With<ParticleMarker>>,
) {
    let x_bound = WORLD_WIDTH as f32;
    let y_bound = WORLD_HEIGHT as f32;

    for (mut vel, mut pos) in query.iter_mut() {
        if pos.0.x.abs() > x_bound {
            match settings.edge_mode {
                crate::resources::settings::EdgeMode::WRAP => {
                    pos.0.x = -(pos.0.x.signum() * 2.0 * x_bound - pos.0.x);
                }
                crate::resources::settings::EdgeMode::BOUNCE => {
                    pos.0.x = pos.0.x.signum() * 2.0 * x_bound - pos.0.x;
                    vel.0.x *= -1.0;
                }
                crate::resources::settings::EdgeMode::STOP => {
                    pos.0.x = pos.0.x.signum() * x_bound;
                    vel.0.x = 0.0;
                }
            }
        }

        if pos.0.y.abs() > y_bound {
            match settings.edge_mode {
                crate::resources::settings::EdgeMode::WRAP => {
                    pos.0.y = -(pos.0.y.signum() * 2.0 * y_bound - pos.0.y);
                }
                crate::resources::settings::EdgeMode::BOUNCE => {
                    pos.0.y = pos.0.y.signum() * 2.0 * y_bound - pos.0.y;
                    vel.0.y *= -1.0;
                }
                crate::resources::settings::EdgeMode::STOP => {
                    pos.0.y = pos.0.y.signum() * y_bound;
                    vel.0.y = 0.0;
                }
            }
        }
    }
}
