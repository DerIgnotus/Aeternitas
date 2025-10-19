use crate::core::position::{CHUNK_SIZE, WORLD_SIZE};
use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Octree {
    pub root: OctreeNode,
    pub min_size: f32,
    pub max_depth: u8,
}

#[derive(Debug)]
pub struct OctreeNode {
    pub center: Vec3,
    pub size: f32,
    pub children: Option<[Box<OctreeNode>; 8]>,
}

impl Octree {
    pub fn new(center: Vec3, size: f32, min_size: f32, max_depth: u8) -> Self {
        Self {
            root: OctreeNode::new(center, size),
            min_size,
            max_depth,
        }
    }

    /// Recursively subdivide nodes near the player
    pub fn subdivide_near_player(&mut self, player_pos: Vec3) {
        self.root.subdivide_if_needed(player_pos, self.min_size, self.max_depth);
    }

    /// Collect all leaf nodes (final chunks)
    pub fn collect_leaves(&self) -> Vec<&OctreeNode> {
        let mut result = Vec::new();
        self.root.collect_leaves(&mut result);
        result
    }
}

impl OctreeNode {
    pub fn new(center: Vec3, size: f32) -> Self {
        Self {
            center,
            size,
            children: None,
        }
    }

    fn subdivide_if_needed(&mut self, player_pos: Vec3, min_size: f32, depth: u8) {

        // Smallest chunk reached
        if depth == 0 || self.size <= min_size {
            return; 
        }

        let distance = player_pos.distance(self.center);

        // Subdivide if close enough to player
        if distance < self.size * 2.0 {
            if self.children.is_none() {
                self.children = Some(Self::generate_children(self.center, self.size / 2.0));
            }
            if let Some(children) = &mut self.children {
                for child in children {
                    child.subdivide_if_needed(player_pos, min_size, depth - 1);
                }
            }
        }
    }

    fn generate_children(center: Vec3, half_size: f32) -> [Box<OctreeNode>; 8] {
        let quarter = half_size / 2.0;
        let offsets = [
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(-1.0, -1.0,  1.0),
            Vec3::new(-1.0,  1.0, -1.0),
            Vec3::new(-1.0,  1.0,  1.0),
            Vec3::new( 1.0, -1.0, -1.0),
            Vec3::new( 1.0, -1.0,  1.0),
            Vec3::new( 1.0,  1.0, -1.0),
            Vec3::new( 1.0,  1.0,  1.0),
        ];

        offsets.map(|offset| {
            let pos = center + (offset * quarter);
            Box::new(OctreeNode::new(pos, half_size))
        })
    }

    fn collect_leaves<'a>(&'a self, out: &mut Vec<&'a OctreeNode>) {
        if let Some(children) = &self.children {
            for child in children {
                child.collect_leaves(out);
            }
        } else {
            out.push(self);
        }
    }
}

pub fn setup_octree(mut commands: Commands) {


    let octree = Octree::new(
        Vec3::ZERO, 
        WORLD_SIZE as f32,  
        CHUNK_SIZE as f32,  
        (WORLD_SIZE as f32 / CHUNK_SIZE as f32).log2() as u8,

    );
    commands.insert_resource(octree);
}