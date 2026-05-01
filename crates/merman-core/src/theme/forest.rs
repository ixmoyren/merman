use crate::MermaidConfig;
use crate::color::{Hsl, Rgb};
use crate::theme::{ensure_xychart_theme_defaults, get_truthy_string, set_if_missing};
use serde_json::{Map, Value};

pub fn apply_forest_theme_defaults(config: &mut MermaidConfig) {
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
