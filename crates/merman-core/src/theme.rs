use crate::MermaidConfig;
use crate::color::{Hsl, Rgb};
use serde_json::{Map, Value};

fn get_truthy_string(map: &Map<String, Value>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

fn set_if_missing(map: &mut Map<String, Value>, key: &str, value: Value) {
    let is_missing = match map.get(key) {
        None => true,
        Some(Value::Null) => true,
        Some(Value::String(s)) => s.trim().is_empty(),
        _ => false,
    };
    if is_missing {
        map.insert(key.to_string(), value);
    }
}

fn ensure_xychart_theme_defaults(tv: &mut Map<String, Value>, default_palette: &str) {
    let background = get_truthy_string(tv, "background").unwrap_or_else(|| "white".to_string());
    let primary_text = get_truthy_string(tv, "primaryTextColor")
        .or_else(|| get_truthy_string(tv, "textColor"))
        .unwrap_or_else(|| "#333".to_string());

    let mut xy = match tv.get("xyChart") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };

    set_if_missing(
        &mut xy,
        "backgroundColor",
        Value::String(background.clone()),
    );
    for key in [
        "titleColor",
        "xAxisTitleColor",
        "xAxisLabelColor",
        "xAxisTickColor",
        "xAxisLineColor",
        "yAxisTitleColor",
        "yAxisLabelColor",
        "yAxisTickColor",
        "yAxisLineColor",
    ] {
        set_if_missing(&mut xy, key, Value::String(primary_text.clone()));
    }
    set_if_missing(
        &mut xy,
        "plotColorPalette",
        Value::String(default_palette.to_string()),
    );

    tv.insert("xyChart".to_string(), Value::Object(xy));
}

pub(crate) fn apply_theme_defaults(config: &mut MermaidConfig) {
    let theme = config.get_str("theme").unwrap_or("default");
    match theme {
        "base" => apply_base_theme_defaults(config),
        "dark" => apply_dark_theme_defaults(config),
        "forest" => apply_forest_theme_defaults(config),
        "neutral" => apply_neutral_theme_defaults(config),
        _ => {}
    }
}

