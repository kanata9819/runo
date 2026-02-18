use crate::Color;

pub type Rgb = (u8, u8, u8);
pub type Rgba = (u8, u8, u8, u8);

pub mod widget {
    use super::{Rgb, Rgba};

    pub const BUTTON_DISABLED_BG: Rgb = (83, 90, 100);
    pub const BUTTON_PRESSED_BG: Rgb = (31, 122, 205);
    pub const BUTTON_HOVER_BG: Rgb = (69, 160, 242);
    pub const BUTTON_ENABLED_BG: Rgb = (50, 144, 229);
    pub const BUTTON_DISABLED_TEXT: Rgb = (178, 184, 192);

    pub const LABEL_DISABLED_TEXT: Rgb = (142, 148, 156);

    pub const CHECKBOX_DISABLED_BG: Rgb = (43, 47, 53);
    pub const CHECKBOX_PRESSED_BG: Rgb = (45, 129, 205);
    pub const CHECKBOX_HOVER_BG: Rgb = (53, 141, 221);
    pub const CHECKBOX_CHECKED_BG: Rgb = (50, 144, 229);
    pub const CHECKBOX_UNCHECKED_BG: Rgb = (36, 42, 50);
    pub const CHECKBOX_BORDER_ENABLED: Rgb = (130, 145, 163);
    pub const CHECKBOX_BORDER_DISABLED: Rgb = (88, 94, 102);
    pub const CHECKBOX_MARK_ENABLED: Rgb = (240, 246, 255);
    pub const CHECKBOX_MARK_DISABLED: Rgb = (167, 173, 181);
    pub const CHECKBOX_TEXT_DISABLED: Rgb = (146, 152, 160);

    pub const RADIO_DISABLED_BG: Rgb = (43, 47, 53);
    pub const RADIO_PRESSED_BG: Rgb = (45, 129, 205);
    pub const RADIO_HOVER_BG: Rgb = (53, 141, 221);
    pub const RADIO_ENABLED_BG: Rgb = (36, 42, 50);
    pub const RADIO_BORDER_ENABLED: Rgb = (130, 145, 163);
    pub const RADIO_BORDER_DISABLED: Rgb = (88, 94, 102);
    pub const RADIO_MARK_ENABLED: Rgb = (240, 246, 255);
    pub const RADIO_MARK_DISABLED: Rgb = (167, 173, 181);
    pub const RADIO_TEXT_DISABLED: Rgb = (146, 152, 160);

    pub const COMBO_DISABLED_BORDER: Rgb = (86, 92, 101);
    pub const COMBO_PRESSED_BORDER: Rgb = (89, 176, 255);
    pub const COMBO_HOVER_BORDER: Rgb = (124, 177, 230);
    pub const COMBO_DISABLED_TEXT: Rgb = (147, 153, 161);
    pub const COMBO_DISABLED_BG: Rgb = (45, 49, 55);
    pub const COMBO_ARROW_ENABLED: Rgb = (186, 196, 210);
    pub const COMBO_ARROW_DISABLED: Rgb = (141, 147, 154);
    pub const COMBO_ITEM_HOVER_BG: Rgb = (63, 80, 102);
    pub const COMBO_ITEM_SELECTED_BG: Rgb = (46, 64, 86);

    pub const SLIDER_TRACK_ENABLED: Rgb = (56, 63, 74);
    pub const SLIDER_TRACK_DISABLED: Rgb = (48, 52, 58);
    pub const SLIDER_ACTIVE_DISABLED: Rgb = (78, 82, 90);
    pub const SLIDER_ACTIVE_PRESSED: Rgb = (37, 132, 214);
    pub const SLIDER_ACTIVE_HOVER: Rgb = (62, 154, 234);
    pub const SLIDER_ACTIVE_ENABLED: Rgb = (50, 144, 229);
    pub const SLIDER_THUMB_ENABLED: Rgb = (240, 246, 255);
    pub const SLIDER_THUMB_DISABLED: Rgb = (163, 169, 177);
    pub const SLIDER_THUMB_BORDER: Rgb = (78, 89, 104);
    pub const SLIDER_TEXT_DISABLED: Rgb = (146, 152, 160);

