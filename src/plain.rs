use robotics_lib::world::tile::{Content, TileType};

#[derive(Debug, PartialEq, Eq)]
pub enum PlainTileType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Street,
    Hill,
    Mountain,
    Snow,
    Lava,
    Teleport,
    Wall,
}

impl PlainTileType {
    pub(crate) fn eq_tile_type(&self, tile_type: &TileType) -> bool {
        match tile_type {
            TileType::DeepWater => *self == PlainTileType::DeepWater,
            TileType::ShallowWater => *self == PlainTileType::ShallowWater,
            TileType::Sand => *self == PlainTileType::Sand,
            TileType::Grass => *self == PlainTileType::Grass,
            TileType::Street => *self == PlainTileType::Street,
            TileType::Hill => *self == PlainTileType::Hill,
            TileType::Mountain => *self == PlainTileType::Mountain,
            TileType::Snow => *self == PlainTileType::Snow,
            TileType::Lava => *self == PlainTileType::Lava,
            TileType::Teleport(_) => *self == PlainTileType::Teleport,
            TileType::Wall => *self == PlainTileType::Wall,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlainContent {
    Rock,
    Tree,
    Garbage,
    Fire,
    Coin,
    Bin,
    Crate,
    Bank,
    Water,
    Market,
    Fish,
    Building,
    Bush,
    JollyBlock,
    Scarecrow,
    None,
}

impl PlainContent {
    pub(crate) fn eq_content(&self, content: &Content) -> bool {
        match content {
            Content::Rock(_) => *self == PlainContent::Rock,
            Content::Tree(_) => *self == PlainContent::Tree,
            Content::Garbage(_) => *self == PlainContent::Garbage,
            Content::Fire => *self == PlainContent::Fire,
            Content::Coin(_) => *self == PlainContent::Coin,
            Content::Bin(_) => *self == PlainContent::Bin,
            Content::Crate(_) => *self == PlainContent::Crate,
            Content::Bank(_) => *self == PlainContent::Bank,
            Content::Water(_) => *self == PlainContent::Water,
            Content::Market(_) => *self == PlainContent::Market,
            Content::Fish(_) => *self == PlainContent::Fish,
            Content::Building => *self == PlainContent::Building,
            Content::Bush(_) => *self == PlainContent::Bush,
            Content::JollyBlock(_) => *self == PlainContent::JollyBlock,
            Content::Scarecrow => *self == PlainContent::Scarecrow,
            Content::None => *self == PlainContent::None,
        }
    }
}
