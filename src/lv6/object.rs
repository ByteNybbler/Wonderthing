use serde::{Serialize, Deserialize};
use crate::random::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Object(pub i32);

impl Object {
    pub const NONE: Self = Self(0);
    pub const STINKY: Self = Self(1);
    pub const LOOF: Self = Self(2);
    pub const QOOKIE: Self = Self(3);
    pub const PEEGUE: Self = Self(4);
    pub const COIN_RAINBOW: Self = Self(5);
    pub const EXIT: Self = Self(6);
    pub const CLOCK: Self = Self(7);
    pub const COIN_BONUS: Self = Self(8);
    pub const BOX_WOOD: Self = Self(9);
    pub const BOX_STEEL: Self = Self(10);
    pub const REFLECTOR_NORTHWEST: Self = Self(11);
    pub const REFLECTOR_NORTHEAST: Self = Self(12);
    pub const BOULDER: Self = Self(13);
    pub const BOX_PLASMA: Self = Self(14);
    pub const POWDER_KEG: Self = Self(15);
    pub const PRISM: Self = Self(16);
    pub const COILY: Self = Self(17);
    pub const ZBOT_NORTH_LEFT: Self = Self(18);
    pub const ZBOT_NORTH_RIGHT: Self = Self(19);
    pub const ZBOT_EAST_LEFT: Self = Self(20);
    pub const ZBOT_EAST_RIGHT: Self = Self(21);
    pub const ZBOT_SOUTH_LEFT: Self = Self(22);
    pub const ZBOT_SOUTH_RIGHT: Self = Self(23);
    pub const ZBOT_WEST_LEFT: Self = Self(24);
    pub const ZBOT_WEST_RIGHT: Self = Self(25);
    pub const ZBOT_BROKEN: Self = Self(26);
    pub const KABOOM_NORTH_LEFT: Self = Self(27);
    pub const KABOOM_NORTH_RIGHT: Self = Self(28);
    pub const KABOOM_EAST_LEFT: Self = Self(29);
    pub const KABOOM_EAST_RIGHT: Self = Self(30);
    pub const KABOOM_SOUTH_LEFT: Self = Self(31);
    pub const KABOOM_SOUTH_RIGHT: Self = Self(32);
    pub const KABOOM_WEST_LEFT: Self = Self(33);
    pub const KABOOM_WEST_RIGHT: Self = Self(34);
    pub const UFO_NORTH_LEFT: Self = Self(35);
    pub const UFO_NORTH_RIGHT: Self = Self(36);
    pub const UFO_EAST_LEFT: Self = Self(37);
    pub const UFO_EAST_RIGHT: Self = Self(38);
    pub const UFO_SOUTH_LEFT: Self = Self(39);
    pub const UFO_SOUTH_RIGHT: Self = Self(40);
    pub const UFO_WEST_LEFT: Self = Self(41);
    pub const UFO_WEST_RIGHT: Self = Self(42);
    pub const CHOMPER_RED: Self = Self(43);
    pub const CHOMPER_YELLOW: Self = Self(44);
    pub const GHOST: Self = Self(45);
    pub const RAINBOW_SPIRIT: Self = Self(46);
    pub const MOTHERSHIP: Self = Self(47);
    pub const FISH: Self = Self(48);
    pub const FIREBALL: Self = Self(49);
    pub const PILLAR: Self = Self(50);
    pub const SPIKE_FAT: Self = Self(51);
    pub const SPIKE_THIN: Self = Self(52);
    pub const FOUNTAIN: Self = Self(53);
    pub const PYRAMID_LARGE: Self = Self(54);
    pub const PYRAMID_SMALL: Self = Self(55);
    pub const BOX_LARGE: Self = Self(56);
    pub const LAMP_POST: Self = Self(57);
    pub const HOUSE_SOUTH: Self = Self(58);
    pub const HOUSE_WEST: Self = Self(59);
    pub const HOUSE_NORTH: Self = Self(60);
    pub const HOUSE_EAST: Self = Self(61);
    pub const HOUSE_CUSTOM_1: Self = Self(62);
    pub const HOUSE_CUSTOM_2: Self = Self(63);
    pub const HOUSE_CUSTOM_3: Self = Self(64);
    pub const HOUSE_CUSTOM_4: Self = Self(65);
    pub const TREE_FAT: Self = Self(66);
    pub const TREE_THIN: Self = Self(67);
    pub const EVERGREEN_NORMAL: Self = Self(68);
    pub const EVERGREEN_SNOWY: Self = Self(69);
    pub const MUSHROOM_SMALL: Self = Self(70);
    pub const MUSHROOM_LARGE: Self = Self(71);
    pub const FLOWERS: Self = Self(72);
    pub const STATUE_STONE: Self = Self(73);
    pub const STATUE_RAINBOW: Self = Self(74);
    pub const SNOWMAN: Self = Self(75);
    pub const CUSTOM_MODEL_A: Self = Self(76);
    pub const CUSTOM_MODEL_B: Self = Self(77);
    pub const CUSTOM_MODEL_C: Self = Self(78);
    pub const CUSTOM_MODEL_D: Self = Self(79);
    pub const LIGHTING_RED: Self = Self(80);
    pub const LIGHTING_GREEN: Self = Self(81);
    pub const LIGHTING_BLUE: Self = Self(82);
    pub const LIGHTING_YELLOW: Self = Self(83);
    pub const WEATHER_SNOW: Self = Self(84);
    pub const WEATHER_RAIN: Self = Self(85);
    pub const WEATHER_LIGHTNING: Self = Self(86);
    pub const LIGHTING_RAINBOW: Self = Self(87);
}

