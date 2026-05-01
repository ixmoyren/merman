use crate::color::{Hsl, Rgb};
use crate::theme::variables::{ThemeVariables, is_truthy};

pub(crate) fn apply_dark_theme_defaults(tv: &mut ThemeVariables) {
    let dark_mode = tv.dark_mode.unwrap_or(true);

    // --- Constructor defaults ---
    tv.set_background_if_none("#333");
    tv.set_primary_color_if_none("#1f2020");
    tv.set_main_bkg_if_none("#1f2020");

    // secondaryColor = lighten(primaryColor, 16) — computed in derived section
    // tertiaryColor = adjust(primaryColor, {h: -160}) — computed in derived section

    tv.set_main_contrast_color_if_none("lightgrey");
    tv.set_border1_if_none("#ccc");
    tv.set_border2_if_none("rgba(255,255,255,0.25)");
    tv.set_label_background_if_none("#181818");
    tv.set_text_color_if_none("#ccc");
    tv.set_font_family_if_none("\"trebuchet ms\", verdana, arial, sans-serif");
    tv.set_font_size_if_none("16px");
    tv.set_title_color_if_none("#F9FFFE");
    tv.set_sequence_number_color_if_none("black");
    tv.set_note_bkg_color_if_none("#fff5ad"); // overridden in updateColors → secondBkg

    // Gantt constructor values (some overridden in updateColors)
    if !is_truthy(&tv.section_bkg_color)
        && let Ok(rgb) = Rgb::try_from("#EAE8D9")
    {
        let hsl = Hsl::from(rgb);
        tv.section_bkg_color = Some(hsl.adjust_hsl(0.0, 0.0, -30.0).to_string());
    }
    tv.set_section_bkg_color2_if_none("#EAE8D9");
    tv.set_task_text_clickable_color_if_none("#003163");
    tv.set_active_task_bkg_color_if_none("#81B1DB");
    tv.set_done_task_border_color_if_none("grey");
    tv.set_crit_border_color_if_none("#E83737");
    tv.set_crit_bkg_color_if_none("#E83737");
    tv.set_today_line_color_if_none("#DB5757");
    tv.set_vert_line_color_if_none("#00BFFF");

    // Architecture constructor values
    tv.set_arch_edge_width_if_none("3");
    tv.set_arch_group_border_width_if_none("2px");

    // Error / gradient / shadow
    tv.set_error_bkg_color_if_none("#a44141");
    tv.set_error_text_color_if_none("#ddd");
    if tv.use_gradient.is_none() {
        tv.use_gradient = Some(true);
    }
    tv.set_drop_shadow_if_none("drop-shadow( 1px 2px 2px rgba(185,185,185,1))");
    tv.set_note_font_weight_if_none("normal");
    tv.set_font_weight_if_none("normal");
    if tv.radius.is_none() {
        tv.radius = Some(5.0);
    }
    if tv.stroke_width.is_none() {
        tv.stroke_width = Some(1.0);
    }
    if tv.dark_mode.is_none() {
        tv.dark_mode = Some(true);
    }

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

    // primaryTextColor = invert(primaryColor) [constructor]
    if !is_truthy(&tv.primary_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_color)
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
    let primary_text_color = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#e0dfdf")
        .to_string();

    // secondaryColor = lighten(primaryColor, 16) [constructor]
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

    // tertiaryColor = adjust(primaryColor, {h: -160}) [constructor]
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

    // primaryBorderColor = invert(background) [constructor]
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

    // lineColor = mainContrastColor [updateColors overrides constructor invert(background)]
    let main_contrast_color = tv
        .main_contrast_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("lightgrey")
        .to_string();
    tv.set_line_color_if_none(&main_contrast_color);
    let line_color = tv
        .line_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("lightgrey")
        .to_string();

    // arrowheadColor = mainContrastColor [updateColors]
    tv.set_arrowhead_color_if_none(&main_contrast_color);

    // darkTextColor = lighten(invert('#323D47'), 10) [constructor]
    if !is_truthy(&tv.dark_text_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from("#323D47")
    {
        let inv = Rgb {
            r: 1.0 - r,
            g: 1.0 - g,
            b: 1.0 - b,
        };
        let hsl = Hsl::from(inv);
        tv.dark_text_color = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }
    let dark_text_color = tv
        .dark_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cdc2b8")
        .to_string();

    // secondBkg = lighten(mainBkg, 16) [updateColors]
    let main_bkg = tv
        .main_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#1f2020")
        .to_string();
    let main_bkg_hsl = Rgb::try_from(&main_bkg)
        .map(Hsl::from)
        .unwrap_or(primary_hsl);
    tv.set_second_bkg_if_none(main_bkg_hsl.adjust_hsl(0.0, 0.0, 16.0).to_string());
    let second_bkg = tv
        .second_bkg
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#3a3a3a")
        .to_string();

    // gradientStart = primaryBorderColor [constructor, references field set above]
    tv.set_gradient_start_if_none(&primary_border_color);

    // gradientStop = secondaryBorderColor [constructor]
    let secondary_border_color = tv
        .secondary_border_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();
    tv.set_gradient_stop_if_none(&secondary_border_color);

    // excludeBkgColor = darken(sectionBkgColor, 10) [constructor, depends on computed sectionBkgColor]
    if !is_truthy(&tv.exclude_bkg_color)
        && let Some(ref sc) = tv.section_bkg_color
        && is_truthy(&tv.section_bkg_color)
        && let Ok(rgb) = Rgb::try_from(sc)
    {
        let hsl = Hsl::from(rgb);
        tv.exclude_bkg_color = Some(hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    }

    // taskBorderColor = rgba(255,255,255,70) → rgba(255,255,255,0.7) [constructor]
    tv.set_task_border_color_if_none("rgba(255,255,255,0.7)");

    // activeTaskBorderColor = rgba(255,255,255,50) → rgba(255,255,255,0.5) [constructor]
    tv.set_active_task_border_color_if_none("rgba(255,255,255,0.5)");

    // --- Flowchart variables [updateColors] ---
    tv.set_node_bkg_if_none(&main_bkg);
    tv.set_node_border_if_none(&primary_border_color); // actually updateColors sets to border1 = '#ccc'
    // Wait — nodeBorder = this.border1 in updateColors. Since border1 was set to '#ccc' and nodeBorder
    // was 'calculated', the final is '#ccc'. Let me use border1.
    let border1 = tv
        .border1
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ccc")
        .to_string();
    tv.set_node_border_if_none(&border1);
    tv.set_cluster_bkg_if_none(&second_bkg);
    let border2_val = tv
        .border2
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("rgba(255,255,255,0.25)")
        .to_string();
    tv.set_cluster_border_if_none(&border2_val);
    tv.set_default_link_color_if_none(&line_color);

    // edgeLabelBackground = lighten(labelBackground, 25) [updateColors]
    if !is_truthy(&tv.edge_label_background)
        && let Some(ref lb) = tv.label_background
        && is_truthy(&tv.label_background)
        && let Ok(rgb) = Rgb::try_from(lb)
    {
        let hsl = Hsl::from(rgb);
        tv.edge_label_background = Some(hsl.adjust_hsl(0.0, 0.0, 25.0).to_string());
    }

    // --- Sequence Diagram variables [updateColors] ---
    tv.set_actor_border_if_none(&border1);
    tv.set_actor_bkg_if_none(&main_bkg);
    tv.set_actor_text_color_if_none(&main_contrast_color);
    tv.set_actor_line_color_if_none(&border1);
    tv.set_signal_color_if_none(&main_contrast_color);
    tv.set_signal_text_color_if_none(&main_contrast_color);
    tv.set_label_box_bkg_color_if_none(&main_bkg);
    tv.set_label_box_border_color_if_none(&border1);
    tv.set_label_text_color_if_none(&main_contrast_color);
    tv.set_loop_text_color_if_none(&main_contrast_color);

    // noteBorderColor = secondaryBorderColor [updateColors]
    tv.set_note_border_color_if_none(&secondary_border_color);

    // noteBkgColor = secondBkg [updateColors overrides constructor '#fff5ad']
    tv.set_note_bkg_color_if_none(&second_bkg);

    // noteTextColor = secondaryTextColor [updateColors]
    let secondary_text_color = tv
        .secondary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();
    tv.set_note_text_color_if_none(&secondary_text_color);

    // activationBorderColor = border1 [updateColors]
    tv.set_activation_border_color_if_none(&border1);
    // activationBkgColor = secondBkg [updateColors]
    tv.set_activation_bkg_color_if_none(&second_bkg);

    // personBorder = primaryBorderColor [constructor]
    tv.set_person_border_if_none(&primary_border_color);
    // personBkg = mainBkg [constructor]
    tv.set_person_bkg_if_none(&main_bkg);

    // --- Gantt chart variables [updateColors] ---
    tv.set_alt_section_bkg_color_if_none(&background);

    // taskBkgColor = lighten(mainBkg, 23) [updateColors]
    tv.set_task_bkg_color_if_none(main_bkg_hsl.adjust_hsl(0.0, 0.0, 23.0).to_string());

    // taskTextColor = darkTextColor [updateColors]
    tv.set_task_text_color_if_none(&dark_text_color);

    // taskTextLightColor = mainContrastColor [updateColors]
    tv.set_task_text_light_color_if_none(&main_contrast_color);

    // taskTextOutsideColor = taskTextLightColor = mainContrastColor [updateColors]
    tv.set_task_text_outside_color_if_none(&main_contrast_color);

    // gridColor = mainContrastColor [updateColors]
    tv.set_grid_color_if_none(&main_contrast_color);

    // doneTaskBkgColor = mainContrastColor [updateColors]
    tv.set_done_task_bkg_color_if_none(&main_contrast_color);

    // taskTextDarkColor = invert(doneTaskBkgColor) = invert(mainContrastColor) [updateColors]
    if !is_truthy(&tv.task_text_dark_color)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(
            tv.done_task_bkg_color
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("lightgrey"),
        )
    {
        tv.task_text_dark_color = Some(
            Rgb {
                r: 1.0 - r,
                g: 1.0 - g,
                b: 1.0 - b,
            }
            .to_string(),
        );
    }

    // --- Architecture Diagram variables [updateColors] ---
    tv.set_arch_edge_color_if_none(&line_color);
    tv.set_arch_edge_arrow_color_if_none(&line_color);
    tv.set_arch_group_border_color_if_none(&primary_border_color);

    // --- State colors [updateColors] ---
    // transitionColor = transitionColor || lineColor
    tv.set_transition_color_if_none(&line_color);
    // transitionLabelColor = transitionLabelColor || textColor
    let text_color = tv
        .text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ccc")
        .to_string();
    tv.set_transition_label_color_if_none(&text_color);
    // stateLabelColor = stateLabelColor || stateBkg || primaryTextColor
    if !is_truthy(&tv.state_label_color) {
        tv.state_label_color = Some(primary_text_color.clone());
    }
    // stateBkg = stateBkg || mainBkg
    tv.set_state_bkg_if_none(&main_bkg);
    // labelBackgroundColor = labelBackgroundColor || stateBkg
    tv.set_label_background_color_if_none(&main_bkg);
    // compositeBackground = compositeBackground || background || tertiaryColor
    tv.set_composite_background_if_none(&background);
    // altBackground = altBackground || '#555'
    tv.set_alt_background_if_none("#555");
    // compositeTitleBackground = compositeTitleBackground || mainBkg
    tv.set_composite_title_background_if_none(&main_bkg);
    // compositeBorder = compositeBorder || nodeBorder (= border1)
    tv.set_composite_border_if_none(&border1);
    // innerEndBackground = primaryBorderColor
    tv.set_inner_end_background_if_none(&primary_border_color);
    // specialStateColor = '#f4f4f4'
    tv.set_special_state_color_if_none("#f4f4f4");
    // errorBkgColor = errorBkgColor || tertiaryColor
    if !is_truthy(&tv.error_bkg_color) {
        tv.error_bkg_color = Some(tertiary_hsl.to_string());
    }
    // errorTextColor = errorTextColor || tertiaryTextColor
    let tertiary_text_color = tv
        .tertiary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#cccccc")
        .to_string();
    if !is_truthy(&tv.error_text_color) {
        tv.error_text_color = Some(tertiary_text_color.clone());
    }

    // --- fillType0-7 [updateColors] ---
    tv.set_fill_type0_if_none(&primary_color);
    tv.set_fill_type1_if_none(secondary_hsl.to_string());
    tv.set_fill_type2_if_none(primary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type3_if_none(secondary_hsl.adjust_hsl(64.0, 0.0, 0.0).to_string());
    tv.set_fill_type4_if_none(primary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type5_if_none(secondary_hsl.adjust_hsl(-64.0, 0.0, 0.0).to_string());
    tv.set_fill_type6_if_none(primary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());
    tv.set_fill_type7_if_none(secondary_hsl.adjust_hsl(128.0, 0.0, 0.0).to_string());

    // --- cScale0-11 [updateColors, dynamic from primary/secondary/tertiary] ---
    tv.set_c_scale0_if_none(&primary_color);
    tv.set_c_scale1_if_none(secondary_hsl.to_string());
    tv.set_c_scale2_if_none(tertiary_hsl.to_string());
    tv.set_c_scale3_if_none(primary_hsl.adjust_hsl(30.0, 0.0, 0.0).to_string());
    tv.set_c_scale4_if_none(primary_hsl.adjust_hsl(60.0, 0.0, 0.0).to_string());
    tv.set_c_scale5_if_none(primary_hsl.adjust_hsl(90.0, 0.0, 0.0).to_string());
    tv.set_c_scale6_if_none(primary_hsl.adjust_hsl(120.0, 0.0, 0.0).to_string());
    tv.set_c_scale7_if_none(primary_hsl.adjust_hsl(150.0, 0.0, 0.0).to_string());
    tv.set_c_scale8_if_none(primary_hsl.adjust_hsl(210.0, 0.0, 0.0).to_string());
    tv.set_c_scale9_if_none(primary_hsl.adjust_hsl(270.0, 0.0, 0.0).to_string());
    tv.set_c_scale10_if_none(primary_hsl.adjust_hsl(300.0, 0.0, 0.0).to_string());
    tv.set_c_scale11_if_none(primary_hsl.adjust_hsl(330.0, 0.0, 0.0).to_string());

    // Build the c_scales array for peer/inv/label computation
    let c_scales: [String; 12] = [
        primary_color.clone(),
        secondary_hsl.to_string(),
        tertiary_hsl.to_string(),
        primary_hsl.adjust_hsl(30.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(90.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(150.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(210.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(270.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(300.0, 0.0, 0.0).to_string(),
        primary_hsl.adjust_hsl(330.0, 0.0, 0.0).to_string(),
    ];

    // scaleLabelColor = scaleLabelColor || (darkMode ? 'black' : labelTextColor)
    let scale_label_color = tv
        .scale_label_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or(if dark_mode {
            "black"
        } else {
            tv.label_text_color
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("lightgrey")
        })
        .to_string();
    tv.set_scale_label_color_if_none(&scale_label_color);

    // peer_delta = 10.0 for lighten(cScale, 10) [updateColors]
    let peer_delta: f64 = 10.0;

    for i in 0..12 {
        macro_rules! set_peer {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(rgb) = Rgb::try_from(&c_scales[i])
                {
                    tv.$field = Some(Hsl::from(rgb).adjust_hsl(0.0, 0.0, peer_delta).to_string());
                }
            };
        }
        macro_rules! set_inv {
            ($field:ident) => {
                if tv.$field.is_none()
                    && let Ok(Rgb { r, g, b }) = Rgb::try_from(&c_scales[i])
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

    // --- Surface colors [updateColors] ---
    // surfaceN = surfaceN || adjust(mainBkg, {h: 30, s: -30, l: -(-10 + i*4)})
    // surfacePeerN = surfacePeerN || adjust(mainBkg, {h: 30, s: -30, l: -(-7 + i*4)})
    for i in 0..5 {
        let surf_l = -(-10.0 + i as f64 * 4.0); // negative of: -( -10 + i*4 ) = 10 - i*4
        let surf_peer_l = -(-7.0 + i as f64 * 4.0); // negative of: -( -7 + i*4 ) = 7 - i*4
        let surf = main_bkg_hsl.adjust_hsl(30.0, -30.0, surf_l).to_string();
        let surf_peer = main_bkg_hsl
            .adjust_hsl(30.0, -30.0, surf_peer_l)
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

    // --- Pie diagram [updateColors] ---
    // pieN = cScaleN for N in 0..12
    for i in 0..12 {
        let val = c_scales[i].clone();
        match i {
            0 => tv.set_pie1_if_none(&val),
            1 => tv.set_pie2_if_none(&val),
            2 => tv.set_pie3_if_none(&val),
            3 => tv.set_pie4_if_none(&val),
            4 => tv.set_pie5_if_none(&val),
            5 => tv.set_pie6_if_none(&val),
            6 => tv.set_pie7_if_none(&val),
            7 => tv.set_pie8_if_none(&val),
            8 => tv.set_pie9_if_none(&val),
            9 => tv.set_pie10_if_none(&val),
            10 => tv.set_pie11_if_none(&val),
            11 => tv.set_pie12_if_none(&val),
            _ => {}
        }
    }

    tv.set_pie_title_text_size_if_none("25px");
    tv.set_pie_title_text_color_if_none(&main_contrast_color);
    tv.set_pie_section_text_size_if_none("17px");
    tv.set_pie_section_text_color_if_none(&text_color);
    tv.set_pie_legend_text_size_if_none("17px");
    tv.set_pie_legend_text_color_if_none(&main_contrast_color);
    tv.set_pie_stroke_color_if_none("black");
    tv.set_pie_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_width_if_none("2px");
    tv.set_pie_outer_stroke_color_if_none("black");
    tv.set_pie_opacity_if_none("0.7");

    // --- Venn [updateColors] ---
    // venn(i+1) = venn(i+1) ?? lighten(cScale(i), 30)
    for i in 0..8 {
        if let Ok(rgb) = Rgb::try_from(&c_scales[i]) {
            let hsl = Hsl::from(rgb);
            let venn_val = hsl.adjust_hsl(0.0, 0.0, 30.0).to_string();
            match i {
                0 => tv.set_venn1_if_none(&venn_val),
                1 => tv.set_venn2_if_none(&venn_val),
                2 => tv.set_venn3_if_none(&venn_val),
                3 => tv.set_venn4_if_none(&venn_val),
                4 => tv.set_venn5_if_none(&venn_val),
                5 => tv.set_venn6_if_none(&venn_val),
                6 => tv.set_venn7_if_none(&venn_val),
                7 => tv.set_venn8_if_none(&venn_val),
                _ => {}
            }
        }
    }

    let title_color = tv
        .title_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#F9FFFE")
        .to_string();
    tv.set_venn_title_text_color_if_none(&title_color);
    tv.set_venn_set_text_color_if_none(&text_color);

    // --- Quadrant [updateColors] ---
    tv.set_quadrant1_fill_if_none(&primary_color);
    // quadrant2Fill = quadrant2Fill || adjust(primaryColor, {r: 5, g: 5, b: 5})
    if !is_truthy(&tv.quadrant2_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_color)
    {
        tv.quadrant2_fill = Some(
            Rgb {
                r: (r + 5.0 / 255.0).clamp(0.0, 1.0),
                g: (g + 5.0 / 255.0).clamp(0.0, 1.0),
                b: (b + 5.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }
    // quadrant3Fill = quadrant3Fill || adjust(primaryColor, {r: 10, g: 10, b: 10})
    if !is_truthy(&tv.quadrant3_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_color)
    {
        tv.quadrant3_fill = Some(
            Rgb {
                r: (r + 10.0 / 255.0).clamp(0.0, 1.0),
                g: (g + 10.0 / 255.0).clamp(0.0, 1.0),
                b: (b + 10.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }
    // quadrant4Fill = quadrant4Fill || adjust(primaryColor, {r: 15, g: 15, b: 15})
    if !is_truthy(&tv.quadrant4_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_color)
    {
        tv.quadrant4_fill = Some(
            Rgb {
                r: (r + 15.0 / 255.0).clamp(0.0, 1.0),
                g: (g + 15.0 / 255.0).clamp(0.0, 1.0),
                b: (b + 15.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    tv.set_quadrant1_text_fill_if_none(&primary_text_color);

    // quadrant2TextFill = quadrant2TextFill || adjust(primaryTextColor, {r: -5, g: -5, b: -5})
    if !is_truthy(&tv.quadrant2_text_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant2_text_fill = Some(
            Rgb {
                r: (r - 5.0 / 255.0).clamp(0.0, 1.0),
                g: (g - 5.0 / 255.0).clamp(0.0, 1.0),
                b: (b - 5.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }
    // quadrant3TextFill = quadrant3TextFill || adjust(primaryTextColor, {r: -10, g: -10, b: -10})
    if !is_truthy(&tv.quadrant3_text_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant3_text_fill = Some(
            Rgb {
                r: (r - 10.0 / 255.0).clamp(0.0, 1.0),
                g: (g - 10.0 / 255.0).clamp(0.0, 1.0),
                b: (b - 10.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }
    // quadrant4TextFill = quadrant4TextFill || adjust(primaryTextColor, {r: -15, g: -15, b: -15})
    if !is_truthy(&tv.quadrant4_text_fill)
        && let Ok(Rgb { r, g, b }) = Rgb::try_from(&primary_text_color)
    {
        tv.quadrant4_text_fill = Some(
            Rgb {
                r: (r - 15.0 / 255.0).clamp(0.0, 1.0),
                g: (g - 15.0 / 255.0).clamp(0.0, 1.0),
                b: (b - 15.0 / 255.0).clamp(0.0, 1.0),
            }
            .to_string(),
        );
    }

    // quadrantPointFill = quadrantPointFill || (isDark(quadrant1Fill) ? lighten : darken)
    // Since this is a dark theme, quadrant1Fill is dark, so lighten(quadrant1Fill)
    if !is_truthy(&tv.quadrant_point_fill)
        && let Some(ref q1f) = tv.quadrant1_fill
        && is_truthy(&tv.quadrant1_fill)
        && let Ok(rgb) = Rgb::try_from(q1f)
    {
        let hsl = Hsl::from(rgb);
        tv.quadrant_point_fill = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }

    tv.set_quadrant_point_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_x_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_y_axis_text_fill_if_none(&primary_text_color);
    tv.set_quadrant_internal_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_external_border_stroke_fill_if_none(&primary_border_color);
    tv.set_quadrant_title_fill_if_none(&primary_text_color);

    // --- xyChart [updateColors] ---
    let xy = tv.xy_chart.get_or_insert_with(Default::default);
    xy.set_background_color_if_none(&background);
    let pt = tv
        .primary_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| tv.text_color.as_deref().filter(|s| !s.trim().is_empty()))
        .unwrap_or("#e0dfdf")
        .to_string();
    xy.set_plot_color_palette_if_none(
        "#3498db,#2ecc71,#e74c3c,#f1c40f,#bdc3c7,#ffffff,#34495e,#9b59b6,#1abc9c,#e67e22",
    );
    xy.fill_prime_color(pt);

    // --- Packet [updateColors] ---
    let packet = tv.packet.get_or_insert_with(Default::default);
    if packet.start_byte_color.is_none() {
        packet.start_byte_color = Some(primary_text_color.clone());
    }
    if packet.end_byte_color.is_none() {
        packet.end_byte_color = Some(primary_text_color.clone());
    }
    if packet.label_color.is_none() {
        packet.label_color = Some(primary_text_color.clone());
    }
    if packet.title_color.is_none() {
        packet.title_color = Some(primary_text_color.clone());
    }
    if packet.block_stroke_color.is_none() {
        packet.block_stroke_color = Some(primary_text_color.clone());
    }
    if packet.block_fill_color.is_none() {
        packet.block_fill_color = Some(background.clone());
    }

    // --- Radar [updateColors] ---
    let radar = tv.radar.get_or_insert_with(Default::default);
    radar.set_axis_color_if_none(&line_color);
    radar.set_axis_stroke_width_if_none(2.0);
    radar.set_axis_label_font_size_if_none(12.0);
    radar.set_curve_opacity_if_none(0.5);
    radar.set_curve_stroke_width_if_none(2.0);
    radar.set_graticule_color_if_none("#DEDEDE");
    radar.set_graticule_stroke_width_if_none(1.0);
    radar.set_graticule_opacity_if_none(0.3);
    radar.set_legend_box_size_if_none(12.0);
    radar.set_legend_font_size_if_none(12.0);

    // --- Wardley [updateColors] ---
    tv.set_wardley_evolution_color_if_none("#ff6b6b");
    let wardley = tv.wardley.get_or_insert_with(Default::default);
    if wardley.background_color.is_none() {
        wardley.background_color = Some(background.clone());
    }
    if wardley.axis_color.is_none() {
        wardley.axis_color = Some(line_color.clone());
    }
    if wardley.axis_text_color.is_none() {
        wardley.axis_text_color = Some(primary_text_color.clone());
    }
    if wardley.grid_color.is_none() {
        wardley.grid_color = Some(
            tv.grid_color
                .as_deref()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or("lightgrey")
                .to_string(),
        );
    }
    if wardley.component_fill.is_none() {
        wardley.component_fill = Some(main_bkg.clone());
    }
    if wardley.component_stroke.is_none() {
        wardley.component_stroke = Some(line_color.clone());
    }
    if wardley.component_label_color.is_none() {
        wardley.component_label_color = Some(primary_text_color.clone());
    }
    if wardley.link_stroke.is_none() {
        wardley.link_stroke = Some(line_color.clone());
    }
    let wardley_evolution_color = tv
        .wardley_evolution_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("#ff6b6b")
        .to_string();
    if wardley.evolution_stroke.is_none() {
        wardley.evolution_stroke = Some(wardley_evolution_color);
    }
    if wardley.annotation_stroke.is_none() {
        wardley.annotation_stroke = Some(line_color.clone());
    }
    if wardley.annotation_text_color.is_none() {
        wardley.annotation_text_color = Some(primary_text_color.clone());
    }
    if wardley.annotation_fill.is_none() {
        wardley.annotation_fill = Some(main_bkg.clone());
    }

    // --- requirement-diagram [updateColors] ---
    tv.set_requirement_background_if_none(&primary_color);
    tv.set_requirement_border_color_if_none(&primary_border_color);
    tv.set_requirement_border_size_if_none("1");
    tv.set_requirement_text_color_if_none(&primary_text_color);
    tv.set_relation_color_if_none(&line_color);
    // relationLabelBackground = relationLabelBackground || (darkMode ? darken(secondaryColor, 30) : secondaryColor)
    if !is_truthy(&tv.relation_label_background) {
        let v = if dark_mode {
            secondary_hsl.adjust_hsl(0.0, 0.0, -30.0)
        } else {
            secondary_hsl
        };
        tv.relation_label_background = Some(v.to_string());
    }
    tv.set_relation_label_color_if_none(&primary_text_color);

    // --- ER [constructor + updateColors] ---
    // rowOdd = rowOdd || lighten(mainBkg, 5) || '#ffffff'
    if !is_truthy(&tv.row_odd) {
        tv.row_odd = Some(main_bkg_hsl.adjust_hsl(0.0, 0.0, 5.0).to_string());
    }
    // rowEven = rowEven || darken(mainBkg, 10)
    if !is_truthy(&tv.row_even) {
        tv.row_even = Some(main_bkg_hsl.adjust_hsl(0.0, 0.0, -10.0).to_string());
    }

    // attributeBackgroundColorOdd = attributeBackgroundColorOdd || lighten(background, 12)
    if !is_truthy(&tv.attribute_background_color_odd)
        && let Ok(rgb) = Rgb::try_from(&background)
    {
        let hsl = Hsl::from(rgb);
        tv.attribute_background_color_odd = Some(hsl.adjust_hsl(0.0, 0.0, 12.0).to_string());
    }
    // attributeBackgroundColorEven = attributeBackgroundColorEven || lighten(background, 2)
    if !is_truthy(&tv.attribute_background_color_even)
        && let Ok(rgb) = Rgb::try_from(&background)
    {
        let hsl = Hsl::from(rgb);
        tv.attribute_background_color_even = Some(hsl.adjust_hsl(0.0, 0.0, 2.0).to_string());
    }

    // --- Event Modeling [updateColors] ---
    tv.set_em_ui_fill_if_none("#2d2d2d");
    tv.set_em_ui_stroke_if_none("#555");
    if !is_truthy(&tv.em_processor_fill)
        && let Ok(rgb) = Rgb::try_from("#5a3d5c")
    {
        let hsl = Hsl::from(rgb);
        tv.em_processor_fill = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }
    tv.set_em_processor_stroke_if_none("#8a6d8c");
    if !is_truthy(&tv.em_read_model_fill)
        && let Ok(rgb) = Rgb::try_from("#3d5a2d")
    {
        let hsl = Hsl::from(rgb);
        tv.em_read_model_fill = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }
    tv.set_em_read_model_stroke_if_none("#6d8c5c");
    if !is_truthy(&tv.em_command_fill)
        && let Ok(rgb) = Rgb::try_from("#2d3d5a")
    {
        let hsl = Hsl::from(rgb);
        tv.em_command_fill = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }
    tv.set_em_command_stroke_if_none("#5c6d8c");
    if !is_truthy(&tv.em_event_fill)
        && let Ok(rgb) = Rgb::try_from("#5a452d")
    {
        let hsl = Hsl::from(rgb);
        tv.em_event_fill = Some(hsl.adjust_hsl(0.0, 0.0, 10.0).to_string());
    }
    tv.set_em_event_stroke_if_none("#8c755c");
    // emSwimlaneBackgroundOdd = emSwimlaneBackgroundOdd || lighten(background, 5)
    if !is_truthy(&tv.em_swimlane_background_odd)
        && let Ok(rgb) = Rgb::try_from(&background)
    {
        let hsl = Hsl::from(rgb);
        tv.em_swimlane_background_odd = Some(hsl.adjust_hsl(0.0, 0.0, 5.0).to_string());
    }
    // emSwimlaneBackgroundStroke = emSwimlaneBackgroundStroke || lighten(background, 12)
    if !is_truthy(&tv.em_swimlane_background_stroke)
        && let Ok(rgb) = Rgb::try_from(&background)
    {
        let hsl = Hsl::from(rgb);
        tv.em_swimlane_background_stroke = Some(hsl.adjust_hsl(0.0, 0.0, 12.0).to_string());
    }
    tv.set_em_arrowhead_if_none(&line_color);
    tv.set_em_relation_stroke_if_none(&line_color);

    // --- Git [updateColors] ---
    // git0 = lighten(secondaryColor, 20)
    // git1 = lighten(pie2 || secondaryColor, 20)
    // etc.
    let git_base: [Hsl; 8] = [
        secondary_hsl,
        secondary_hsl, // pie2 || secondaryColor → secondaryColor (pie2 = cScale2 = secondaryColor in dark theme)
        tertiary_hsl,  // pie3 || tertiaryColor
        primary_hsl.adjust_hsl(-30.0, 0.0, 0.0), // pie4 || adjust(primaryColor, {h: -30})
        primary_hsl.adjust_hsl(-60.0, 0.0, 0.0), // pie5 || adjust(primaryColor, {h: -60})
        primary_hsl.adjust_hsl(-90.0, 0.0, 0.0), // pie6 || adjust(primaryColor, {h: -90})
        primary_hsl.adjust_hsl(60.0, 0.0, 0.0), // pie7 || adjust(primaryColor, {h: +60})
        primary_hsl.adjust_hsl(120.0, 0.0, 0.0), // pie8 || adjust(primaryColor, {h: +120})
    ];
    // git0-7: lighten(base, 20) except git5 and git6 which are lighten(base, 10)
    let git: [String; 8] = {
        let mut arr: [String; 8] = Default::default();
        for i in 0..8 {
            let delta: f64 = if i == 5 || i == 6 { 10.0 } else { 20.0 };
            arr[i] = git_base[i].adjust_hsl(0.0, 0.0, delta).to_string();
        }
        arr
    };

    tv.set_git0_if_none(&git[0]);
    tv.set_git1_if_none(&git[1]);
    tv.set_git2_if_none(&git[2]);
    tv.set_git3_if_none(&git[3]);
    tv.set_git4_if_none(&git[4]);
    tv.set_git5_if_none(&git[5]);
    tv.set_git6_if_none(&git[6]);
    tv.set_git7_if_none(&git[7]);

    // gitInv = invert(git)
    for i in 0..8 {
        if let Ok(Rgb { r, g, b }) = Rgb::try_from(&git[i]) {
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

    // gitBranchLabel0 = invert(labelTextColor)
    // gitBranchLabel1-7 = labelTextColor
    let label_text_color = tv
        .label_text_color
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("lightgrey")
        .to_string();
    let git_branch_label0 = if let Ok(Rgb { r, g, b }) = Rgb::try_from(&label_text_color) {
        Rgb {
            r: 1.0 - r,
            g: 1.0 - g,
            b: 1.0 - b,
        }
        .to_string()
    } else {
        "black".to_string()
    };
    tv.set_git_branch_label0_if_none(&git_branch_label0);
    tv.set_git_branch_label1_if_none(&label_text_color);
    tv.set_git_branch_label2_if_none(&label_text_color);
    tv.set_git_branch_label3_if_none(&label_text_color);
    tv.set_git_branch_label4_if_none(&label_text_color);
    tv.set_git_branch_label5_if_none(&label_text_color);
    tv.set_git_branch_label6_if_none(&label_text_color);
    tv.set_git_branch_label7_if_none(&label_text_color);

    // tag/commit labels [updateColors]
    tv.set_tag_label_color_if_none(&primary_text_color);
    tv.set_tag_label_background_if_none(&primary_color);
    tv.set_tag_label_border_if_none(&primary_border_color);
    tv.set_tag_label_font_size_if_none("10px");
    tv.set_commit_label_color_if_none(&secondary_text_color);
    tv.set_commit_label_background_if_none(secondary_hsl.to_string());
    tv.set_commit_label_font_size_if_none("10px");

    // nodeBorder = nodeBorder || '#999' [updateColors last line]
    // (already set to border1 = '#ccc' above, so this || keeps it)
    tv.set_node_border_if_none("#999");
}
