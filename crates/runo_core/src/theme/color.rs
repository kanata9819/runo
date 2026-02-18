use crate::Color;

pub type Rgb = (u8, u8, u8);
pub type Rgba = (u8, u8, u8, u8);

pub const WHITE: Rgb = (255, 255, 255);
pub const BLACK: Rgb = (0, 0, 0);

pub const GRAY_50: Rgb = (245, 247, 250);
pub const GRAY_100: Rgb = (236, 240, 244);
pub const GRAY_200: Rgb = (215, 221, 229);
pub const GRAY_300: Rgb = (191, 200, 211);
pub const GRAY_400: Rgb = (160, 171, 185);
pub const GRAY_500: Rgb = (127, 139, 155);
pub const GRAY_600: Rgb = (98, 110, 127);
pub const GRAY_700: Rgb = (72, 84, 101);
pub const GRAY_800: Rgb = (47, 57, 71);
pub const GRAY_900: Rgb = (24, 30, 39);

pub const BLUE_50: Rgb = (239, 246, 255);
pub const BLUE_100: Rgb = (219, 234, 254);
pub const BLUE_200: Rgb = (191, 219, 254);
pub const BLUE_300: Rgb = (147, 197, 253);
pub const BLUE_400: Rgb = (96, 165, 250);
pub const BLUE_500: Rgb = (59, 130, 246);
pub const BLUE_600: Rgb = (37, 99, 235);
pub const BLUE_700: Rgb = (29, 78, 216);
pub const BLUE_800: Rgb = (30, 64, 175);
pub const BLUE_900: Rgb = (30, 58, 138);

pub const APP_BG: Rgb = (18, 20, 23);
pub const PANEL_BG: Rgb = (29, 34, 41);
pub const PANEL_BG_ACTIVE: Rgb = (21, 49, 66);
pub const PANEL_BORDER: Rgb = (70, 80, 95);

pub const TEXT_PRIMARY: Rgb = (236, 241, 247);
pub const TEXT_SECONDARY: Rgb = (174, 185, 198);
pub const TEXT_MUTED: Rgb = (129, 142, 158);
pub const TEXT_INVERSE: Rgb = (20, 24, 30);

pub const PRIMARY_100: Rgb = (215, 236, 255);
pub const PRIMARY_300: Rgb = (114, 182, 255);
pub const PRIMARY_500: Rgb = (45, 144, 255);
pub const PRIMARY_700: Rgb = (27, 101, 201);
pub const PRIMARY_900: Rgb = (18, 62, 130);

pub const SUCCESS_100: Rgb = (220, 248, 231);
pub const SUCCESS_300: Rgb = (120, 216, 153);
pub const SUCCESS_500: Rgb = (42, 173, 96);
pub const SUCCESS_700: Rgb = (25, 124, 67);

pub const WARNING_100: Rgb = (255, 244, 215);
pub const WARNING_300: Rgb = (255, 210, 118);
pub const WARNING_500: Rgb = (240, 164, 45);
pub const WARNING_700: Rgb = (181, 112, 16);

pub const DANGER_100: Rgb = (255, 225, 225);
pub const DANGER_300: Rgb = (255, 145, 145);
pub const DANGER_500: Rgb = (226, 78, 78);
pub const DANGER_700: Rgb = (164, 43, 43);

pub const INFO_100: Rgb = (222, 241, 255);
pub const INFO_300: Rgb = (132, 199, 255);
pub const INFO_500: Rgb = (54, 154, 235);
pub const INFO_700: Rgb = (30, 104, 173);

macro_rules! color_group {
    ($group:ident { $($fn_name:ident => $const_name:ident),+ $(,)? }) => {
        pub struct $group;

        impl $group {
            $(
                pub fn $fn_name() -> Color {
                    rgb($const_name)
                }
            )+
        }
    };
}

color_group!(Gray {
    gray_50 => GRAY_50,
    gray_100 => GRAY_100,
    gray_200 => GRAY_200,
    gray_300 => GRAY_300,
    gray_400 => GRAY_400,
    gray_500 => GRAY_500,
    gray_600 => GRAY_600,
    gray_700 => GRAY_700,
    gray_800 => GRAY_800,
    gray_900 => GRAY_900,
});

color_group!(Blue {
    blue_50 => BLUE_50,
    blue_100 => BLUE_100,
    blue_200 => BLUE_200,
    blue_300 => BLUE_300,
    blue_400 => BLUE_400,
    blue_500 => BLUE_500,
    blue_600 => BLUE_600,
    blue_700 => BLUE_700,
    blue_800 => BLUE_800,
    blue_900 => BLUE_900,
});

pub struct Semantic;

impl Semantic {
    pub fn app_bg() -> Color {
        rgb(APP_BG)
    }
    pub fn panel_bg() -> Color {
        rgb(PANEL_BG)
    }
    pub fn panel_active() -> Color {
        rgb(PANEL_BG_ACTIVE)
    }
    pub fn panel_border() -> Color {
        rgb(PANEL_BORDER)
    }
    pub fn text_primary() -> Color {
        rgb(TEXT_PRIMARY)
    }
    pub fn text_secondary() -> Color {
        rgb(TEXT_SECONDARY)
    }
    pub fn text_muted() -> Color {
        rgb(TEXT_MUTED)
    }
    pub fn success() -> Color {
        rgb(SUCCESS_500)
    }
    pub fn warning() -> Color {
        rgb(WARNING_500)
    }
    pub fn danger() -> Color {
        rgb(DANGER_500)
    }
    pub fn info() -> Color {
        rgb(INFO_500)
    }
}

