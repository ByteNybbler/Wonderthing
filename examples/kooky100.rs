use wonderthing::*;

fn main() {
    let mut rng = Pcg32::from_current_time_and_address();

    let name_data = NameData::new("KOOKY100".to_owned(), "A Dark Shape".to_owned());
    let custom_content = CustomContent::from_none();
    let time_limit = TimeLimit::from_minutes_and_seconds(50, 00);
    let level_appearance = LevelAppearance::new(Style::GARDEN, Background::WARP);
    
    let mut level_content = LevelContent::from_uniform(500, 80, Tile::MODE_3D, Object::NONE);
    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::FLOWERS, 100);
    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::MUSHROOM_SMALL, 80);
    level_content.randomly_replace_objects_count(&mut rng, Object::NONE, Object::TREE_FAT, 800);
    level_content.set_object_at_center(Object::STINKY);
    level_content.set_object_at_coordinates(Object::LIGHTING_BLUE, 0, 0);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 19);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 39);
    level_content.set_object_at_coordinates(Object::EXIT, 4, 59);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 19);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 39);
    level_content.set_object_at_coordinates(Object::EXIT, 495, 59);
    
    let signs = Signs::from_none();
    let music = Music::SPOOKY;
    let mut lv6 = Lv6::new(name_data, custom_content, time_limit, level_appearance, level_content, signs, music);
    lv6.to_file(&mut rng, "").unwrap();
}