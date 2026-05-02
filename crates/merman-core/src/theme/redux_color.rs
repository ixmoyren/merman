use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_redux_color_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);

    // =========================================================================
    // 1. Constructor defaults (hardcoded values)
    // =========================================================================
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

    // =========================================================================
    // 2. Read base colors & compute HSL representations
    // =========================================================================
    let primary_color = tv
        .primary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();

    let primary_hsl = Rgb::try_from(&primary_color).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });

    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ffffff")
        .to_string();

    // =========================================================================
    // 3. Derived main colors
    // =========================================================================

    // secondaryColor = adjust(primaryColor, {h: -120})
    let secondary_hsl = if let Some(ref v) = tv.secondary_color
        && is_truthy(&tv.secondary_color)
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else {
        primary_hsl.adjust_hsl(-120.0, 0.0, 0.0)
    };
    tv.set_secondary_color_if_none(secondary_hsl.to_string());

    // tertiaryColor = adjust(primaryColor, {h: 180, l: 5})
    // Note: constructor already set tertiary_color to "#ffffff"; this fills if a
    // user cleared it back to None/empty.
    let tertiary_hsl = if let Some(ref v) = tv.tertiary_color
        && is_truthy(&tv.tertiary_color)
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else {
        primary_hsl.adjust_hsl(180.0, 0.0, 5.0)
    };
    tv.set_tertiary_color_if_none(tertiary_hsl.to_string());

    // =========================================================================
    // 4. mkBorder (primaryBorderColor, secondaryBorderColor, tertiaryBorderColor,
    //    noteBorderColor)
    // =========================================================================
    let mk_border_l = if dark_mode { 10.0 } else { -10.0 };

    if !is_truthy(&tv.primary_border_color) {
        tv.primary_border_color = Some(primary_hsl.adjust_hsl(0.0, -40.0, mk_border_l).to_string());
    }
    if !is_truthy(&tv.secondary_border_color) {
        tv.secondary_border_color = Some(
            secondary_hsl
                .adjust_hsl(0.0, -40.0, mk_border_l)
                .to_string(),
        );
    }
    if !is_truthy(&tv.tertiary_border_color) {
        tv.tertiary_border_color =
            Some(tertiary_hsl.adjust_hsl(0.0, -40.0, mk_border_l).to_string());
    }
    if !is_truthy(&tv.note_border_color) {
        // noteBorderColor = mkBorder(noteBkgColor, darkMode)
        let note_bkg = tv
            .note_bkg_color
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("#fff5ad");
        if let Ok(rgb) = Rgb::try_from(note_bkg) {
            tv.note_border_color = Some(
                Hsl::from(rgb)
                    .adjust_hsl(0.0, -40.0, mk_border_l)
                    .to_string(),
            );
        }
    }

    // =========================================================================
    // 5. Text colors
    // =========================================================================

    // secondaryTextColor = invert(secondaryColor)
    if !is_truthy(&tv.secondary_text_color) {
        let rgb = Rgb::from(secondary_hsl);
        tv.secondary_text_color = Some(rgb.invert_from_js());
    }

    // tertiaryTextColor = invert(tertiaryColor)
    if !is_truthy(&tv.tertiary_text_color) {
        let rgb = Rgb::from(tertiary_hsl);
        tv.tertiary_text_color = Some(rgb.invert_from_js());
    }

    // =========================================================================
    // 6. Line / Arrowhead / Text colors
    // =========================================================================

    // lineColor = invert(background)
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
        .unwrap_or("#000000")
        .to_string();
    tv.set_arrowhead_color_if_none(&line_color);

    // textColor = primaryTextColor
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#28253D" })
        .to_string();
    tv.set_text_color_if_none(&primary_text_color);

    let text_color = tv
        .text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#28253D" })
        .to_string();

    // =========================================================================
    // 7. border2 = tertiaryBorderColor
    // =========================================================================
    let tertiary_border_color = tv
        .tertiary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| tertiary_hsl.adjust_hsl(0.0, -40.0, mk_border_l).to_string());
    tv.set_border2_if_none(&tertiary_border_color);

    // =========================================================================
    // 8. Flowchart variables
    // =========================================================================
    tv.set_node_bkg_if_none(&primary_color);
    tv.set_main_bkg_if_none(&primary_color);
    let primary_border_color = tv
        .primary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| primary_hsl.adjust_hsl(0.0, -40.0, mk_border_l).to_string());
    tv.set_node_border_if_none(&primary_border_color);

    let tertiary_color_str = tv
        .tertiary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| primary_hsl.adjust_hsl(180.0, 0.0, 5.0).to_string());
    tv.set_cluster_bkg_if_none(&tertiary_color_str);
    tv.set_cluster_border_if_none(&tertiary_border_color);
    tv.set_default_link_color_if_none(&line_color);

    // titleColor = tertiaryTextColor
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Rgb::from(tertiary_hsl).invert_from_js());
    tv.set_title_color_if_none(&tertiary_text_color);

    // edgeLabelBackground = darkMode ? darken(secondaryColor, 30) : secondaryColor
    if !is_truthy(&tv.edge_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.edge_label_background = Some(v.to_string());
    }

    tv.set_node_text_color_if_none(&primary_text_color);

    // =========================================================================
    // 9. Sequence Diagram variables
    // =========================================================================
    tv.set_actor_border_if_none(&primary_border_color);

    // actorBkg = mainBkg
    let main_bkg = tv
        .main_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ffffff")
        .to_string();
    tv.set_actor_bkg_if_none(&main_bkg);

    // actorTextColor = primaryTextColor
    tv.set_actor_text_color_if_none(&primary_text_color);

    // actorLineColor = actorBorder
    tv.set_actor_line_color_if_none(&primary_border_color);

    // labelBoxBkgColor = actorBkg
    tv.set_label_box_bkg_color_if_none(&main_bkg);

    // signalColor = textColor
    tv.set_signal_color_if_none(&text_color);

    // signalTextColor = textColor
    tv.set_signal_text_color_if_none(&text_color);

    // labelBoxBorderColor = actorBorder
    tv.set_label_box_border_color_if_none(&primary_border_color);

    // labelTextColor = actorTextColor
    tv.set_label_text_color_if_none(&primary_text_color);

    // loopTextColor = actorTextColor
    tv.set_loop_text_color_if_none(&primary_text_color);

    // activationBorderColor = darken(secondaryColor, 10)
    if !is_truthy(&tv.activation_border_color) {
        tv.activation_border_color = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    }

    // activationBkgColor = secondaryColor
    tv.set_activation_bkg_color_if_none(secondary_hsl.to_string());

    // sequenceNumberColor = invert(lineColor)
    if !is_truthy(&tv.sequence_number_color)
        && let Ok(line_rgb) = Rgb::try_from(&line_color)
    {
        tv.sequence_number_color = Some(
            Rgb {
                r: 1.0 - line_rgb.r,
                g: 1.0 - line_rgb.g,
                b: 1.0 - line_rgb.b,
            }
            .to_string(),
        );
    }

    // personBorder = primaryBorderColor
    tv.set_person_border_if_none(&primary_border_color);

    // personBkg = mainBkg
    tv.set_person_bkg_if_none(&main_bkg);

    // =========================================================================
    // 10. Gantt chart variables (local colors: #ECECFE, #E9E9F1)
    // =========================================================================
    let gantt_primary = "#ECECFE";
    let gantt_secondary = "#E9E9F1";
    let gantt_primary_hsl = Rgb::try_from(gantt_primary).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });
    let gantt_secondary_hsl = Rgb::try_from(gantt_secondary)
        .map(Hsl::from)
        .unwrap_or(Hsl {
            h_deg: 0.0,
            s_pct: 0.0,
            l_pct: 100.0,
        });
    let gantt_tertiary_hsl = gantt_primary_hsl.adjust_hsl(180.0, 0.0, 5.0);

    // sectionBkgColor = tertiaryColor (gantt local)
    tv.set_section_bkg_color_if_none(gantt_tertiary_hsl.to_string());
    // altSectionBkgColor = 'white'
    tv.set_alt_section_bkg_color_if_none("white");
    // (re-)set sectionBkgColor = secondaryColor (gantt local) — JS sets it twice
    tv.set_section_bkg_color_if_none(gantt_secondary_hsl.to_string());
    // sectionBkgColor2 = primaryColor (gantt local)
    tv.set_section_bkg_color2_if_none(gantt_primary_hsl.to_string());
    // excludeBkgColor = '#eeeeee'
    tv.set_exclude_bkg_color_if_none("#eeeeee");
    // taskBorderColor = primaryBorderColor
    tv.set_task_border_color_if_none(&primary_border_color);
    // taskBkgColor = primaryColor (gantt local)
    tv.set_task_bkg_color_if_none(gantt_primary_hsl.to_string());
    // activeTaskBorderColor = primaryColor (gantt local)
    tv.set_active_task_border_color_if_none(gantt_primary_hsl.to_string());
    // activeTaskBkgColor = lighten(primaryColor, 23) (gantt local)
    tv.set_active_task_bkg_color_if_none(gantt_primary_hsl.adjust_hsl(0.0, 0.0, 23.0).to_string());
    // gridColor = 'lightgrey'
    tv.set_grid_color_if_none("lightgrey");
    // doneTaskBkgColor = 'lightgrey'
    tv.set_done_task_bkg_color_if_none("lightgrey");
    // doneTaskBorderColor = 'grey'
    tv.set_done_task_border_color_if_none("grey");
    // critBorderColor = '#ff8888'
    tv.set_crit_border_color_if_none("#ff8888");
    // critBkgColor = 'red'
    tv.set_crit_bkg_color_if_none("red");
    // todayLineColor = 'red'
    tv.set_today_line_color_if_none("red");
    // taskTextColor = primaryTextColor (overridden later in JS)
    tv.set_task_text_color_if_none(&primary_text_color);
    // vertLineColor = primaryBorderColor
    tv.set_vert_line_color_if_none(&primary_border_color);
    // taskTextOutsideColor = textColor
    tv.set_task_text_outside_color_if_none(&text_color);
    // taskTextLightColor = textColor
    tv.set_task_text_light_color_if_none(&text_color);
    // taskTextDarkColor = textColor
    tv.set_task_text_dark_color_if_none(&text_color);
    // taskTextClickableColor = '#003163'
    tv.set_task_text_clickable_color_if_none("#003163");

    // =========================================================================
    // 11. Architecture Diagram variables
    // =========================================================================
    tv.set_arch_edge_color_if_none(&line_color);
    tv.set_arch_edge_arrow_color_if_none(&line_color);

    // =========================================================================
    // 12. State colors
    // =========================================================================
    tv.set_transition_color_if_none(&line_color);
    tv.set_transition_label_color_if_none(&text_color);
    // stateLabelColor = stateBkg || primaryTextColor
    let state_label_color = tv
        .state_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or(tv.primary_text_color.as_deref())
        .unwrap_or("#28253D")
        .to_string();
    tv.set_state_label_color_if_none(state_label_color);
    // stateBkg = mainBkg
    tv.set_state_bkg_if_none(&main_bkg);
    // labelBackgroundColor = stateBkg
    let state_bkg = tv
        .state_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&main_bkg)
        .to_string();
    tv.set_label_background_color_if_none(&state_bkg);
    // compositeBackground = background || tertiaryColor
    let composite_background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or(tv.tertiary_color.as_deref())
        .unwrap_or("#ffffff")
        .to_string();
    tv.set_composite_background_if_none(&composite_background);
    // altBackground = '#f0f0f0'
    tv.set_alt_background_if_none("#f0f0f0");
    // compositeTitleBackground = mainBkg
    tv.set_composite_title_background_if_none(&main_bkg);
    // compositeBorder = nodeBorder
    let node_border = tv
        .node_border
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_border_color)
        .to_string();
    tv.set_composite_border_if_none(&node_border);
    // innerEndBackground = nodeBorder
    tv.set_inner_end_background_if_none(&node_border);
    // errorBkgColor = tertiaryColor
    tv.set_error_bkg_color_if_none(&tertiary_color_str);
    // errorTextColor = tertiaryTextColor
    tv.set_error_text_color_if_none(&tertiary_text_color);
    // transitionColor = lineColor (already set above)
    // specialStateColor = lineColor
    tv.set_special_state_color_if_none(&line_color);

    // =========================================================================
    // 13. Color Scales (cScale0-11) — hardcoded Tailwind 300-level colors
    //     No darkening applied (commented out in JS source).
    // =========================================================================
    let c_scales_hex: [&str; 12] = [
        "#f4a8ff", "#46ecd5", "#ffb86a", "#dab2ff", "#7bf1a8", "#c4b4ff", "#ffa2a2", "#ffdf20",
        "#a3b3ff", "#bbf451", "#74d4ff", "#ffa1ad",
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

    // cScalePeer / cScaleInv / cScaleLabel loop
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
                    && let Ok(rgb) = Rgb::try_from(c_scales_hex[i])
                {
                    tv.$peer = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
                if tv.$inv.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scales_hex[i])
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

    // =========================================================================
    // 14. Tailwind color arrays (borderColorArray = 400-level, bkgColorArray = 50-level)
    // =========================================================================
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

    // =========================================================================
    // 15. surface0-4 and surfacePeer0-4
    // =========================================================================
    let main_bkg_str = tv
        .main_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ffffff");
    let main_bkg_hsl = Rgb::try_from(main_bkg_str).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });
    let surface_multiplier = if dark_mode { -4.0 } else { -1.0 };

    macro_rules! set_surface {
        ($field:ident, $l:expr) => {
            if tv.$field.is_none() {
                tv.$field = Some(
                    main_bkg_hsl
                        .adjust_hsl(180.0, -15.0, surface_multiplier * $l)
                        .to_string(),
                );
            }
        };
    }

    for i in 0..5 {
        let i_f = i as f64;
        match i {
            0 => {
                set_surface!(surface0, 5.0 + i_f * 3.0);
                set_surface!(surface_peer0, 8.0 + i_f * 3.0);
            }
            1 => {
                set_surface!(surface1, 5.0 + i_f * 3.0);
                set_surface!(surface_peer1, 8.0 + i_f * 3.0);
            }
            2 => {
                set_surface!(surface2, 5.0 + i_f * 3.0);
                set_surface!(surface_peer2, 8.0 + i_f * 3.0);
            }
            3 => {
                set_surface!(surface3, 5.0 + i_f * 3.0);
                set_surface!(surface_peer3, 8.0 + i_f * 3.0);
            }
            4 => {
                set_surface!(surface4, 5.0 + i_f * 3.0);
                set_surface!(surface_peer4, 8.0 + i_f * 3.0);
            }
            _ => {}
        }
    }

    // =========================================================================
    // 16. classText = textColor
    // =========================================================================
    tv.set_class_text_if_none(&text_color);

    // =========================================================================
    // 17. fillType0-7 (user-journey) — use local Gantt colors
    // =========================================================================
    tv.set_fill_type0_if_none(gantt_primary_hsl.to_string());
    tv.set_fill_type1_if_none(gantt_secondary_hsl.to_string());
    tv.set_fill_type2_if_none(gantt_primary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type3_if_none(gantt_secondary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type4_if_none(gantt_primary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type5_if_none(gantt_secondary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type6_if_none(gantt_primary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());
    tv.set_fill_type7_if_none(gantt_secondary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());

    // =========================================================================
    // 18. Pie colors (1-12) + pie settings
    // =========================================================================
    tv.set_pie1_if_none(gantt_primary_hsl.to_string());
    tv.set_pie2_if_none(gantt_secondary_hsl.to_string());
    tv.set_pie3_if_none(gantt_tertiary_hsl.to_string());
    tv.set_pie4_if_none(gantt_primary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie5_if_none(gantt_secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie6_if_none(gantt_tertiary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    tv.set_pie7_if_none(gantt_primary_hsl.adjust_hsl(60.0, 0.0, -10.0).to_string());
    tv.set_pie8_if_none(gantt_primary_hsl.adjust_hsl(-60.0, 0.0, -10.0).to_string());
    tv.set_pie9_if_none(gantt_primary_hsl.adjust_hsl(120.0, 0.0, 0.0).to_string());
    tv.set_pie10_if_none(gantt_primary_hsl.adjust_hsl(60.0, 0.0, -20.0).to_string());
    tv.set_pie11_if_none(gantt_primary_hsl.adjust_hsl(-60.0, 0.0, -20.0).to_string());
    tv.set_pie12_if_none(gantt_primary_hsl.adjust_hsl(120.0, 0.0, -10.0).to_string());

    // Pie settings
    let task_text_dark = tv
        .task_text_dark_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&text_color)
        .to_string();
    tv.set_pie_title_text_size_if_none("25px");
    tv.set_pie_title_text_color_if_none(&task_text_dark);
    tv.set_pie_section_text_size_if_none("17px");
    tv.set_pie_section_text_color_if_none(&text_color);
    tv.set_pie_legend_text_size_if_none("17px");
    tv.set_pie_legend_text_color_if_none(&task_text_dark);
    tv.set_pie_stroke_color_if_none("black");
    tv.set_pie_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_color_if_none("black");
    tv.set_pie_opacity_if_none("0.7");

    // =========================================================================
    // 19. Venn
    // =========================================================================
    let venn_title_color = tv
        .title_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&tertiary_text_color)
        .to_string();
    tv.set_venn_title_text_color_if_none(venn_title_color);
    tv.set_venn_set_text_color_if_none(&text_color);

    // =========================================================================
    // 20. Quadrant chart — use local Gantt primary
    // =========================================================================
    tv.set_quadrant1_fill_if_none(gantt_primary_hsl.to_string());

    // quadrant2Fill = adjust(primaryColor, {r:5, g:5, b:5}) — RGB adjust
    if !is_truthy(&tv.quadrant2_fill)
        && let Ok(rgb) = Rgb::try_from(gantt_primary)
    {
        let r = (rgb.r + 5.0 / 255.0).clamp(0.0, 1.0);
        let g = (rgb.g + 5.0 / 255.0).clamp(0.0, 1.0);
        let b = (rgb.b + 5.0 / 255.0).clamp(0.0, 1.0);
        tv.quadrant2_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrant3Fill = adjust(primaryColor, {r:10, g:10, b:10})
    if !is_truthy(&tv.quadrant3_fill)
        && let Ok(rgb) = Rgb::try_from(gantt_primary)
    {
        let r = (rgb.r + 10.0 / 255.0).clamp(0.0, 1.0);
        let g = (rgb.g + 10.0 / 255.0).clamp(0.0, 1.0);
        let b = (rgb.b + 10.0 / 255.0).clamp(0.0, 1.0);
        tv.quadrant3_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrant4Fill = adjust(primaryColor, {r:15, g:15, b:15})
    if !is_truthy(&tv.quadrant4_fill)
        && let Ok(rgb) = Rgb::try_from(gantt_primary)
    {
        let r = (rgb.r + 15.0 / 255.0).clamp(0.0, 1.0);
        let g = (rgb.g + 15.0 / 255.0).clamp(0.0, 1.0);
        let b = (rgb.b + 15.0 / 255.0).clamp(0.0, 1.0);
        tv.quadrant4_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrant1TextFill = primaryTextColor
    tv.set_quadrant1_text_fill_if_none(&primary_text_color);

    // quadrant2TextFill = adjust(primaryTextColor, {r:-5, g:-5, b:-5})
    if !is_truthy(&tv.quadrant2_text_fill)
        && let Ok(rgb) = Rgb::try_from(&primary_text_color)
    {
        let r = (rgb.r - 5.0 / 255.0).max(0.0);
        let g = (rgb.g - 5.0 / 255.0).max(0.0);
        let b = (rgb.b - 5.0 / 255.0).max(0.0);
        tv.quadrant2_text_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrant3TextFill = adjust(primaryTextColor, {r:-10, g:-10, b:-10})
    if !is_truthy(&tv.quadrant3_text_fill)
        && let Ok(rgb) = Rgb::try_from(&primary_text_color)
    {
        let r = (rgb.r - 10.0 / 255.0).max(0.0);
        let g = (rgb.g - 10.0 / 255.0).max(0.0);
        let b = (rgb.b - 10.0 / 255.0).max(0.0);
        tv.quadrant3_text_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrant4TextFill = adjust(primaryTextColor, {r:-15, g:-15, b:-15})
    if !is_truthy(&tv.quadrant4_text_fill)
        && let Ok(rgb) = Rgb::try_from(&primary_text_color)
    {
        let r = (rgb.r - 15.0 / 255.0).max(0.0);
        let g = (rgb.g - 15.0 / 255.0).max(0.0);
        let b = (rgb.b - 15.0 / 255.0).max(0.0);
        tv.quadrant4_text_fill = Some(Rgb { r, g, b }.to_string());
    }

    // quadrantPointFill = isDark(quadrant1Fill) ? lighten : darken
    if !is_truthy(&tv.quadrant_point_fill) {
        let q1_l = gantt_primary_hsl.l_pct;
        let point_delta = if q1_l < 50.0 { 10.0 } else { -10.0 };
        tv.quadrant_point_fill = Some(
            gantt_primary_hsl
                .adjust_hsl(0.0, 0.0, point_delta)
                .to_string(),
        );
    }

    // quadrantPointTextFill = primaryTextColor
    tv.set_quadrant_point_text_fill_if_none(&primary_text_color);
    // quadrantXAxisTextFill = primaryTextColor
    tv.set_quadrant_x_axis_text_fill_if_none(&primary_text_color);
    // quadrantYAxisTextFill = primaryTextColor
    tv.set_quadrant_y_axis_text_fill_if_none(&primary_text_color);
    // quadrantInternalBorderStrokeFill = primaryBorderColor
    tv.set_quadrant_internal_border_stroke_fill_if_none(&primary_border_color);
    // quadrantExternalBorderStrokeFill = primaryBorderColor
    tv.set_quadrant_external_border_stroke_fill_if_none(&primary_border_color);
    // quadrantTitleFill = primaryTextColor
    tv.set_quadrant_title_fill_if_none(&primary_text_color);

    // =========================================================================
    // 21. Requirement Diagram
    // =========================================================================
    tv.set_requirement_background_if_none(gantt_primary_hsl.to_string());
    tv.set_requirement_border_color_if_none(&primary_border_color);
    tv.set_requirement_border_size_if_none("1");
    tv.set_requirement_text_color_if_none(&primary_text_color);

    // relationColor = lineColor
    tv.set_relation_color_if_none(&line_color);

    // relationLabelBackground = darkMode ? darken(secondaryColor, 30) : secondaryColor
    let relation_label_background = if dark_mode {
        secondary_hsl.adjust_hsl(0.0, 0.0, -30.0).to_string()
    } else {
        secondary_hsl.to_string()
    };
    tv.set_relation_label_background_if_none(relation_label_background);

    // relationLabelColor = actorTextColor
    let relation_label_color = tv
        .actor_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_relation_label_color_if_none(relation_label_color);

    // =========================================================================
    // 22. Git colors (use local Gantt colors with lighten/darken 25)
    // =========================================================================
    let git_l_delta = if dark_mode { 25.0 } else { -25.0 };

    // git0 = primaryColor (lighten/darken 25)
    let git0_hsl = gantt_primary_hsl.adjust_hsl(0.0, 0.0, git_l_delta);
    let git1_hsl = gantt_secondary_hsl.adjust_hsl(0.0, 0.0, git_l_delta);
    let git2_hsl = gantt_tertiary_hsl.adjust_hsl(0.0, 0.0, git_l_delta);
    let git3_hsl = gantt_primary_hsl
        .adjust_hsl(-30.0, 0.0, 0.0)
        .adjust_hsl(0.0, 0.0, git_l_delta);
    let git4_hsl = gantt_primary_hsl
        .adjust_hsl(-60.0, 0.0, 0.0)
        .adjust_hsl(0.0, 0.0, git_l_delta);
    let git5_hsl = gantt_primary_hsl
        .adjust_hsl(-90.0, 0.0, 0.0)
        .adjust_hsl(0.0, 0.0, git_l_delta);
    let git6_hsl = gantt_primary_hsl
        .adjust_hsl(60.0, 0.0, 0.0)
        .adjust_hsl(0.0, 0.0, git_l_delta);
    let git7_hsl = gantt_primary_hsl
        .adjust_hsl(120.0, 0.0, 0.0)
        .adjust_hsl(0.0, 0.0, git_l_delta);

    tv.set_git0_if_none(git0_hsl.to_string());
    tv.set_git1_if_none(git1_hsl.to_string());
    tv.set_git2_if_none(git2_hsl.to_string());
    tv.set_git3_if_none(git3_hsl.to_string());
    tv.set_git4_if_none(git4_hsl.to_string());
    tv.set_git5_if_none(git5_hsl.to_string());
    tv.set_git6_if_none(git6_hsl.to_string());
    tv.set_git7_if_none(git7_hsl.to_string());

    // gitInv0-7 = invert(git0-7)
    if !is_truthy(&tv.git_inv0) {
        let rgb = Rgb::from(git0_hsl);
        tv.git_inv0 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv1) {
        let rgb = Rgb::from(git1_hsl);
        tv.git_inv1 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv2) {
        let rgb = Rgb::from(git2_hsl);
        tv.git_inv2 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv3) {
        let rgb = Rgb::from(git3_hsl);
        tv.git_inv3 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv4) {
        let rgb = Rgb::from(git4_hsl);
        tv.git_inv4 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv5) {
        let rgb = Rgb::from(git5_hsl);
        tv.git_inv5 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv6) {
        let rgb = Rgb::from(git6_hsl);
        tv.git_inv6 = Some(rgb.invert_from_js());
    }
    if !is_truthy(&tv.git_inv7) {
        let rgb = Rgb::from(git7_hsl);
        tv.git_inv7 = Some(rgb.invert_from_js());
    }

    // branchLabelColor = darkMode ? 'black' : labelTextColor
    let label_text_color = tv
        .label_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    let branch_label_default = if dark_mode {
        "black".to_string()
    } else {
        label_text_color.clone()
    };
    tv.set_branch_label_color_if_none(&branch_label_default);

    // gitBranchLabel0-7 = branchLabelColor
    tv.set_git_branch_label0_if_none(&branch_label_default);
    tv.set_git_branch_label1_if_none(&branch_label_default);
    tv.set_git_branch_label2_if_none(&branch_label_default);
    tv.set_git_branch_label3_if_none(&branch_label_default);
    tv.set_git_branch_label4_if_none(&branch_label_default);
    tv.set_git_branch_label5_if_none(&branch_label_default);
    tv.set_git_branch_label6_if_none(&branch_label_default);
    tv.set_git_branch_label7_if_none(&branch_label_default);

    // tagLabelColor = primaryTextColor
    tv.set_tag_label_color_if_none(&primary_text_color);
    // tagLabelBackground = primaryColor
    tv.set_tag_label_background_if_none(&primary_color);
    // tagLabelBorder = tagBorder || primaryBorderColor
    tv.set_tag_label_border_if_none(&primary_border_color);
    // tagLabelFontSize = '10px'
    tv.set_tag_label_font_size_if_none("10px");

    // commitLabelColor = secondaryTextColor
    let secondary_text_color = tv
        .secondary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Rgb::from(secondary_hsl).invert_from_js());
    tv.set_commit_label_color_if_none(&secondary_text_color);
    // commitLabelBackground = secondaryColor
    tv.set_commit_label_background_if_none(secondary_hsl.to_string());
    // commitLineColor = '#BDBCCC' (already set in constructor defaults)
    // commitLabelFontSize = '10px'
    tv.set_commit_label_font_size_if_none("10px");

    // =========================================================================
    // 23. EntityRelationship diagrams
    // =========================================================================
    // erEdgeLabelBackground = '#FFFFFF' (already set in constructor defaults)
    tv.set_attribute_background_color_odd_if_none("#ffffff");
    tv.set_attribute_background_color_even_if_none("#f2f2f2");

    // =========================================================================
    // 24. xyChart
    // =========================================================================
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
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );
    xy.fill_prime_color(pt.clone());
    // dataLabelColor
    if xy.data_label_color.is_none() {
        xy.data_label_color = Some(pt);
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::variables::ThemeVariables;

    static REDUX_COLOR_THEME_JSON: &str = include_str!("../../assets/theme/redux-color.json");

    #[test]
    fn compare_with_mermaid_theme_json() {
        let mut base = ThemeVariables::default();
        super::apply_redux_color_theme_defaults(&mut base);
        let json = serde_json::to_string_pretty(&base).unwrap();
        let from_mermaid = serde_json::from_str::<ThemeVariables>(REDUX_COLOR_THEME_JSON).unwrap();
        let from_mermaid_json = serde_json::to_string_pretty(&from_mermaid).unwrap();
        let diff =
            similar_asserts::SimpleDiff::from_str(&json, &from_mermaid_json, "merman", "mermaid");
        println!("{}", diff);
    }
}
