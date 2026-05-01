use crate::MermaidConfig;
use crate::color::{Hsl, Rgb};
use crate::theme::{ensure_xychart_theme_defaults, get_truthy_string, set_if_missing};
use serde_json::{Map, Value};

pub(crate) fn apply_dark_theme_defaults(config: &mut MermaidConfig) {
    let mut tv = match config.as_value().get("themeVariables") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };

    // Mermaid 11.12.2: `theme-dark` color scale seeds.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-dark.js`.
    //
    // Note: `theme-dark` keeps `cScale*` as the provided hex strings, while derived
    // `cScalePeer*` values are produced via `khroma.lighten(...)` (serialized as `hsl(...)`).
    let c_scales_hex: [&str; 12] = [
        "#1f2020", // primaryColor
        "#0b0000", "#4d1037", "#3f5258", "#4f2f1b", "#6e0a0a", "#3b0048", "#995a01", "#154706",
        "#161722", "#00296f", "#01629c",
    ];

    // Minimal `theme-dark` seeds needed for diagram render parity when users set `theme: dark`.
    //
    // Mermaid's JS theme sets many more variables (and calculates derived values in `updateColors()`).
    // We seed the commonly-consumed surfaces + xychart palette here; other missing values are left
    // for future parity work as fixtures demand.
    set_if_missing(&mut tv, "background", Value::String("#333".to_string()));
    set_if_missing(
        &mut tv,
        "primaryColor",
        Value::String("#1f2020".to_string()),
    );
    if get_truthy_string(&tv, "primaryTextColor").is_none()
        && let Some(primary_color) = get_truthy_string(&tv, "primaryColor")
        && let Ok(Rgb {
            r: primary_color_r,
            g: primary_color_g,
            b: primary_color_b,
        }) = Rgb::try_from(&primary_color)
    {
        tv.insert(
            "primaryTextColor".to_string(),
            Value::String(
                Rgb {
                    r: 1.0 - primary_color_r,
                    g: 1.0 - primary_color_g,
                    b: 1.0 - primary_color_b,
                }
                .to_string(),
            ),
        );
    }
    set_if_missing(&mut tv, "textColor", Value::String("#ccc".to_string()));
    set_if_missing(
        &mut tv,
        "fontFamily",
        Value::String("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
    );
    set_if_missing(&mut tv, "fontSize", Value::String("16px".to_string()));
    set_if_missing(&mut tv, "border1", Value::String("#ccc".to_string()));
    set_if_missing(
        &mut tv,
        "border2",
        Value::String("rgba(255, 255, 255, 0.25)".to_string()),
    );
    set_if_missing(
        &mut tv,
        "labelBackground",
        Value::String("#181818".to_string()),
    );
    set_if_missing(&mut tv, "titleColor", Value::String("#F9FFFE".to_string()));
    set_if_missing(
        &mut tv,
        "errorBkgColor",
        Value::String("#a44141".to_string()),
    );
    set_if_missing(&mut tv, "errorTextColor", Value::String("#ddd".to_string()));

    set_if_missing(
        &mut tv,
        "labelTextColor",
        Value::String("lightgrey".to_string()),
    );
    // Mermaid's `config.ts` calls `theme-dark.getThemeVariables(conf.themeVariables)` without
    // injecting `darkMode=true`, so `theme-dark.js` falls back to `labelTextColor` here.
    let label_text_color =
        get_truthy_string(&tv, "labelTextColor").unwrap_or_else(|| "lightgrey".to_string());
    set_if_missing(
        &mut tv,
        "scaleLabelColor",
        Value::String(label_text_color.clone()),
    );
    let scale_label_color =
        get_truthy_string(&tv, "scaleLabelColor").unwrap_or_else(|| label_text_color.clone());

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

        // `theme-dark` peers: `lighten(cScale, 10)`.
        set_if_missing(
            &mut tv,
            &format!("cScalePeer{i}"),
            Value::String(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string()),
        );

        // `theme-dark` inverted scale: `invert(cScale)`.
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

        // `theme-dark` label scale: `scaleLabelColor`.
        set_if_missing(
            &mut tv,
            &format!("cScaleLabel{i}"),
            Value::String(scale_label_color.clone()),
        );
    }

    // `theme-dark` xychart palette + colors.
    // Source: `theme-dark.js`.
    ensure_xychart_theme_defaults(
        &mut tv,
        "#3498db,#2ecc71,#e74c3c,#f1c40f,#bdc3c7,#ffffff,#34495e,#9b59b6,#1abc9c,#e67e22",
    );

    config.set_value("themeVariables", Value::Object(tv));
}