    pub const TEXT_BOX_DISABLED_BG: Rgb = (45, 49, 55);
    pub const TEXT_BOX_DISABLED_BORDER: Rgb = (86, 92, 101);
    pub const TEXT_BOX_FOCUSED_BORDER: Rgb = (89, 176, 255);
    pub const TEXT_BOX_DISABLED_TEXT: Rgb = (147, 153, 161);
    pub const TEXT_BOX_PLACEHOLDER_TEXT: Rgb = (142, 151, 163);
    pub const TEXT_BOX_CARET: Rgb = (220, 228, 240);
    pub const TEXT_BOX_SCROLLBAR_TRACK_ENABLED: Rgba = (255, 255, 255, 35);
    pub const TEXT_BOX_SCROLLBAR_TRACK_DISABLED: Rgba = (255, 255, 255, 20);
    pub const TEXT_BOX_SCROLLBAR_THUMB_ENABLED: Rgba = (255, 255, 255, 150);
    pub const TEXT_BOX_SCROLLBAR_THUMB_DISABLED: Rgba = (255, 255, 255, 90);
}

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

pub const PURPLE_300: Rgb = (192, 166, 255);
pub const PURPLE_500: Rgb = (141, 103, 238);
pub const PINK_300: Rgb = (247, 156, 212);
pub const PINK_500: Rgb = (227, 94, 175);
pub const TEAL_300: Rgb = (127, 226, 214);
pub const TEAL_500: Rgb = (44, 184, 167);
pub const ORANGE_300: Rgb = (255, 184, 125);
pub const ORANGE_500: Rgb = (236, 132, 44);

pub const SLATE_50: Rgb = (248, 250, 252);
pub const SLATE_100: Rgb = (241, 245, 249);
pub const SLATE_200: Rgb = (226, 232, 240);
pub const SLATE_300: Rgb = (203, 213, 225);
pub const SLATE_400: Rgb = (148, 163, 184);
pub const SLATE_500: Rgb = (100, 116, 139);
pub const SLATE_600: Rgb = (71, 85, 105);
pub const SLATE_700: Rgb = (51, 65, 85);
pub const SLATE_800: Rgb = (30, 41, 59);
pub const SLATE_900: Rgb = (15, 23, 42);

pub const ZINC_50: Rgb = (250, 250, 250);
pub const ZINC_100: Rgb = (244, 244, 245);
pub const ZINC_200: Rgb = (228, 228, 231);
pub const ZINC_300: Rgb = (212, 212, 216);
pub const ZINC_400: Rgb = (161, 161, 170);
pub const ZINC_500: Rgb = (113, 113, 122);
pub const ZINC_600: Rgb = (82, 82, 91);
pub const ZINC_700: Rgb = (63, 63, 70);
pub const ZINC_800: Rgb = (39, 39, 42);
pub const ZINC_900: Rgb = (24, 24, 27);

pub const RED_50: Rgb = (254, 242, 242);
pub const RED_100: Rgb = (254, 226, 226);
pub const RED_200: Rgb = (254, 202, 202);
pub const RED_300: Rgb = (252, 165, 165);
pub const RED_400: Rgb = (248, 113, 113);
pub const RED_500: Rgb = (239, 68, 68);
pub const RED_600: Rgb = (220, 38, 38);
pub const RED_700: Rgb = (185, 28, 28);
pub const RED_800: Rgb = (153, 27, 27);
pub const RED_900: Rgb = (127, 29, 29);

pub const ORANGE_50: Rgb = (255, 247, 237);
pub const ORANGE_100: Rgb = (255, 237, 213);
pub const ORANGE_200: Rgb = (254, 215, 170);
pub const ORANGE_400: Rgb = (251, 146, 60);
pub const ORANGE_600: Rgb = (234, 88, 12);
pub const ORANGE_700: Rgb = (194, 65, 12);
pub const ORANGE_800: Rgb = (154, 52, 18);
pub const ORANGE_900: Rgb = (124, 45, 18);

