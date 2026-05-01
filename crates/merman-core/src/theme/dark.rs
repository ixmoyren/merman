use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_dark_theme_defaults(tv: &mut ThemeVariables) {
    let c_scales_hex: [&str; 12] = [
        "#1f2020", "#0b0000", "#4d1037", "#3f5258", "#4f2f1b", "#6e0a0a", "#3b0048", "#995a01",
        "#154706", "#161722", "#00296f", "#01629c",
    ];

    tv.set_background_if_none("#333");
    tv.set_primary_color_if_none("#1f2020");

    if !is_truthy(&tv.primary_text_color)
        && let Some(ref primary_color) = tv.primary_color
        && is_truthy(&tv.primary_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(primary_color)
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
    tv.set_text_color_if_none("#ccc");
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");
    tv.set_border1_if_none("#ccc");
    tv.set_border2_if_none("rgba(255, 255, 255, 0.25)");
    tv.set_label_background_if_none("#181818");
    tv.set_title_color_if_none("#F9FFFE");
    tv.set_error_bkg_color_if_none("#a44141");
    tv.set_error_text_color_if_none("#ddd");
    tv.set_label_text_color_if_none("lightgrey");

    let label_text_color = tv
        .label_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("lightgrey")
        .to_string();
    tv.set_scale_label_color_if_none(&label_text_color);

    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&label_text_color)
        .to_string();

    macro_rules! set_scale {
        ($field:ident, $hex:expr) => {
            if tv.$field.is_none() {
                tv.$field = Some($hex.to_string());
            }
        };
    }
    macro_rules! set_scale_peer {
        ($field:ident, $hex:expr) => {
            if tv.$field.is_none()
                && let Ok(rgb) = Rgb::try_from($hex)
            {
                let hsl = Hsl::from(rgb);
                tv.$field = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
            }
        };
    }
    macro_rules! set_scale_inv {
        ($field:ident, $hex:expr) => {
            if tv.$field.is_none()
                && let Ok(Rgb { r, g, b }) = Rgb::try_from($hex)
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
        ($field:ident) => {
            if tv.$field.is_none() {
                tv.$field = Some(scale_label_color.clone());
            }
        };
    }

    set_scale!(c_scale0, c_scales_hex[0]);
    set_scale_peer!(c_scale_peer0, c_scales_hex[0]);
    set_scale_inv!(c_scale_inv0, c_scales_hex[0]);
    set_scale_label!(c_scale_label0);

    set_scale!(c_scale1, c_scales_hex[1]);
    set_scale_peer!(c_scale_peer1, c_scales_hex[1]);
    set_scale_inv!(c_scale_inv1, c_scales_hex[1]);
    set_scale_label!(c_scale_label1);

    set_scale!(c_scale2, c_scales_hex[2]);
    set_scale_peer!(c_scale_peer2, c_scales_hex[2]);
    set_scale_inv!(c_scale_inv2, c_scales_hex[2]);
    set_scale_label!(c_scale_label2);

    set_scale!(c_scale3, c_scales_hex[3]);
    set_scale_peer!(c_scale_peer3, c_scales_hex[3]);
    set_scale_inv!(c_scale_inv3, c_scales_hex[3]);
    set_scale_label!(c_scale_label3);

    set_scale!(c_scale4, c_scales_hex[4]);
    set_scale_peer!(c_scale_peer4, c_scales_hex[4]);
    set_scale_inv!(c_scale_inv4, c_scales_hex[4]);
    set_scale_label!(c_scale_label4);

    set_scale!(c_scale5, c_scales_hex[5]);
    set_scale_peer!(c_scale_peer5, c_scales_hex[5]);
    set_scale_inv!(c_scale_inv5, c_scales_hex[5]);
    set_scale_label!(c_scale_label5);

    set_scale!(c_scale6, c_scales_hex[6]);
    set_scale_peer!(c_scale_peer6, c_scales_hex[6]);
    set_scale_inv!(c_scale_inv6, c_scales_hex[6]);
    set_scale_label!(c_scale_label6);

    set_scale!(c_scale7, c_scales_hex[7]);
    set_scale_peer!(c_scale_peer7, c_scales_hex[7]);
    set_scale_inv!(c_scale_inv7, c_scales_hex[7]);
    set_scale_label!(c_scale_label7);

    set_scale!(c_scale8, c_scales_hex[8]);
    set_scale_peer!(c_scale_peer8, c_scales_hex[8]);
    set_scale_inv!(c_scale_inv8, c_scales_hex[8]);
    set_scale_label!(c_scale_label8);

    set_scale!(c_scale9, c_scales_hex[9]);
    set_scale_peer!(c_scale_peer9, c_scales_hex[9]);
    set_scale_inv!(c_scale_inv9, c_scales_hex[9]);
    set_scale_label!(c_scale_label9);

    set_scale!(c_scale10, c_scales_hex[10]);
    set_scale_peer!(c_scale_peer10, c_scales_hex[10]);
    set_scale_inv!(c_scale_inv10, c_scales_hex[10]);
    set_scale_label!(c_scale_label10);

    set_scale!(c_scale11, c_scales_hex[11]);
    set_scale_peer!(c_scale_peer11, c_scales_hex[11]);
    set_scale_inv!(c_scale_inv11, c_scales_hex[11]);
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
            "#3498db,#2ecc71,#e74c3c,#f1c40f,#bdc3c7,#ffffff,#34495e,#9b59b6,#1abc9c,#e67e22"
                .to_string(),
        );
    }
}
