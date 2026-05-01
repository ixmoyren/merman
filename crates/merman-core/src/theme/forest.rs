use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_forest_theme_defaults(tv: &mut ThemeVariables) {
    tv.set_primary_color_if_none("#cde498");
    tv.set_secondary_color_if_none("#cdffb2");
    tv.set_background_if_none("white");
    tv.set_border1_if_none("#13540c");
    tv.set_border2_if_none("#6eaa49");
    tv.set_arrowhead_color_if_none("green");
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");
    tv.set_title_color_if_none("#333");
    tv.set_edge_label_background_if_none("#e8e8e8");
    tv.set_error_bkg_color_if_none("#552222");
    tv.set_error_text_color_if_none("#552222");

    let Some(primary_color) = tv.primary_color.clone() else {
        return;
    };
    if !is_truthy(&tv.primary_color) {
        return;
    }
    let Ok(primary_rgb) = Rgb::try_from(&primary_color) else {
        return;
    };
    let primary_hsl = Hsl::from(primary_rgb);

    if !is_truthy(&tv.primary_text_color) {
        tv.primary_text_color = Some(
            Rgb {
                r: 1.0 - primary_rgb.r,
                g: 1.0 - primary_rgb.g,
                b: 1.0 - primary_rgb.b,
            }
            .to_string(),
        );
    }

    let secondary_color = tv
        .secondary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cdffb2")
        .to_string();
    let secondary_hsl = Rgb::try_from(&secondary_color)
        .map(Hsl::from)
        .unwrap_or(primary_hsl);

    tv.set_main_bkg_if_none(&primary_color);
    tv.set_second_bkg_if_none(&secondary_color);
    tv.set_row_odd_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 75.0).to_string());
    tv.set_row_even_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 20.0).to_string());

    tv.set_line_color_if_none("#000000");
    tv.set_text_color_if_none("#000000");

    tv.set_node_bkg_if_none(&primary_color);
    tv.set_node_border_if_none("#13540c");
    tv.set_cluster_bkg_if_none(&secondary_color);
    tv.set_cluster_border_if_none("#6eaa49");
    tv.set_default_link_color_if_none("#000000");

    let dark_mode = tv.dark_mode.unwrap_or(false);
    let mk_border_delta_l = if dark_mode { 10.0 } else { -10.0 };
    tv.set_primary_border_color_if_none(
        primary_hsl
            .adjust_hsl(0.0, -40.0, mk_border_delta_l)
            .to_string(),
    );
    tv.set_secondary_border_color_if_none(
        secondary_hsl
            .adjust_hsl(0.0, -40.0, mk_border_delta_l)
            .to_string(),
    );

    let tertiary_hsl = if let Some(ref tc) = tv.tertiary_color
        && is_truthy(&tv.tertiary_color)
        && let Ok(tc_rgb) = Rgb::try_from(tc)
    {
        tc_rgb.into()
    } else {
        primary_hsl.adjust_hsl(0.0, 0.0, 10.0)
    };
    tv.set_tertiary_color_if_none(tertiary_hsl.to_string());
    tv.set_tertiary_border_color_if_none(
        tertiary_hsl
            .adjust_hsl(0.0, -40.0, mk_border_delta_l)
            .to_string(),
    );

    tv.set_label_text_color_if_none("black");
    tv.set_scale_label_color_if_none("black");
    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("black")
        .to_string();

    // Color scales
    let amount = 10.0;
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
            .adjust_hsl(210.0, 0.0, 0.0)
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

    if tv.c_scale_peer1.is_none() {
        tv.c_scale_peer1 = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -45.0).to_string());
    }
    if tv.c_scale_peer2.is_none() {
        tv.c_scale_peer2 = Some(tertiary_hsl.adjust_hsl(0.0, 0.0, -40.0).to_string());
    }

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

    macro_rules! set_scale_peer {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none() {
                tv.$field = Some(c_scales[$idx].adjust_hsl(0.0, 0.0, -25.0).to_string());
            }
        };
    }
    macro_rules! set_scale_inv {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none() {
                tv.$field = Some(c_scales[$idx].adjust_hsl(180.0, 0.0, 0.0).to_string());
            }
        };
    }
    macro_rules! set_scale_label {
        ($field:ident) => {
            if tv.$field.is_none() {
                tv.$field = Some(scale_label_color.clone());
            }
        };
    }

    set_scale_peer!(c_scale_peer0, 0);
    set_scale_inv!(c_scale_inv0, 0);
    set_scale_label!(c_scale_label0);
    set_scale_peer!(c_scale_peer1, 1);
    set_scale_inv!(c_scale_inv1, 1);
    set_scale_label!(c_scale_label1);
    set_scale_peer!(c_scale_peer2, 2);
    set_scale_inv!(c_scale_inv2, 2);
    set_scale_label!(c_scale_label2);
    set_scale_peer!(c_scale_peer3, 3);
    set_scale_inv!(c_scale_inv3, 3);
    set_scale_label!(c_scale_label3);
    set_scale_peer!(c_scale_peer4, 4);
    set_scale_inv!(c_scale_inv4, 4);
    set_scale_label!(c_scale_label4);
    set_scale_peer!(c_scale_peer5, 5);
    set_scale_inv!(c_scale_inv5, 5);
    set_scale_label!(c_scale_label5);
    set_scale_peer!(c_scale_peer6, 6);
    set_scale_inv!(c_scale_inv6, 6);
    set_scale_label!(c_scale_label6);
    set_scale_peer!(c_scale_peer7, 7);
    set_scale_inv!(c_scale_inv7, 7);
    set_scale_label!(c_scale_label7);
    set_scale_peer!(c_scale_peer8, 8);
    set_scale_inv!(c_scale_inv8, 8);
    set_scale_label!(c_scale_label8);
    set_scale_peer!(c_scale_peer9, 9);
    set_scale_inv!(c_scale_inv9, 9);
    set_scale_label!(c_scale_label9);
    set_scale_peer!(c_scale_peer10, 10);
    set_scale_inv!(c_scale_inv10, 10);
    set_scale_label!(c_scale_label10);
    set_scale_peer!(c_scale_peer11, 11);
    set_scale_inv!(c_scale_inv11, 11);
    set_scale_label!(c_scale_label11);

    // xyChart
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    let bg = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("white")
        .to_string();
    let pt = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| tv.text_color.as_deref().filter(|s| !s.trim().is_empty()))
        .unwrap_or("#333")
        .to_string();
    if xy.background_color.is_none() {
        xy.background_color = Some(bg);
    }
    xy.fill_prime_color(pt);
    if xy.plot_color_palette.is_none() {
        xy.plot_color_palette = Some(
            "#CDE498,#FF6B6B,#A0D2DB,#D7BDE2,#F0F0F0,#FFC3A0,#7FD8BE,#FF9A8B,#FAF3E0,#FFF176"
                .to_string(),
        );
    }
}