pub const AMBER_50: Rgb = (255, 251, 235);
pub const AMBER_100: Rgb = (254, 243, 199);
pub const AMBER_200: Rgb = (253, 230, 138);
pub const AMBER_300: Rgb = (252, 211, 77);
pub const AMBER_400: Rgb = (251, 191, 36);
pub const AMBER_500: Rgb = (245, 158, 11);
pub const AMBER_600: Rgb = (217, 119, 6);
pub const AMBER_700: Rgb = (180, 83, 9);
pub const AMBER_800: Rgb = (146, 64, 14);
pub const AMBER_900: Rgb = (120, 53, 15);

pub const YELLOW_50: Rgb = (254, 252, 232);
pub const YELLOW_100: Rgb = (254, 249, 195);
pub const YELLOW_200: Rgb = (254, 240, 138);
pub const YELLOW_300: Rgb = (253, 224, 71);
pub const YELLOW_400: Rgb = (250, 204, 21);
pub const YELLOW_500: Rgb = (234, 179, 8);
pub const YELLOW_600: Rgb = (202, 138, 4);
pub const YELLOW_700: Rgb = (161, 98, 7);
pub const YELLOW_800: Rgb = (133, 77, 14);
pub const YELLOW_900: Rgb = (113, 63, 18);

pub const LIME_50: Rgb = (247, 254, 231);
pub const LIME_100: Rgb = (236, 252, 203);
pub const LIME_200: Rgb = (217, 249, 157);
pub const LIME_300: Rgb = (190, 242, 100);
pub const LIME_400: Rgb = (163, 230, 53);
pub const LIME_500: Rgb = (132, 204, 22);
pub const LIME_600: Rgb = (101, 163, 13);
pub const LIME_700: Rgb = (77, 124, 15);
pub const LIME_800: Rgb = (63, 98, 18);
pub const LIME_900: Rgb = (54, 83, 20);

pub const GREEN_50: Rgb = (240, 253, 244);
pub const GREEN_100: Rgb = (220, 252, 231);
pub const GREEN_200: Rgb = (187, 247, 208);
pub const GREEN_300: Rgb = (134, 239, 172);
pub const GREEN_400: Rgb = (74, 222, 128);
pub const GREEN_500: Rgb = (34, 197, 94);
pub const GREEN_600: Rgb = (22, 163, 74);
pub const GREEN_700: Rgb = (21, 128, 61);
pub const GREEN_800: Rgb = (22, 101, 52);
pub const GREEN_900: Rgb = (20, 83, 45);

pub const EMERALD_50: Rgb = (236, 253, 245);
pub const EMERALD_100: Rgb = (209, 250, 229);
pub const EMERALD_200: Rgb = (167, 243, 208);
pub const EMERALD_300: Rgb = (110, 231, 183);
pub const EMERALD_400: Rgb = (52, 211, 153);
pub const EMERALD_500: Rgb = (16, 185, 129);
pub const EMERALD_600: Rgb = (5, 150, 105);
pub const EMERALD_700: Rgb = (4, 120, 87);
pub const EMERALD_800: Rgb = (6, 95, 70);
pub const EMERALD_900: Rgb = (6, 78, 59);

pub const CYAN_50: Rgb = (236, 254, 255);
pub const CYAN_100: Rgb = (207, 250, 254);
pub const CYAN_200: Rgb = (165, 243, 252);
pub const CYAN_300: Rgb = (103, 232, 249);
pub const CYAN_400: Rgb = (34, 211, 238);
pub const CYAN_500: Rgb = (6, 182, 212);
pub const CYAN_600: Rgb = (8, 145, 178);
pub const CYAN_700: Rgb = (14, 116, 144);
pub const CYAN_800: Rgb = (21, 94, 117);
pub const CYAN_900: Rgb = (22, 78, 99);

