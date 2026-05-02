mod base;
mod dark;
mod default;
mod forest;
mod neo;
mod neo_dark;
mod neutral;
mod redux;
mod redux_color;
mod redux_dark;
mod redux_dark_color;
pub mod variables;

use crate::MermaidConfig;
use crate::theme::base::apply_base_theme_defaults;
use crate::theme::dark::apply_dark_theme_defaults;
use crate::theme::default::apply_default_theme_defaults;
use crate::theme::forest::apply_forest_theme_defaults;
use crate::theme::neo::apply_neo_theme_defaults;
use crate::theme::neo_dark::apply_neo_dark_theme_defaults;
use crate::theme::neutral::apply_neutral_theme_defaults;
use crate::theme::redux::apply_redux_theme_defaults;
use crate::theme::redux_color::apply_redux_color_theme_defaults;
use crate::theme::redux_dark::apply_redux_dark_theme_defaults;
use crate::theme::redux_dark_color::apply_redux_dark_color_theme_defaults;
use crate::theme::variables::ThemeVariables;

pub(crate) fn apply_theme_defaults(config: &mut MermaidConfig) {
    let theme = config.get_str("theme").unwrap_or("default");

    let mut tv = match config.as_value().get("themeVariables") {
        Some(v) => ThemeVariables::from_value(v),
        _ => ThemeVariables::default(),
    };

    match theme {
        "base" => apply_base_theme_defaults(&mut tv),
        "dark" => apply_dark_theme_defaults(&mut tv),
        "default" => apply_default_theme_defaults(&mut tv),
        "forest" => apply_forest_theme_defaults(&mut tv),
        "neo" => apply_neo_theme_defaults(&mut tv),
        "neo-dark" => apply_neo_dark_theme_defaults(&mut tv),
        "neutral" => apply_neutral_theme_defaults(&mut tv),
        "redux" => apply_redux_theme_defaults(&mut tv),
        "redux-color" => apply_redux_color_theme_defaults(&mut tv),
        "redux-dark" => apply_redux_dark_theme_defaults(&mut tv),
        "redux-dark-color" => apply_redux_dark_color_theme_defaults(&mut tv),
        _ => {}
    }

    config.set_value("themeVariables", tv.to_value());
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn base_theme_derivation_matches_upstream_fixture_values() {
        let mut cfg = MermaidConfig::from_value(json!({
            "theme": "base",
            "themeVariables": {
                "primaryColor": "#411d4e",
                "titleColor": "white",
                "darkMode": true
            }
        }));
        apply_theme_defaults(&mut cfg);

        let tv = cfg
            .as_value()
            .get("themeVariables")
            .and_then(|v| v.as_object())
            .unwrap();

        assert_eq!(tv.get("textColor").and_then(|v| v.as_str()), Some("#eee"));
        assert_eq!(
            tv.get("lineColor").and_then(|v| v.as_str()),
            Some("#0b0b0b")
        );
        assert_eq!(
            tv.get("nodeBorder").and_then(|v| v.as_str()),
            Some("hsl(284.0816326531, 5.7943925234%, 30.9803921569%)")
        );
        assert_eq!(tv.get("mainBkg").and_then(|v| v.as_str()), Some("#411d4e"));
        assert_eq!(
            tv.get("clusterBkg").and_then(|v| v.as_str()),
            Some("hsl(104.0816326531, 45.7943925234%, 25.9803921569%)")
        );
        assert_eq!(
            tv.get("clusterBorder").and_then(|v| v.as_str()),
            Some("hsl(104.0816326531, 5.7943925234%, 35.9803921569%)")
        );
        assert_eq!(
            tv.get("edgeLabelBackground").and_then(|v| v.as_str()),
            Some("hsl(164.0816326531, 45.7943925234%, 0%)")
        );
        assert_eq!(
            tv.get("errorBkgColor").and_then(|v| v.as_str()),
            Some("hsl(104.0816326531, 45.7943925234%, 25.9803921569%)")
        );
        assert_eq!(
            tv.get("errorTextColor").and_then(|v| v.as_str()),
            Some("rgb(202.9906542056, 158.4112149531, 219.0887850467)")
        );
        assert_eq!(tv.get("titleColor").and_then(|v| v.as_str()), Some("white"));
    }

    #[test]
    fn forest_theme_derives_cscale_palette_like_upstream() {
        let mut cfg = MermaidConfig::from_value(json!({
            "theme": "forest"
        }));
        apply_theme_defaults(&mut cfg);

        let tv = cfg
            .as_value()
            .get("themeVariables")
            .and_then(|v| v.as_object())
            .unwrap();

        assert_eq!(
            tv.get("cScale0").and_then(|v| v.as_str()),
            Some("hsl(78.1578947368, 58.4615384615%, 64.5098039216%)")
        );
        assert_eq!(
            tv.get("cScalePeer0").and_then(|v| v.as_str()),
            Some("hsl(78.1578947368, 58.4615384615%, 39.5098039216%)")
        );
        assert_eq!(
            tv.get("cScalePeer1").and_then(|v| v.as_str()),
            Some("hsl(98.961038961, 100%, 39.9019607843%)")
        );
        assert_eq!(
            tv.get("cScalePeer2").and_then(|v| v.as_str()),
            Some("hsl(78.1578947368, 58.4615384615%, 44.5098039216%)")
        );
    }

    #[test]
    fn dark_theme_derives_peer_and_inverted_scales_like_upstream() {
        let mut cfg = MermaidConfig::from_value(json!({
            "theme": "dark"
        }));
        apply_theme_defaults(&mut cfg);

        let tv = cfg
            .as_value()
            .get("themeVariables")
            .and_then(|v| v.as_object())
            .unwrap();

        assert_eq!(tv.get("cScale1").and_then(|v| v.as_str()), Some("#0b0000"));
        assert_eq!(
            tv.get("cScalePeer1").and_then(|v| v.as_str()),
            Some("hsl(0, 100%, 12.1568627451%)")
        );
        assert_eq!(
            tv.get("cScaleInv1").and_then(|v| v.as_str()),
            Some("#f4ffff")
        );
        assert_eq!(
            tv.get("cScaleLabel1").and_then(|v| v.as_str()),
            Some("lightgrey")
        );
    }

    #[test]
    fn neutral_theme_derives_peer_and_label_scales_like_upstream() {
        let mut cfg = MermaidConfig::from_value(json!({
            "theme": "neutral"
        }));
        apply_theme_defaults(&mut cfg);

        let tv = cfg
            .as_value()
            .get("themeVariables")
            .and_then(|v| v.as_object())
            .unwrap();

        assert_eq!(tv.get("cScale0").and_then(|v| v.as_str()), Some("#555"));
        assert_eq!(
            tv.get("cScalePeer0").and_then(|v| v.as_str()),
            Some("hsl(0, 0%, 23.3333333333%)")
        );
        assert_eq!(
            tv.get("cScaleInv0").and_then(|v| v.as_str()),
            Some("#aaaaaa")
        );
        assert_eq!(
            tv.get("cScaleLabel0").and_then(|v| v.as_str()),
            Some("#F4F4F4")
        );
    }

    /// Utility: generate JSON for all themes so they can be diffed against upstream.
    /// Run with: cargo test -p merman-core -- generate_theme_json --nocapture
    #[test]
    fn generate_theme_json_for_diff() {
        let out_dir = std::env::temp_dir().join("merman-themes");
        std::fs::create_dir_all(&out_dir).unwrap();

        let themes: Vec<(&str, fn(&mut ThemeVariables))> = vec![
            ("base", apply_base_theme_defaults),
            ("dark", apply_dark_theme_defaults),
            ("default", apply_default_theme_defaults),
            ("forest", apply_forest_theme_defaults),
            ("neutral", apply_neutral_theme_defaults),
            ("neo", apply_neo_theme_defaults),
            ("neo-dark", apply_neo_dark_theme_defaults),
            ("redux", apply_redux_theme_defaults),
            ("redux-color", apply_redux_color_theme_defaults),
            ("redux-dark", apply_redux_dark_theme_defaults),
            ("redux-dark-color", apply_redux_dark_color_theme_defaults),
        ];

        for (name, apply_fn) in themes {
            let mut tv = ThemeVariables::default();
            apply_fn(&mut tv);
            let json = serde_json::to_value(&tv).unwrap();
            let file_path = out_dir.join(format!("{}.json", name));
            let pretty = serde_json::to_string_pretty(&json).unwrap();
            std::fs::write(&file_path, pretty + "\n").unwrap();
            eprintln!(
                "Wrote {} (fields: {})",
                file_path.display(),
                json.as_object().map_or(0, |o| o.len())
            );
        }

        eprintln!("\nTheme JSONs written to {}", out_dir.display());
        eprintln!("Diff with: diff <(cd {} && ... ) ...", out_dir.display());
    }
}
