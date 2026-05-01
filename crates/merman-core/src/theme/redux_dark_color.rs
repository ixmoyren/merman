use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_redux_dark_color_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(true);

    // =========================================================================
    // Constructor defaults
    // =========================================================================
    tv.set_background_if_none("#333");
    tv.set_primary_color_if_none("#1f2020");
    tv.set_main_bkg_if_none("#111113");
    tv.set_main_contrast_color_if_none("lightgrey");
    tv.set_border1_if_none("#ccc");
    tv.set_border2_if_none("rgba(255,255,255,0.25)");
    tv.set_label_background_if_none("#111113");
    tv.set_text_color_if_none("#ccc");
    tv.set_font_family_if_none("\"Recursive Variable\", arial, sans-serif");
    tv.set_font_size_if_none("14px");
    if tv.radius.is_none() {
        tv.radius = Some(12.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(2.0);
    }
    tv.set_note_bkg_color_if_none("#FEF9C3");
    tv.set_note_text_color_if_none("#28253D");
    tv.set_node_border_if_none("#FFFFFF");
    tv.set_state_border_if_none("#FFFFFF");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(false);
    }
    tv.set_drop_shadow_if_none("url(#drop-shadow)");
    if tv.node_shadow.is_none() {
        tv.node_shadow = Some(true);
    }
    tv.set_cluster_bkg_if_none("#1E1A2E");
    tv.set_cluster_border_if_none("#BDBCCC");
    tv.set_note_border_color_if_none("#FACC15");
    tv.set_actor_border_if_none("#FFFFFF");
    tv.set_signal_color_if_none("#FFFFFF");
    tv.set_filter_color_if_none("#FFFFFF");
    tv.set_label_box_border_color_if_none("#BDBCCC");
    tv.set_note_font_weight_if_none("600");
    tv.set_font_weight_if_none("600");

    // State-specific dark backgrounds
    tv.set_composite_background_if_none("#16141F");
    tv.set_alt_background_if_none("#16141F");
    tv.set_composite_title_background_if_none("#16141F");
    tv.set_state_edge_label_background_if_none("#16141F");

    // ER diagram edge label
    tv.set_er_edge_label_background_if_none("#16141F");

    // Requirement diagram edge label
    tv.set_requirement_edge_label_background_if_none("#16141F");

    // Mindmap
    tv.set_root_label_color_if_none("#FFFFFF");

    // =========================================================================
    // primaryTextColor — depends on dark_mode
    // =========================================================================
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#FFFFFF" });
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#FFFFFF" })
        .to_string();

    // =========================================================================
    // primaryBorderColor = invert(background)  [constructor]
    // =========================================================================
    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

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
    let primary_border_color = tv
        .primary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();

    // =========================================================================
    // Derived colors from primaryColor
    // =========================================================================
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

    // secondaryColor = adjust(primaryColor, {h: -120})
    if !is_truthy(&tv.secondary_color) {
        tv.secondary_color = Some(primary_hsl.adjust_hsl(-120.0, 0.0, 0.0).to_string());
    }
    let secondary_hsl = tv
        .secondary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(-120.0, 0.0, 0.0));

    // tertiaryColor = adjust(primaryColor, {h: 180, l: 5})
    if !is_truthy(&tv.tertiary_color) {
        tv.tertiary_color = Some(primary_hsl.adjust_hsl(180.0, 0.0, 5.0).to_string());
    }
    let tertiary_hsl = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(180.0, 0.0, 5.0));

    // =========================================================================
    // Border colors via mkBorder — dark: adjust_hsl(0, -40, 10)
    // =========================================================================
    let mk_border_delta_l = if dark_mode { 10.0 } else { -10.0 };

    // secondaryBorderColor = mkBorder(secondaryColor, darkMode)
    if !is_truthy(&tv.secondary_border_color) {
        tv.secondary_border_color = Some(
            secondary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_delta_l)
                .to_string(),
        );
    }

    // tertiaryBorderColor = mkBorder(tertiaryColor, darkMode)
    if !is_truthy(&tv.tertiary_border_color) {
        tv.tertiary_border_color = Some(
            tertiary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_delta_l)
                .to_string(),
        );
    }
    let tertiary_border_color = tv
        .tertiary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();

    // =========================================================================
    // Text colors via invert
    // =========================================================================
    // secondaryTextColor = invert(secondaryColor)
    if !is_truthy(&tv.secondary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(secondary_hsl.to_string())
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

    // tertiaryTextColor = invert(tertiaryColor)
    if !is_truthy(&tv.tertiary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(tertiary_hsl.to_string())
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
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();

    // =========================================================================
    // lineColor = invert(background)
    // =========================================================================
    if !is_truthy(&tv.line_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&background)
    {
        tv.line_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }
    let line_color = tv
        .line_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();

    // arrowheadColor = invert(background)
    tv.set_arrowhead_color_if_none(&line_color);

    // textColor = primaryTextColor
    tv.set_text_color_if_none(&primary_text_color);

    // border2 = tertiaryBorderColor
    tv.set_border2_if_none(&tertiary_border_color);

    // =========================================================================
    // Flowchart variables
    // =========================================================================
    tv.set_node_bkg_if_none(&primary_color);
    tv.set_main_bkg_if_none(&primary_color);
    tv.set_node_border_if_none(&primary_border_color);
    tv.set_cluster_bkg_if_none(tertiary_hsl.to_string());
    tv.set_cluster_border_if_none(&tertiary_border_color);
    tv.set_default_link_color_if_none(&line_color);
    tv.set_title_color_if_none(&tertiary_text_color);
    tv.set_node_text_color_if_none(&primary_text_color);

    // edgeLabelBackground = darkMode ? darken(secondaryColor, 30) : secondaryColor
    if !is_truthy(&tv.edge_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.edge_label_background = Some(v.to_string());
    }

    // =========================================================================
    // Sequence Diagram variables
    // =========================================================================
    tv.set_actor_bkg_if_none(&primary_color);
    tv.set_actor_text_color_if_none(&primary_text_color);
    tv.set_actor_line_color_if_none(&primary_border_color);
    tv.set_label_box_bkg_color_if_none(&primary_color);
    tv.set_signal_text_color_if_none(&primary_text_color);
    tv.set_label_text_color_if_none(&primary_text_color);
    tv.set_loop_text_color_if_none(&primary_text_color);

    // activationBorderColor = darken(secondaryColor, 10)
    if !is_truthy(&tv.activation_border_color) {
        tv.activation_border_color = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    }
    tv.set_activation_bkg_color_if_none(secondary_hsl.to_string());

    // sequenceNumberColor = invert(lineColor)
    if !is_truthy(&tv.sequence_number_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&line_color)
    {
        tv.sequence_number_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }

    tv.set_person_border_if_none(&primary_border_color);
    tv.set_person_bkg_if_none(&primary_color);

    // =========================================================================
    // Gantt chart variables
    // =========================================================================
    tv.set_section_bkg_color_if_none(tertiary_hsl.to_string());
    tv.set_alt_section_bkg_color_if_none("white");
    tv.set_section_bkg_color2_if_none(&primary_color);
    tv.set_exclude_bkg_color_if_none("#eeeeee");
    tv.set_task_border_color_if_none(&primary_border_color);
    tv.set_task_bkg_color_if_none(&primary_color);
    tv.set_active_task_border_color_if_none(&primary_color);
    // activeTaskBkgColor = lighten(primaryColor, 23)
    tv.set_active_task_bkg_color_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 23.0).to_string());
    tv.set_grid_color_if_none("lightgrey");
    tv.set_done_task_bkg_color_if_none("lightgrey");
    tv.set_done_task_border_color_if_none("grey");
    tv.set_crit_border_color_if_none("#ff8888");
    tv.set_crit_bkg_color_if_none("red");
    tv.set_today_line_color_if_none("red");
    tv.set_vert_line_color_if_none(&primary_border_color);
    tv.set_task_text_color_if_none(&primary_text_color);
    tv.set_task_text_outside_color_if_none(&primary_text_color);
    tv.set_task_text_light_color_if_none(&primary_text_color);
    tv.set_task_text_dark_color_if_none(&primary_text_color);
    tv.set_task_text_clickable_color_if_none("#003163");

    // =========================================================================
    // Architecture Diagram variables
    // =========================================================================
    tv.set_arch_edge_color_if_none(&line_color);
    tv.set_arch_edge_arrow_color_if_none(&line_color);
    tv.set_arch_edge_width_if_none("3");
    tv.set_arch_group_border_color_if_none(&primary_border_color);
    tv.set_arch_group_border_width_if_none("2px");

    // =========================================================================
    // State colors
    // =========================================================================
    tv.set_transition_color_if_none(&line_color);
    tv.set_transition_label_color_if_none(&primary_text_color);
    tv.set_state_label_color_if_none(&primary_text_color);
    tv.set_state_bkg_if_none(&primary_color);
    tv.set_label_background_color_if_none(&primary_color);
    tv.set_composite_border_if_none(&primary_border_color);
    tv.set_inner_end_background_if_none(&primary_border_color);
    tv.set_error_bkg_color_if_none(tertiary_hsl.to_string());
    tv.set_error_text_color_if_none(&tertiary_text_color);
    tv.set_special_state_color_if_none(&line_color);

    // =========================================================================
    // cScale0-11 — Hardcoded Tailwind 300-level colors
    // =========================================================================
    let c_scale_colors: [&str; 12] = [
        "#f4a8ff", "#46ecd5", "#ffb86a", "#dab2ff", "#7bf1a8", "#c4b4ff", "#ffa2a2", "#ffdf20",
        "#a3b3ff", "#bbf451", "#74d4ff", "#ffa1ad",
    ];

    tv.set_c_scale0_if_none(c_scale_colors[0]);
    tv.set_c_scale1_if_none(c_scale_colors[1]);
    tv.set_c_scale2_if_none(c_scale_colors[2]);
    tv.set_c_scale3_if_none(c_scale_colors[3]);
    tv.set_c_scale4_if_none(c_scale_colors[4]);
    tv.set_c_scale5_if_none(c_scale_colors[5]);
    tv.set_c_scale6_if_none(c_scale_colors[6]);
    tv.set_c_scale7_if_none(c_scale_colors[7]);
    tv.set_c_scale8_if_none(c_scale_colors[8]);
    tv.set_c_scale9_if_none(c_scale_colors[9]);
    tv.set_c_scale10_if_none(c_scale_colors[10]);
    tv.set_c_scale11_if_none(c_scale_colors[11]);

    // cScalePeer = lighten(10) for dark, darken(10) for light
    // cScaleInv = invert
    // cScaleLabel = darken(cScale, 75) — UNIQUE to redux-dark-color!
    let peer_delta = if dark_mode { 10.0 } else { -10.0 };

    for i in 0..12 {
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scale_colors[i])
                {
                    tv.$field = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scale_colors[i])
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
        macro_rules! set_label {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scale_colors[i])
                {
                    // cScaleLabel[i] = darken(cScale[i], 75)
                    tv.$field = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, -75.0).to_string());
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

    // =========================================================================
    // surface0-4, surfacePeer0-4 — adjust mainBkg
    // =========================================================================
    let surface_multiplier: f64 = if dark_mode { -4.0 } else { -1.0 };
    let main_bkg_hsl = Rgb::try_from(
        tv.main_bkg
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("#111113"),
    )
    .map(Hsl::from)
    .unwrap_or(primary_hsl);

    for i in 0..5 {
        let surf_l = surface_multiplier * (5.0 + i as f64 * 3.0);
        let surf_peer_l = surface_multiplier * (8.0 + i as f64 * 3.0);
        let surf = main_bkg_hsl.adjust_hsl(180.0, -15.0, surf_l).to_string();
        let surf_peer = main_bkg_hsl
            .adjust_hsl(180.0, -15.0, surf_peer_l)
            .to_string();

        match i {
            0 => {
                tv.set_surface0_if_none(&surf);
                tv.set_surface_peer0_if_none(&surf_peer);
            }
            1 => {
                tv.set_surface1_if_none(&surf);
                tv.set_surface_peer1_if_none(&surf_peer);
            }
            2 => {
                tv.set_surface2_if_none(&surf);
                tv.set_surface_peer2_if_none(&surf_peer);
            }
            3 => {
                tv.set_surface3_if_none(&surf);
                tv.set_surface_peer3_if_none(&surf_peer);
            }
            4 => {
                tv.set_surface4_if_none(&surf);
                tv.set_surface_peer4_if_none(&surf_peer);
            }
            _ => {}
        }
    }

    // =========================================================================
    // classText
    // =========================================================================
    tv.set_class_text_if_none(&primary_text_color);

    // =========================================================================
    // fillType0-7 — user-journey
    // =========================================================================
    tv.set_fill_type0_if_none(&primary_color);
    tv.set_fill_type1_if_none(secondary_hsl.to_string());
    tv.set_fill_type2_if_none(primary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type3_if_none(secondary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type4_if_none(primary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type5_if_none(secondary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type6_if_none(primary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());
    tv.set_fill_type7_if_none(secondary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());

    // =========================================================================
    // pie1-12 + pie settings
    // =========================================================================
    tv.set_pie1_if_none(&primary_color);
    tv.set_pie2_if_none(secondary_hsl.to_string());
    tv.set_pie3_if_none(tertiary_hsl.to_string());
    tv.set_pie4_if_none(primary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie5_if_none(secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie6_if_none(tertiary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie7_if_none(primary_hsl.adjust_hsl(60.0, 0.0, -10.0).to_string());
    tv.set_pie8_if_none(primary_hsl.adjust_hsl(-60.0, 0.0, -10.0).to_string());
    tv.set_pie9_if_none(primary_hsl.adjust_hsl(120.0, 0.0, 0.0).to_string());
    tv.set_pie10_if_none(primary_hsl.adjust_hsl(60.0, 0.0, -20.0).to_string());
    tv.set_pie11_if_none(primary_hsl.adjust_hsl(-60.0, 0.0, -20.0).to_string());
    tv.set_pie12_if_none(primary_hsl.adjust_hsl(120.0, 0.0, -10.0).to_string());
    tv.set_pie_title_text_size_if_none("25px");
    tv.set_pie_title_text_color_if_none(&primary_text_color);
    tv.set_pie_section_text_size_if_none("17px");
    tv.set_pie_section_text_color_if_none(&primary_text_color);
    tv.set_pie_legend_text_size_if_none("17px");
    tv.set_pie_legend_text_color_if_none(&primary_text_color);
    tv.set_pie_stroke_color_if_none("black");
    tv.set_pie_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_color_if_none("black");
    tv.set_pie_opacity_if_none("0.7");

    // =========================================================================
    // Venn
    // =========================================================================
    tv.set_venn_title_text_color_if_none(&primary_text_color);
    tv.set_venn_set_text_color_if_none(&primary_text_color);

    // =========================================================================
    // Quadrant
    // =========================================================================
    tv.set_quadrant1_fill_if_none(&primary_color);
    // quadrant2Fill = adjust(primaryColor, {r:5, g:5, b:5}) — approximated as lighten
    tv.set_quadrant2_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 2.0).to_string());
    tv.set_quadrant3_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 4.0).to_string());
    tv.set_quadrant4_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 6.0).to_string());
    tv.set_quadrant1_text_fill_if_none(&primary_text_color);
    tv.set_quadrant2_text_fill_if_none(&primary_text_color);
    tv.set_quadrant3_text_fill_if_none(&primary_text_color);
    tv.set_quadrant4_text_fill_if_none(&primary_text_color);
    // quadrantPointFill = isDark(quadrant1Fill) ? lighten : darken — dark theme
    tv.set_quadrant_point_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    tv.set_quadrant_point_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_x_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_y_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_internal_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_external_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_title_fill_if_none(&primary_text_color);

    // =========================================================================
    // Requirement diagram
    // =========================================================================
    tv.set_requirement_background_if_none(&primary_color);
    tv.set_requirement_border_color_if_none(&primary_border_color);
    tv.set_requirement_border_size_if_none("1");
    tv.set_requirement_text_color_if_none(&primary_text_color);
    tv.set_relation_color_if_none(&line_color);

    // relationLabelBackground = darkMode ? darken(secondaryColor, 30) : secondaryColor
    if !is_truthy(&tv.relation_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.relation_label_background = Some(v.to_string());
    }
    tv.set_relation_label_color_if_none(&primary_text_color);

    // =========================================================================
    // Git
    // =========================================================================
    let git_adj: f64 = if dark_mode { 25.0 } else { -25.0 };

    // git0 = lighten/darken(primaryColor, 25)
    // git1 = lighten/darken(secondaryColor, 25)
    // git2 = lighten/darken(tertiaryColor, 25)
    // git3 = lighten/darken(adjust(primaryColor, {h:-30}), 25)
    // etc.
    let git_hsl: [Hsl; 8] = [
        primary_hsl,
        secondary_hsl,
        tertiary_hsl,
        primary_hsl.adjust_hsl(-30.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(-60.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(-90.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0),
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0),
    ]
    .map(|hsl| hsl.adjust_hsl(0.0, 0.0, git_adj));

    tv.set_git0_if_none(git_hsl[0].to_string());
    tv.set_git1_if_none(git_hsl[1].to_string());
    tv.set_git2_if_none(git_hsl[2].to_string());
    tv.set_git3_if_none(git_hsl[3].to_string());
    tv.set_git4_if_none(git_hsl[4].to_string());
    tv.set_git5_if_none(git_hsl[5].to_string());
    tv.set_git6_if_none(git_hsl[6].to_string());
    tv.set_git7_if_none(git_hsl[7].to_string());

    // gitInv = invert(git)
    for i in 0..8 {
        if let Ok(Rgb { r, g, b }) = Rgb::try_from(git_hsl[i].to_string()) {
            let inv = Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string();
            match i {
                0 => tv.set_git_inv0_if_none(&inv),
                1 => tv.set_git_inv1_if_none(&inv),
                2 => tv.set_git_inv2_if_none(&inv),
                3 => tv.set_git_inv3_if_none(&inv),
                4 => tv.set_git_inv4_if_none(&inv),
                5 => tv.set_git_inv5_if_none(&inv),
                6 => tv.set_git_inv6_if_none(&inv),
                7 => tv.set_git_inv7_if_none(&inv),
                _ => {}
            }
        }
    }

    // branchLabelColor = darkMode ? 'black' : labelTextColor
    let branch_label = if dark_mode {
        "black"
    } else {
        &primary_text_color
    };
    tv.set_branch_label_color_if_none(branch_label);
    tv.set_git_branch_label0_if_none(branch_label);
    tv.set_git_branch_label1_if_none(branch_label);
    tv.set_git_branch_label2_if_none(branch_label);
    tv.set_git_branch_label3_if_none(branch_label);
    tv.set_git_branch_label4_if_none(branch_label);
    tv.set_git_branch_label5_if_none(branch_label);
    tv.set_git_branch_label6_if_none(branch_label);
    tv.set_git_branch_label7_if_none(branch_label);

    tv.set_tag_label_color_if_none(&primary_text_color);
    tv.set_tag_label_background_if_none(&primary_color);
    tv.set_tag_label_border_if_none(&primary_border_color);
    tv.set_tag_label_font_size_if_none("10px");
    tv.set_commit_label_color_if_none(&primary_text_color);
    tv.set_commit_label_background_if_none(secondary_hsl.to_string());
    tv.set_commit_label_font_size_if_none("10px");
    tv.set_commit_line_color_if_none("#BDBCCC");

    // =========================================================================
    // ER diagram
    // =========================================================================
    tv.set_attribute_background_color_odd_if_none("#ffffff");
    tv.set_attribute_background_color_even_if_none("#f2f2f2");

    // =========================================================================
    // borderColorArray — Tailwind 400-level
    // =========================================================================
    tv.set_border_color_array_if_none(vec![
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

    // bkgColorArray is empty in JS — leave as None (default)

    // =========================================================================
    // xyChart
    // =========================================================================
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    xy.set_background_color_if_none(&background);
    xy.set_plot_color_palette_if_none(
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );
    xy.fill_prime_color(primary_text_color);
}
