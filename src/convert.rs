use std::collections::HashMap;

use thiserror::Error;

use mc_classic_js::{ChangedBlocks, Data, JSLevel, Settings};
use mc_classic::Level;

#[derive(Error, Debug)]
pub enum ConversionError {}

pub fn js_to_classic (data: Data) -> Result<Level,ConversionError> {
    let mut level: Level = Level::new();

    //Setting fields
    println!("Setting fields");
    level.width = Some(data.js_level.worldSize);
    level.depth = Some(64);
    level.height = Some(data.js_level.worldSize);
    level.creator = Some(data.settings.username);

    //Writing blocks
    println!("Writing blocks");
    let mut tile_map: Vec<u8> = mc_classic_js::get_tile_map(data.js_level.worldSize, data.js_level.worldSeed);

    for i in 0..level.depth.unwrap() {
        for j in 0..level.height.unwrap() {
            for k in 0..level.width.unwrap() {
                let key: String = String::from(format!(r#"p{}_{}_{}"#,k,i,j));

                if data.js_level.changedBlocks.contains_key(&key) {
                    tile_map[((i*level.height.unwrap()*level.width.unwrap()) + (j*level.width.unwrap()) + k) as usize] = data.js_level.changedBlocks.get(&key).unwrap().bt;
                }
            }
        }
    }

    //Converting blocks
    println!("Converting tile ids");
    let mut i: usize = 0;
    for tile in tile_map.clone() {
        match tile {
            1 => tile_map[i] = 2, //Grass Block
            2 => tile_map[i] = 1, //Stone
            3 => tile_map[i] = 3, //Dirt
            4 => tile_map[i] = 5, //Planks
            5 => tile_map[i] = 38, //Rose
            6 => tile_map[i] = 37, //Dandelion
            7 => tile_map[i] = 9, //Water
            8 => tile_map[i] = 6, //Sapling
            9 => tile_map[i] = 4, //Cobblestone
            10 => tile_map[i] = 7, //Bedrock
            11 => tile_map[i] = 12, //Sand
            12 => tile_map[i] = 13, //Gravel
            13 => tile_map[i] = 17, //Logs
            14 => tile_map[i] = 18, //Leaves
            15 => tile_map[i] = 40, //Red Mushroom
            16 => tile_map[i] = 39, //Brown Mushroom
            17 => tile_map[i] = 11, //Lava
            18 => tile_map[i] = 14, //Gold Ore
            19 => tile_map[i] = 15, //Iron Ore
            20 => tile_map[i] = 16, //Coal Ore
            21 => tile_map[i] = 41, //Block of Gold
            22 => tile_map[i] = 19, //Sponge
            23 => tile_map[i] = 20, //Glass
            24 => tile_map[i] = 21, //Red Cloth
            25 => tile_map[i] = 22, //Orange Cloth
            26 => tile_map[i] = 23, //Yellow Cloth
            27 => tile_map[i] = 24, //Chartreuse Cloth
            28 => tile_map[i] = 25, //Green Cloth
            29 => tile_map[i] = 26, //Spring Green Cloth
            30 => tile_map[i] = 27, //Cyan Cloth
            31 => tile_map[i] = 28, //Capri Cloth
            32 => tile_map[i] = 29, //Ultramarine Cloth
            33 => tile_map[i] = 31, //Violet Cloth
            34 => tile_map[i] = 30, //Purple Cloth
            35 => tile_map[i] = 32, //Magenta Cloth
            36 => tile_map[i] = 33, //Rose Cloth
            37 => tile_map[i] = 34, //Dark Gray Cloth
            38 => tile_map[i] = 35, //Light Gray Cloth
            39 => tile_map[i] = 36, //White Cloth
            _ => tile_map[i] = 0,
        }
        i += 1;
    }

    level.blocks = Some(tile_map);

    return Ok(level);
}