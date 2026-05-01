use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_default_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);

    tv.set_background_if_none("white");
    tv.set_primary_color_if_none("#ECECFF");
    tv.set_secondary_color_if_none("#ffffde");
    tv.set_line_color_if_none("#333333");
    tv.set_border1_if_none("#9370DB");
    tv.set_border2_if_none("#aaaa33");
    tv.set_arrowhead_color_if_none("#333333");
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");
    tv.set_text_color_if_none("#333");
    tv.set_label_background_if_none("rgba(232,232,232, 0.8)");
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#333" });
    tv.set_main_bkg_if_none("#ECECFF");
    tv.set_second_bkg_if_none("#ffffde");
    tv.set_note_bkg_color_if_none("#fff5ad");
    tv.set_activation_border_color_if_none("#666");
    tv.set_activation_bkg_color_if_none("#f4f4f4");
    tv.set_sequence_number_color_if_none("white");

    let primary_color = tv
        .primary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ECECFF")
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

    // mkBorder
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

    let line_color = tv
        .line_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333333")
        .to_string();
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#333" })
        .to_string();
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
    let tertiary_color = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("hsl(80, 100%, 96.2745098039%)")
        .to_string();

    tv.set_node_bkg_if_none(&primary_color);
    tv.set_node_border_if_none(&primary_border_color);
    tv.set_cluster_bkg_if_none(&tertiary_color);
    tv.set_cluster_border_if_none(&tertiary_border_color);
    tv.set_node_text_color_if_none(&primary_text_color);
    tv.set_arrowhead_color_if_none(&line_color);

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

    tv.set_error_bkg_color_if_none(&tertiary_color);
    tv.set_error_text_color_if_none(&tertiary_text_color);

    // Color scales — default theme darkens by 10 only
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

    // Pre-darkened scales for peer/inv derivation
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
    .map(|base| base.adjust_hsl(0.0, 0.0, -10.0));

    // Peer: cScalePeer1/2 special, rest darken(25)
    if tv.c_scale_peer1.is_none() {
        tv.c_scale_peer1 = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -45.0).to_string());
    }
    if tv.c_scale_peer2.is_none() {
        tv.c_scale_peer2 = Some(tertiary_hsl.adjust_hsl(0.0, 0.0, -40.0).to_string());
    }

    let label_text_color = tv
        .label_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

    for i in 0..12 {
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none() {
                    tv.$field = Some(c_scales[i].adjust_hsl(0.0, 0.0, -25.0).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none() {
                    // Default theme uses 180-degree hue rotation for invert
                    tv.$field = Some(c_scales[i].adjust_hsl(180.0, 0.0, 0.0).to_string());
                }
            };
        }
        macro_rules! set_label {
            ($field:ident) => {
                if tv.$field.is_none() {
                    tv.$field = Some(label_text_color.clone());
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

    // xyChart — unique palette for default theme
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    let bg = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("white")
        .to_string();
    if xy.background_color.is_none() {
        xy.background_color = Some(bg);
    }
    xy.fill_prime_color(primary_text_color);
    if xy.plot_color_palette.is_none() {
        xy.plot_color_palette = Some(
            "#ECECFF,#8493A6,#FFC3A0,#DCDDE1,#B8E994,#D1A36F,#C3CDE6,#FFB6C1,#496078,#F8F3E3"
                .to_string(),
        );
    }
}
