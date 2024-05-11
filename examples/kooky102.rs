use wonderthing::*;

fn main() -> std::result::Result<(), Lv6Error> {
    let mut rng = Pcg32::from_current_time_and_address();
    let mut lv6 = Lv6::from_file("tut1kooky102.LV6")?;
    lv6.set_names("KOOKY102".to_owned(), "first steps but with friends and advancing spikes".to_owned());
    let level_content = lv6.borrow_level_content_mut();
    level_content.set_object_at_coordinates(Object::LOOF, 4, 2);
    level_content.set_object_at_coordinates(Object::QOOKIE, 2, 4);
    level_content.generate_spike_wave(Object::STINKY, 2, 1);
    lv6.to_file(&mut rng, "").unwrap();
    Ok(())
}