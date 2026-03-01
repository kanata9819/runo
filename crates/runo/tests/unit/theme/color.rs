
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

#[test]
fn gray_and_blue_scales_cover_all_steps() {
    let grays = [
        Gray::gray_50(),
        Gray::gray_100(),
        Gray::gray_200(),
        Gray::gray_300(),
        Gray::gray_400(),
        Gray::gray_500(),
        Gray::gray_600(),
        Gray::gray_700(),
        Gray::gray_800(),
        Gray::gray_900(),
    ];
    let gray_consts = [
        GRAY_50, GRAY_100, GRAY_200, GRAY_300, GRAY_400, GRAY_500, GRAY_600, GRAY_700, GRAY_800,
        GRAY_900,
    ];
    for (actual, expected) in grays.into_iter().zip(gray_consts) {
        assert_eq!(format!("{actual:?}"), format!("{:?}", rgb(expected)));
    }

    let blues = [
        Blue::blue_50(),
        Blue::blue_100(),
        Blue::blue_200(),
        Blue::blue_300(),
        Blue::blue_400(),
        Blue::blue_500(),
        Blue::blue_600(),
        Blue::blue_700(),
        Blue::blue_800(),
        Blue::blue_900(),
    ];
    let blue_consts = [
        BLUE_50, BLUE_100, BLUE_200, BLUE_300, BLUE_400, BLUE_500, BLUE_600, BLUE_700, BLUE_800,
        BLUE_900,
    ];
    for (actual, expected) in blues.into_iter().zip(blue_consts) {
        assert_eq!(format!("{actual:?}"), format!("{:?}", rgb(expected)));
    }
}

#[test]
fn semantic_and_palette_helpers_return_expected_values() {
    let semantic = [
        Semantic::app_bg(),
        Semantic::panel_bg(),
        Semantic::panel_active(),
        Semantic::panel_border(),
        Semantic::text_primary(),
        Semantic::text_secondary(),
        Semantic::text_muted(),
        Semantic::success(),
        Semantic::warning(),
        Semantic::danger(),
        Semantic::info(),
    ];
    let semantic_consts = [
        APP_BG,
        PANEL_BG,
        PANEL_BG_ACTIVE,
        PANEL_BORDER,
        TEXT_PRIMARY,
        TEXT_SECONDARY,
        TEXT_MUTED,
        SUCCESS_500,
        WARNING_500,
        DANGER_500,
        INFO_500,
    ];
    for (actual, expected) in semantic.into_iter().zip(semantic_consts) {
        assert_eq!(format!("{actual:?}"), format!("{:?}", rgb(expected)));
    }

    let neutral = [
        Neutral::tone_36_42_50(),
        Neutral::tone_43_47_53(),
        Neutral::tone_45_49_55(),
        Neutral::tone_46_64_86(),
        Neutral::tone_48_52_58(),
        Neutral::tone_56_63_74(),
        Neutral::tone_63_80_102(),
        Neutral::tone_78_82_90(),
        Neutral::tone_78_89_104(),
        Neutral::tone_83_90_100(),
        Neutral::tone_86_92_101(),
        Neutral::tone_88_94_102(),
        Neutral::tone_130_145_163(),
        Neutral::tone_141_147_154(),
        Neutral::tone_142_148_156(),
        Neutral::tone_142_151_163(),
        Neutral::tone_146_152_160(),
        Neutral::tone_147_153_161(),
        Neutral::tone_163_169_177(),
        Neutral::tone_167_173_181(),
        Neutral::tone_178_184_192(),
    ];
    assert_eq!(neutral.len(), 21);

    let accent = [
        AccentBlue::tone_31_122_205(),
        AccentBlue::tone_37_132_214(),
        AccentBlue::tone_45_129_205(),
        AccentBlue::tone_50_144_229(),
        AccentBlue::tone_53_141_221(),
        AccentBlue::tone_62_154_234(),
        AccentBlue::tone_69_160_242(),
        AccentBlue::tone_89_176_255(),
        AccentBlue::tone_124_177_230(),
    ];
    assert_eq!(accent.len(), 9);

    let soft_white = [
        SoftWhite::tone_186_196_210(),
        SoftWhite::tone_220_228_240(),
        SoftWhite::tone_240_246_255(),
    ];
    assert_eq!(soft_white.len(), 3);

    let white_alpha = [
        WhiteAlpha::tone_255_255_255_20(),
        WhiteAlpha::tone_255_255_255_35(),
        WhiteAlpha::tone_255_255_255_90(),
        WhiteAlpha::tone_255_255_255_150(),
    ];
    let expected_alpha = [
        rgba((255, 255, 255, 20)),
        rgba((255, 255, 255, 35)),
        rgba((255, 255, 255, 90)),
        rgba((255, 255, 255, 150)),
    ];
    for (actual, expected) in white_alpha.into_iter().zip(expected_alpha) {
        assert_eq!(format!("{actual:?}"), format!("{expected:?}"));
    }
}