pub const SKY_50: Rgb = (240, 249, 255);
pub const SKY_100: Rgb = (224, 242, 254);
pub const SKY_200: Rgb = (186, 230, 253);
pub const SKY_300: Rgb = (125, 211, 252);
pub const SKY_400: Rgb = (56, 189, 248);
pub const SKY_500: Rgb = (14, 165, 233);
pub const SKY_600: Rgb = (2, 132, 199);
pub const SKY_700: Rgb = (3, 105, 161);
pub const SKY_800: Rgb = (7, 89, 133);
pub const SKY_900: Rgb = (12, 74, 110);

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

pub const INDIGO_50: Rgb = (238, 242, 255);
pub const INDIGO_100: Rgb = (224, 231, 255);
pub const INDIGO_200: Rgb = (199, 210, 254);
pub const INDIGO_300: Rgb = (165, 180, 252);
pub const INDIGO_400: Rgb = (129, 140, 248);
pub const INDIGO_500: Rgb = (99, 102, 241);
pub const INDIGO_600: Rgb = (79, 70, 229);
pub const INDIGO_700: Rgb = (67, 56, 202);
pub const INDIGO_800: Rgb = (55, 48, 163);
pub const INDIGO_900: Rgb = (49, 46, 129);

pub const VIOLET_50: Rgb = (245, 243, 255);
pub const VIOLET_100: Rgb = (237, 233, 254);
pub const VIOLET_200: Rgb = (221, 214, 254);
pub const VIOLET_300: Rgb = (196, 181, 253);
pub const VIOLET_400: Rgb = (167, 139, 250);
pub const VIOLET_500: Rgb = (139, 92, 246);
pub const VIOLET_600: Rgb = (124, 58, 237);
pub const VIOLET_700: Rgb = (109, 40, 217);
pub const VIOLET_800: Rgb = (91, 33, 182);
pub const VIOLET_900: Rgb = (76, 29, 149);

pub const FUCHSIA_50: Rgb = (253, 244, 255);
pub const FUCHSIA_100: Rgb = (250, 232, 255);
pub const FUCHSIA_200: Rgb = (245, 208, 254);
pub const FUCHSIA_300: Rgb = (240, 171, 252);
pub const FUCHSIA_400: Rgb = (232, 121, 249);
pub const FUCHSIA_500: Rgb = (217, 70, 239);
pub const FUCHSIA_600: Rgb = (192, 38, 211);
pub const FUCHSIA_700: Rgb = (162, 28, 175);
pub const FUCHSIA_800: Rgb = (134, 25, 143);
pub const FUCHSIA_900: Rgb = (112, 26, 117);

pub const ROSE_50: Rgb = (255, 241, 242);
pub const ROSE_100: Rgb = (255, 228, 230);
pub const ROSE_200: Rgb = (254, 205, 211);
pub const ROSE_300: Rgb = (253, 164, 175);
pub const ROSE_400: Rgb = (251, 113, 133);
pub const ROSE_500: Rgb = (244, 63, 94);
pub const ROSE_600: Rgb = (225, 29, 72);
pub const ROSE_700: Rgb = (190, 18, 60);
pub const ROSE_800: Rgb = (159, 18, 57);
pub const ROSE_900: Rgb = (136, 19, 55);

pub const BROWN_50: Rgb = (250, 245, 239);
pub const BROWN_100: Rgb = (238, 225, 210);
pub const BROWN_200: Rgb = (221, 198, 171);
pub const BROWN_300: Rgb = (199, 166, 128);
pub const BROWN_400: Rgb = (176, 136, 94);
pub const BROWN_500: Rgb = (151, 108, 66);
pub const BROWN_600: Rgb = (128, 88, 52);
pub const BROWN_700: Rgb = (104, 70, 41);
pub const BROWN_800: Rgb = (81, 55, 34);
pub const BROWN_900: Rgb = (66, 44, 27);

