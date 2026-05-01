use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_neo_dark_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(true);

    // --- Constructor defaults ---
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
    tv.set_note_font_weight_if_none("normal");
    tv.set_font_weight_if_none("normal");

    // --- Derived colors ---
    let background = tv
        .background
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#333")
        .to_string();

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

    // primaryTextColor
    tv.set_primary_text_color_if_none(if dark_mode { "#eee" } else { "#333" });
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode { "#eee" } else { "#333" })
        .to_string();

    // secondaryColor = lighten(primaryColor, 16) [constructor]; updateColors: adjust(primaryColor, {h: -120})
    if !is_truthy(&tv.secondary_color) {
        tv.secondary_color = Some(primary_hsl.adjust_hsl(0.0, 0.0, 16.0).to_string());
    }
    let secondary_hsl = tv
        .secondary_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .and_then(|s| Rgb::try_from(s).ok())
        .map(Hsl::from)
        .unwrap_or_else(|| primary_hsl.adjust_hsl(0.0, 0.0, 16.0));

    // tertiaryColor = adjust(primaryColor, {h: -160}) [constructor]; updateColors: adjust(primaryColor, {h: 180, l: 5})
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

    // primaryBorderColor = invert(background) [constructor]; updateColors: mkBorder(primaryColor, darkMode)
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

    // secondaryBorderColor = mkBorder(secondaryColor, darkMode) [constructor]
    if !is_truthy(&tv.secondary_border_color) {
        tv.secondary_border_color = Some(
            secondary_hsl
                .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
                .to_string(),
        );
    }

    // tertiaryBorderColor = mkBorder(tertiaryColor, darkMode) [constructor]
    if !is_truthy(&tv.tertiary_border_color) {
        tv.tertiary_border_color = Some(
            tertiary_hsl
                .adjust_hsl(0.0, -40.0, if dark_mode { 10.0 } else { -10.0 })
                .to_string(),
        );
    }

    // secondaryTextColor = invert(secondaryColor) [constructor]
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

    // tertiaryTextColor = invert(tertiaryColor) [constructor]
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

    // lineColor = invert(background)
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

    // textColor = primaryTextColor [updateColors]
    tv.set_text_color_if_none(&primary_text_color);

    // border2 = tertiaryBorderColor [updateColors]
    let tertiary_border_color = tv
        .tertiary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();
    tv.set_border2_if_none(&tertiary_border_color);

    // --- Flowchart variables ---
    tv.set_node_bkg_if_none(&primary_color);
    tv.set_main_bkg_if_none(&primary_color);
    tv.set_node_border_if_none(&primary_border_color);
    tv.set_cluster_bkg_if_none(tertiary_hsl.to_string());
    tv.set_cluster_border_if_none(&tertiary_border_color);
    tv.set_default_link_color_if_none(&line_color);
    let title_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "#ccc".to_string());
    tv.set_title_color_if_none(title_color);
    tv.set_node_text_color_if_none(&primary_text_color);

    // edgeLabelBackground
    if !is_truthy(&tv.edge_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.edge_label_background = Some(v.to_string());
    }

    // --- Sequence Diagram variables ---
    tv.set_actor_border_if_none(&primary_border_color);
    tv.set_actor_bkg_if_none(&primary_color); // mainBkg
    tv.set_actor_text_color_if_none(&primary_text_color);
    tv.set_actor_line_color_if_none(&primary_border_color);
    tv.set_label_box_bkg_color_if_none(&primary_color);
    tv.set_signal_color_if_none(&primary_text_color);
    tv.set_signal_text_color_if_none(&primary_text_color);
    tv.set_label_box_border_color_if_none(&primary_border_color);
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

    // --- Gantt chart variables ---
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

    // --- Architecture Diagram variables ---
    tv.set_arch_edge_color_if_none(&line_color);
    tv.set_arch_edge_arrow_color_if_none(&line_color);
    tv.set_arch_edge_width_if_none("3");
    tv.set_arch_group_border_color_if_none(&primary_border_color);
    tv.set_arch_group_border_width_if_none("2px");

    // --- State colors ---
    tv.set_transition_color_if_none(&line_color);
    tv.set_transition_label_color_if_none(&primary_text_color);
    if !is_truthy(&tv.state_label_color) {
        tv.state_label_color = Some(primary_text_color.clone());
    }
    tv.set_state_bkg_if_none(&primary_color);
    tv.set_label_background_color_if_none(&primary_color);
    tv.set_composite_background_if_none(&background);
    tv.set_alt_background_if_none("#f0f0f0");
    tv.set_composite_title_background_if_none(&primary_color);
    tv.set_composite_border_if_none(&primary_border_color);
    tv.set_inner_end_background_if_none(&primary_border_color);
    tv.set_error_bkg_color_if_none(tertiary_hsl.to_string());
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();
    tv.set_error_text_color_if_none(&tertiary_text_color);
    tv.set_special_state_color_if_none(&line_color);

    // --- Color Scales ---
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

    // scaleLabelColor = labelTextColor
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
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(c_scales[i].to_string())
                {
                    tv.$field = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(c_scales[i].to_string())
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

    // --- Surface colors ---
    let surface_multiplier: f64 = if dark_mode { -4.0 } else { -1.0 };
    let main_bkg_hsl = Rgb::try_from(
        tv.main_bkg
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or("#2a2020"),
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

    // --- class ---
    tv.set_class_text_if_none(&primary_text_color);

    // --- user-journey fill types ---
    tv.set_fill_type0_if_none(&primary_color);
    tv.set_fill_type1_if_none(secondary_hsl.to_string());
    tv.set_fill_type2_if_none(primary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type3_if_none(secondary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type4_if_none(primary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type5_if_none(secondary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type6_if_none(primary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());
    tv.set_fill_type7_if_none(secondary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());

    // --- Pie ---
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
    let task_text_dark = tv
        .task_text_dark_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_pie_title_text_color_if_none(&task_text_dark);
    tv.set_pie_section_text_size_if_none("17px");
    tv.set_pie_section_text_color_if_none(&primary_text_color);
    tv.set_pie_legend_text_size_if_none("17px");
    tv.set_pie_legend_text_color_if_none(&task_text_dark);
    tv.set_pie_stroke_color_if_none("black");
    tv.set_pie_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_color_if_none("black");
    tv.set_pie_opacity_if_none("0.7");

    // --- Venn ---
    let venn_title = tv
        .title_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_venn_title_text_color_if_none(venn_title);
    tv.set_venn_set_text_color_if_none(&primary_text_color);

    // --- Quadrant ---
    tv.set_quadrant1_fill_if_none(&primary_color);
    // quadrant2Fill = adjust(primaryColor, {r: 5, g: 5, b: 5}) — approximated as lighten
    tv.set_quadrant2_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 2.0).to_string());
    tv.set_quadrant3_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 4.0).to_string());
    tv.set_quadrant4_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 6.0).to_string());
    tv.set_quadrant1_text_fill_if_none(&primary_text_color);
    tv.set_quadrant2_text_fill_if_none(&primary_text_color);
    tv.set_quadrant3_text_fill_if_none(&primary_text_color);
    tv.set_quadrant4_text_fill_if_none(&primary_text_color);

    // quadrantPointFill = isDark(quadrant1Fill) ? lighten : darken — dark theme so lighten
    tv.set_quadrant_point_fill_if_none(primary_hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    tv.set_quadrant_point_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_x_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_y_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_internal_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_external_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_title_fill_if_none(&primary_text_color);

    // --- Requirement diagram ---
    tv.set_requirement_background_if_none(&primary_color);
    tv.set_requirement_border_color_if_none(&primary_border_color);
    tv.set_requirement_border_size_if_none("1");
    tv.set_requirement_text_color_if_none(&primary_text_color);
    tv.set_relation_color_if_none(&line_color);
    if !is_truthy(&tv.relation_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.relation_label_background = Some(v.to_string());
    }
    tv.set_relation_label_color_if_none(&primary_text_color);

    // --- Git ---
    let git0_hsl = Rgb::try_from("#0b0000").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 100.0,
        l_pct: 2.0,
    });
    let git1_hsl = Rgb::try_from("#4d1037").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 320.0,
        s_pct: 65.0,
        l_pct: 18.0,
    });
    let git2_hsl = Rgb::try_from("#3f5258").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 195.0,
        s_pct: 16.0,
        l_pct: 30.0,
    });
    let git3_hsl = Rgb::try_from("#4f2f1b").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 25.0,
        s_pct: 50.0,
        l_pct: 21.0,
    });
    let git4_hsl = Rgb::try_from("#6e0a0a").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 0.0,
        s_pct: 83.0,
        l_pct: 24.0,
    });
    let git5_hsl = Rgb::try_from("#3b0048").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 290.0,
        s_pct: 100.0,
        l_pct: 14.0,
    });
    let git6_hsl = Rgb::try_from("#995a01").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 35.0,
        s_pct: 99.0,
        l_pct: 30.0,
    });
    let git7_hsl = Rgb::try_from("#154706").map(Hsl::from).unwrap_or(Hsl {
        h_deg: 107.0,
        s_pct: 84.0,
        l_pct: 15.0,
    });

    // gitDarkMode = true, so lighten(25)
    let git_adj: f64 = 25.0;
    let git = [
        git0_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git1_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git2_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git3_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git4_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git5_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git6_hsl.adjust_hsl(0.0, 0.0, git_adj),
        git7_hsl.adjust_hsl(0.0, 0.0, git_adj),
    ];

    tv.set_git0_if_none(git[0].to_string());
    tv.set_git1_if_none(git[1].to_string());
    tv.set_git2_if_none(git[2].to_string());
    tv.set_git3_if_none(git[3].to_string());
    tv.set_git4_if_none(git[4].to_string());
    tv.set_git5_if_none(git[5].to_string());
    tv.set_git6_if_none(git[6].to_string());
    tv.set_git7_if_none(git[7].to_string());

    // gitInv = invert(git)
    for i in 0..8 {
        if let Ok(Rgb { r, g, b }) = Rgb::try_from(git[i].to_string()) {
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
    let commit_label_color = tv
        .secondary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(&primary_text_color)
        .to_string();
    tv.set_commit_label_color_if_none(commit_label_color);
    tv.set_commit_label_background_if_none(secondary_hsl.to_string());
    tv.set_commit_label_font_size_if_none("10px");

    // --- ER diagram ---
    tv.set_attribute_background_color_odd_if_none("#ffffff");
    tv.set_attribute_background_color_even_if_none("#f2f2f2");

    // --- xyChart ---
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    xy.set_background_color_if_none(&background);
    xy.set_plot_color_palette_if_none(
        "#FFF4DD,#FFD8B1,#FFA07A,#ECEFF1,#D6DBDF,#C3E0A8,#FFB6A4,#FFD74D,#738FA7,#FFFFF0",
    );
    xy.fill_prime_color(primary_text_color);
}
