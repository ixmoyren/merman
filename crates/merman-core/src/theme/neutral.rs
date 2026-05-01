use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_neutral_theme_defaults(tv: &mut ThemeVariables) {
    tv.set_background_if_none("#ffffff");
    tv.set_primary_color_if_none("#eee");

    if !is_truthy(&tv.primary_text_color)
        && let Some(ref primary_color) = tv.primary_color
        && is_truthy(&tv.primary_color)
        && let Ok(rgb) = Rgb::try_from(primary_color)
    {
        tv.primary_text_color = Some(
            Rgb {
                r: 1.0 - rgb.r,
                g: 1.0 - rgb.g,
                b: 1.0 - rgb.b,
            }
            .to_string(),
        );
    }

    let c_scales_hex: [&str; 12] = [
        "#555", "#F4F4F4", "#555", "#BBB", "#777", "#999", "#DDD", "#FFF", "#DDD", "#BBB", "#999",
        "#777",
    ];

    tv.set_label_text_color_if_none("#333");
    tv.set_scale_label_color_if_none("#333");
    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

    macro_rules! set_scale {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none() {
                tv.$field = Some(c_scales_hex[$idx].to_string());
            }
        };
    }
    macro_rules! set_scale_peer {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none()
                && let Ok(rgb) = Rgb::try_from(c_scales_hex[$idx])
            {
                let hsl = Hsl::from(rgb);
                tv.$field = Some(hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
            }
        };
    }
    macro_rules! set_scale_inv {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none()
                && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scales_hex[$idx])
            {
                tv.$field = Some(
                    Rgb {
                        r: 1.0 - r,
                        g: 1.0 - g,
                        b: 1.0 - b,
                    }
                    .to_string(),
                );
            }
        };
    }
    macro_rules! set_scale_label {
        ($field:ident, $idx:literal) => {
            if tv.$field.is_none() {
                if $idx == 0 || $idx == 2 {
                    tv.$field = Some(c_scales_hex[1].to_string());
                } else {
                    tv.$field = Some(scale_label_color.clone());
                }
            }
        };
    }

    set_scale!(c_scale0, 0);
    set_scale_peer!(c_scale_peer0, 0);
    set_scale_inv!(c_scale_inv0, 0);
    set_scale_label!(c_scale_label0, 0);

    set_scale!(c_scale1, 1);
    set_scale_peer!(c_scale_peer1, 1);
    set_scale_inv!(c_scale_inv1, 1);
    set_scale_label!(c_scale_label1, 1);

    set_scale!(c_scale2, 2);
    set_scale_peer!(c_scale_peer2, 2);
    set_scale_inv!(c_scale_inv2, 2);
    set_scale_label!(c_scale_label2, 2);

    set_scale!(c_scale3, 3);
    set_scale_peer!(c_scale_peer3, 3);
    set_scale_inv!(c_scale_inv3, 3);
    set_scale_label!(c_scale_label3, 3);

    set_scale!(c_scale4, 4);
    set_scale_peer!(c_scale_peer4, 4);
    set_scale_inv!(c_scale_inv4, 4);
    set_scale_label!(c_scale_label4, 4);

    set_scale!(c_scale5, 5);
    set_scale_peer!(c_scale_peer5, 5);
    set_scale_inv!(c_scale_inv5, 5);
    set_scale_label!(c_scale_label5, 5);

    set_scale!(c_scale6, 6);
    set_scale_peer!(c_scale_peer6, 6);
    set_scale_inv!(c_scale_inv6, 6);
    set_scale_label!(c_scale_label6, 6);

    set_scale!(c_scale7, 7);
    set_scale_peer!(c_scale_peer7, 7);
    set_scale_inv!(c_scale_inv7, 7);
    set_scale_label!(c_scale_label7, 7);

    set_scale!(c_scale8, 8);
    set_scale_peer!(c_scale_peer8, 8);
    set_scale_inv!(c_scale_inv8, 8);
    set_scale_label!(c_scale_label8, 8);

    set_scale!(c_scale9, 9);
    set_scale_peer!(c_scale_peer9, 9);
    set_scale_inv!(c_scale_inv9, 9);
    set_scale_label!(c_scale_label9, 9);

    set_scale!(c_scale10, 10);
    set_scale_peer!(c_scale_peer10, 10);
    set_scale_inv!(c_scale_inv10, 10);
    set_scale_label!(c_scale_label10, 10);

    set_scale!(c_scale11, 11);
    set_scale_peer!(c_scale_peer11, 11);
    set_scale_inv!(c_scale_inv11, 11);
    set_scale_label!(c_scale_label11, 11);

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
            "#EEE,#6BB8E4,#8ACB88,#C7ACD6,#E8DCC2,#FFB2A8,#FFF380,#7E8D91,#FFD8B1,#FAF3E0"
                .to_string(),
        );
    }
}
