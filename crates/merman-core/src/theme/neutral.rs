use crate::MermaidConfig;
use crate::color::{Hsl, Rgb};
use crate::theme::{ensure_xychart_theme_defaults, get_truthy_string, set_if_missing};
use serde_json::{Map, Value};

pub fn apply_neutral_theme_defaults(config: &mut MermaidConfig) {
    let mut tv = match config.as_value().get("themeVariables") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };

    // `theme-neutral` constructor defaults.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-neutral.js`.
    set_if_missing(&mut tv, "background", Value::String("#ffffff".to_string()));
    set_if_missing(&mut tv, "primaryColor", Value::String("#eee".to_string()));
    if get_truthy_string(&tv, "primaryTextColor").is_none()
        && let Some(primary_color) = get_truthy_string(&tv, "primaryColor")
        && let Ok(rgb) = Rgb::try_from(&primary_color)
    {
        tv.insert(
            "primaryTextColor".to_string(),
            Value::String(
                Rgb {
                    r: 1.0 - rgb.r,
                    g: 1.0 - rgb.g,
                    b: 1.0 - rgb.b,
                }
                .to_string(),
            ),
        );
    }

    // Mermaid 11.12.2: `theme-neutral` color scale seeds.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-neutral.js`.
    let c_scales_hex: [&str; 12] = [
        "#555", "#F4F4F4", "#555", "#BBB", "#777", "#999", "#DDD", "#FFF", "#DDD", "#BBB", "#999",
        "#777",
    ];

    set_if_missing(&mut tv, "labelTextColor", Value::String("#333".to_string()));
    set_if_missing(
        &mut tv,
        "scaleLabelColor",
        Value::String("#333".to_string()),
    );
    let scale_label_color =
        get_truthy_string(&tv, "scaleLabelColor").unwrap_or_else(|| "#333".to_string());

    for (i, c_hex) in c_scales_hex.iter().enumerate() {
        set_if_missing(
            &mut tv,
            &format!("cScale{i}"),
            Value::String((*c_hex).to_string()),
        );

        let Ok(rgb) = Rgb::try_from(*c_hex) else {
            continue;
        };
        let hsl = Hsl::from(rgb);

        // `theme-neutral` peers: `darken(cScale, 10)` (darkMode defaults to false).
        set_if_missing(
            &mut tv,
            &format!("cScalePeer{i}"),
            Value::String(hsl.adjust_hsl(0.0, 0.0, -10.0).to_string()),
        );

        // `theme-neutral` inverted scale: `invert(cScale)`.
        set_if_missing(
            &mut tv,
            &format!("cScaleInv{i}"),
            Value::String(
                Rgb {
                    r: 1.0 - rgb.r,
                    g: 1.0 - rgb.g,
                    b: 1.0 - rgb.b,
                }
                .to_string(),
            ),
        );

        // `theme-neutral` label scale: `scaleLabelColor`, with special-cased indices.
        // - `cScaleLabel0` and `cScaleLabel2`: `cScale1` (light fill needs dark text)
        if i == 0 || i == 2 {
            set_if_missing(
                &mut tv,
                &format!("cScaleLabel{i}"),
                Value::String(c_scales_hex[1].to_string()),
            );
        }
        set_if_missing(
            &mut tv,
            &format!("cScaleLabel{i}"),
            Value::String(scale_label_color.clone()),
        );
    }

    // `theme-neutral` xychart palette + colors.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-neutral.js`.
    ensure_xychart_theme_defaults(
        &mut tv,
        "#EEE,#6BB8E4,#8ACB88,#C7ACD6,#E8DCC2,#FFB2A8,#FFF380,#7E8D91,#FFD8B1,#FAF3E0",
    );

    config.set_value("themeVariables", Value::Object(tv));
}
