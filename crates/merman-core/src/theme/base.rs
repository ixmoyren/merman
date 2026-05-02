use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_base_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(false);

    // ========================================================================
    // Constructor defaults
    // ========================================================================
    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#f4f4f4")
        .to_string();
    let primary_color = tv
        .primary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#fff4dd")
        .to_string();

    tv.set_background_if_none(&background);
    tv.set_primary_color_if_none(&primary_color);
    tv.set_note_bkg_color_if_none("#fff5ad");
    tv.set_note_text_color_if_none("#333");
    if tv.radius.is_none() {
        tv.radius = Some(5.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(1.0);
    }
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(true);
    }
    tv.set_drop_shadow_if_none("drop-shadow( 1px 2px 2px rgba(185,185,185,1))");

    // ========================================================================
    // Derived main colors
    // ========================================================================

    // primaryTextColor
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#333" });
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#333" })
        .to_string();

    // primary HSL
    let primary_hsl = Rgb::try_from(&primary_color).map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });

    // secondaryColor = adjust(primaryColor, { h: -120 })
    let secondary_hsl = if let Some(ref v) = tv.secondary_color
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else if let Some(ref v) = tv.secondary_color
        && let Ok(hsl) = Hsl::try_from(v)
    {
        hsl
    } else {
        primary_hsl.adjust_hsl(-120.0, 0.0, 0.0)
    };
    tv.set_secondary_color_if_none(secondary_hsl.to_string());

    // tertiaryColor = adjust(primaryColor, { h: 180, l: 5 })
    let tertiary_hsl = if let Some(ref v) = tv.tertiary_color
        && let Ok(c) = Rgb::try_from(v)
    {
        c.into()
    } else if let Some(ref v) = tv.tertiary_color
        && let Ok(hsl) = Hsl::try_from(v)
    {
        hsl
    } else {
        primary_hsl.adjust_hsl(180.0, 0.0, 5.0)
    };
    tv.set_tertiary_color_if_none(tertiary_hsl.to_string());

    // primaryBorderColor = mkBorder(primaryColor, darkMode)
    let primary_border = primary_hsl
        .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
        .to_string();
    tv.set_primary_border_color_if_none(&primary_border);

    // secondaryBorderColor = mkBorder(secondaryColor, darkMode)
    let secondary_border = secondary_hsl
        .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
        .to_string();
    tv.set_secondary_border_color_if_none(&secondary_border);

    // tertiaryBorderColor = mkBorder(tertiaryColor, darkMode)
    let tertiary_border = tertiary_hsl
        .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
        .to_string();
    tv.set_tertiary_border_color_if_none(&tertiary_border);

    // noteBorderColor = mkBorder(noteBkgColor, darkMode)
    // noteBkgColor is fixed to "#fff5ad" in constructor/updateColors
    let note_bkg_hsl = Rgb::try_from("#fff5ad").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 0.0,
        l_pct: 100.0,
    });
    let note_border = note_bkg_hsl
        .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
        .to_string();
    tv.set_note_border_color_if_none(&note_border);

    // noteBkgColor (keep)
    tv.set_note_bkg_color_if_none("#fff5ad");
    // noteTextColor (keep)
    tv.set_note_text_color_if_none("#333");

    // secondaryTextColor = invert(secondaryColor)
    tv.set_secondary_text_color_if_none(Rgb::from(secondary_hsl).invert_from_hsl().to_string());

    // tertiaryTextColor = invert(tertiaryColor)
    tv.set_tertiary_text_color_if_none(Rgb::from(tertiary_hsl).invert_from_hsl().to_string());

    // lineColor = invert(background)
    tv.set_line_color_if_none(
        Rgb::try_from(&background)
            .unwrap()
            .invert_from_hsl()
            .to_string(),
    );
    let line_color = tv
        .line_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333333")
        .to_string();

    // arrowheadColor = lineColor
    tv.set_arrowhead_color_if_none(&line_color);

    // textColor = primaryTextColor
    tv.set_text_color_if_none(&primary_text_color);

    // border2 = tertiaryBorderColor
    tv.set_border2_if_none(&tertiary_border);

    // ========================================================================
    // Flowchart variables
    // ========================================================================
    tv.set_node_bkg_if_none(&primary_color);
    tv.set_main_bkg_if_none(&primary_color);
    tv.set_node_border_if_none(&primary_border);
    tv.set_cluster_bkg_if_none(tertiary_hsl.to_string());
    tv.set_cluster_border_if_none(&tertiary_border);
    tv.set_default_link_color_if_none(&line_color);

    // tertiaryTextColor → titleColor
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();
    tv.set_title_color_if_none(&tertiary_text_color);

    // edgeLabelBackground
    if !is_truthy(&tv.edge_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.edge_label_background = Some(v.to_string());
    }

    tv.set_node_text_color_if_none(&primary_text_color);

    // ========================================================================
    // Sequence Diagram variables
    // ========================================================================
    // actorBorder = primaryBorderColor
    tv.set_actor_border_if_none(&primary_border);
    // actorBkg = mainBkg (= primaryColor at this point)
    tv.set_actor_bkg_if_none(&primary_color);
    // actorTextColor = primaryTextColor
    tv.set_actor_text_color_if_none(&primary_text_color);
    // actorLineColor = actorBorder (= primaryBorderColor)
    tv.set_actor_line_color_if_none(&primary_border);
    // labelBoxBkgColor = actorBkg (= primaryColor)
    tv.set_label_box_bkg_color_if_none(&primary_color);
    // signalColor = textColor (= primaryTextColor)
    tv.set_signal_color_if_none(&primary_text_color);
    // signalTextColor = textColor (= primaryTextColor)
    tv.set_signal_text_color_if_none(&primary_text_color);
    // labelBoxBorderColor = actorBorder (= primaryBorderColor)
    tv.set_label_box_border_color_if_none(&primary_border);
    // labelTextColor = actorTextColor (= primaryTextColor)
    tv.set_label_text_color_if_none(&primary_text_color);
    // loopTextColor = actorTextColor (= primaryTextColor)
    tv.set_loop_text_color_if_none(&primary_text_color);

    // activationBorderColor = darken(secondaryColor, 10)
    if !is_truthy(&tv.activation_border_color) {
        tv.activation_border_color = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    }
    // activationBkgColor = secondaryColor
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

    // ========================================================================
    // Sequence Diagram — Person
    // ========================================================================
    // personBorder = primaryBorderColor
    tv.set_person_border_if_none(&primary_border);
    // personBkg = mainBkg (= primaryColor)
    tv.set_person_bkg_if_none(&primary_color);

    // ========================================================================
    // Gantt chart variables
    // ========================================================================
    // sectionBkgColor = tertiaryColor (first assignment, line 89)
    tv.set_section_bkg_color_if_none(tertiary_hsl.to_string());
    // altSectionBkgColor = 'white'
    tv.set_alt_section_bkg_color_if_none("white");
    // sectionBkgColor = secondaryColor (line 91 — overrides the tertiary assignment
    // only if both did not trigger; in practice tertiaryColor wins because the
    // first set_section_bkg_color_if_none already populated it)
    tv.set_section_bkg_color_if_none(secondary_hsl.to_string());
    // sectionBkgColor2 = primaryColor
    tv.set_section_bkg_color2_if_none(&primary_color);
    // excludeBkgColor = '#eeeeee'
    tv.set_exclude_bkg_color_if_none("#eeeeee");
    // taskBorderColor = primaryBorderColor
    tv.set_task_border_color_if_none(&primary_border);
    // taskBkgColor = primaryColor
    tv.set_task_bkg_color_if_none(&primary_color);
    // activeTaskBorderColor = primaryColor
    tv.set_active_task_border_color_if_none(&primary_color);
    // activeTaskBkgColor = lighten(primaryColor, 23)
    tv.set_active_task_bkg_color_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 23.0).to_string());
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
    // vertLineColor = 'navy'
    tv.set_vert_line_color_if_none("navy");

    // taskTextColor = textColor (line 105, first), then taskTextColor = primaryTextColor (line 108, final)
    tv.set_task_text_color_if_none(&primary_text_color);
    tv.set_task_text_color_if_none(&primary_text_color); // same as above, second assignment is idempotent
    // taskTextOutsideColor = textColor (= primaryTextColor)
    tv.set_task_text_outside_color_if_none(&primary_text_color);
    // taskTextLightColor = textColor (= primaryTextColor)
    tv.set_task_text_light_color_if_none(&primary_text_color);
    // taskTextDarkColor = textColor (= primaryTextColor)
    tv.set_task_text_dark_color_if_none(&primary_text_color);
    // taskTextClickableColor = '#003163'
    tv.set_task_text_clickable_color_if_none("#003163");

    // noteFontWeight = 'normal'
    tv.set_note_font_weight_if_none("normal");
    // fontWeight = 'normal'
    tv.set_font_weight_if_none("normal");

    // ========================================================================
    // Architecture Diagram variables
    // ========================================================================
    // archEdgeColor = '#777'
    tv.set_arch_edge_color_if_none("#777");
    // archEdgeArrowColor = '#777'
    tv.set_arch_edge_arrow_color_if_none("#777");
    // archEdgeWidth = '3'
    tv.set_arch_edge_width_if_none("3");
    // archGroupBorderColor = '#000'
    tv.set_arch_group_border_color_if_none("#000");
    // archGroupBorderWidth = '2px'
    tv.set_arch_group_border_width_if_none("2px");

    // ========================================================================
    // State colors
    // ========================================================================
    // transitionColor = lineColor
    tv.set_transition_color_if_none(&line_color);
    // transitionLabelColor = textColor (= primaryTextColor)
    tv.set_transition_label_color_if_none(&primary_text_color);

    // stateLabelColor = stateBkg || primaryTextColor
    if !is_truthy(&tv.state_label_color) {
        let fallback = tv
            .state_bkg
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&primary_text_color)
            .to_string();
        tv.state_label_color = Some(fallback);
    }

    // stateBkg = mainBkg (= primaryColor)
    tv.set_state_bkg_if_none(&primary_color);
    // labelBackgroundColor = stateBkg (= primaryColor)
    tv.set_label_background_color_if_none(&primary_color);
    // compositeBackground = background || tertiaryColor
    if !is_truthy(&tv.composite_background) {
        let fallback = tv
            .background
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&tertiary_hsl.to_string())
            .to_string();
        tv.composite_background = Some(fallback);
    }
    // altBackground = tertiaryColor
    tv.set_alt_background_if_none(tertiary_hsl.to_string());
    // compositeTitleBackground = mainBkg (= primaryColor)
    tv.set_composite_title_background_if_none(&primary_color);
    // compositeBorder = nodeBorder (= primaryBorderColor)
    tv.set_composite_border_if_none(&primary_border);
    // innerEndBackground = nodeBorder (DIRECT)
    tv.set_inner_end_background_if_none(&primary_border);
    // errorBkgColor = tertiaryColor
    tv.set_error_bkg_color_if_none(tertiary_hsl.to_string());
    // errorTextColor = tertiaryTextColor
    tv.set_error_text_color_if_none(&tertiary_text_color);
    // specialStateColor = lineColor (DIRECT)
    tv.set_special_state_color_if_none(&line_color);

    // ========================================================================
    // Class
    // ========================================================================
    // classText = textColor (= primaryTextColor)
    tv.set_class_text_if_none(&primary_text_color);

    // ========================================================================
    // Color Scales
    // ========================================================================
    let cscale_amount = if dark_mode { 75.0 } else { 25.0 };
    tv.set_c_scale0_if_none(primary_hsl.adjust_hsl(0.0, 0.0, -cscale_amount).to_string());
    tv.set_c_scale1_if_none(
        secondary_hsl
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale2_if_none(
        tertiary_hsl
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale3_if_none(
        primary_hsl
            .adjust_hsl(30.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale4_if_none(
        primary_hsl
            .adjust_hsl(60.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale5_if_none(
        primary_hsl
            .adjust_hsl(90.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale6_if_none(
        primary_hsl
            .adjust_hsl(120.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale7_if_none(
        primary_hsl
            .adjust_hsl(150.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale8_if_none(
        primary_hsl
            .adjust_hsl(210.0, 0.0, 150.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale9_if_none(
        primary_hsl
            .adjust_hsl(270.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale10_if_none(
        primary_hsl
            .adjust_hsl(300.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );
    tv.set_c_scale11_if_none(
        primary_hsl
            .adjust_hsl(330.0, 0.0, 0.0)
            .adjust_hsl(0.0, 0.0, -cscale_amount)
            .to_string(),
    );

    // Pre-darkened cScales for peer/inv/label derivation
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
    .map(|base| base.adjust_hsl(0.0, 0.0, -cscale_amount));

    // scaleLabelColor = labelTextColor (falls back to primaryTextColor)
    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            tv.label_text_color
                .as_deref()
                .filter(|s| !s.trim().is_empty())
        })
        .unwrap_or(&primary_text_color)
        .to_string();

    let peer_delta = if dark_mode { 10.0 } else { -10.0 };

    for i in 0..12 {
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none() {
                    let rgb = Rgb::from(c_scales[i]);
                    tv.$field = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none() {
                    let Rgb { r, g, b } = Rgb::from(c_scales[i]);
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

    // ========================================================================
    // Surface colors
    // ========================================================================
    let surface_multiplier: f64 = if dark_mode { -4.0 } else { -1.0 };

    // mainBkg was set to primaryColor, so use primary_hsl for surface
    let main_bkg_hsl = Rgb::try_from(
        tv.main_bkg
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&primary_color),
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

    // ========================================================================
    // User Journey fill types
    // ========================================================================
    // fillType0 = primaryColor
    tv.set_fill_type0_if_none(&primary_color);
    // fillType1 = secondaryColor
    tv.set_fill_type1_if_none(secondary_hsl.to_string());
    // fillType2 = adjust(primaryColor, { h: 64 })
    tv.set_fill_type2_if_none(primary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    // fillType3 = adjust(secondaryColor, { h: 64 })
    tv.set_fill_type3_if_none(secondary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    // fillType4 = adjust(primaryColor, { h: -64 })
    tv.set_fill_type4_if_none(primary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    // fillType5 = adjust(secondaryColor, { h: -64 })
    tv.set_fill_type5_if_none(secondary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    // fillType6 = adjust(primaryColor, { h: 128 })
    tv.set_fill_type6_if_none(primary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());
    // fillType7 = adjust(secondaryColor, { h: 128 })
    tv.set_fill_type7_if_none(secondary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());

    // ========================================================================
    // Pie colors
    // ========================================================================
    // pie1 = primaryColor
    tv.set_pie1_if_none(&primary_color);
    // pie2 = secondaryColor
    tv.set_pie2_if_none(secondary_hsl.to_string());
    // pie3 = tertiaryColor
    tv.set_pie3_if_none(tertiary_hsl.to_string());
    // pie4 = adjust(primaryColor, { l: -10 })
    tv.set_pie4_if_none(primary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    // pie5 = adjust(secondaryColor, { l: -10 })
    tv.set_pie5_if_none(secondary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    // pie6 = adjust(tertiaryColor, { l: -10 })
    tv.set_pie6_if_none(tertiary_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    // pie7 = adjust(primaryColor, { h: +60, l: -10 })
    tv.set_pie7_if_none(primary_hsl.adjust_hsl(60.0, 0.0, -10.0).to_string());
    // pie8 = adjust(primaryColor, { h: -60, l: -10 })
    tv.set_pie8_if_none(primary_hsl.adjust_hsl(-60.0, 0.0, -10.0).to_string());
    // pie9 = adjust(primaryColor, { h: 120, l: 0 })
    tv.set_pie9_if_none(primary_hsl.adjust_hsl(120.0, 0.0, 0.0).to_string());
    // pie10 = adjust(primaryColor, { h: +60, l: -20 })
    tv.set_pie10_if_none(primary_hsl.adjust_hsl(60.0, 0.0, -20.0).to_string());
    // pie11 = adjust(primaryColor, { h: -60, l: -20 })
    tv.set_pie11_if_none(primary_hsl.adjust_hsl(-60.0, 0.0, -20.0).to_string());
    // pie12 = adjust(primaryColor, { h: 120, l: -10 })
    tv.set_pie12_if_none(primary_hsl.adjust_hsl(120.0, 0.0, -10.0).to_string());

    // pieTitleTextSize = '25px'
    tv.set_pie_title_text_size_if_none("25px");
    // pieTitleTextColor = taskTextDarkColor
    let task_text_dark = tv
        .task_text_dark_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_pie_title_text_color_if_none(&task_text_dark);
    // pieSectionTextSize = '17px'
    tv.set_pie_section_text_size_if_none("17px");
    // pieSectionTextColor = textColor (= primaryTextColor)
    tv.set_pie_section_text_color_if_none(&primary_text_color);
    // pieLegendTextSize = '17px'
    tv.set_pie_legend_text_size_if_none("17px");
    // pieLegendTextColor = taskTextDarkColor
    tv.set_pie_legend_text_color_if_none(&task_text_dark);
    // pieStrokeColor = 'black'
    tv.set_pie_stroke_color_if_none("black");
    // pieStrokeWidth = '2px'
    tv.set_pie_stroke_width_if_none("2px");
    // pieOuterStrokeWidth = '2px'
    tv.set_pie_outer_stroke_width_if_none("2px");
    // pieOuterStrokeColor = 'black'
    tv.set_pie_outer_stroke_color_if_none("black");
    // pieOpacity = '0.7'
    tv.set_pie_opacity_if_none("0.7");

    // ========================================================================
    // Venn colors
    // ========================================================================
    // venn1 = adjust(primaryColor, { l: -30 })  (?? semantics: None only)
    if tv.venn1.is_none() {
        tv.venn1 = Some(primary_hsl.adjust_hsl(0.0, 0.0, -30.0).to_string());
    }
    // venn2 = adjust(secondaryColor, { l: -30 })
    if tv.venn2.is_none() {
        tv.venn2 = Some(secondary_hsl.adjust_hsl(0.0, 0.0, -30.0).to_string());
    }
    // venn3 = adjust(tertiaryColor, { l: -30 })
    if tv.venn3.is_none() {
        tv.venn3 = Some(tertiary_hsl.adjust_hsl(0.0, 0.0, -30.0).to_string());
    }
    // venn4 = adjust(primaryColor, { h: 60, l: -30 })
    if tv.venn4.is_none() {
        tv.venn4 = Some(primary_hsl.adjust_hsl(60.0, 0.0, -30.0).to_string());
    }
    // venn5 = adjust(primaryColor, { h: -60, l: -30 })
    if tv.venn5.is_none() {
        tv.venn5 = Some(primary_hsl.adjust_hsl(-60.0, 0.0, -30.0).to_string());
    }
    // venn6 = adjust(secondaryColor, { h: 60, l: -30 })
    if tv.venn6.is_none() {
        tv.venn6 = Some(secondary_hsl.adjust_hsl(60.0, 0.0, -30.0).to_string());
    }
    // venn7 = adjust(primaryColor, { h: 120, l: -30 })
    if tv.venn7.is_none() {
        tv.venn7 = Some(primary_hsl.adjust_hsl(120.0, 0.0, -30.0).to_string());
    }
    // venn8 = adjust(secondaryColor, { h: 120, l: -30 })
    if tv.venn8.is_none() {
        tv.venn8 = Some(secondary_hsl.adjust_hsl(120.0, 0.0, -30.0).to_string());
    }

    // vennTitleTextColor = titleColor (= tertiaryTextColor at this point)
    let venn_title = tv
        .title_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&tertiary_text_color)
        .to_string();
    tv.set_venn_title_text_color_if_none(venn_title);
    // vennSetTextColor = textColor (= primaryTextColor)
    tv.set_venn_set_text_color_if_none(&primary_text_color);

    // ========================================================================
    // Quadrant Chart variables
    // ========================================================================
    // quadrant1Fill = primaryColor
    tv.set_quadrant1_fill_if_none(&primary_color);

    // quadrant2Fill = adjust(primaryColor, { r: 5, g: 5, b: 5 })
    // This means adding 5/255 to each Rgb channel
    if !is_truthy(&tv.quadrant2_fill)
        && let Ok(primary_rgb) = Rgb::try_from(&primary_color)
    {
        tv.quadrant2_fill = Some(
            Rgb {
                r: (primary_rgb.r + 5.0 / 255.0).clamp(0.0, 1.0),
                g: (primary_rgb.g + 5.0 / 255.0).clamp(0.0, 1.0),
                b: (primary_rgb.b + 5.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrant3Fill = adjust(primaryColor, { r: 10, g: 10, b: 10 })
    if !is_truthy(&tv.quadrant3_fill)
        && let Ok(primary_rgb) = Rgb::try_from(&primary_color)
    {
        tv.quadrant3_fill = Some(
            Rgb {
                r: (primary_rgb.r + 10.0 / 255.0).clamp(0.0, 1.0),
                g: (primary_rgb.g + 10.0 / 255.0).clamp(0.0, 1.0),
                b: (primary_rgb.b + 10.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrant4Fill = adjust(primaryColor, { r: 15, g: 15, b: 15 })
    if !is_truthy(&tv.quadrant4_fill)
        && let Ok(primary_rgb) = Rgb::try_from(&primary_color)
    {
        tv.quadrant4_fill = Some(
            Rgb {
                r: (primary_rgb.r + 15.0 / 255.0).clamp(0.0, 1.0),
                g: (primary_rgb.g + 15.0 / 255.0).clamp(0.0, 1.0),
                b: (primary_rgb.b + 15.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrant1TextFill = primaryTextColor
    tv.set_quadrant1_text_fill_if_none(&primary_text_color);

    // quadrant2TextFill = adjust(primaryTextColor, { r: -5, g: -5, b: -5 })
    if !is_truthy(&tv.quadrant2_text_fill)
        && let Ok(pt_rgb) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant2_text_fill = Some(
            Rgb {
                r: (pt_rgb.r - 5.0 / 255.0).clamp(0.0, 1.0),
                g: (pt_rgb.g - 5.0 / 255.0).clamp(0.0, 1.0),
                b: (pt_rgb.b - 5.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrant3TextFill = adjust(primaryTextColor, { r: -10, g: -10, b: -10 })
    if !is_truthy(&tv.quadrant3_text_fill)
        && let Ok(pt_rgb) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant3_text_fill = Some(
            Rgb {
                r: (pt_rgb.r - 10.0 / 255.0).clamp(0.0, 1.0),
                g: (pt_rgb.g - 10.0 / 255.0).clamp(0.0, 1.0),
                b: (pt_rgb.b - 10.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrant4TextFill = adjust(primaryTextColor, { r: -15, g: -15, b: -15 })
    if !is_truthy(&tv.quadrant4_text_fill)
        && let Ok(pt_rgb) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant4_text_fill = Some(
            Rgb {
                r: (pt_rgb.r - 15.0 / 255.0).clamp(0.0, 1.0),
                g: (pt_rgb.g - 15.0 / 255.0).clamp(0.0, 1.0),
                b: (pt_rgb.b - 15.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrantPointFill = isDark(quadrant1Fill) ? lighten(quadrant1Fill) : darken(quadrant1Fill)
    // isDark: check if average luminance < 0.5
    if !is_truthy(&tv.quadrant_point_fill)
        && let Ok(q1_rgb) = Rgb::try_from(&primary_color)
    {
        let avg_lum = (q1_rgb.r + q1_rgb.g + q1_rgb.b) / 3.0;
        let q1_hsl = Hsl::from(q1_rgb);
        let result = if avg_lum < 0.5 {
            q1_hsl.adjust_hsl(0.0, 0.0, 10.0) // lighten
        } else {
            q1_hsl.adjust_hsl(0.0, 0.0, -10.0) // darken
        };
        tv.quadrant_point_fill = Some(result.to_string());
    }

    // quadrantPointTextFill = primaryTextColor
    tv.set_quadrant_point_text_fill_if_none(&primary_text_color);
    // quadrantXAxisTextFill = primaryTextColor
    tv.set_quadrant_x_axis_text_fill_if_none(&primary_text_color);
    // quadrantYAxisTextFill = primaryTextColor
    tv.set_quadrant_y_axis_text_fill_if_none(&primary_text_color);
    // quadrantInternalBorderStrokeFill = primaryBorderColor
    tv.set_quadrant_internal_border_stroke_fill_if_none(&primary_border);
    // quadrantExternalBorderStrokeFill = primaryBorderColor
    tv.set_quadrant_external_border_stroke_fill_if_none(&primary_border);
    // quadrantTitleFill = primaryTextColor
    tv.set_quadrant_title_fill_if_none(&primary_text_color);

    // ========================================================================
    // Requirement Diagram variables
    // ========================================================================
    // requirementBackground = primaryColor
    tv.set_requirement_background_if_none(&primary_color);
    // requirementBorderColor = primaryBorderColor
    tv.set_requirement_border_color_if_none(&primary_border);
    // requirementBorderSize = '1'
    tv.set_requirement_border_size_if_none("1");
    // requirementTextColor = primaryTextColor
    tv.set_requirement_text_color_if_none(&primary_text_color);
    // relationColor = lineColor
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
    // relationLabelColor = actorTextColor (= primaryTextColor)
    tv.set_relation_label_color_if_none(&primary_text_color);

    // ========================================================================
    // Git colors
    // ========================================================================
    // git0 = primaryColor
    let git_base: [Hsl; 8] = [
        primary_hsl,                             // git0
        secondary_hsl,                           // git1
        tertiary_hsl,                            // git2
        primary_hsl.adjust_hsl(-30.0, 0.0, 0.0), // git3
        primary_hsl.adjust_hsl(-60.0, 0.0, 0.0), // git4
        primary_hsl.adjust_hsl(-90.0, 0.0, 0.0), // git5
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0),  // git6
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0), // git7
    ];

    let git_adj: f64 = 25.0;
    let git_darkened: [Hsl; 8] = if dark_mode {
        git_base.map(|h| h.adjust_hsl(0.0, 0.0, git_adj))
    // lighten(25) for dark mode
    } else {
        git_base.map(|h| h.adjust_hsl(0.0, 0.0, -git_adj))
        // darken(25) for light mode
    };

    tv.set_git0_if_none(git_darkened[0].to_string());
    tv.set_git1_if_none(git_darkened[1].to_string());
    tv.set_git2_if_none(git_darkened[2].to_string());
    tv.set_git3_if_none(git_darkened[3].to_string());
    tv.set_git4_if_none(git_darkened[4].to_string());
    tv.set_git5_if_none(git_darkened[5].to_string());
    tv.set_git6_if_none(git_darkened[6].to_string());
    tv.set_git7_if_none(git_darkened[7].to_string());

    // gitInv0-7 = invert(git0-7)
    for i in 0..8 {
        let Rgb { r, g, b } = Rgb::from(git_darkened[i]);
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

    // branchLabelColor = darkMode ? 'black' : labelTextColor
    let branch_label = if dark_mode {
        "black".to_string()
    } else {
        tv.label_text_color
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or(&primary_text_color)
            .to_string()
    };
    tv.set_branch_label_color_if_none(&branch_label);
    tv.set_git_branch_label0_if_none(&branch_label);
    tv.set_git_branch_label1_if_none(&branch_label);
    tv.set_git_branch_label2_if_none(&branch_label);
    tv.set_git_branch_label3_if_none(&branch_label);
    tv.set_git_branch_label4_if_none(&branch_label);
    tv.set_git_branch_label5_if_none(&branch_label);
    tv.set_git_branch_label6_if_none(&branch_label);
    tv.set_git_branch_label7_if_none(&branch_label);

    // tagLabelColor = primaryTextColor
    tv.set_tag_label_color_if_none(&primary_text_color);
    // tagLabelBackground = primaryColor
    tv.set_tag_label_background_if_none(&primary_color);
    // tagLabelBorder = tagBorder || primaryBorderColor  (JS: this.tagBorder — maps to tagLabelBorder)
    tv.set_tag_label_border_if_none(&primary_border);
    // tagLabelFontSize = '10px'
    tv.set_tag_label_font_size_if_none("10px");
    // commitLabelColor = secondaryTextColor
    let secondary_text_color = tv
        .secondary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_commit_label_color_if_none(&secondary_text_color);
    // commitLabelBackground = secondaryColor
    tv.set_commit_label_background_if_none(secondary_hsl.to_string());
    // commitLabelFontSize = '10px'
    tv.set_commit_label_font_size_if_none("10px");

    // ========================================================================
    // Event Modeling diagram variables
    // ========================================================================
    // emUiFill = 'white'
    tv.set_em_ui_fill_if_none("white");
    // emUiStroke = '#dbdada'
    tv.set_em_ui_stroke_if_none("#dbdada");
    // emProcessorFill = '#edb3f6'
    tv.set_em_processor_fill_if_none("#edb3f6");
    // emProcessorStroke = '#b88cbf'
    tv.set_em_processor_stroke_if_none("#b88cbf");
    // emReadModelFill = '#d3f1a2'
    tv.set_em_read_model_fill_if_none("#d3f1a2");
    // emReadModelStroke = '#a3b732'
    tv.set_em_read_model_stroke_if_none("#a3b732");
    // emCommandFill = '#bcd6fe'
    tv.set_em_command_fill_if_none("#bcd6fe");
    // emCommandStroke = '#679ac3'
    tv.set_em_command_stroke_if_none("#679ac3");
    // emEventFill = '#ffb778'
    tv.set_em_event_fill_if_none("#ffb778");
    // emEventStroke = '#c19a0f'
    tv.set_em_event_stroke_if_none("#c19a0f");
    // emSwimlaneBackgroundOdd = 'rgb(250,250,250)'
    tv.set_em_swimlane_background_odd_if_none("rgb(250,250,250)");
    // emSwimlaneBackgroundStroke = 'rgb(240,240,240)'
    tv.set_em_swimlane_background_stroke_if_none("rgb(240,240,240)");
    // emArrowhead = lineColor
    tv.set_em_arrowhead_if_none(&line_color);
    // emRelationStroke = lineColor
    tv.set_em_relation_stroke_if_none(&line_color);

    // ========================================================================
    // ER (Entity Relationship) diagram variables
    // ========================================================================
    // attributeBackgroundColorOdd = oldAttributeBackgroundColorOdd = '#ffffff'
    tv.set_attribute_background_color_odd_if_none("#ffffff");
    // attributeBackgroundColorEven = oldAttributeBackgroundColorEven = '#f2f2f2'
    tv.set_attribute_background_color_even_if_none("#f2f2f2");

    // rowOdd / rowEven
    // darkMode: rowOdd = darken(mainBkg, 5) || '#ffffff'
    //           rowEven = darken(mainBkg, 10)
    // lightMode: rowOdd = lighten(mainBkg, 75) || '#ffffff'
    //           rowEven = lighten(mainBkg, 5)
    if dark_mode {
        if !is_truthy(&tv.row_odd) {
            let v = main_bkg_hsl.adjust_hsl(0.0, 0.0, -5.0).to_string();
            tv.row_odd = Some(v);
        }
        if !is_truthy(&tv.row_even) {
            tv.row_even = Some(main_bkg_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
        }
    } else {
        if !is_truthy(&tv.row_odd) {
            let v = main_bkg_hsl.adjust_hsl(0.0, 0.0, 75.0).to_string();
            tv.row_odd = Some(v);
        }
        if !is_truthy(&tv.row_even) {
            tv.row_even = Some(main_bkg_hsl.adjust_hsl(0.0, 0.0, 5.0).to_string());
        }
    }

    // ========================================================================
    // Gradient
    // ========================================================================
    // gradientStart = primaryBorderColor (DIRECT)
    tv.set_gradient_start_if_none(&primary_border);
    // gradientStop = secondaryBorderColor (DIRECT)
    tv.set_gradient_stop_if_none(&secondary_border);

    // ========================================================================
    // Wardley
    // ========================================================================
    // wardleyEvolutionColor = '#dc3545'
    tv.set_wardley_evolution_color_if_none("#dc3545");

    let wardley = tv.wardley.get_or_insert_with(Default::default);
    if wardley
        .background_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.background_color = Some(background.clone());
    }
    if wardley
        .axis_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.axis_color = Some(line_color.clone());
    }
    if wardley
        .axis_text_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.axis_text_color = Some(primary_text_color.clone());
    }
    // gridColor from JS = this.gridColor (which was set to 'lightgrey' in Gantt section)
    let grid_color = tv
        .grid_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("lightgrey")
        .to_string();
    if wardley
        .grid_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.grid_color = Some(grid_color);
    }
    if wardley
        .component_fill
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.component_fill = Some(background.clone());
    }
    if wardley
        .component_stroke
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.component_stroke = Some(line_color.clone());
    }
    if wardley
        .component_label_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.component_label_color = Some(primary_text_color.clone());
    }
    if wardley
        .link_stroke
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.link_stroke = Some(line_color.clone());
    }
    if wardley
        .evolution_stroke
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        // evolutionStroke = wardleyEvolutionColor (#dc3545)
        wardley.evolution_stroke = Some("#dc3545".to_string());
    }
    if wardley
        .annotation_stroke
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.annotation_stroke = Some(line_color.clone());
    }
    if wardley
        .annotation_text_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.annotation_text_color = Some(primary_text_color.clone());
    }
    if wardley
        .annotation_fill
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        wardley.annotation_fill = Some(background.clone());
    }

    // ========================================================================
    // xyChart
    // ========================================================================
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    let bg = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("white")
        .to_string();
    xy.set_background_color_if_none(bg);
    xy.set_plot_color_palette_if_none(
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );
    xy.fill_prime_color(primary_text_color.clone());
    // dataLabelColor = primaryTextColor (no dedicated setter — assign directly)
    if xy
        .data_label_color
        .as_ref()
        .is_none_or(|s| s.trim().is_empty())
    {
        xy.data_label_color = Some(primary_text_color.clone());
    }

    // ========================================================================
    // Radar
    // ========================================================================
    let radar = tv.radar.get_or_insert_with(Default::default);
    radar.set_axis_color_if_none(&line_color);
    radar.set_axis_stroke_width_if_none(2);
    radar.set_axis_label_font_size_if_none(12);
    radar.set_curve_opacity_if_none(0.5);
    radar.set_curve_stroke_width_if_none(2);
    radar.set_graticule_color_if_none("#DEDEDE");
    radar.set_graticule_opacity_if_none(0.3);
    radar.set_graticule_stroke_width_if_none(1);
    radar.set_legend_box_size_if_none(12);
    radar.set_legend_font_size_if_none(12);
}

#[cfg(test)]
mod tests {
    use crate::theme::variables::ThemeVariables;

    static BASE_THEME_JSON: &str = include_str!("../../assets/theme/base.json");

    #[test]
    fn compare_with_mermaid_base_theme_json() {
        let mut base = ThemeVariables::default();
        super::apply_base_theme_defaults(&mut base);
        let base_json = serde_json::to_string_pretty(&base).unwrap();
        let base_from_mermaid = serde_json::from_str::<ThemeVariables>(BASE_THEME_JSON).unwrap();
        let base_from_mermaid_json = serde_json::to_string_pretty(&base_from_mermaid).unwrap();
        let diff = similar_asserts::SimpleDiff::from_str(
            &base_json,
            &base_from_mermaid_json,
            "merman",
            "mermaid",
        );
        println!("{}", diff);
    }
}
