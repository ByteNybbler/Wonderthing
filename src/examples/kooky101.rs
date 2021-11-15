let mut lv6 = Lv6::from_file("tut1.LV6")?;
lv6.set_names("KOOKY101".to_owned(), "Back Steps".to_owned());
lv6.set_time_limit(TimeLimit::from_minutes_and_seconds(2002, 00));
lv6.set_music(Music::PSEUDO_SILENT);
let level_content = lv6.borrow_level_content_mut();
let tiles = level_content.borrow_tiles_mut();
tiles.replace_target(Tile::EMPTY, || Tile::FLOOR_INVISIBLE);
let objects = level_content.borrow_objects_mut();
objects.replace_target(Object::is_stinker, || Object::NONE);
objects.replace_target(Object::is_pickup, || Object::NONE);
level_content.expand_all(|| Tile::FLOOR_INVISIBLE, || Object::NONE, 1);
level_content.duplicate_horizontal(6);
level_content.duplicate_vertical(6);
level_content.set_object_at_coordinates(Object::STINKY, 72, 69);
level_content.set_tile_at_coordinates(Tile::MODE_3D, 72, 69);
lv6.to_file(&mut rng, "")