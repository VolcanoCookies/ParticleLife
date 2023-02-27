use bevy::prelude::{Color, Resource};
use rand::Rng;

#[derive(Resource)]
pub struct Rules<const S: usize> {
    pub amount: [usize; S],
    pub attractions: [[f32; S]; S],
    pub colors: [Color; S],
    pub rep_range: [f32; S],
    pub rep_force: [f32; S],
}

pub struct Rule<const S: usize> {
    pub amount: usize,
    pub attractions: [f32; S],
    pub color: Color,
    pub repulsion_range: f32,
    pub repulsion_force: f32,
}

impl<const S: usize> Rules<S> {
    pub fn new(
        amount: [usize; S],
        attractions: [[f32; S]; S],
        colors: [Color; S],
        rep_range: [f32; S],
        rep_force: [f32; S],
    ) -> Self {
        return Self {
            amount,
            attractions,
            colors,
            rep_range,
            rep_force,
        };
    }

    const default_colors_hsla: [Color; 4] = [
        Color::hsla(349.0, 1.0, 0.6, 1.0),
        Color::hsla(223.0, 1.0, 0.6, 1.0),
        Color::hsla(135.0, 1.0, 0.6, 1.0),
        Color::hsla(53.0, 1.0, 0.6, 1.0),
    ];
    const default_colors: [Color; 4] = [Color::RED, Color::GREEN, Color::BLUE, Color::WHITE];

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        let mut amount = [0; S];
        let mut attractions = [[0.; S]; S];
        let mut colors = [Color::WHITE; S];
        let mut rep_range = [0.; S];
        let mut rep_force = [0.; S];

        for i in 0..S {
            amount[i] = 1000;
            for j in 0..S {
                attractions[i][j] = rng.gen_range((-1.)..(1.));
            }
            colors[i] = Self::default_colors_hsla[i % S];
            rep_range[i] = 15.;
            rep_force[i] = -1.0;
        }

        return Self {
            amount,
            attractions,
            colors,
            rep_range,
            rep_force,
        };
    }
}
