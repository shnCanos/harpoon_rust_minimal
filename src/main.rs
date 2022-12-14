use std::fmt::Display;

#[derive(Debug)]
enum ConnectionType {
    None,

    // Propagation specific connections
    Propagating, 
    
    // Math specific connections
    Output,
    InputA,
    InputB,

    // ValueTransmiter specific connection
    Transmitting
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
enum BlockType {
    NormalBlock,
    MathBlock,
    ValueTransmitter, // The green line
}

#[derive(Debug)]
struct Connection {
    right: ConnectionType,
    left: ConnectionType,
    up: ConnectionType,
    down: ConnectionType,
}

impl Default for Connection {
    fn default() -> Self {
        Self { right: Default::default(), left: Default::default(), up: Default::default(), down: Default::default() }
    }
}

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn from(x: i32, y: i32) -> Self {
       Self {
        x,y
       } 
    }
}

#[derive(Debug)]
struct Block {
    block_type: BlockType,
    coordinates: Vec2,
    propagation: Connection,
    number_connections: Connection,
    // This value is NormalBlock specific
    value: i32,
}

impl Block {
    pub fn spawn_block(block_type: BlockType, coordinates: Vec2,) -> Self {
        // println!("Creating block!");
        Self {
            block_type: block_type,
            coordinates: coordinates,
            propagation: Connection::default(),
            number_connections: Connection::default(),
            value: 0,
        }
    }
}

struct Map {
    blocks: Vec<Block>
}
impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blocks: {:#?}", &self.blocks)
    }
}
impl Default for Map {
    fn default() -> Self {
        Self { blocks: Vec::new() }
    }
}

impl Map {
    fn add_block(&mut self, block_type: BlockType, coordinates: Vec2) {
        self.blocks.push(Block::spawn_block(block_type, coordinates))
    }
}


fn startup(map: &mut Map) {
    map.add_block(BlockType::NormalBlock, Vec2::from(0, 0));
    map.add_block(BlockType::MathBlock, Vec2::from(1, 0));
    map.add_block(BlockType::NormalBlock, Vec2::from(-1, 0));
    map.add_block(BlockType::NormalBlock, Vec2::from(0, 1));
}

fn main() {
    let mut main_map = Map::default();
    startup(&mut main_map);
    println!("{}", main_map);
}
