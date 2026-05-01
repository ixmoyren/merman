use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_neo_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);

    tv.set_background_if_none("#ffffff");
    tv.set_primary_color_if_none("#cccccc");
    tv.set_main_bkg_if_none("#ffffff");
    tv.set_note_bkg_color_if_none("#fff5ad");
    tv.set_note_text_color_if_none("#333");
    tv.set_font_family_if_none("arial, sans-serif");
    tv.set_font_size_if_none("14px");
    if tv.radius.is_none() {
        tv.radius = Some(3.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(2.0);
    }
    tv.set_node_border_if_none("#000000");
    tv.set_state_border_if_none("#000000");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(true);
    }
    tv.set_gradient_start_if_none("#0042eb");
    tv.set_gradient_stop_if_none("#eb0042");
    tv.set_drop_shadow_if_none("drop-shadow(0px 1px 2px rgba(0,0,0,0.25))");
    tv.set_tertiary_color_if_none("#ffffff");

    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#333" });

    let primary_hsl = Rgb::try_from("#cccccc").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 80.0,
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

    if !is_truthy(&tv.tertiary_color) || tv.tertiary_color.as_deref() == Some("#ffffff") {
        tv.tertiary_color = Some(primary_hsl.adjust_hsl(180.0, 0.0, 5.0).to_string());
    }
    let _tertiary_hsl = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(180.0, 0.0, 5.0));

    // Local Gantt colors for color scale derivation
    let local_primary = "#ECECFE";
    let local_primary_hsl = Rgb::try_from(local_primary).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 240.0,
        s_pct: 80.0,
        l_pct: 93.0,
    });
    let local_secondary = "#E9E9F1";
    let local_secondary_hsl = Rgb::try_from(local_secondary)
        .map(Hsl::from)
        .unwrap_or(Hsl {
            h_deg: 240.0,
            s_pct: 22.0,
            l_pct: 93.0,
        });
    let local_tertiary_hsl = local_primary_hsl.adjust_hsl(180.0, 0.0, 5.0);

    let amount = if dark_mode { 75.0 } else { 25.0 };
    tv.set_c_scale0_if_none(local_primary_hsl.adjust_hsl(0.0, 0.0, -amount).to_string());
    tv.set_c_scale1_if_none(
        local_secondary_hsl
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale2_if_none(local_tertiary_hsl.adjust_hsl(0.0, 0.0, -amount).to_string());
    tv.set_c_scale3_if_none(
        local_primary_hsl
            .adjust_hsl(30.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale4_if_none(
        local_primary_hsl
            .adjust_hsl(60.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale5_if_none(
        local_primary_hsl
            .adjust_hsl(90.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale6_if_none(
        local_primary_hsl
            .adjust_hsl(120.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale7_if_none(
        local_primary_hsl
            .adjust_hsl(150.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale8_if_none(
        local_primary_hsl
            .adjust_hsl(210.0, 0.0, 150.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale9_if_none(
        local_primary_hsl
            .adjust_hsl(270.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale10_if_none(
        local_primary_hsl
            .adjust_hsl(300.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );
    tv.set_c_scale11_if_none(
        local_primary_hsl
            .adjust_hsl(330.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -amount)
            .to_string(),
    );

    let c_scales: [Hsl; 12] = [
        local_primary_hsl,
        local_secondary_hsl,
        local_tertiary_hsl,
        local_primary_hsl.adjust_hsl(30.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(60.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(90.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(120.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(150.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(210.0, 0.0, 150.0),
        local_primary_hsl.adjust_hsl(270.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(300.0, 0.0, 0.0),
        local_primary_hsl.adjust_hsl(330.0, 0.0, 0.0),
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
        .unwrap_or("#333")
        .to_string();

    let peer_delta = if dark_mode { 10.0 } else { -10.0 };

    for i in 0..12 {
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scales[i].to_string())
                {
                    let hsl = Hsl::from(rgb);
                    tv.$field = Some(hsl.adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scales[i].to_string())
                {
                    tv.$field = Some(
                        Rgb {
                            r: 1.0 - rgb.r,
                            g: 1.0 - rgb.g,
                            b: 1.0 - rgb.b,
                        }
                        .to_string(),
                    );
                }
            };
        }
        macro_rules! set_label {
            ($field:ident) => {
                if tv.$field.is_none() {
                    tv.$field = Some(scale_label_color.clone());
                }
            };
        }

        match i {
            0 => {
                set_peer!(c_scale_peer0);
                set_inv!(c_scale_inv0);
                set_label!(c_scale_label0);
            }
            1 => {
                set_peer!(c_scale_peer1);
                set_inv!(c_scale_inv1);
                set_label!(c_scale_label1);
            }
            2 => {
                set_peer!(c_scale_peer2);
                set_inv!(c_scale_inv2);
                set_label!(c_scale_label2);
            }
            3 => {
                set_peer!(c_scale_peer3);
                set_inv!(c_scale_inv3);
                set_label!(c_scale_label3);
            }
            4 => {
                set_peer!(c_scale_peer4);
                set_inv!(c_scale_inv4);
                set_label!(c_scale_label4);
            }
            5 => {
                set_peer!(c_scale_peer5);
                set_inv!(c_scale_inv5);
                set_label!(c_scale_label5);
            }
            6 => {
                set_peer!(c_scale_peer6);
                set_inv!(c_scale_inv6);
                set_label!(c_scale_label6);
            }
            7 => {
                set_peer!(c_scale_peer7);
                set_inv!(c_scale_inv7);
                set_label!(c_scale_label7);
            }
            8 => {
                set_peer!(c_scale_peer8);
                set_inv!(c_scale_inv8);
                set_label!(c_scale_label8);
            }
            9 => {
                set_peer!(c_scale_peer9);
                set_inv!(c_scale_inv9);
                set_label!(c_scale_label9);
            }
            10 => {
                set_peer!(c_scale_peer10);
                set_inv!(c_scale_inv10);
                set_label!(c_scale_label10);
            }
            11 => {
                set_peer!(c_scale_peer11);
                set_inv!(c_scale_inv11);
                set_label!(c_scale_label11);
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
