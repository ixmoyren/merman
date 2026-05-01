use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_neo_dark_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(true);

    tv.set_background_if_none("#333");
    tv.set_primary_color_if_none("#1f2020");
    tv.set_main_bkg_if_none("#2a2020");
    tv.set_main_contrast_color_if_none("lightgrey");
    tv.set_border1_if_none("#ccc");
    tv.set_border2_if_none("rgba(255,255,255,0.25)");
    tv.set_label_background_if_none("#181818");
    tv.set_text_color_if_none("#ccc");
    tv.set_font_family_if_none("arial, sans-serif");
    tv.set_font_size_if_none("14px");
    if tv.radius.is_none() {
        tv.radius = Some(3.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(1.0);
    }
    tv.set_note_bkg_color_if_none("#fff5ad");
    tv.set_note_text_color_if_none("#333");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(true);
    }
    tv.set_gradient_start_if_none("#0042eb");
    tv.set_gradient_stop_if_none("#eb0042");
    tv.set_drop_shadow_if_none("drop-shadow(1px 2px 2px rgba(185,185,185,0.2))");
    if tv.dark_mode.is_none() {
        tv.dark_mode = Some(true);
    }

    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

    // Hardcoded inverted values from constructor
    if !is_truthy(&tv.primary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&background)
    {
        tv.primary_text_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }
    if !is_truthy(&tv.secondary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&background)
    {
        tv.secondary_text_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }
    if !is_truthy(&tv.tertiary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&background)
    {
        tv.tertiary_text_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }
    if !is_truthy(&tv.primary_border_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&background)
    {
        tv.primary_border_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }

    let primary_color = tv
        .primary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#1f2020")
        .to_string();
    let primary_hsl = Rgb::try_from(&primary_color).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 12.0,
    });

    if !is_truthy(&tv.secondary_color) {
        // lighten(primaryColor, 16)
        tv.secondary_color = Some(primary_hsl.adjust_hsl(0.0, 0.0, 16.0).to_string());
    }
    let secondary_hsl = tv
        .secondary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(0.0, 0.0, 16.0));

    if !is_truthy(&tv.tertiary_color) {
        tv.tertiary_color = Some(primary_hsl.adjust_hsl(-160.0, 0.0, 0.0).to_string());
    }
    let tertiary_hsl = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(-160.0, 0.0, 0.0));

    // Color scales — dark mode: darken(75)
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

    let c_scales: [Hsl; 12] = [
        primary_hsl,
        secondary_hsl,
        tertiary_hsl,
        primary_hsl.adjust_hsl(30.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(90.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(150.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(210.0, 0.0, 150.0),
        primary_hsl.adjust_hsl(270.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(300.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(330.0, 0.0, 0.0),
    ]
    .map(|base| base.adjust_hsl(0.0, 0.0, -amount));

    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            tv.label_text_color
                .as_deref()
                .filter(|s| !s.trim().is_empty())
        })
        .unwrap_or("lightgrey")
        .to_string();

    let peer_delta = if dark_mode { 10.0 } else { -10.0 };

    for i in 0..12 {
        macro_rules! set_3 {
            ($peer:ident, $inv:ident, $label:ident) => {
                if tv.$peer.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scales[i].to_string())
                {
                    tv.$peer = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
                if tv.$inv.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scales[i].to_string())
                {
                    tv.$inv = Some(
                        Rgb {
                            r: 1.0 - r,
                            g: 1.0 - g,
                            b: 1.0 - b,
                        }
                        .to_string(),
                    );
                }
                if tv.$label.is_none() {
                    tv.$label = Some(scale_label_color.clone());
                }
            };
        }
        match i {
            0 => {
                set_3!(c_scale_peer0, c_scale_inv0, c_scale_label0);
            }
            1 => {
                set_3!(c_scale_peer1, c_scale_inv1, c_scale_label1);
            }
            2 => {
                set_3!(c_scale_peer2, c_scale_inv2, c_scale_label2);
            }
            3 => {
                set_3!(c_scale_peer3, c_scale_inv3, c_scale_label3);
            }
            4 => {
                set_3!(c_scale_peer4, c_scale_inv4, c_scale_label4);
            }
            5 => {
                set_3!(c_scale_peer5, c_scale_inv5, c_scale_label5);
            }
            6 => {
                set_3!(c_scale_peer6, c_scale_inv6, c_scale_label6);
            }
            7 => {
                set_3!(c_scale_peer7, c_scale_inv7, c_scale_label7);
            }
            8 => {
                set_3!(c_scale_peer8, c_scale_inv8, c_scale_label8);
            }
            9 => {
                set_3!(c_scale_peer9, c_scale_inv9, c_scale_label9);
            }
            10 => {
                set_3!(c_scale_peer10, c_scale_inv10, c_scale_label10);
            }
            11 => {
                set_3!(c_scale_peer11, c_scale_inv11, c_scale_label11);
            }
            _ => {}
        }
    }

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
            "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0"
                .to_string(),
        );
    }
}