pub const SURFACE_0: Rgb = (255, 255, 255);
pub const SURFACE_1: Rgb = (250, 250, 250);
pub const SURFACE_2: Rgb = (245, 245, 245);
pub const SURFACE_3: Rgb = (238, 238, 238);
pub const SURFACE_DARK_0: Rgb = (20, 22, 26);
pub const SURFACE_DARK_1: Rgb = (27, 30, 36);
pub const SURFACE_DARK_2: Rgb = (34, 39, 47);
pub const SURFACE_DARK_3: Rgb = (43, 49, 59);

pub const BORDER_LIGHT: Rgb = (220, 226, 234);
pub const BORDER_DEFAULT: Rgb = (166, 178, 194);
pub const BORDER_DARK: Rgb = (86, 98, 115);
pub const OVERLAY_LIGHT: Rgb = (245, 248, 252);
pub const OVERLAY_DARK: Rgb = (14, 17, 22);

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

color_group!(Slate {
    slate_50 => SLATE_50,
    slate_100 => SLATE_100,
    slate_200 => SLATE_200,
    slate_300 => SLATE_300,
    slate_400 => SLATE_400,
    slate_500 => SLATE_500,
    slate_600 => SLATE_600,
    slate_700 => SLATE_700,
    slate_800 => SLATE_800,
    slate_900 => SLATE_900,
});

color_group!(Zinc {
    zinc_50 => ZINC_50,
    zinc_100 => ZINC_100,
    zinc_200 => ZINC_200,
    zinc_300 => ZINC_300,
    zinc_400 => ZINC_400,
    zinc_500 => ZINC_500,
    zinc_600 => ZINC_600,
    zinc_700 => ZINC_700,
    zinc_800 => ZINC_800,
    zinc_900 => ZINC_900,
});

color_group!(Red {
    red_50 => RED_50,
    red_100 => RED_100,
    red_200 => RED_200,
    red_300 => RED_300,
    red_400 => RED_400,
    red_500 => RED_500,
    red_600 => RED_600,
    red_700 => RED_700,
    red_800 => RED_800,
    red_900 => RED_900,
});

color_group!(Orange {
    orange_50 => ORANGE_50,
    orange_100 => ORANGE_100,
    orange_200 => ORANGE_200,
    orange_300 => ORANGE_300,
    orange_400 => ORANGE_400,
    orange_500 => ORANGE_500,
    orange_600 => ORANGE_600,
    orange_700 => ORANGE_700,
    orange_800 => ORANGE_800,
    orange_900 => ORANGE_900,
});

color_group!(Yellow {
    yellow_50 => YELLOW_50,
    yellow_100 => YELLOW_100,
    yellow_200 => YELLOW_200,
    yellow_300 => YELLOW_300,
    yellow_400 => YELLOW_400,
    yellow_500 => YELLOW_500,
    yellow_600 => YELLOW_600,
    yellow_700 => YELLOW_700,
    yellow_800 => YELLOW_800,
    yellow_900 => YELLOW_900,
});

color_group!(Lime {
    lime_50 => LIME_50,
    lime_100 => LIME_100,
    lime_200 => LIME_200,
    lime_300 => LIME_300,
    lime_400 => LIME_400,
    lime_500 => LIME_500,
    lime_600 => LIME_600,
    lime_700 => LIME_700,
    lime_800 => LIME_800,
    lime_900 => LIME_900,
});

color_group!(Green {
    green_50 => GREEN_50,
    green_100 => GREEN_100,
    green_200 => GREEN_200,
    green_300 => GREEN_300,
    green_400 => GREEN_400,
    green_500 => GREEN_500,
    green_600 => GREEN_600,
    green_700 => GREEN_700,
    green_800 => GREEN_800,
    green_900 => GREEN_900,
});

