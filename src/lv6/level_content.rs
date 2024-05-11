use serde::{Serialize, Deserialize, Deserializer, de::Visitor, de::SeqAccess, de};
use super::object::*;
use super::tile::*;
use crate::random::*;
use std::collections::HashSet;

#[derive(Serialize)]
pub struct LevelContent {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    objects: Vec<Object>
}

impl LevelContent {
    pub fn from_uniform(width: usize, height: usize, tile: Tile, object: Object) -> LevelContent {
        let total = width * height;
        LevelContent {
            width,
            height,
            tiles: vec![tile; total],
            objects: vec![object; total]
        }
    }

    pub fn borrow_tiles_mut(&mut self) -> &mut [Tile] {
        &mut self.tiles
    }

    pub fn borrow_objects_mut(&mut self) -> &mut [Object] {
        &mut self.objects
    }

    pub fn get_randomizer<'a, R>(&self, rng: &'a mut R) -> RandomWithMax<'a, R>
    where 
        R: Rng<u32>
    {
        RandomWithMax {
            max: (self.width * self.height) as u32,
            rng
        }
    }

    fn get_index_at_coordinates(&self, x: usize, y: usize) -> usize {
        (x + y * self.width) as usize
    }

    fn get_coordinates_at_index(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get_index_at_center(&self) -> usize {
        self.get_index_at_coordinates(self.width / 2, self.height / 2)
    }

    fn set_object_at_index(&mut self, object: Object, index: usize) {
        self.objects[index] = object;
    }

    fn set_tile_at_index(&mut self, tile: Tile, index: usize) {
        self.tiles[index] = tile;
    }

    pub fn set_object_at_center(&mut self, object: Object) {
        let center = self.get_index_at_center();
        self.set_object_at_index(object, center);
    }

    pub fn set_tile_at_center(&mut self, tile: Tile) {
        let center = self.get_index_at_center();
        self.set_tile_at_index(tile, center);
    }

    pub fn set_object_at_coordinates(&mut self, object: Object, x: usize, y: usize) {
        let index = self.get_index_at_coordinates(x, y);
        self.set_object_at_index(object, index);
    }

    pub fn set_tile_at_coordinates(&mut self, tile: Tile, x: usize, y: usize) {
        let index = self.get_index_at_coordinates(x, y);
        self.set_tile_at_index(tile, index);
    }

    pub fn set_objects_at_indices<I>(&mut self, object: Object, iter: I)
    where
        I: IntoIterator<Item=usize>
    {
        for index in iter {
            self.set_object_at_index(object, index);
        }
    }

    pub fn set_tiles_at_indices<I>(&mut self, tile: Tile, iter: I)
    where
        I: IntoIterator<Item=usize>
    {
        for index in iter {
            self.set_tile_at_index(tile, index);
        }
    }

    fn get_index_count(&self) -> usize {
        self.width * self.height
    }

    pub fn set_tile_at_object(&mut self, object: Object, tile: Tile) {
        let total = self.get_index_count();
        for i in 0..total {
            if self.objects[i] == object {
                self.tiles[i] = tile;
            }
        }
    }

    pub fn set_object_at_tile(&mut self, tile: Tile, object: Object) {
        let total = self.get_index_count();
        for i in 0..total {
            if self.tiles[i] == tile {
                self.objects[i] = object;
            }
        }
    }

    pub fn expand_up<F, G>(&mut self, tile: F, object: G, count: usize)
    where
        F: FnMut() -> Tile,
        G: FnMut() -> Object
    {
        let total_additions = self.width * count;

        let mut new_tiles: Vec<_> = std::iter::repeat_with(tile).take(total_additions).collect();
        new_tiles.append(&mut self.tiles);
        self.tiles = new_tiles;

        let mut new_objects: Vec<_> = std::iter::repeat_with(object).take(total_additions).collect();
        new_objects.append(&mut self.objects);
        self.objects = new_objects;

        self.height += count;
    }

    pub fn expand_down<F, G>(&mut self, tile: F, object: G, count: usize)
    where
        F: FnMut() -> Tile,
        G: FnMut() -> Object
    {
        let total_additions = self.width * count;

        self.tiles.extend(std::iter::repeat_with(tile).take(total_additions));
        self.objects.extend(std::iter::repeat_with(object).take(total_additions));

        self.height += count;
    }

    pub fn expand_left<F, G>(&mut self, mut tile: F, mut object: G, count: usize)
    where
        F: FnMut() -> Tile,
        G: FnMut() -> Object
    {
        let mut current_index = 0;

        for _ in 0..self.height {
            for _ in 0..count {
                self.tiles.insert(current_index, tile());
                self.objects.insert(current_index, object());
                current_index += 1;
            }
            current_index += self.width;
        }

        self.width += count;
    }

    pub fn expand_right<F, G>(&mut self, mut tile: F, mut object: G, count: usize)
    where
        F: FnMut() -> Tile,
        G: FnMut() -> Object
    {
        let mut current_index = 0;

        for _ in 0..self.height {
            current_index += self.width;
            for _ in 0..count {
                self.tiles.insert(current_index, tile());
                self.objects.insert(current_index, object());
                current_index += 1;
            }
        }

        self.width += count;
    }

    pub fn expand_all<F, G>(&mut self, mut tile: F, mut object: G, count: usize)
    where
        F: FnMut() -> Tile,
        G: FnMut() -> Object
    {
        self.expand_up(&mut tile, &mut object, count);
        self.expand_down(&mut tile, &mut object, count);
        self.expand_left(&mut tile, &mut object, count);
        self.expand_right(tile, object, count);
    }

    pub fn duplicate_vertical(&mut self, count: usize) {
        let tiles_original = self.tiles.clone();
        let objects_original = self.objects.clone();
        for _ in 0..count {
            self.tiles.extend(tiles_original.iter());
            self.objects.extend(objects_original.iter());
        }
        self.height += count * self.height;
    }

    pub fn duplicate_horizontal(&mut self, count: usize) {
        let tiles_original = self.tiles.clone();
        let objects_original = self.objects.clone();
        let original_width = self.width;
        for _ in 0..count {
            let mut current_index_tile = 0;
            let mut current_index_object = 0;
            let tile_fn = || {
                let result = tiles_original[current_index_tile];
                current_index_tile += 1;
                result
            };
            let object_fn = || {
                let result = objects_original[current_index_object];
                current_index_object += 1;
                result
            };
            self.expand_right(tile_fn, object_fn, original_width);
        }
    }

    // TODO: crop_out_tile(&mut self, tile: impl ReplacementTarget)

    pub fn generate_spike_wave(&mut self, start: Object, starting_speed: i32, wave_speed: i32) {
        let mut positions = vec![];
        let position_start = self.objects.iter().position(|o| *o == start).expect("generate_spike_wave: couldn't find specified object");
        let coords_start = self.get_coordinates_at_index(position_start);
        positions.push(coords_start);
        for i in starting_speed-1.. {
            let speed = i / wave_speed + 1;
            //println!("Generating spikes of speed {} (position count: {})", speed, positions.len());
            if speed > 100 {
                break;
            }
            for (x, y) in positions.iter() {
                self.set_tile_at_coordinates(Tile::spikes_with_speed(speed), *x, *y);
            }
            let mut new_positions = HashSet::new(); // HashSet to prevent duplicate entries.
            while let Some((x, y)) = positions.pop() {
                new_positions.extend(self.get_adjacent_tile(x, y, Tile::FLOOR));
            }
            positions.extend(new_positions);
            if positions.len() == 0 {
                break;
            }
        }
    }

    fn get_adjacent(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        if x > 0 {
            result.push((x-1, y));
        }
        if x < self.width {
            result.push((x+1, y));
        }
        if y > 0 {
            result.push((x, y-1));
        }
        if y < self.width {
            result.push((x, y+1));
        }
        result
    }

    fn get_adjacent_tile(&self, x: usize, y: usize, tile: Tile) -> Vec<(usize, usize)> {
        let vector = self.get_adjacent(x, y);
        vector.into_iter().filter(|(x, y)| self.tiles[self.get_index_at_coordinates(*x, *y)] == tile).collect()
    }

    // TODO: make_conveyor_2_invisible
}

pub struct RandomWithMax<'a, R>
where
    R: Rng<u32>
{
    max: u32,
    rng: &'a mut R
}

impl<'a, R> Iterator for RandomWithMax<'a, R>
where
    R: Rng<u32>
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.random_bounded(self.max) as usize)
    }
}

impl<'de> Deserialize<'de> for LevelContent {
    fn deserialize<D>(deserializer: D) -> Result<LevelContent, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_struct("LevelContent", &["width", "height", "tiles", "objects"], LevelContentVisitor)
    }
}

pub struct LevelContentVisitor;

impl<'de> Visitor<'de> for LevelContentVisitor {
    type Value = LevelContent;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "struct LevelContent")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<LevelContent, V::Error>
    where
        V: SeqAccess<'de>
    {
        let width: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let height: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let width = width as usize;
        let height = height as usize;
        let total = width * height;
        let mut tiles = Vec::with_capacity(total);
        for _ in 0..total {
            let id = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            tiles.push(Tile(id));
        }
        let mut objects = Vec::with_capacity(total);
        for _ in 0..total {
            let id = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            objects.push(Object(id));
        }
        Ok(LevelContent {
            width,
            height,
            tiles,
            objects
        })
    }
}