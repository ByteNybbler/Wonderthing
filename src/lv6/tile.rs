use serde::{Serialize, Deserialize};
use crate::random::*;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct Tile(pub i32);

impl Tile {
    pub const EMPTY: Self = Self(0);
    pub const FLOOR: Self = Self(100);
    pub const FLOOR_B: Self = Self(101);
    pub const FLOOR_C: Self = Self(102);
    pub const FLOOR_D: Self = Self(103);
    pub const FLOOR_WALL: Self = Self(104);
    pub const FLOOR_INVISIBLE: Self = Self(105);
    pub const WALL_HEIGHT_1: Self = Self(200);
    pub const WALL_PIT: Self = Self(201);
    pub const WALL_HEIGHT_1_5: Self = Self(202);
    pub const WALL_HEIGHT_2: Self = Self(203);
    pub const WALL_INVISIBLE: Self = Self(204);
    pub const WATER: Self = Self(300);
    pub const WATER_INVISIBLE: Self = Self(301);
    pub const ICE_STRAIGHT: Self = Self(400);
    pub const ICE_CURVE_SOUTHEAST: Self = Self(401);
    pub const ICE_CURVE_SOUTHWEST: Self = Self(402);
    pub const ICE_CURVE_NORTHWEST: Self = Self(403);
    pub const ICE_CURVE_NORTHEAST: Self = Self(404);
    pub const ICE_DEADLY_INVISIBLE: Self = Self(405);
    pub const CONVEYOR_1_GRAY_NORTH: Self = Self(500);
    pub const CONVEYOR_1_GRAY_EAST: Self = Self(501);
    pub const CONVEYOR_1_GRAY_SOUTH: Self = Self(502);
    pub const CONVEYOR_1_GRAY_WEST: Self = Self(503);
    pub const CONVEYOR_1_SWITCH_NORTH: Self = Self(504);
    pub const CONVEYOR_1_SWITCH_EAST: Self = Self(505);
    pub const CONVEYOR_1_SWITCH_SOUTH: Self = Self(506);
    pub const CONVEYOR_1_SWITCH_WEST: Self = Self(507);
    pub const CONVEYOR_2_GRAY_NORTH: Self = Self(508);
    pub const CONVEYOR_2_GRAY_EAST: Self = Self(509);
    pub const CONVEYOR_2_GRAY_SOUTH: Self = Self(510);
    pub const CONVEYOR_2_GRAY_WEST: Self = Self(511);
    pub const CONVEYOR_2_SWITCH_NORTH: Self = Self(512);
    pub const CONVEYOR_2_SWITCH_EAST: Self = Self(513);
    pub const CONVEYOR_2_SWITCH_SOUTH: Self = Self(514);
    pub const CONVEYOR_2_SWITCH_WEST: Self = Self(515);
    pub const CONVEYOR_2_GRAY_NORTH_INVISIBLE: Self = Self(516);
    pub const CONVEYOR_2_GRAY_EAST_INVISIBLE: Self = Self(517);
    pub const CONVEYOR_2_GRAY_SOUTH_INVISIBLE: Self = Self(518);
    pub const CONVEYOR_2_GRAY_WEST_INVISIBLE: Self = Self(519);
    pub const CONVEYOR_2_SWITCH_NORTH_INVISIBLE: Self = Self(520);
    pub const CONVEYOR_2_SWITCH_EAST_INVISIBLE: Self = Self(521);
    pub const CONVEYOR_2_SWITCH_SOUTH_INVISIBLE: Self = Self(522);
    pub const CONVEYOR_2_SWITCH_WEST_INVISIBLE: Self = Self(523);
    pub const BREAK_TILE_WATER_1: Self = Self(600);
    pub const BREAK_TILE_WATER_2: Self = Self(601);
    pub const BREAK_TILE_WATER_3: Self = Self(602);
    pub const BREAK_TILE_LAVA_1: Self = Self(603);
    pub const BREAK_TILE_LAVA_2: Self = Self(604);
    pub const BREAK_TILE_LAVA_3: Self = Self(605);
    pub const BREAK_TILE_EMPTY_1: Self = Self(606);
    pub const BREAK_TILE_EMPTY_2: Self = Self(607);
    pub const BREAK_TILE_EMPTY_3: Self = Self(608);
    pub const ELECTRO_SPEED_1: Self = Self(700);
    pub const ELECTRO_SPEED_2: Self = Self(701);
    pub const ELECTRO_SPEED_3: Self = Self(702);
    pub const ELECTRO_SPEED_4: Self = Self(703);
    pub const SCOUGE_PURPLE_NORTH_SPEED_1: Self = Self(800);
    pub const SCOUGE_PURPLE_NORTH_SPEED_2: Self = Self(801);
    pub const SCOUGE_PURPLE_NORTH_SPEED_3: Self = Self(802);
    pub const SCOUGE_PURPLE_NORTH_SPEED_4: Self = Self(803);
    pub const SCOUGE_PURPLE_EAST_SPEED_1: Self = Self(804);
    pub const SCOUGE_PURPLE_EAST_SPEED_2: Self = Self(805);
    pub const SCOUGE_PURPLE_EAST_SPEED_3: Self = Self(806);
    pub const SCOUGE_PURPLE_EAST_SPEED_4: Self = Self(807);
    pub const SCOUGE_PURPLE_SOUTH_SPEED_1: Self = Self(808);
    pub const SCOUGE_PURPLE_SOUTH_SPEED_2: Self = Self(809);
    pub const SCOUGE_PURPLE_SOUTH_SPEED_3: Self = Self(810);
    pub const SCOUGE_PURPLE_SOUTH_SPEED_4: Self = Self(811);
    pub const SCOUGE_PURPLE_WEST_SPEED_1: Self = Self(812);
    pub const SCOUGE_PURPLE_WEST_SPEED_2: Self = Self(813);
    pub const SCOUGE_PURPLE_WEST_SPEED_3: Self = Self(814);
    pub const SCOUGE_PURPLE_WEST_SPEED_4: Self = Self(815);
    pub const LAVA: Self = Self(900);
    pub const LAVA_INVISIBLE: Self = Self(901);
    pub const GATE_TOLL_HORIZONTAL: Self = Self(1000);
    pub const GATE_TOLL_VERTICAL: Self = Self(1001);
    pub const GATE_PURPLE_HORIZONTAL: Self = Self(1002);
    pub const GATE_PURPLE_VERTICAL: Self = Self(1003);
    pub const GATE_YELLOW_HORIZONTAL: Self = Self(1004);
    pub const GATE_YELLOW_VERTICAL: Self = Self(1005);
    pub const GATE_GREEN_HORIZONTAL: Self = Self(1006);
    pub const GATE_GREEN_VERTICAL: Self = Self(1007);
    pub const GATE_BLUE_HORIZONTAL: Self = Self(1008);
    pub const GATE_BLUE_VERTICAL: Self = Self(1009);
    pub const GATE_RED_HORIZONTAL: Self = Self(1010);
    pub const GATE_RED_VERTICAL: Self = Self(1011);
    pub const GATE_INDIGO_HORIZONTAL: Self = Self(1012);
    pub const GATE_INDIGO_VERTICAL: Self = Self(1013);
    pub const GATE_WHITE_HORIZONTAL: Self = Self(1014);
    pub const GATE_WHITE_VERTICAL: Self = Self(1015);
    pub const GATE_FAKE_HORIZONTAL: Self = Self(1050);
    pub const GATE_FAKE_VERTICAL: Self = Self(1051);
    pub const BUTTON_SQUARE_CONVEYOR_1: Self = Self(1100);
    pub const BUTTON_CIRCLE_CONVEYOR_1: Self = Self(1101);
    pub const BUTTON_STAR_CONVEYOR_1: Self = Self(1102);
    pub const BUTTON_SQUARE_CONVEYOR_2: Self = Self(1103);
    pub const BUTTON_CIRCLE_CONVEYOR_2: Self = Self(1104);
    pub const BUTTON_STAR_CONVEYOR_2: Self = Self(1105);
    pub const BUTTON_SQUARE_PURPLE: Self = Self(1106);
    pub const BUTTON_CIRCLE_PURPLE: Self = Self(1107);
    pub const BUTTON_STAR_PURPLE: Self = Self(1108);
    pub const BUTTON_SQUARE_YELLOW: Self = Self(1109);
    pub const BUTTON_CIRCLE_YELLOW: Self = Self(1110);
    pub const BUTTON_STAR_YELLOW: Self = Self(1111);
    pub const BUTTON_SQUARE_GREEN: Self = Self(1112);
    pub const BUTTON_CIRCLE_GREEN: Self = Self(1113);
    pub const BUTTON_STAR_GREEN: Self = Self(1114);
    pub const BUTTON_SQUARE_BLUE: Self = Self(1115);
    pub const BUTTON_CIRCLE_BLUE: Self = Self(1116);
    pub const BUTTON_STAR_BLUE: Self = Self(1117);
    pub const BUTTON_SQUARE_RED: Self = Self(1118);
    pub const BUTTON_CIRCLE_RED: Self = Self(1119);
    pub const BUTTON_STAR_RED: Self = Self(1120);
    pub const BUTTON_SQUARE_INDIGO: Self = Self(1121);
    pub const BUTTON_CIRCLE_INDIGO: Self = Self(1122);
    pub const BUTTON_STAR_INDIGO: Self = Self(1123);
    pub const BUTTON_SQUARE_WHITE: Self = Self(1124);
    pub const BUTTON_CIRCLE_WHITE: Self = Self(1125);
    pub const BUTTON_STAR_WHITE: Self = Self(1126);
    pub const TELEPORTER_RED: Self = Self(1200);
    pub const TELEPORTER_YELLOW: Self = Self(1201);
    pub const TELEPORTER_GREEN: Self = Self(1202);
    pub const TELEPORTER_INDIGO: Self = Self(1203);
    pub const TELEPORTER_PURPLE: Self = Self(1204);
    pub const TELEPORTER_BLUE: Self = Self(1205);
    pub const TELEPORTER_WHITE: Self = Self(1206);
    pub const TELEPORTER_BLACK: Self = Self(1207);
    pub const SIGN_1: Self = Self(1300);
    pub const SIGN_2: Self = Self(1301);
    pub const SIGN_3: Self = Self(1302);
    pub const SIGN_4: Self = Self(1303);
    pub const SIGN_5: Self = Self(1304);
    pub const SIGN_6: Self = Self(1305);
    pub const SIGN_7: Self = Self(1306);
    pub const SIGN_8: Self = Self(1307);
    pub const SIGN_9: Self = Self(1308);
    pub const SIGN_10: Self = Self(1309);
    pub const SIGN_11: Self = Self(1310);
    pub const SIGN_12: Self = Self(1311);
    pub const SIGN_13: Self = Self(1312);
    pub const SIGN_14: Self = Self(1313);
    pub const SIGN_15: Self = Self(1314);
    pub const SIGN_16: Self = Self(1315);
    pub const SIGN_17: Self = Self(1316);
    pub const SIGN_18: Self = Self(1317);
    pub const SIGN_19: Self = Self(1318);
    pub const SIGN_20: Self = Self(1319);
    pub const SIGN_FAKE: Self = Self(1320);
    pub const SPIKES_SPEED_1: Self = Self(1400);
    pub const SPIKES_SPEED_2: Self = Self(1401);
    pub const SPIKES_SPEED_3: Self = Self(1402);
    pub const SPIKES_SPEED_4: Self = Self(1403);
    pub const WALL_HEIGHT_1_FAKE: Self = Self(1500);
    pub const WALL_HEIGHT_1_5_FAKE: Self = Self(1501);
    pub const WALL_HEIGHT_2_FAKE: Self = Self(1502);
    pub const FACTORY_BOX_WOOD: Self = Self(1600);
    pub const FACTORY_BOX_STEEL: Self = Self(1601);
    pub const FACTORY_REFLECTOR_NORTHWEST: Self = Self(1602);
    pub const FACTORY_REFLECTOR_NORTHEAST: Self = Self(1603);
    pub const FACTORY_BOULDER: Self = Self(1604);
    pub const FACTORY_BOX_PLASMA: Self = Self(1605);
    pub const FACTORY_POWDER_KEG: Self = Self(1606);
    pub const FACTORY_PRISM: Self = Self(1607);
    pub const FACTORY_STICKY_CUBE: Self = Self(1608);
    pub const FACTORY_LINK_SPHERE_RED: Self = Self(1609);
    pub const FACTORY_LINK_SPHERE_YELLOW: Self = Self(1610);
    pub const FACTORY_LINK_SPHERE_GREEN: Self = Self(1611);
    pub const FACTORY_LINK_SPHERE_BLUE: Self = Self(1612);
    pub const FACTORY_GLITCHY: Self = Self(1613);
    pub const TRANSPORTER_WATER: Self = Self(1700);
    pub const TRANSPORTER_EMPTY: Self = Self(1701);
    pub const TRANSPORTER_LAVA: Self = Self(1702);
    pub const TRANSPORTER_TRAMPOLINE_UNKNOWN: Self = Self(1703);
    pub const TRANSPORTER_TRAMPOLINE_3: Self = Self(1790);
    pub const TRANSPORTER_TRAMPOLINE_2: Self = Self(1791);
    pub const TRANSPORTER_TRAMPOLINE_0: Self = Self(1799);
    pub const TRAMPOLINE: Self = Self(1800);
    pub const TRAMPOLINE_INVISIBLE: Self = Self(1801);
    pub const SCOUGE_GREEN_NORTH_SPEED_1: Self = Self(1900);
    pub const SCOUGE_GREEN_NORTH_SPEED_2: Self = Self(1901);
    pub const SCOUGE_GREEN_NORTH_SPEED_3: Self = Self(1902);
    pub const SCOUGE_GREEN_NORTH_SPEED_4: Self = Self(1903);
    pub const SCOUGE_GREEN_EAST_SPEED_1: Self = Self(1904);
    pub const SCOUGE_GREEN_EAST_SPEED_2: Self = Self(1905);
    pub const SCOUGE_GREEN_EAST_SPEED_3: Self = Self(1906);
    pub const SCOUGE_GREEN_EAST_SPEED_4: Self = Self(1907);
    pub const SCOUGE_GREEN_SOUTH_SPEED_1: Self = Self(1908);
    pub const SCOUGE_GREEN_SOUTH_SPEED_2: Self = Self(1909);
    pub const SCOUGE_GREEN_SOUTH_SPEED_3: Self = Self(1910);
    pub const SCOUGE_GREEN_SOUTH_SPEED_4: Self = Self(1911);
    pub const SCOUGE_GREEN_WEST_SPEED_1: Self = Self(1912);
    pub const SCOUGE_GREEN_WEST_SPEED_2: Self = Self(1913);
    pub const SCOUGE_GREEN_WEST_SPEED_3: Self = Self(1914);
    pub const SCOUGE_GREEN_WEST_SPEED_4: Self = Self(1915);
    pub const SCOUGE_GREEN_NORTH_INERT: Self = Self(1916);
    pub const STICKY_CUBE: Self = Self(2000);
    pub const LINK_SPHERE_RED: Self = Self(2100);
    pub const LINK_SPHERE_YELLOW: Self = Self(2101);
    pub const LINK_SPHERE_GREEN: Self = Self(2102);
    pub const LINK_SPHERE_BLUE: Self = Self(2103);
    pub const THIRTY_EIGHT_OH_EIGHT: Self = Self(2104);
    pub const WARP_GATE_COILY: Self = Self(2200);
    pub const WARP_GATE_ZBOT_LEFT: Self = Self(2201);
    pub const WARP_GATE_ZBOT_RIGHT: Self = Self(2202);
    pub const WARP_GATE_KABOOM_LEFT: Self = Self(2203);
    pub const WARP_GATE_KABOOM_RIGHT: Self = Self(2204);
    pub const WARP_GATE_UFO_LEFT: Self = Self(2205);
    pub const WARP_GATE_UFO_RIGHT: Self = Self(2206);
    pub const WARP_GATE_CHOMPER_RED: Self = Self(2207);
    pub const WARP_GATE_CHOMPER_YELLOW: Self = Self(2208);
    pub const WARP_GATE_GHOST: Self = Self(2209);
    pub const WARP_GATE_RAINBOW_SPIRIT: Self = Self(2210);
    pub const WARP_GATE_ZBOT_BROKEN: Self = Self(2211);
    pub const WARP_GATE_SHADOW_STINKY_BLUE: Self = Self(2212);
    pub const WARP_GATE_SHADOW_LOOF_BLUE: Self = Self(2213);
    pub const WARP_GATE_SHADOW_QOOKIE_BLUE: Self = Self(2215);
    pub const WARP_GATE_SHADOW_PEEGUE_BLUE: Self = Self(2216);
    pub const WARP_GATE_SHADOW_STINKY_RED: Self = Self(2217);
    pub const WARP_GATE_SHADOW_LOOF_RED: Self = Self(2218);
    pub const WARP_GATE_SHADOW_QOOKIE_RED: Self = Self(2219);
    pub const WARP_GATE_SHADOW_PEEGUE_RED: Self = Self(2220);
    pub const MODE_3D: Self = Self(2300);
    pub const SHADOW_STINKY_BLUE: Self = Self(2400);
    pub const SHADOW_LOOF_BLUE: Self = Self(2401);
    pub const SHADOW_QOOKIE_BLUE: Self = Self(2402);
    pub const SHADOW_PEEGUE_BLUE: Self = Self(2403);
    pub const SHADOW_STINKY_RED: Self = Self(2404);
    pub const SHADOW_LOOF_RED: Self = Self(2405);
    pub const SHADOW_QOOKIE_RED: Self = Self(2406);
    pub const SHADOW_PEEGUE_RED: Self = Self(2407);

