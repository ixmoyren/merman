use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_base_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);
    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#f4f4f4")
        .to_string();
    let primary_color = tv
        .primary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#fff4dd")
        .to_string();

    tv.set_background_if_none(&background);
    tv.set_primary_color_if_none(&primary_color);
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#333" });
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");

    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#333" })
        .to_string();

    let primary_hsl = Rgb::try_from(&primary_color).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });

    let secondary_hsl = if let Some(ref v) = tv.secondary_color
        && is_truthy(&tv.secondary_color)
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else {
        primary_hsl.adjust_hsl(-120.0, 0.0, 0.0)
    };
    tv.set_secondary_color_if_none(secondary_hsl.to_string());

    let tertiary_hsl = if let Some(ref v) = tv.tertiary_color
        && is_truthy(&tv.tertiary_color)
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else {
        primary_hsl.adjust_hsl(180.0, 0.0, 5.0)
    };
    tv.set_tertiary_color_if_none(tertiary_hsl.to_string());

    if !is_truthy(&tv.primary_border_color) {
        tv.primary_border_color = Some(
            primary_hsl
                .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
                .to_string(),
        );
    }

    if !is_truthy(&tv.tertiary_border_color) {
        tv.tertiary_border_color = Some(
            tertiary_hsl
                .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
                .to_string(),
        );
    }

    if !is_truthy(&tv.line_color)
        && let Ok(bg_rgb) = Rgb::try_from(&background)
    {
        tv.line_color = Some(
            Rgb {
                r: 1.0 - bg_rgb.r,
                g: 1.0 - bg_rgb.g,
                b: 1.0 - bg_rgb.b,
            }
            .to_string(),
        );
    }
    let line_color = tv
        .line_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333333")
        .to_string();
    tv.set_arrowhead_color_if_none(&line_color);
    tv.set_text_color_if_none(&primary_text_color);

    let primary_border_color = tv
        .primary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#9370DB")
        .to_string();
    let tertiary_border_color = tv
        .tertiary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#aaaa33")
        .to_string();
    let tertiary_color_str = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("hsl(80, 100%, 96.2745098039%)")
        .to_string();

    tv.set_node_bkg_if_none(&primary_color);
    tv.set_main_bkg_if_none(&primary_color);
    tv.set_node_border_if_none(&primary_border_color);
    tv.set_cluster_bkg_if_none(&tertiary_color_str);
    tv.set_cluster_border_if_none(&tertiary_border_color);
    tv.set_node_text_color_if_none(&primary_text_color);

    if !is_truthy(&tv.tertiary_text_color) {
        let rgb = Rgb::from(tertiary_hsl);
        tv.tertiary_text_color = Some(rgb.invert_rgb_to_rgb_string());
    }
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();
    tv.set_title_color_if_none(&tertiary_text_color);

    if !is_truthy(&tv.edge_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.edge_label_background = Some(v.to_string());
    }

    tv.set_error_bkg_color_if_none(&tertiary_color_str);
    tv.set_error_text_color_if_none(&tertiary_text_color);

    // Color scales
    let amount = if dark_mode { 75.0 } else { 25.0 };
    tv.set_c_scale0_if_none(primary_hsl.adjust_hsl(0.0, 0.0, -amount).to_string());
    tv.set_c_scale1_if_none(secondary_hsl.adjust_hsl(0.0, 0.0, -amount).to_string());
    tv.set_c_scale2_if_none(tertiary_hsl.adjust_hsl(0.0, 0.0, -amount).to_string());
    tv.set_c_scale3_if_none(
        primary_hsl
            .adjust_hsl(30.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale4_if_none(
        primary_hsl
            .adjust_hsl(60.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale5_if_none(
        primary_hsl
            .adjust_hsl(90.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale6_if_none(
        primary_hsl
            .adjust_hsl(120.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale7_if_none(
        primary_hsl
            .adjust_hsl(150.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale8_if_none(
        primary_hsl
            .adjust_hsl(210.0, 0.0, 150.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale9_if_none(
        primary_hsl
            .adjust_hsl(270.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale10_if_none(
        primary_hsl
            .adjust_hsl(300.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale11_if_none(
        primary_hsl
            .adjust_hsl(330.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );

    // Radar sub-object
    let radar = tv.radar.get_or_insert_with(Default::default);
    radar.set_axis_color_if_none(line_color);
    radar.set_axis_stroke_width_if_none(2);
    radar.set_axis_label_font_size_if_none(12);
    radar.set_curve_opacity_if_none(0.5);
    radar.set_curve_stroke_width_if_none(2);
    radar.set_graticule_color_if_none("#DEDEDE");
    radar.set_graticule_opacity_if_none(0.3);
    radar.set_graticule_stroke_width_if_none(1);
    radar.set_legend_box_size_if_none(12);
    radar.set_legend_font_size_if_none(12);

    // xyChart
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    let bg = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("white")
        .to_string();
    xy.set_background_color_if_none(bg);
    xy.set_plot_color_palette_if_none(
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );
    xy.fill_prime_color(primary_text_color);
}