fn apply_dark_theme_defaults(config: &mut MermaidConfig) {
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

fn apply_forest_theme_defaults(config: &mut MermaidConfig) {
    let mut tv = match config.as_value().get("themeVariables") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };

    // Mermaid 11.12.2: `theme-forest` base colors.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-forest.js`.
    //
    // NOTE: `theme-forest` is not a thin palette override. It sets several diagram-facing
    // variables (flowchart/state/sequence/...) in its `constructor()` + `updateColors()`.
    // We explicitly seed those values here so headless SVG rendering can match upstream.
    set_if_missing(
        &mut tv,
        "primaryColor",
        Value::String("#cde498".to_string()),
    );
    set_if_missing(
        &mut tv,
        "secondaryColor",
        Value::String("#cdffb2".to_string()),
    );
    set_if_missing(&mut tv, "background", Value::String("white".to_string()));
    set_if_missing(&mut tv, "border1", Value::String("#13540c".to_string()));
    set_if_missing(&mut tv, "border2", Value::String("#6eaa49".to_string()));
    set_if_missing(
        &mut tv,
        "arrowheadColor",
        Value::String("green".to_string()),
    );
    set_if_missing(
        &mut tv,
        "fontFamily",
        Value::String("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
    );
    set_if_missing(&mut tv, "fontSize", Value::String("16px".to_string()));
    set_if_missing(&mut tv, "titleColor", Value::String("#333".to_string()));
    set_if_missing(
        &mut tv,
        "edgeLabelBackground",
        Value::String("#e8e8e8".to_string()),
    );
    set_if_missing(
        &mut tv,
        "errorBkgColor",
        Value::String("#552222".to_string()),
    );
    set_if_missing(
        &mut tv,
        "errorTextColor",
        Value::String("#552222".to_string()),
    );

    let Some(primary_color) = get_truthy_string(&tv, "primaryColor") else {
        config.set_value("themeVariables", Value::Object(tv));
        return;
    };
    let Ok(primary_rgb) = Rgb::try_from(&primary_color) else {
        config.set_value("themeVariables", Value::Object(tv));
        return;
    };
    let primary_hsl = Hsl::from(primary_rgb);
    if get_truthy_string(&tv, "primaryTextColor").is_none() {
        tv.insert(
            "primaryTextColor".to_string(),
            Value::String(
                Rgb {
                    r: 1.0 - primary_rgb.r,
                    g: 1.0 - primary_rgb.g,
                    b: 1.0 - primary_rgb.b,
                }
                .to_string(),
            ),
        );
    }

    let secondary_color =
        get_truthy_string(&tv, "secondaryColor").unwrap_or_else(|| "#cdffb2".to_string());
    let secondary_hsl = Rgb::try_from(&secondary_color)
        .map(Hsl::from)
        .unwrap_or(primary_hsl);

    // `theme-forest` diagram-facing surfaces.
    // Source: `theme-forest.js` constructor + `updateColors()`.
    set_if_missing(&mut tv, "mainBkg", Value::String(primary_color.clone()));
    set_if_missing(&mut tv, "secondBkg", Value::String(secondary_color.clone()));
    // Table striping colors (used by ER diagrams).
    // Source: `theme-forest.js`:
    //   rowOdd  = lighten(mainBkg, 75) || '#ffffff'
    //   rowEven = lighten(mainBkg, 20)
    set_if_missing(
        &mut tv,
        "rowOdd",
        Value::String(primary_hsl.adjust_hsl(0.0, 0.0, 75.0).to_string()),
    );
    set_if_missing(
        &mut tv,
        "rowEven",
        Value::String(primary_hsl.adjust_hsl(0.0, 0.0, 20.0).to_string()),
    );

    // `invert('white')` in `khroma` ends up as a pure black in Mermaid's serialized SVG output.
    set_if_missing(&mut tv, "lineColor", Value::String("#000000".to_string()));
    set_if_missing(&mut tv, "textColor", Value::String("#000000".to_string()));

    // Flowchart variables (after `updateColors()`).
    set_if_missing(&mut tv, "nodeBkg", Value::String(primary_color.clone()));
    set_if_missing(&mut tv, "nodeBorder", Value::String("#13540c".to_string()));
    set_if_missing(
        &mut tv,
        "clusterBkg",
        Value::String(secondary_color.clone()),
    );
    set_if_missing(
        &mut tv,
        "clusterBorder",
        Value::String("#6eaa49".to_string()),
    );
    set_if_missing(
        &mut tv,
        "defaultLinkColor",
        Value::String("#000000".to_string()),
    );

    // mkBorder(...) helper (shared across themes).
    let dark_mode = tv
        .get("darkMode")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let mk_border_delta_l = if dark_mode { 10.0 } else { -10.0 };
    set_if_missing(
        &mut tv,
        "primaryBorderColor",
        Value::String(
            primary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_delta_l)
                .to_string(),
        ),
    );
    set_if_missing(
        &mut tv,
        "secondaryBorderColor",
        Value::String(
            secondary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_delta_l)
                .to_string(),
        ),
    );

    // `theme-forest` sets: `tertiaryColor = lighten(primaryColor, 10)`.
    let tertiary_hsl = if let Some(tertiary_color) = get_truthy_string(&tv, "tertiaryColor")
        && let Ok(tertiary_color) = Rgb::try_from(tertiary_color)
    {
        tertiary_color.into()
    } else {
        primary_hsl.adjust_hsl(0.0, 0.0, 10.0)
    };
    set_if_missing(
        &mut tv,
        "tertiaryColor",
        Value::String(tertiary_hsl.to_string()),
    );
    set_if_missing(
        &mut tv,
        "tertiaryBorderColor",
        Value::String(
            tertiary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_delta_l)
                .to_string(),
        ),
    );

    // `theme-forest` ends up using black label text (via `actorTextColor`).
    set_if_missing(
        &mut tv,
        "labelTextColor",
        Value::String("black".to_string()),
    );
    set_if_missing(
        &mut tv,
        "scaleLabelColor",
        Value::String("black".to_string()),
    );
    let scale_label_color =
        get_truthy_string(&tv, "scaleLabelColor").unwrap_or_else(|| "black".to_string());

    // Color scales: match `theme-forest` `updateColors()`:
    // - derive from base colors / hue shifts
    // - darken each `cScale*` by 10
    // - `cScalePeer1/2` use special darken amounts, others are darken(`cScale*`, 25)
    let c_scales: [Hsl; 12] = [
        primary_hsl,
        secondary_hsl,
        tertiary_hsl,
        primary_hsl.adjust_hsl(30.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(90.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(150.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(210.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(270.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(300.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(330.0, 0.0, 0.0),
    ]
    .map(|base| base.adjust_hsl(0.0, 0.0, -10.0));

    for (i, v) in c_scales.iter().enumerate() {
        set_if_missing(&mut tv, &format!("cScale{i}"), Value::String(v.to_string()));
    }

    set_if_missing(
        &mut tv,
        "cScalePeer1",
        Value::String(secondary_hsl.adjust_hsl(0.0, 0.0, -45.0).to_string()),
    );
    set_if_missing(
        &mut tv,
        "cScalePeer2",
        Value::String(tertiary_hsl.adjust_hsl(0.0, 0.0, -40.0).to_string()),
    );

    for (i, c_hsl) in c_scales.iter().enumerate() {
        set_if_missing(
            &mut tv,
            &format!("cScalePeer{i}"),
            Value::String(c_hsl.adjust_hsl(0.0, 0.0, -25.0).to_string()),
        );
        set_if_missing(
            &mut tv,
            &format!("cScaleInv{i}"),
            Value::String(c_hsl.adjust_hsl(180.0, 0.0, 0.0).to_string()),
        );
        set_if_missing(
            &mut tv,
            &format!("cScaleLabel{i}"),
            Value::String(scale_label_color.clone()),
        );
    }

    // `theme-forest` xychart palette + colors.
    // Source: `theme-forest.js`.
    ensure_xychart_theme_defaults(
        &mut tv,
        "#CDE498,#FF6B6B,#A0D2DB,#D7BDE2,#F0F0F0,#FFC3A0,#7FD8BE,#FF9A8B,#FAF3E0,#FFF176",
    );

    config.set_value("themeVariables", Value::Object(tv));
}

fn apply_neutral_theme_defaults(config: &mut MermaidConfig) {
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

fn apply_base_theme_defaults(config: &mut MermaidConfig) {
    let mut tv = match config.as_value().get("themeVariables") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };

    let dark_mode = tv
        .get("darkMode")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let background = get_truthy_string(&tv, "background").unwrap_or_else(|| "#f4f4f4".to_string());
    let primary_color =
        get_truthy_string(&tv, "primaryColor").unwrap_or_else(|| "#fff4dd".to_string());

    // `theme-base` constructor defaults.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-base.js`.
    set_if_missing(&mut tv, "background", Value::String(background.clone()));
    set_if_missing(
        &mut tv,
        "primaryColor",
        Value::String(primary_color.clone()),
    );

    set_if_missing(
        &mut tv,
        "primaryTextColor",
        Value::String(if dark_mode { "#eee" } else { "#333" }.to_string()),
    );
    set_if_missing(
        &mut tv,
        "fontFamily",
        Value::String("\"trebuchet ms\", verdana, arial, sans-serif".to_string()),
    );
    set_if_missing(&mut tv, "fontSize", Value::String("16px".to_string()));

    let primary_text_color = get_truthy_string(&tv, "primaryTextColor")
        .unwrap_or_else(|| if dark_mode { "#eee" } else { "#333" }.to_string());

    let primary_hsl = Rgb::try_from(&primary_color).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });

    let secondary_hsl = if let Some(v) = get_truthy_string(&tv, "secondaryColor")
        && let Ok(secondary_color) = Rgb::try_from(v)
    {
        secondary_color.into()
    } else {
        primary_hsl.adjust_hsl(-120.0, 0.0, 0.0)
    };
    set_if_missing(
        &mut tv,
        "secondaryColor",
        Value::String(secondary_hsl.to_string()),
    );

    let tertiary_hsl = if let Some(v) = get_truthy_string(&tv, "tertiaryColor")
        && let Ok(tertiary_color) = Rgb::try_from(v)
    {
        tertiary_color.into()
    } else {
        primary_hsl.adjust_hsl(180.0, 0.0, 5.0)
    };
    set_if_missing(
        &mut tv,
        "tertiaryColor",
        Value::String(tertiary_hsl.to_string()),
    );

    let primary_border_hsl = if get_truthy_string(&tv, "primaryBorderColor").is_some() {
        None
    } else {
        Some(primary_hsl.adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 }))
    };
    if let Some(hsl) = primary_border_hsl {
        tv.insert(
            "primaryBorderColor".to_string(),
            Value::String(hsl.to_string()),
        );
    }

    let tertiary_border_hsl = if get_truthy_string(&tv, "tertiaryBorderColor").is_some() {
        None
    } else {
        Some(tertiary_hsl.adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 }))
    };
    if let Some(hsl) = tertiary_border_hsl {
        tv.insert(
            "tertiaryBorderColor".to_string(),
            Value::String(hsl.to_string()),
        );
    }

    if get_truthy_string(&tv, "lineColor").is_none()
        && let Ok(bg_rgb) = Rgb::try_from(&background)
    {
        tv.insert(
            "lineColor".to_string(),
            Value::String(
                Rgb {
                    r: 1.0 - bg_rgb.r,
                    g: 1.0 - bg_rgb.g,
                    b: 1.0 - bg_rgb.b,
                }
                .to_string(),
            ),
        );
    }
    let line_color = get_truthy_string(&tv, "lineColor").unwrap_or_else(|| "#333333".to_string());
    set_if_missing(&mut tv, "arrowheadColor", Value::String(line_color));

    set_if_missing(
        &mut tv,
        "textColor",
        Value::String(primary_text_color.clone()),
    );

    let primary_border_color =
        get_truthy_string(&tv, "primaryBorderColor").unwrap_or_else(|| "#9370DB".to_string());
    let tertiary_border_color =
        get_truthy_string(&tv, "tertiaryBorderColor").unwrap_or_else(|| "#aaaa33".to_string());
    let tertiary_color = get_truthy_string(&tv, "tertiaryColor")
        .unwrap_or_else(|| "hsl(80, 100%, 96.2745098039%)".to_string());

    set_if_missing(&mut tv, "nodeBkg", Value::String(primary_color.clone()));
    set_if_missing(&mut tv, "mainBkg", Value::String(primary_color.clone()));
    set_if_missing(&mut tv, "nodeBorder", Value::String(primary_border_color));
    set_if_missing(&mut tv, "clusterBkg", Value::String(tertiary_color.clone()));
    set_if_missing(
        &mut tv,
        "clusterBorder",
        Value::String(tertiary_border_color),
    );
    set_if_missing(&mut tv, "nodeTextColor", Value::String(primary_text_color));

    if get_truthy_string(&tv, "tertiaryTextColor").is_none() {
        let rgb = Rgb::from(tertiary_hsl);
        tv.insert(
            "tertiaryTextColor".to_string(),
            Value::String(rgb.invert_rgb_to_rgb_string()),
        );
    }
    let tertiary_text_color =
        get_truthy_string(&tv, "tertiaryTextColor").unwrap_or_else(|| "#333".to_string());
    set_if_missing(
        &mut tv,
        "titleColor",
        Value::String(tertiary_text_color.clone()),
    );

    if get_truthy_string(&tv, "edgeLabelBackground").is_none() {
        let mut v = secondary_hsl;
        if dark_mode {
            v = secondary_hsl.adjust_hsl(0.0, 0.0, -30.0);
        }
        tv.insert(
            "edgeLabelBackground".to_string(),
            Value::String(v.to_string()),
        );
    }

    set_if_missing(&mut tv, "errorBkgColor", Value::String(tertiary_color));
    set_if_missing(
        &mut tv,
        "errorTextColor",
        Value::String(tertiary_text_color),
    );

    // Theme color scales (used across multiple diagrams, including radar's `cScale*` palette).
    // Mermaid's base theme derives these from `primaryColor` and then darkens them.
    let darken_amount = if dark_mode { 75.0 } else { 25.0 };
    for (key, base) in [
        ("cScale0", primary_hsl),
        ("cScale1", secondary_hsl),
        ("cScale2", tertiary_hsl),
        ("cScale3", primary_hsl.adjust_hsl(30.0, 0.0, 0.0)),
        ("cScale4", primary_hsl.adjust_hsl(60.0, 0.0, 0.0)),
        ("cScale5", primary_hsl.adjust_hsl(90.0, 0.0, 0.0)),
        ("cScale6", primary_hsl.adjust_hsl(120.0, 0.0, 0.0)),
        ("cScale7", primary_hsl.adjust_hsl(150.0, 0.0, 0.0)),
        ("cScale8", primary_hsl.adjust_hsl(210.0, 0.0, 150.0)),
        ("cScale9", primary_hsl.adjust_hsl(270.0, 0.0, 0.0)),
        ("cScale10", primary_hsl.adjust_hsl(300.0, 0.0, 0.0)),
        ("cScale11", primary_hsl.adjust_hsl(330.0, 0.0, 0.0)),
    ] {
        let v = base.adjust_hsl(0.0, 0.0, -darken_amount);
        set_if_missing(&mut tv, key, Value::String(v.to_string()));
    }

    // Diagram style defaults (themeVariables.radar.*).
    let mut radar = match tv.get("radar") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    };
    let line_color = get_truthy_string(&tv, "lineColor").unwrap_or_else(|| "#333333".to_string());
    set_if_missing(&mut radar, "axisColor", Value::String(line_color));
    set_if_missing(&mut radar, "axisStrokeWidth", Value::Number(2.into()));
    set_if_missing(&mut radar, "axisLabelFontSize", Value::Number(12.into()));
    set_if_missing(
        &mut radar,
        "curveOpacity",
        Value::Number(serde_json::Number::from_f64(0.5).unwrap()),
    );
    set_if_missing(&mut radar, "curveStrokeWidth", Value::Number(2.into()));
    set_if_missing(
        &mut radar,
        "graticuleColor",
        Value::String("#DEDEDE".to_string()),
    );
    set_if_missing(&mut radar, "graticuleStrokeWidth", Value::Number(1.into()));
    set_if_missing(
        &mut radar,
        "graticuleOpacity",
        Value::Number(serde_json::Number::from_f64(0.3).unwrap()),
    );
    set_if_missing(&mut radar, "legendBoxSize", Value::Number(12.into()));
    set_if_missing(&mut radar, "legendFontSize", Value::Number(12.into()));
    tv.insert("radar".to_string(), Value::Object(radar));

    // `theme-base` xychart palette + colors.
    // Source: `repo-ref/mermaid/packages/mermaid/src/themes/theme-base.js`.
    ensure_xychart_theme_defaults(
        &mut tv,
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );

    config.set_value("themeVariables", Value::Object(tv));
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
}
