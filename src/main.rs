use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone)]
enum BlockType {
    NormalBlock,
    MathBlock,
    ValueTransmitter, // The green line
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
struct Block {
    block_type: BlockType,
    coordinates: Vec2,
    propagation: Connection,
    number_connections: Connection,
    // This value is NormalBlock specific
    value: i32,
    active: bool,
}

enum Side {
    Right,
    Left,
    Up,
    Down,
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
            active: false,
        }
    }
    
    fn get_single_number_connection_side(&self, connection: ConnectionType) -> Side {
        if connection == self.number_connections.left { return Side::Left }
        if connection == self.number_connections.right { return Side::Right}
        if connection == self.number_connections.up { return Side::Up}
        if connection == self.number_connections.down { return Side::Down}
        panic!("Connection does not exist!");
    }
}
#[derive(Clone)]
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
    // Create the map, should look like example.png
    map.add_block(BlockType::MathBlock, Vec2::from(0, 0));
    map.blocks[0].number_connections.left = ConnectionType::InputA;
    map.blocks[0].number_connections.right = ConnectionType::InputB;
    map.blocks[0].number_connections.up = ConnectionType::Output;
    map.add_block(BlockType::NormalBlock, Vec2::from(1, 0));
    map.blocks[1].value = 1;
    map.add_block(BlockType::NormalBlock, Vec2::from(0, 1));
    map.add_block(BlockType::ValueTransmitter, Vec2::from(-1, 1));
    map.blocks[3].number_connections.down = ConnectionType::Transmitting;
    map.blocks[3].number_connections.right = ConnectionType::Transmitting;
    map.add_block(BlockType::ValueTransmitter, Vec2::from(-1, 0));
    map.blocks[4].number_connections.up = ConnectionType::Transmitting;
    map.blocks[4].number_connections.right = ConnectionType::Transmitting;
    map.add_block(BlockType::NormalBlock, Vec2::from(-1, 0));
}

fn get_index_of_transmitting_block(map: &Map, side: Side, block_coordinates: Vec2) -> Option<usize> {
    let mut neighbour_coordinates = block_coordinates;

    if let Side::Left = side   { neighbour_coordinates.x -= 1}
    if let Side::Right = side  { neighbour_coordinates.x += 1}
    if let Side::Up = side     { neighbour_coordinates.y += 1}
    if let Side::Down = side   { neighbour_coordinates.y -= 1}

    let mut index = 0;
    for block in map.blocks.iter() {
        if neighbour_coordinates == block.coordinates {
            return Some(index);
        }
        index += 1;
    }
    return None;
    
}
fn math_blocks_logic(map: &mut Map) {
    for block in map.blocks.clone().iter_mut() {
        // If it is not an active MathBlock, skip
        if let BlockType::MathBlock = block.block_type { if !block.active { continue; }} else { continue; }

        let input_a_index = get_index_of_transmitting_block(
            &map, 
            block.get_single_number_connection_side(ConnectionType::InputA), 
            block.coordinates
        ).expect("Expected InputA block!");
        let input_b_index = get_index_of_transmitting_block(
            &map, 
            block.get_single_number_connection_side(ConnectionType::InputB), 
            block.coordinates
        ).expect("Expected InputB block");
        let output_index = get_index_of_transmitting_block(
            &map, 
            block.get_single_number_connection_side(ConnectionType::Output), 
            block.coordinates
        ).expect("Expected Output Block!");

        dbg!(output_index, input_a_index, input_b_index);
        
        map.blocks[output_index].value = map.blocks[input_a_index].value + map.blocks[input_b_index].value;
    }
}

fn main() {
    let mut main_map = Map::default();
    startup(&mut main_map);
    

    // Systems
    //loop {
        // We don't have a button
        main_map.blocks[0].active = true;
        // Here would normally go the function for the propagation


        math_blocks_logic(&mut main_map);

    //}
    println!("{}", main_map);
}
