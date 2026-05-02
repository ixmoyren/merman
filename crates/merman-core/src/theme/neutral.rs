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

    tv.set_label_text_color_if_none("#333");
    tv.set_scale_label_color_if_none("#333");
    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

    let c_scales_hex: [&str; 12] = [
        "#555", "#F4F4F4", "#555", "#BBB", "#777", "#999", "#DDD", "#FFF", "#DDD", "#BBB", "#999",
        "#777",
    ];

    tv.set_c_scale0_if_none(c_scales_hex[0]);
    tv.set_c_scale1_if_none(c_scales_hex[1]);
    tv.set_c_scale2_if_none(c_scales_hex[2]);
    tv.set_c_scale3_if_none(c_scales_hex[3]);
    tv.set_c_scale4_if_none(c_scales_hex[4]);
    tv.set_c_scale5_if_none(c_scales_hex[5]);
    tv.set_c_scale6_if_none(c_scales_hex[6]);
    tv.set_c_scale7_if_none(c_scales_hex[7]);
    tv.set_c_scale8_if_none(c_scales_hex[8]);
    tv.set_c_scale9_if_none(c_scales_hex[9]);
    tv.set_c_scale10_if_none(c_scales_hex[10]);
    tv.set_c_scale11_if_none(c_scales_hex[11]);

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

    set_scale_peer!(c_scale_peer0, 0);
    set_scale_inv!(c_scale_inv0, 0);
    set_scale_label!(c_scale_label0, 0);

    set_scale_peer!(c_scale_peer1, 1);
    set_scale_inv!(c_scale_inv1, 1);
    set_scale_label!(c_scale_label1, 1);

    set_scale_peer!(c_scale_peer2, 2);
    set_scale_inv!(c_scale_inv2, 2);
    set_scale_label!(c_scale_label2, 2);

    set_scale_peer!(c_scale_peer3, 3);
    set_scale_inv!(c_scale_inv3, 3);
    set_scale_label!(c_scale_label3, 3);

    set_scale_peer!(c_scale_peer4, 4);
    set_scale_inv!(c_scale_inv4, 4);
    set_scale_label!(c_scale_label4, 4);

    set_scale_peer!(c_scale_peer5, 5);
    set_scale_inv!(c_scale_inv5, 5);
    set_scale_label!(c_scale_label5, 5);

    set_scale_peer!(c_scale_peer6, 6);
    set_scale_inv!(c_scale_inv6, 6);
    set_scale_label!(c_scale_label6, 6);

    set_scale_peer!(c_scale_peer7, 7);
    set_scale_inv!(c_scale_inv7, 7);
    set_scale_label!(c_scale_label7, 7);

    set_scale_peer!(c_scale_peer8, 8);
    set_scale_inv!(c_scale_inv8, 8);
    set_scale_label!(c_scale_label8, 8);

    set_scale_peer!(c_scale_peer9, 9);
    set_scale_inv!(c_scale_inv9, 9);
    set_scale_label!(c_scale_label9, 9);

    set_scale_peer!(c_scale_peer10, 10);
    set_scale_inv!(c_scale_inv10, 10);
    set_scale_label!(c_scale_label10, 10);

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
    xy.set_background_color_if_none(bg);
    xy.set_plot_color_palette_if_none(
        "#EEE,#6BB8E4,#8ACB88,#C7ACD6,#E8DCC2,#FFB2A8,#FFF380,#7E8D91,#FFD8B1,#FAF3E0",
    );
    xy.fill_prime_color(pt);
}
#[cfg(test)]
mod tests {
    use crate::theme::variables::ThemeVariables;

    static NEUTRAL_THEME_JSON: &str = include_str!("../../assets/theme/neutral.json");

    #[test]
    fn compare_with_mermaid_theme_json() {
        let mut base = ThemeVariables::default();
        super::apply_neutral_theme_defaults(&mut base);
        let json = serde_json::to_string_pretty(&base).unwrap();
        let from_mermaid = serde_json::from_str::<ThemeVariables>(NEUTRAL_THEME_JSON).unwrap();
        let from_mermaid_json = serde_json::to_string_pretty(&from_mermaid).unwrap();
        let diff =
            similar_asserts::SimpleDiff::from_str(&json, &from_mermaid_json, "merman", "mermaid");
        println!("{}", diff);
    }
}