pub struct Neutral;

impl Neutral {
    pub fn tone_36_42_50() -> Color {
        rgb((36, 42, 50))
    }
    pub fn tone_43_47_53() -> Color {
        rgb((43, 47, 53))
    }
    pub fn tone_45_49_55() -> Color {
        rgb((45, 49, 55))
    }
    pub fn tone_46_64_86() -> Color {
        rgb((46, 64, 86))
    }
    pub fn tone_48_52_58() -> Color {
        rgb((48, 52, 58))
    }
    pub fn tone_56_63_74() -> Color {
        rgb((56, 63, 74))
    }
    pub fn tone_63_80_102() -> Color {
        rgb((63, 80, 102))
    }
    pub fn tone_78_82_90() -> Color {
        rgb((78, 82, 90))
    }
    pub fn tone_78_89_104() -> Color {
        rgb((78, 89, 104))
    }
    pub fn tone_83_90_100() -> Color {
        rgb((83, 90, 100))
    }
    pub fn tone_86_92_101() -> Color {
        rgb((86, 92, 101))
    }
    pub fn tone_88_94_102() -> Color {
        rgb((88, 94, 102))
    }
    pub fn tone_130_145_163() -> Color {
        rgb((130, 145, 163))
    }
    pub fn tone_141_147_154() -> Color {
        rgb((141, 147, 154))
    }
    pub fn tone_142_148_156() -> Color {
        rgb((142, 148, 156))
    }
    pub fn tone_142_151_163() -> Color {
        rgb((142, 151, 163))
    }
    pub fn tone_146_152_160() -> Color {
        rgb((146, 152, 160))
    }
    pub fn tone_147_153_161() -> Color {
        rgb((147, 153, 161))
    }
    pub fn tone_163_169_177() -> Color {
        rgb((163, 169, 177))
    }
    pub fn tone_167_173_181() -> Color {
        rgb((167, 173, 181))
    }
    pub fn tone_178_184_192() -> Color {
        rgb((178, 184, 192))
    }
}

pub struct AccentBlue;

impl AccentBlue {
    pub fn tone_31_122_205() -> Color {
        rgb((31, 122, 205))
    }
    pub fn tone_37_132_214() -> Color {
        rgb((37, 132, 214))
    }
    pub fn tone_45_129_205() -> Color {
        rgb((45, 129, 205))
    }
    pub fn tone_50_144_229() -> Color {
        rgb((50, 144, 229))
    }
    pub fn tone_53_141_221() -> Color {
        rgb((53, 141, 221))
    }
    pub fn tone_62_154_234() -> Color {
        rgb((62, 154, 234))
    }
    pub fn tone_69_160_242() -> Color {
        rgb((69, 160, 242))
    }
    pub fn tone_89_176_255() -> Color {
        rgb((89, 176, 255))
    }
    pub fn tone_124_177_230() -> Color {
        rgb((124, 177, 230))
    }
}

pub struct SoftWhite;

impl SoftWhite {
    pub fn tone_186_196_210() -> Color {
        rgb((186, 196, 210))
    }
    pub fn tone_220_228_240() -> Color {
        rgb((220, 228, 240))
    }
    pub fn tone_240_246_255() -> Color {
        rgb((240, 246, 255))
    }
}

pub struct WhiteAlpha;

impl WhiteAlpha {
    pub fn tone_255_255_255_20() -> Color {
        rgba((255, 255, 255, 20))
    }
    pub fn tone_255_255_255_35() -> Color {
        rgba((255, 255, 255, 35))
    }
    pub fn tone_255_255_255_90() -> Color {
        rgba((255, 255, 255, 90))
    }
    pub fn tone_255_255_255_150() -> Color {
        rgba((255, 255, 255, 150))
    }
}

pub fn rgb(rgb: Rgb) -> Color {
    Color::from_rgb8(rgb.0, rgb.1, rgb.2)
}

pub fn rgba(rgba: Rgba) -> Color {
    Color::from_rgba8(rgba.0, rgba.1, rgba.2, rgba.3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_uses_exact_components() {
        let a = rgb((12, 34, 56));
        let b = Color::from_rgb8(12, 34, 56);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    #[test]
    fn rgba_uses_exact_components() {
        let a = rgba((12, 34, 56, 78));
        let b = Color::from_rgba8(12, 34, 56, 78);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    #[test]
    fn group_api_matches_constants() {
        let gray = Gray::gray_50();
        let expected_gray = rgb(GRAY_50);
        assert_eq!(format!("{gray:?}"), format!("{expected_gray:?}"));

        let blue = Blue::blue_500();
        let expected_blue = rgb(BLUE_500);
        assert_eq!(format!("{blue:?}"), format!("{expected_blue:?}"));
    }

    #[test]
    fn semantic_api_matches_constants() {
        let success = Semantic::success();
        let expected_success = rgb(SUCCESS_500);
        assert_eq!(format!("{success:?}"), format!("{expected_success:?}"));
    }
}
