use wonderthing::lev::*;
use wonderthing::lv6::{self, *};
use wonderthing::random::*;

use std::fs;

fn main() -> Result<(), lv6::Lv6Error> {
    convert_tow_to_lv6()?;
    operate_on_folder("TOW-LV6", "TOW-NOSIGNS", |lv6| {
        lv6.clear_signs();
    })?;
    operate_on_folder("TOW-LV6", "TOW-REVERSE", |lv6| {
        lv6.reverse_signs();
        lv6.reverse_level_name();
        let level_content = lv6.borrow_level_content_mut();
        let tiles = level_content.borrow_tiles_mut();
        tiles.replace_target(Tile::is_tollgate, || Tile::FLOOR);
        let objects = level_content.borrow_objects_mut();
        objects.swap_targets(Object::STINKY, Object::EXIT);
        level_content.set_tile_at_object(Object::EXIT, Tile::GATE_TOLL_HORIZONTAL);
    })?;
    operate_on_folder("TOW-LV6", "TOW-TRAMP", |lv6| {
        let level_content = lv6.borrow_level_content_mut();
        let tiles = level_content.borrow_tiles_mut();
        tiles.replace_target(Tile::EMPTY, || Tile::TRAMPOLINE);
        tiles.replace_target(Tile::WATER, || Tile::TRAMPOLINE);
        tiles.replace_target(Tile::LAVA, || Tile::TRAMPOLINE);
        level_content.set_tile_at_object(Object::BOX_WOOD, Tile::TRAMPOLINE);
        level_content.set_tile_at_object(Object::BOX_STEEL, Tile::TRAMPOLINE);
    })?;
    operate_on_folder("TOW-LV6", "TOW-SPIKES", |lv6| {
        let level_content = lv6.borrow_level_content_mut();
        let tiles = level_content.borrow_tiles_mut();
        tiles.replace_target(Tile::is_immediately_enterable, || Tile::FLOOR);
        tiles.replace_target(Tile::is_color_gate, || Tile::FLOOR);
        tiles.replace_target(Tile::is_liquid, || Tile::FLOOR);
        level_content.generate_spike_wave(Object::STINKY, 2, 1);
    })?;
    operate_on_folder("TOW-LV6", "TOW-SHADOWS", |lv6| {
        let level_content = lv6.borrow_level_content_mut();
        let tiles = level_content.borrow_tiles_mut();
        tiles.replace_target(Tile::WALL_HEIGHT_1, || Tile::SHADOW_STINKY_BLUE);
    })?;
    Ok(())
}

pub fn convert_tow_to_lv6() -> Result<(), lv6::Lv6Error> {
    let mut rng = Pcg32::from_current_time_and_address();
    for entry in fs::read_dir("TOW-LEV").map_err(lv6::Lv6Error::InputOutput)? {
        let entry = entry.map_err(lv6::Lv6Error::InputOutput)?;
        println!("Converting {:?}", entry.path());
        let lev = Lev::from_file(entry.path())?;
        let mut lv6: Lv6 = lev.into();
        //println!("Conversion OK.");
        lv6.to_file(&mut rng, "TOW-LV6")?;
        //println!("To file OK.");
    }
    Ok(())
}

pub fn operate_on_folder<F, P: AsRef<std::path::Path>>(in_path: P, out_path: &str, mut f: F) -> Result<(), lv6::Lv6Error>
where
    F: FnMut(&mut Lv6)
{
    let mut rng = Pcg32::from_current_time_and_address();
    for entry in fs::read_dir(in_path).map_err(lv6::Lv6Error::InputOutput)? {
        let entry = entry.map_err(lv6::Lv6Error::InputOutput)?;
        let mut lv6 = Lv6::from_file(entry.path())?;
        f(&mut lv6);
        lv6.to_file(&mut rng, out_path)?;
    }
    Ok(())
}