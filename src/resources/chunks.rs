use bevy::prelude::{Entity, Resource};

use crate::entity::particle::{GroupId, Position, Velocity};

#[derive(Resource)]
pub struct Chunks {
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub chunks: Vec<Chunk>,
}

impl Chunks {
    pub fn get_chunk(&self, x: f32, y: f32) -> &Chunk {
        let chunk_x = (x / (self.size as f32)).floor() as usize;
        let chunk_y = (y / (self.size as f32)).floor() as usize;
        &self.chunks[chunk_x + chunk_y * self.width]
    }

    pub fn get_chunks_around(&self, x: f32, y: f32) -> Vec<&Chunk> {
        let mut chunks: Vec<&Chunk> = Vec::new();
        for dx in -1..2 {
            for dy in -1..2 {
                chunks.push(self.get_chunk(
                    x + (dx * self.size as i32) as f32,
                    y + (dy * self.size as i32) as f32,
                ));
            }
        }
        chunks
    }

    pub fn clear(&mut self) {
        for chunk in &mut self.chunks {
            chunk.clear();
        }
    }

    pub fn insert_particle(&mut self, pos: Position, id: GroupId) {
        let chunk_x = (pos.0.x / (self.size as f32)).floor() as usize;
        let chunk_y = (pos.0.y / (self.size as f32)).floor() as usize;
        self.chunks[chunk_x + chunk_y * self.width]
            .particles
            .push((pos, id));
    }

    pub fn new(width: usize, height: usize, size: usize) -> Self {
        let mut chunks: Vec<Chunk> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                chunks.push(Chunk::empty());
            }
        }

        Self {
            width,
            height,
            size,
            chunks,
        }
    }
}

pub struct Chunk {
    pub particles: Vec<(Position, GroupId)>,
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