    pub fn break_tile_empty(steps: i32) -> Self {
        assert!(steps > 0 && steps <= 94, "steps out of range");
        Self(605 + steps)
    }

    pub fn teleporter_with_color(color_index: i32) -> Self {
        assert!(color_index >= 0 && color_index < 100, "color index out of range");
        Self(1200 + color_index)
    }

    pub fn sign(number: i32) -> Self {
        assert!(number > 0 && number <= 100, "sign number out of range");
        Self(1300 + number)
    }

    pub fn spikes_with_speed(speed: i32) -> Self {
        assert!(speed > 0 && speed <= 100, "spike speed {} out of range", speed);
        Self(1399 + speed)
    }

    pub fn is_immediately_enterable(&self) -> bool {
        self.is_floor_walkable() ||
        self.is_ice() ||
        self.is_conveyor() ||
        self.is_break_tile() ||
        self.is_electro() ||
        self.is_button() ||
        self.is_teleporter() ||
        self.is_sign() ||
        self.is_spikes() ||
        self.is_wall_fake() ||
        self.is_transporter() ||
        self.is_trampoline() ||
        self.is_mode_3d()
    }

    pub fn is_liquid(&self) -> bool {
        self.is_water() ||
        self.is_lava()
    }

    pub fn is_wall(&self) -> bool {
        self.is_wall_solid() ||
        self.is_wall_fake()
    }