color_group!(Emerald {
    emerald_50 => EMERALD_50,
    emerald_100 => EMERALD_100,
    emerald_200 => EMERALD_200,
    emerald_300 => EMERALD_300,
    emerald_400 => EMERALD_400,
    emerald_500 => EMERALD_500,
    emerald_600 => EMERALD_600,
    emerald_700 => EMERALD_700,
    emerald_800 => EMERALD_800,
    emerald_900 => EMERALD_900,
});

color_group!(Teal {
    teal_300 => TEAL_300,
    teal_500 => TEAL_500,
});

color_group!(Cyan {
    cyan_50 => CYAN_50,
    cyan_100 => CYAN_100,
    cyan_200 => CYAN_200,
    cyan_300 => CYAN_300,
    cyan_400 => CYAN_400,
    cyan_500 => CYAN_500,
    cyan_600 => CYAN_600,
    cyan_700 => CYAN_700,
    cyan_800 => CYAN_800,
    cyan_900 => CYAN_900,
});

color_group!(Sky {
    sky_50 => SKY_50,
    sky_100 => SKY_100,
    sky_200 => SKY_200,
    sky_300 => SKY_300,
    sky_400 => SKY_400,
    sky_500 => SKY_500,
    sky_600 => SKY_600,
    sky_700 => SKY_700,
    sky_800 => SKY_800,
    sky_900 => SKY_900,
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

color_group!(Indigo {
    indigo_50 => INDIGO_50,
    indigo_100 => INDIGO_100,
    indigo_200 => INDIGO_200,
    indigo_300 => INDIGO_300,
    indigo_400 => INDIGO_400,
    indigo_500 => INDIGO_500,
    indigo_600 => INDIGO_600,
    indigo_700 => INDIGO_700,
    indigo_800 => INDIGO_800,
    indigo_900 => INDIGO_900,
});

color_group!(Violet {
    violet_50 => VIOLET_50,
    violet_100 => VIOLET_100,
    violet_200 => VIOLET_200,
    violet_300 => VIOLET_300,
    violet_400 => VIOLET_400,
    violet_500 => VIOLET_500,
    violet_600 => VIOLET_600,
    violet_700 => VIOLET_700,
    violet_800 => VIOLET_800,
    violet_900 => VIOLET_900,
});

color_group!(Purple {
    purple_300 => PURPLE_300,
    purple_500 => PURPLE_500,
});

color_group!(Fuchsia {
    fuchsia_50 => FUCHSIA_50,
    fuchsia_100 => FUCHSIA_100,
    fuchsia_200 => FUCHSIA_200,
    fuchsia_300 => FUCHSIA_300,
    fuchsia_400 => FUCHSIA_400,
    fuchsia_500 => FUCHSIA_500,
    fuchsia_600 => FUCHSIA_600,
    fuchsia_700 => FUCHSIA_700,
    fuchsia_800 => FUCHSIA_800,
    fuchsia_900 => FUCHSIA_900,
});

color_group!(Pink {
    pink_300 => PINK_300,
    pink_500 => PINK_500,
});

color_group!(Rose {
    rose_50 => ROSE_50,
    rose_100 => ROSE_100,
    rose_200 => ROSE_200,
    rose_300 => ROSE_300,
    rose_400 => ROSE_400,
    rose_500 => ROSE_500,
    rose_600 => ROSE_600,
    rose_700 => ROSE_700,
    rose_800 => ROSE_800,
    rose_900 => ROSE_900,
});

color_group!(Brown {
    brown_50 => BROWN_50,
    brown_100 => BROWN_100,
    brown_200 => BROWN_200,
    brown_300 => BROWN_300,
    brown_400 => BROWN_400,
    brown_500 => BROWN_500,
    brown_600 => BROWN_600,
    brown_700 => BROWN_700,
    brown_800 => BROWN_800,
    brown_900 => BROWN_900,
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
    fn palette_constants_are_distinct_where_expected() {
        assert_ne!(APP_BG, PANEL_BG);
        assert_ne!(PANEL_BG, PANEL_BG_ACTIVE);
        assert_ne!(PANEL_BG, PANEL_BORDER);
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
