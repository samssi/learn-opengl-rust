use std::collections::HashMap;
use crate::file_reader::read_file_to_string;

pub fn read_shaders_into_memory() -> HashMap<String, String> {
    // TODO: read all shader dir contents into HashMap and return them
    let shaders = HashMap::from([
        ("default.frag".to_string(), read_file_to_string("assets/shaders/fragment/default.frag")),
        ("default.vert".to_string(), read_file_to_string("assets/shaders/vertex/default.vert"))
    ]);

    return shaders;
}