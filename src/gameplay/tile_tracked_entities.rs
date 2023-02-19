use bevy::{math::vec2, prelude::*, utils::HashMap};

use super::TILE_SIZE;

pub fn track_tile_entities(
    entities: Query<(Entity, &GlobalTransform), With<TileTrackedEntity>>,
    mut tracked_entities: ResMut<TileTrackedEntities>,
) {
    tracked_entities.clear();

    for (entity, transform) in entities.iter() {
        tracked_entities.add(transform.translation().truncate(), entity);
    }
}

#[derive(Component)]
pub struct TileTrackedEntity;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> TilePosition {
        TilePosition { x, y }
    }

    pub fn from_world(position: Vec2) -> TilePosition {
        TilePosition {
            x: (position.x / TILE_SIZE).floor() as i32,
            y: (position.y / TILE_SIZE).floor() as i32,
        }
    }

    pub fn to_world(&self) -> Vec2 {
        vec2((self.x as f32) * TILE_SIZE, (self.y as f32) * TILE_SIZE)
    }

    pub fn from_vec(vec: Vec2) -> TilePosition {
        TilePosition {
            x: vec.x.floor() as i32,
            y: vec.y.floor() as i32,
        }
    }

    pub fn to_vec(&self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }

    pub fn offset(&self, x: i32, y: i32) -> TilePosition {
        TilePosition {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn snap_world(position: Vec2) -> Vec2 {
        TilePosition::from_world(position).to_world()
    }
}

#[derive(Resource)]
pub struct TileTrackedEntities {
    map: HashMap<TilePosition, Vec<Entity>>,
}

impl TileTrackedEntities {
    pub fn new() -> TileTrackedEntities {
        TileTrackedEntities {
            map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn add(&mut self, world_position: Vec2, entity: Entity) {
        let tile_pos = TilePosition::from_world(world_position);

        if let Some(vec) = self.get_entities_in_tile_mut(tile_pos) {
            vec.push(entity);
        } else {
            self.map.insert(tile_pos, vec![entity]);
        }
    }

    pub fn get_entities_in_tile(&self, tile_pos: TilePosition) -> Option<&Vec<Entity>> {
        self.map.get(&tile_pos)
    }

    pub fn get_entities_in_tile_mut(&mut self, tile_pos: TilePosition) -> Option<&mut Vec<Entity>> {
        self.map.get_mut(&tile_pos)
    }
}
