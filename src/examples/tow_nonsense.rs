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