use wonderthing::lev::*;
use wonderthing::lv6::{self, *};
use wonderthing::random::*;

use std::fs;

fn main() -> Result<(), wonderthing::lv6::Lv6Error> {
    //let mut rng = Pcg32::from_current_time_and_address();

    /*
    let name_data = NameData::new("KOOKY101".to_owned(), "A Dank Shape".to_owned());
    let custom_content = CustomContent::from_none();
    let time_limit = TimeLimit::from_minutes_and_seconds(50, 00);
    let level_appearance = LevelAppearance::new(Style::GARDEN, Background::WARP);

    let mut level_content = LevelContent::from_uniform(500, 80, Tile::MODE_3D, Object::NONE);

    level_content.set_object_at_center(Object::STINKY);
    level_content.set_object_at_coordinates(Object::LIGHTING_BLUE, 0, 0);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 19);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 39);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 59);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 19);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 39);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 59);

    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::FLOWERS, 100);
    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::MUSHROOM_SMALL, 80);
    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::TREE_FAT, 800);

    let signs = Signs::from_none();
    let music = Music::SPOOKY;
    let mut lv6 = Lv6::new(name_data, custom_content, time_limit, level_appearance, level_content, signs, music);
    lv6.to_file(&mut rng)
    */

    //let mut lv6 = Lv6::from_file("tut1kooky102.LV6")?;
    //lv6.set_names("KOOKY102".to_owned(), "first steps but with friends and advancing spikes".to_owned());
    //let level_content = lv6.borrow_level_content_mut();

    //lv6.to_file(&mut rng, "")

    // operate_on_folder("TOW-LV6", "TOW-SHADOWS", |lv6| {
    //     let level_content = lv6.borrow_level_content_mut();
    //     let tiles = level_content.borrow_tiles_mut();
    //     tiles.replace_target(Tile::WALL_HEIGHT_1, || Tile::SHADOW_STINKY_BLUE);
    // })

    operate_on_folder("TOW-LV6", "TOW-SPIKES-SPEED3", |lv6| {
        let level_content = lv6.borrow_level_content_mut();
        let tiles = level_content.borrow_tiles_mut();
        tiles.replace_target(Tile::is_immediately_enterable, || Tile::FLOOR);
        tiles.replace_target(Tile::is_color_gate, || Tile::FLOOR);
        tiles.replace_target(Tile::is_liquid, || Tile::FLOOR);
        level_content.generate_spike_wave(Object::STINKY, 2, 3);
    })
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