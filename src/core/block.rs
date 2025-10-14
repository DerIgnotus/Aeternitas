use bevy::prelude::*;
use std::collections::HashMap;

/// Block ID (65,536 limit rn, will expand later)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u16);

impl BlockId {
    pub const AIR: BlockId = BlockId(0);
    pub const STONE: BlockId = BlockId(1);
    pub const DIRT: BlockId = BlockId(2);
    pub const GRASS: BlockId = BlockId(3);
}

/// Block Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

/// Efficient Tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolType {
    None,
    Pickaxe,
    Shovel,
    Axe,
    Sword,
}

/// Block Properties
#[derive(Debug, Clone)]
pub struct BlockProperties {
    pub id: BlockId,
    pub name: String,
    pub hardness: f32,
    pub tool_type: ToolType,
    pub required_tool: Option<ToolType>,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub can_contain_fluid: bool,
    pub is_multiblock_part: bool,
    pub is_multiblock_controller: bool,
    // Textures will be added later
    pub debug_color: Color,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            id: BlockId::AIR,
            name: "Air".to_string(),
            hardness: 0.0,
            tool_type: ToolType::None,
            required_tool: None,
            is_solid: false,
            is_transparent: true,
            can_contain_fluid: false,
            is_multiblock_part: false,
            is_multiblock_controller: false,
            debug_color: Color::WHITE,
        }
    }
}

/// Global Registry for Blocks
#[derive(Resource, Default)]
pub struct BlockRegistry {
    blocks: HashMap<BlockId, BlockProperties>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            blocks: HashMap::new(),
        };
        registry.register_default_blocks();
        registry
    }

    pub fn register(&mut self, props: BlockProperties) {
        self.blocks.insert(props.id, props);
    }

    pub fn get(&self, id: BlockId) -> Option<&BlockProperties> {
        self.blocks.get(&id)
    }

    pub fn get_or_air(&self, id: BlockId) -> &BlockProperties {
        self.blocks.get(&id).unwrap_or_else(|| {
            self.blocks.get(&BlockId::AIR).unwrap()
        })
    }

    fn register_default_blocks(&mut self) {
        // Air
        self.register(BlockProperties {
            id: BlockId::AIR,
            name: "Air".to_string(),
            is_transparent: true,
            debug_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
            ..Default::default()
        });

        // Stone
        self.register(BlockProperties {
            id: BlockId::STONE,
            name: "Stone".to_string(),
            hardness: 1.5,
            tool_type: ToolType::Pickaxe,
            required_tool: Some(ToolType::Pickaxe),
            is_solid: true,
            debug_color: Color::srgb(0.5, 0.5, 0.5),
            ..Default::default()
        });

        // Dirt
        self.register(BlockProperties {
            id: BlockId::DIRT,
            name: "Dirt".to_string(),
            hardness: 0.5,
            tool_type: ToolType::Shovel,
            is_solid: true,
            debug_color: Color::srgb(0.6, 0.4, 0.2),
            ..Default::default()
        });

        // Grass
        self.register(BlockProperties {
            id: BlockId::GRASS,
            name: "Grass".to_string(),
            hardness: 0.6,
            tool_type: ToolType::Shovel,
            is_solid: true,
            debug_color: Color::srgb(0.2, 0.8, 0.2),
            ..Default::default()
        });
    }
}