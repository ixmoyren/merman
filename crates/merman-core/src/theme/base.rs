use crate::MermaidConfig;
use crate::color::{Hsl, Rgb};
use crate::theme::{ensure_xychart_theme_defaults, get_truthy_string, set_if_missing};
use serde_json::{Map, Value};

pub(crate) fn apply_base_theme_defaults(config: &mut MermaidConfig) {
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