impl Object {
    pub fn random<N>(rng: &mut N) -> Object
    where
        N: Rng<i32>
    {
        Object(rng.random_bounded(88))
    }

    pub fn random_pickup<N>(rng: &mut N) -> Object
    where
        N: Rng<usize>
    {
        *[Object::COIN_RAINBOW, Object::COIN_BONUS, Object::CLOCK].choose(rng)
    }

    pub fn is_pickup(&self) -> bool {
        *self == Object::COIN_RAINBOW ||
        *self == Object::COIN_BONUS ||
        *self == Object::CLOCK
    }

    pub fn is_stinker(&self) -> bool {
        *self == Object::STINKY ||
        *self == Object::LOOF ||
        *self == Object::QOOKIE ||
        *self == Object::PEEGUE
    }

    pub fn from_lev_to_lv6(&mut self) {
        // TODO: Finish this list.
        *self = match self.0 {
            0 => Self::NONE,
            1 => Self::STINKY,
            2 => Self::LOOF,
            3 => Self::BOX_WOOD,
            4 => Self::BOX_STEEL,
            5 => Self::EXIT,
            6 => Self::COIN_RAINBOW,
            7 => Self::COILY,
            8 => Self::ZBOT_NORTH_LEFT,
            9 => Self::ZBOT_NORTH_RIGHT,
            10 => Self::ZBOT_EAST_LEFT,
            11 => Self::ZBOT_EAST_RIGHT,
            12 => Self::ZBOT_SOUTH_LEFT,
            13 => Self::ZBOT_SOUTH_RIGHT,
            14 => Self::ZBOT_WEST_LEFT,
            15 => Self::ZBOT_WEST_RIGHT,
            16 => Self::KABOOM_NORTH_LEFT,
            17 => Self::KABOOM_NORTH_RIGHT,
            18 => Self::KABOOM_EAST_LEFT,
            19 => Self::KABOOM_EAST_RIGHT,
            20 => Self::KABOOM_SOUTH_LEFT,
            21 => Self::KABOOM_SOUTH_RIGHT,
            22 => Self::KABOOM_WEST_LEFT,
            23 => Self::KABOOM_WEST_RIGHT,
            25 => Self::COIN_BONUS,
            26 => Self::FISH,
            27 => Self::WEATHER_SNOW,
            28 => Self::PILLAR,
            29 => Self::SPIKE_FAT,
            30 => Self::SPIKE_THIN,
            unknown => {
                println!("LEV: Found unknown object ID {}; replacing with placeholder...", unknown);
                Self::EVERGREEN_SNOWY
            }
        }
    }
}

impl ReplacementPattern<Object> for Object
{
    fn satisfied_by(&self, other: &Object) -> bool {
        self == other
    }
}