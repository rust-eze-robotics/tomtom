# Rust-eze tomtom
### Finds the path with the smallest energy cost to the tile at the specified coordinates or to the _nearest_ tile that matches the specified optional tile type and content, considering every world variable, teleports included.

#### *get_path_to_coordinates* returns the path having the smallest energy cost to reach the destination tile at the given coordinates (or the 'nearest' adjacent tile), considering: go interface costs, tiles' walkability and elevation, environmental conditions and teleports.
```rust
pub fn get_path_to_coordinates(
    &self,
    robot: &impl Runnable,
    world: &World,
    adjacent: bool,
    destination: (usize, usize),
) -> Result<Path, String>
```
#### Arguments
- robot: &impl Runnable
- world: &World
- adjacent: bool => if true the function will target the adjacent tiles to destination; if false it will target destination itself.
- destination: (usize, usize) => destination tile of coordinates (row, col).
#### Return
- Result<Path, String> => Ok(path) returns the path, Err(e) represents a possible error described by String e.

#### *get_path_to_tile* returns the path having the smallest energy cost to reach the 'nearest' matched tile (or the 'nearest' adjacent tile), considering: go interface costs, tiles' walkability and elevation, environmental conditions and teleports. Matched tiles are the tiles, discovered by the robot, that match the optional tile type and content.
```rust
pub fn get_path_to_tile(
    &self,
    robot: &impl Runnable,
    world: &World,
    adjacent: bool,
    tile_type: Option<TileType>,
    content: Option<Content>,
) -> Result<Path, String>
```
#### Arguments
- robot: &impl Runnable
- world: &World
- adjacent: bool => if true the function will target the adjacent tiles to the matched tiles, if false it will target the matched tiles themselves.
- tile_type: Option<TileType> => optional tile type to be matched.
- content: Option<Content> => optional content to be matched.  
#### Return
- Result<Path, String> => Ok(path) returns the path, Err(e) represents a possible error described by String e.

#### *go_to_coordinates* calls *get_path_to_coordinates*: if the result is Ok(path) and the robot has enough energy to complete the path, it moves the robot to the path's destination tile.
```rust
pub fn go_to_coordinates(
    &self,
    robot: &mut impl Runnable,
    world: &mut World,
    adjacent: bool,
    destination: (usize, usize),
) -> Result<Path, String> 
```
#### Arguments
- robot: &impl Runnable
- world: &World
- adjacent: bool => if true the function will target the adjacent tiles to destination; if false it will target destination itself.
- destination: (usize, usize) => destination tile of coordinates (row, col).
#### Return
- Result<Path, String> => Ok(path) returns the path, Err(e) represents a possible error described by String e.

#### *go_to_tile* calls *get_path_to_tile*: if the result is Ok(path) and the robot has enough energy to complete the path, it moves the robot to the path's destination tile.
```rust
pub fn go_to_tile(
    &self,
    robot: &mut impl Runnable,
    world: &mut World,
    adjacent: bool,
    tile_type: Option<TileType>,
    content: Option<Content>,
) -> Result<Path, String>
```
#### Arguments
- robot: &impl Runnable
- world: &World
- adjacent: bool => if true the function will target the adjacent tiles to the matched tiles, if false it will target the matched tiles themselves.
- tile_type: Option<TileType> => optional tile type to be matched.
- content: Option<Content> => optional content to be matched.  
#### Return
- Result<Path, String> => Ok(path) returns the path, Err(e) represents a possible error described by String e.