    pub fn is_scouge(&self) -> bool {
        self.is_scouge_purple() ||
        self.is_scouge_green()
    }

    pub fn is_floor_walkable(&self) -> bool {
        self.0 >= 100 && self.0 < 200 && self.0 != 104
    }

    pub fn is_wall_solid(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    pub fn is_water(&self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    pub fn is_ice(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    pub fn is_conveyor(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }

    pub fn is_break_tile(&self) -> bool {
        self.0 >= 600 && self.0 < 700
    }

    pub fn is_electro(&self) -> bool {
        self.0 >= 700 && self.0 < 800
    }

    pub fn is_scouge_purple(&self) -> bool {
        self.0 >= 800 && self.0 < 900
    }

    pub fn is_lava(&self) -> bool {
        self.0 >= 900 && self.0 < 1000
    }

    pub fn is_gate(&self) -> bool {
        self.0 >= 1000 && self.0 < 1100
    }

    pub fn is_tollgate(&self) -> bool {
        self.0 == 1000 || self.0 == 1001
    }

    pub fn is_color_gate(&self) -> bool {
        self.0 >= 1002 && self.0 < 1100
    }

    pub fn is_button(&self) -> bool {
        self.0 >= 1100 && self.0 < 1200
    }

    pub fn is_teleporter(&self) -> bool {
        self.0 >= 1200 && self.0 < 1300
    }

    pub fn is_sign(&self) -> bool {
        self.0 >= 1300 && self.0 < 1400
    }

    pub fn is_spikes(&self) -> bool {
        self.0 >= 1400 && self.0 < 1500
    }

    pub fn is_wall_fake(&self) -> bool {
        self.0 >= 1500 && self.0 < 1600
    }

    pub fn is_factory(&self) -> bool {
        self.0 >= 1600 && self.0 < 1700
    }

    pub fn is_transporter(&self) -> bool {
        self.0 >= 1700 && self.0 < 1800
    }

    pub fn is_trampoline(&self) -> bool {
        self.0 >= 1800 && self.0 < 1900
    }

    pub fn is_scouge_green(&self) -> bool {
        self.0 >= 1900 && self.0 < 2000
    }

    pub fn is_sticky_cube(&self) -> bool {
        self.0 >= 2000 && self.0 < 2100
    }

    pub fn is_link_sphere(&self) -> bool {
        self.0 >= 2100 && self.0 < 2200
    }

    pub fn is_warp_gate(&self) -> bool {
        self.0 >= 2200 && self.0 < 2300
    }

    pub fn is_mode_3d(&self) -> bool {
        self.0 >= 2300 && self.0 < 2400
    }

    pub fn is_shadow_stinker(&self) -> bool {
        self.0 >= 2400 && self.0 < 2500
    }

    pub fn from_lev_to_lv6(&mut self) {
        // Skip conveyor 2 buttons.
        if self.is_button() && self.0 > 1102 {
            self.0 += 3;
        }
    }
}

impl ReplacementPattern<Tile> for Tile
{
    fn satisfied_by(&self, other: &Tile) -> bool {
        self == other
    }
}