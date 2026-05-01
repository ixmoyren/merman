use crate::color::{Hsl, Rgb};
use crate::theme::variables::ThemeVariables;

pub(crate) fn apply_redux_color_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);

    tv.set_background_if_none("#ffffff");
    tv.set_primary_color_if_none("#cccccc");
    tv.set_main_bkg_if_none("#ffffff");
    tv.set_note_bkg_color_if_none("#fff5ad");
    tv.set_note_text_color_if_none("#28253D");
    if tv.radius.is_none() {
        tv.radius = Some(12.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(2.0);
    }
    tv.set_primary_border_color_if_none("#28253D");
    tv.set_font_family_if_none("\"Recursive Variable\", arial, sans-serif");
    tv.set_font_size_if_none("14px");
    tv.set_node_border_if_none("#28253D");
    tv.set_state_border_if_none("#28253D");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(false);
    }
    tv.set_drop_shadow_if_none("url(#drop-shadow)");
    if tv.node_shadow.is_none() {
        tv.node_shadow = Some(true);
    }
    tv.set_tertiary_color_if_none("#ffffff");
    tv.set_cluster_bkg_if_none("#F9F9FB");
    tv.set_cluster_border_if_none("#BDBCCC");
    tv.set_note_border_color_if_none("#FACC15");
    tv.set_actor_border_if_none("#28253D");
    tv.set_filter_color_if_none("#000000");
    tv.set_composite_title_background_if_none("#F9F9FB");
    tv.set_alt_background_if_none("#F9F9FB");
    tv.set_state_edge_label_background_if_none("#FFFFFF");
    tv.set_er_edge_label_background_if_none("#FFFFFF");
    tv.set_commit_line_color_if_none("#BDBCCC");
    tv.set_font_weight_if_none("600");
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#28253D" });

    // Hardcoded Tailwind 300-level colors — no darkening applied
    tv.set_c_scale0_if_none("#f4a8ff".to_string());
    tv.set_c_scale1_if_none("#46ecd5".to_string());
    tv.set_c_scale2_if_none("#ffb86a".to_string());
    tv.set_c_scale3_if_none("#dab2ff".to_string());
    tv.set_c_scale4_if_none("#7bf1a8".to_string());
    tv.set_c_scale5_if_none("#c4b4ff".to_string());
    tv.set_c_scale6_if_none("#ffa2a2".to_string());
    tv.set_c_scale7_if_none("#ffdf20".to_string());
    tv.set_c_scale8_if_none("#a3b3ff".to_string());
    tv.set_c_scale9_if_none("#bbf451".to_string());
    tv.set_c_scale10_if_none("#74d4ff".to_string());
    tv.set_c_scale11_if_none("#ffa1ad".to_string());

    let c_scale_colors: [&str; 12] = [
        "#f4a8ff", "#46ecd5", "#ffb86a", "#dab2ff", "#7bf1a8", "#c4b4ff", "#ffa2a2", "#ffdf20",
        "#a3b3ff", "#bbf451", "#74d4ff", "#ffa1ad",
    ];

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
        macro_rules! set_3 {
            ($peer:ident, $inv:ident, $label:ident) => {
                if tv.$peer.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scale_colors[i])
                {
                    tv.$peer = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
                if tv.$inv.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scale_colors[i])
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

    // Tailwind color arrays
    if tv.border_color_array.is_none() {
        tv.border_color_array = Some(vec![
            "#E879F9".into(),
            "#2DD4BF".into(),
            "#FB923C".into(),
            "#22D3EE".into(),
            "#4ADE80".into(),
            "#A78BFA".into(),
            "#F87171".into(),
            "#FACC15".into(),
            "#818CF8".into(),
            "#A3E635".into(),
            "#38BDF8".into(),
            "#FB7185".into(),
        ]);
    }
    if tv.bkg_color_array.is_none() {
        tv.bkg_color_array = Some(vec![
            "#FDF4FF".into(),
            "#F0FDFA".into(),
            "#FFF7ED".into(),
            "#ECFEFF".into(),
            "#F0FDF4".into(),
            "#F5F3FF".into(),
            "#FEF2F2".into(),
            "#FEFCE8".into(),
            "#EEF2FF".into(),
            "#F7FEE7".into(),
            "#F0F9FF".into(),
            "#FFF1F2".into(),
        ]);
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
