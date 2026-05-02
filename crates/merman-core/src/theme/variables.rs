use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct XyChartThemeVars {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_title_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_tick_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_title_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_tick_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plot_color_palette: Option<String>,
}

impl XyChartThemeVars {
    pub(crate) fn set_plot_color_palette_if_none(&mut self, value: impl Into<String>) {
        if self
            .plot_color_palette
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.plot_color_palette = Some(value.into());
        }
    }

    pub(crate) fn set_background_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .background_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.background_color = Some(value.into());
        }
    }

    pub(crate) fn fill_prime_color(&mut self, prime_color: String) {
        if self.title_color.is_none() {
            self.title_color = Some(prime_color.clone());
        }
        if self.x_axis_title_color.is_none() {
            self.x_axis_title_color = Some(prime_color.clone());
        }
        if self.x_axis_label_color.is_none() {
            self.x_axis_label_color = Some(prime_color.clone());
        }
        if self.x_axis_tick_color.is_none() {
            self.x_axis_tick_color = Some(prime_color.clone());
        }
        if self.x_axis_line_color.is_none() {
            self.x_axis_line_color = Some(prime_color.clone());
        }
        if self.y_axis_title_color.is_none() {
            self.y_axis_title_color = Some(prime_color.clone());
        }
        if self.y_axis_label_color.is_none() {
            self.y_axis_label_color = Some(prime_color.clone());
        }
        if self.y_axis_tick_color.is_none() {
            self.y_axis_tick_color = Some(prime_color.clone());
        }
        if self.y_axis_line_color.is_none() {
            self.y_axis_line_color = Some(prime_color.clone());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RadarThemeVars {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_stroke_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_label_font_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve_stroke_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graticule_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graticule_stroke_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graticule_opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_box_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_font_size: Option<f64>,
}

impl RadarThemeVars {
    pub(crate) fn set_axis_color_if_none(&mut self, value: impl Into<String>) {
        if self.axis_color.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.axis_color = Some(value.into());
        }
    }

    pub(crate) fn set_axis_stroke_width_if_none(&mut self, value: impl Into<f64>) {
        if self.axis_stroke_width.as_ref().is_none() {
            self.axis_stroke_width = Some(value.into());
        }
    }

    pub(crate) fn set_axis_label_font_size_if_none(&mut self, value: impl Into<f64>) {
        if self.axis_label_font_size.as_ref().is_none() {
            self.axis_label_font_size = Some(value.into());
        }
    }

    pub(crate) fn set_curve_opacity_if_none(&mut self, value: impl Into<f64>) {
        if self.curve_opacity.as_ref().is_none() {
            self.curve_opacity = Some(value.into());
        }
    }

    pub(crate) fn set_curve_stroke_width_if_none(&mut self, value: impl Into<f64>) {
        if self.curve_stroke_width.as_ref().is_none() {
            self.curve_stroke_width = Some(value.into());
        }
    }

    pub(crate) fn set_graticule_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .graticule_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.graticule_color = Some(value.into());
        }
    }

    pub(crate) fn set_graticule_stroke_width_if_none(&mut self, value: impl Into<f64>) {
        if self.graticule_stroke_width.as_ref().is_none() {
            self.graticule_stroke_width = Some(value.into());
        }
    }

    pub(crate) fn set_graticule_opacity_if_none(&mut self, value: impl Into<f64>) {
        if self.graticule_opacity.as_ref().is_none() {
            self.graticule_opacity = Some(value.into());
        }
    }

    pub(crate) fn set_legend_box_size_if_none(&mut self, value: impl Into<f64>) {
        if self.legend_box_size.as_ref().is_none() {
            self.legend_box_size = Some(value.into());
        }
    }

    pub(crate) fn set_legend_font_size_if_none(&mut self, value: impl Into<f64>) {
        if self.legend_font_size.as_ref().is_none() {
            self.legend_font_size = Some(value.into());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WardleyThemeVars {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evolution_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_fill: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PacketThemeVars {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_byte_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_byte_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_stroke_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_fill_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVariables {
    // --- Base colors ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_color: Option<String>,

    // --- Border colors ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_border_color: Option<String>,

    // --- Text colors ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tertiary_text_color: Option<String>,

    // --- Lines ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrowhead_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,

    // --- Font ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_font_weight: Option<String>,

    // --- Border 1 / Border 2 ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border2: Option<String>,

    // --- Flowchart ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_link_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_label_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_text_color: Option<String>,

    // --- Label ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,

    // --- Notes ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_border_color: Option<String>,

    // --- Sequence Diagram ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_box_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_box_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number_color: Option<String>,

    // --- Gantt ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_section_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_bkg_color2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text_light_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text_dark_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text_outside_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text_clickable_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_task_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_task_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_task_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_task_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crit_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crit_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub today_line_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vert_line_color: Option<String>,

    // --- C4 / Person ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_bkg: Option<String>,

    // --- ER ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_odd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_even: Option<String>,

    // --- State ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_bkg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_title_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_end_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_bkg_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_state_color: Option<String>,

    // --- Neo-specific state ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_edge_label_background: Option<String>,

    // --- Class ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_text: Option<String>,

    // --- Architecture ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_edge_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_edge_arrow_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_edge_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_group_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_group_border_width: Option<String>,

    // --- Requirement ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_border_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_border_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_edge_label_background: Option<String>,

    // --- Relation ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_label_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_label_color: Option<String>,

    // --- ER edge ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub er_edge_label_background: Option<String>,

    // --- Event Modeling ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_ui_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_ui_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_processor_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_processor_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_read_model_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_read_model_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_command_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_command_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_event_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_event_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_swimlane_background_odd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_swimlane_background_stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_arrowhead: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub em_relation_stroke: Option<String>,

    // --- ER attribute background ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_background_color_odd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_background_color_even: Option<String>,

    // --- Gradient ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_start: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_shadow: Option<String>,

    // --- Dark theme extras ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_contrast_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_text_color: Option<String>,

    // --- Neutral theme extras ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done: Option<String>,

    // --- Mindmap ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_label_color: Option<String>,

    // --- Filter ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_color: Option<String>,

    // --- Scale label ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_label_color: Option<String>,

    // --- Branch / Git labels ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_label_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_label_border: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_label_font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_label_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_label_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_label_font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_line_color: Option<String>,

    // --- Wardley evolution ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wardley_evolution_color: Option<String>,

    // --- Pie ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_title_text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_title_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_section_text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_section_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_legend_text_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_legend_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_stroke_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_stroke_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_outer_stroke_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_outer_stroke_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_opacity: Option<String>,

    // --- Venn ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn_title_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn_set_text_color: Option<String>,

    // --- Quadrant ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant1_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant2_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant3_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant4_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant1_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant2_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant3_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant4_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_point_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_point_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_x_axis_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_y_axis_text_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_internal_border_stroke_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_external_border_stroke_fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quadrant_title_fill: Option<String>,

    // --- Sub-objects ---
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xy_chart: Option<XyChartThemeVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar: Option<RadarThemeVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wardley: Option<WardleyThemeVars>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet: Option<PacketThemeVars>,

    // === Indexed color scales (0–11) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale10: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale11: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv10: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_inv11: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer10: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_peer11: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label10: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_scale_label11: Option<String>,

    // === surface (0–4) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_peer0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_peer1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_peer2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_peer3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_peer4: Option<String>,

    // === fillType (0–7) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_type7: Option<String>,

    // === pie (1–12) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie9: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie10: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie11: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie12: Option<String>,

    // === venn (1–8) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn7: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venn8: Option<String>,

    // === git (0–7) ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git7: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_inv7: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_branch_label7: Option<String>,

    // === Boolean / Numeric ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dark_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_gradient: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_shadow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,

    // === Arrays ===
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color_array: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bkg_color_array: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Helper methods
// ---------------------------------------------------------------------------

/// True if the Option<String> is Some and contains non-whitespace content.
pub(crate) fn is_truthy(opt: &Option<String>) -> bool {
    opt.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false)
}

#[allow(dead_code)]
impl ThemeVariables {
    /// Deserialize from a JSON Value.
    pub fn from_value(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }

    /// Serialize to a JSON Value (Object).
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Object(serde_json::Map::new()))
    }

    pub(crate) fn set_background_if_none(&mut self, value: impl Into<String>) {
        if self.background.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.background = Some(value.into());
        }
    }
    pub(crate) fn set_primary_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .primary_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.primary_color = Some(value.into());
        }
    }
    pub(crate) fn set_secondary_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .secondary_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.secondary_color = Some(value.into());
        }
    }
    pub(crate) fn set_tertiary_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .tertiary_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tertiary_color = Some(value.into());
        }
    }
    pub(crate) fn set_primary_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .primary_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.primary_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_secondary_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .secondary_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.secondary_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_tertiary_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .tertiary_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tertiary_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_primary_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .primary_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.primary_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_secondary_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .secondary_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.secondary_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_tertiary_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .tertiary_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tertiary_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_line_color_if_none(&mut self, value: impl Into<String>) {
        if self.line_color.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.line_color = Some(value.into());
        }
    }
    pub(crate) fn set_arrowhead_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .arrowhead_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arrowhead_color = Some(value.into());
        }
    }
    pub(crate) fn set_text_color_if_none(&mut self, value: impl Into<String>) {
        if self.text_color.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.text_color = Some(value.into());
        }
    }
    pub(crate) fn set_font_family_if_none(&mut self, value: impl Into<String>) {
        if self
            .font_family
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.font_family = Some(value.into());
        }
    }
    pub(crate) fn set_font_size_if_none(&mut self, value: impl Into<String>) {
        if self.font_size.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.font_size = Some(value.into());
        }
    }
    pub(crate) fn set_font_weight_if_none(&mut self, value: impl Into<String>) {
        if self
            .font_weight
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.font_weight = Some(value.into());
        }
    }
    pub(crate) fn set_note_font_weight_if_none(&mut self, value: impl Into<String>) {
        if self
            .note_font_weight
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.note_font_weight = Some(value.into());
        }
    }
    pub(crate) fn set_border1_if_none(&mut self, value: impl Into<String>) {
        if self.border1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.border1 = Some(value.into());
        }
    }
    pub(crate) fn set_border2_if_none(&mut self, value: impl Into<String>) {
        if self.border2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.border2 = Some(value.into());
        }
    }
    pub(crate) fn set_main_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.main_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.main_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_second_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.second_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.second_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_node_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.node_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.node_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_node_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .node_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.node_border = Some(value.into());
        }
    }
    pub(crate) fn set_cluster_bkg_if_none(&mut self, value: impl Into<String>) {
        if self
            .cluster_bkg
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.cluster_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_cluster_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .cluster_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.cluster_border = Some(value.into());
        }
    }
    pub(crate) fn set_default_link_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .default_link_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.default_link_color = Some(value.into());
        }
    }
    pub(crate) fn set_title_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .title_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.title_color = Some(value.into());
        }
    }
    pub(crate) fn set_edge_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .edge_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.edge_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_node_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .node_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.node_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_background = Some(value.into());
        }
    }
    pub(crate) fn set_label_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_color = Some(value.into());
        }
    }
    pub(crate) fn set_note_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .note_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.note_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_note_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .note_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.note_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_note_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .note_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.note_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_actor_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .actor_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.actor_border = Some(value.into());
        }
    }
    pub(crate) fn set_actor_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.actor_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.actor_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_actor_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .actor_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.actor_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_actor_line_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .actor_line_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.actor_line_color = Some(value.into());
        }
    }
    pub(crate) fn set_signal_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .signal_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.signal_color = Some(value.into());
        }
    }
    pub(crate) fn set_signal_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .signal_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.signal_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_label_box_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_box_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_box_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_label_box_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_box_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_box_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_loop_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .loop_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.loop_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_activation_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .activation_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.activation_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_activation_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .activation_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.activation_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_sequence_number_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .sequence_number_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.sequence_number_color = Some(value.into());
        }
    }
    pub(crate) fn set_section_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .section_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.section_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_alt_section_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .alt_section_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.alt_section_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_section_bkg_color2_if_none(&mut self, value: impl Into<String>) {
        if self
            .section_bkg_color2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.section_bkg_color2 = Some(value.into());
        }
    }
    pub(crate) fn set_exclude_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .exclude_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.exclude_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_text_light_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_text_light_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_text_light_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_text_dark_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_text_dark_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_text_dark_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_text_outside_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_text_outside_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_text_outside_color = Some(value.into());
        }
    }
    pub(crate) fn set_task_text_clickable_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .task_text_clickable_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.task_text_clickable_color = Some(value.into());
        }
    }
    pub(crate) fn set_active_task_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .active_task_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.active_task_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_active_task_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .active_task_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.active_task_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_grid_color_if_none(&mut self, value: impl Into<String>) {
        if self.grid_color.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.grid_color = Some(value.into());
        }
    }
    pub(crate) fn set_done_task_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .done_task_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.done_task_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_done_task_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .done_task_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.done_task_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_crit_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .crit_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.crit_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_crit_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .crit_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.crit_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_today_line_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .today_line_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.today_line_color = Some(value.into());
        }
    }
    pub(crate) fn set_vert_line_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .vert_line_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.vert_line_color = Some(value.into());
        }
    }
    pub(crate) fn set_person_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .person_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.person_border = Some(value.into());
        }
    }
    pub(crate) fn set_person_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.person_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.person_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_row_odd_if_none(&mut self, value: impl Into<String>) {
        if self.row_odd.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.row_odd = Some(value.into());
        }
    }
    pub(crate) fn set_row_even_if_none(&mut self, value: impl Into<String>) {
        if self.row_even.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.row_even = Some(value.into());
        }
    }
    pub(crate) fn set_transition_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .transition_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.transition_color = Some(value.into());
        }
    }
    pub(crate) fn set_transition_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .transition_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.transition_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_state_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .state_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.state_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_state_bkg_if_none(&mut self, value: impl Into<String>) {
        if self.state_bkg.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.state_bkg = Some(value.into());
        }
    }
    pub(crate) fn set_state_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .state_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.state_border = Some(value.into());
        }
    }
    pub(crate) fn set_label_background_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .label_background_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.label_background_color = Some(value.into());
        }
    }
    pub(crate) fn set_composite_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .composite_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.composite_background = Some(value.into());
        }
    }
    pub(crate) fn set_alt_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .alt_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.alt_background = Some(value.into());
        }
    }
    pub(crate) fn set_composite_title_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .composite_title_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.composite_title_background = Some(value.into());
        }
    }
    pub(crate) fn set_composite_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .composite_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.composite_border = Some(value.into());
        }
    }
    pub(crate) fn set_inner_end_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .inner_end_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.inner_end_background = Some(value.into());
        }
    }
    pub(crate) fn set_error_bkg_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .error_bkg_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.error_bkg_color = Some(value.into());
        }
    }
    pub(crate) fn set_error_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .error_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.error_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_special_state_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .special_state_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.special_state_color = Some(value.into());
        }
    }
    pub(crate) fn set_state_edge_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .state_edge_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.state_edge_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_class_text_if_none(&mut self, value: impl Into<String>) {
        if self.class_text.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.class_text = Some(value.into());
        }
    }
    pub(crate) fn set_arch_edge_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .arch_edge_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arch_edge_color = Some(value.into());
        }
    }
    pub(crate) fn set_arch_edge_arrow_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .arch_edge_arrow_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arch_edge_arrow_color = Some(value.into());
        }
    }
    pub(crate) fn set_arch_edge_width_if_none(&mut self, value: impl Into<String>) {
        if self
            .arch_edge_width
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arch_edge_width = Some(value.into());
        }
    }
    pub(crate) fn set_arch_group_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .arch_group_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arch_group_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_arch_group_border_width_if_none(&mut self, value: impl Into<String>) {
        if self
            .arch_group_border_width
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.arch_group_border_width = Some(value.into());
        }
    }
    pub(crate) fn set_requirement_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .requirement_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.requirement_background = Some(value.into());
        }
    }
    pub(crate) fn set_requirement_border_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .requirement_border_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.requirement_border_color = Some(value.into());
        }
    }
    pub(crate) fn set_requirement_border_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .requirement_border_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.requirement_border_size = Some(value.into());
        }
    }
    pub(crate) fn set_requirement_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .requirement_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.requirement_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_requirement_edge_label_background_if_none(
        &mut self,
        value: impl Into<String>,
    ) {
        if self
            .requirement_edge_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.requirement_edge_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_relation_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .relation_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.relation_color = Some(value.into());
        }
    }
    pub(crate) fn set_relation_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .relation_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.relation_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_relation_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .relation_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.relation_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_er_edge_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .er_edge_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.er_edge_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_em_ui_fill_if_none(&mut self, value: impl Into<String>) {
        if self.em_ui_fill.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.em_ui_fill = Some(value.into());
        }
    }
    pub(crate) fn set_em_ui_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_ui_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_ui_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_processor_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_processor_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_processor_fill = Some(value.into());
        }
    }
    pub(crate) fn set_em_processor_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_processor_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_processor_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_read_model_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_read_model_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_read_model_fill = Some(value.into());
        }
    }
    pub(crate) fn set_em_read_model_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_read_model_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_read_model_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_command_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_command_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_command_fill = Some(value.into());
        }
    }
    pub(crate) fn set_em_command_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_command_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_command_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_event_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_event_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_event_fill = Some(value.into());
        }
    }
    pub(crate) fn set_em_event_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_event_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_event_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_swimlane_background_odd_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_swimlane_background_odd
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_swimlane_background_odd = Some(value.into());
        }
    }
    pub(crate) fn set_em_swimlane_background_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_swimlane_background_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_swimlane_background_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_em_arrowhead_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_arrowhead
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_arrowhead = Some(value.into());
        }
    }
    pub(crate) fn set_em_relation_stroke_if_none(&mut self, value: impl Into<String>) {
        if self
            .em_relation_stroke
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.em_relation_stroke = Some(value.into());
        }
    }
    pub(crate) fn set_attribute_background_color_odd_if_none(&mut self, value: impl Into<String>) {
        if self
            .attribute_background_color_odd
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.attribute_background_color_odd = Some(value.into());
        }
    }
    pub(crate) fn set_attribute_background_color_even_if_none(&mut self, value: impl Into<String>) {
        if self
            .attribute_background_color_even
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.attribute_background_color_even = Some(value.into());
        }
    }
    pub(crate) fn set_gradient_start_if_none(&mut self, value: impl Into<String>) {
        if self
            .gradient_start
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.gradient_start = Some(value.into());
        }
    }
    pub(crate) fn set_gradient_stop_if_none(&mut self, value: impl Into<String>) {
        if self
            .gradient_stop
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.gradient_stop = Some(value.into());
        }
    }
    pub(crate) fn set_drop_shadow_if_none(&mut self, value: impl Into<String>) {
        if self
            .drop_shadow
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.drop_shadow = Some(value.into());
        }
    }
    pub(crate) fn set_main_contrast_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .main_contrast_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.main_contrast_color = Some(value.into());
        }
    }
    pub(crate) fn set_dark_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .dark_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.dark_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_contrast_if_none(&mut self, value: impl Into<String>) {
        if self.contrast.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.contrast = Some(value.into());
        }
    }
    pub(crate) fn set_note_if_none(&mut self, value: impl Into<String>) {
        if self.note.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.note = Some(value.into());
        }
    }
    pub(crate) fn set_text_if_none(&mut self, value: impl Into<String>) {
        if self.text.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.text = Some(value.into());
        }
    }
    pub(crate) fn set_critical_if_none(&mut self, value: impl Into<String>) {
        if self.critical.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.critical = Some(value.into());
        }
    }
    pub(crate) fn set_done_if_none(&mut self, value: impl Into<String>) {
        if self.done.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.done = Some(value.into());
        }
    }
    pub(crate) fn set_root_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .root_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.root_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_filter_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .filter_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.filter_color = Some(value.into());
        }
    }
    pub(crate) fn set_scale_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .scale_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.scale_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_branch_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .branch_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.branch_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_tag_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .tag_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tag_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_tag_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .tag_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tag_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_tag_label_border_if_none(&mut self, value: impl Into<String>) {
        if self
            .tag_label_border
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tag_label_border = Some(value.into());
        }
    }
    pub(crate) fn set_tag_label_font_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .tag_label_font_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.tag_label_font_size = Some(value.into());
        }
    }
    pub(crate) fn set_commit_label_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .commit_label_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.commit_label_color = Some(value.into());
        }
    }
    pub(crate) fn set_commit_label_background_if_none(&mut self, value: impl Into<String>) {
        if self
            .commit_label_background
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.commit_label_background = Some(value.into());
        }
    }
    pub(crate) fn set_commit_label_font_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .commit_label_font_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.commit_label_font_size = Some(value.into());
        }
    }
    pub(crate) fn set_commit_line_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .commit_line_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.commit_line_color = Some(value.into());
        }
    }
    pub(crate) fn set_wardley_evolution_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .wardley_evolution_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.wardley_evolution_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_title_text_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_title_text_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_title_text_size = Some(value.into());
        }
    }
    pub(crate) fn set_pie_title_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_title_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_title_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_section_text_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_section_text_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_section_text_size = Some(value.into());
        }
    }
    pub(crate) fn set_pie_section_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_section_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_section_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_legend_text_size_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_legend_text_size
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_legend_text_size = Some(value.into());
        }
    }
    pub(crate) fn set_pie_legend_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_legend_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_legend_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_stroke_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_stroke_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_stroke_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_stroke_width_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_stroke_width
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_stroke_width = Some(value.into());
        }
    }
    pub(crate) fn set_pie_outer_stroke_width_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_outer_stroke_width
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_outer_stroke_width = Some(value.into());
        }
    }
    pub(crate) fn set_pie_outer_stroke_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_outer_stroke_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_outer_stroke_color = Some(value.into());
        }
    }
    pub(crate) fn set_pie_opacity_if_none(&mut self, value: impl Into<String>) {
        if self
            .pie_opacity
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.pie_opacity = Some(value.into());
        }
    }
    pub(crate) fn set_venn_title_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .venn_title_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.venn_title_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_venn_set_text_color_if_none(&mut self, value: impl Into<String>) {
        if self
            .venn_set_text_color
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.venn_set_text_color = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant1_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant1_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant1_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant2_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant2_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant2_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant3_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant3_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant3_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant4_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant4_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant4_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant1_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant1_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant1_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant2_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant2_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant2_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant3_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant3_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant3_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant4_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant4_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant4_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_point_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant_point_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_point_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_point_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant_point_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_point_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_x_axis_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant_x_axis_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_x_axis_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_y_axis_text_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant_y_axis_text_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_y_axis_text_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_internal_border_stroke_fill_if_none(
        &mut self,
        value: impl Into<String>,
    ) {
        if self
            .quadrant_internal_border_stroke_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_internal_border_stroke_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_external_border_stroke_fill_if_none(
        &mut self,
        value: impl Into<String>,
    ) {
        if self
            .quadrant_external_border_stroke_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_external_border_stroke_fill = Some(value.into());
        }
    }
    pub(crate) fn set_quadrant_title_fill_if_none(&mut self, value: impl Into<String>) {
        if self
            .quadrant_title_fill
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.quadrant_title_fill = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale0_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale0.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale0 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale1_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale1 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale2_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale2 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale3_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale3 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale4_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale4 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale5_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale5 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale6_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale6 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale7_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale7 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale8_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale8.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale8 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale9_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale9.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale9 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale10_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale10.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale10 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale11_if_none(&mut self, value: impl Into<String>) {
        if self.c_scale11.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.c_scale11 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv0_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv0
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv0 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv1_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv1
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv1 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv2_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv2 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv3_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv3
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv3 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv4_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv4
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv4 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv5_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv5
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv5 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv6_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv6
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv6 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv7_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv7
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv7 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv8_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv8
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv8 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv9_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv9
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv9 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv10_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv10
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv10 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_inv11_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_inv11
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_inv11 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer0_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer0
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer0 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer1_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer1
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer1 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer2_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer2 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer3_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer3
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer3 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer4_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer4
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer4 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer5_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer5
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer5 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer6_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer6
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer6 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer7_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer7
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer7 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer8_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer8
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer8 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer9_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer9
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer9 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer10_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer10
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer10 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_peer11_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_peer11
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_peer11 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label0_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label0
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label0 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label1_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label1
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label1 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label2_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label2 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label3_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label3
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label3 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label4_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label4
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label4 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label5_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label5
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label5 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label6_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label6
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label6 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label7_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label7
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label7 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label8_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label8
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label8 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label9_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label9
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label9 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label10_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label10
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label10 = Some(value.into());
        }
    }
    pub(crate) fn set_c_scale_label11_if_none(&mut self, value: impl Into<String>) {
        if self
            .c_scale_label11
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.c_scale_label11 = Some(value.into());
        }
    }
    pub(crate) fn set_surface0_if_none(&mut self, value: impl Into<String>) {
        if self.surface0.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.surface0 = Some(value.into());
        }
    }
    pub(crate) fn set_surface1_if_none(&mut self, value: impl Into<String>) {
        if self.surface1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.surface1 = Some(value.into());
        }
    }
    pub(crate) fn set_surface2_if_none(&mut self, value: impl Into<String>) {
        if self.surface2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.surface2 = Some(value.into());
        }
    }
    pub(crate) fn set_surface3_if_none(&mut self, value: impl Into<String>) {
        if self.surface3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.surface3 = Some(value.into());
        }
    }
    pub(crate) fn set_surface4_if_none(&mut self, value: impl Into<String>) {
        if self.surface4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.surface4 = Some(value.into());
        }
    }
    pub(crate) fn set_surface_peer0_if_none(&mut self, value: impl Into<String>) {
        if self
            .surface_peer0
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.surface_peer0 = Some(value.into());
        }
    }
    pub(crate) fn set_surface_peer1_if_none(&mut self, value: impl Into<String>) {
        if self
            .surface_peer1
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.surface_peer1 = Some(value.into());
        }
    }
    pub(crate) fn set_surface_peer2_if_none(&mut self, value: impl Into<String>) {
        if self
            .surface_peer2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.surface_peer2 = Some(value.into());
        }
    }
    pub(crate) fn set_surface_peer3_if_none(&mut self, value: impl Into<String>) {
        if self
            .surface_peer3
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.surface_peer3 = Some(value.into());
        }
    }
    pub(crate) fn set_surface_peer4_if_none(&mut self, value: impl Into<String>) {
        if self
            .surface_peer4
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.surface_peer4 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type0_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type0.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type0 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type1_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type1 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type2_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type2 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type3_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type3 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type4_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type4 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type5_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type5 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type6_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type6 = Some(value.into());
        }
    }
    pub(crate) fn set_fill_type7_if_none(&mut self, value: impl Into<String>) {
        if self.fill_type7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.fill_type7 = Some(value.into());
        }
    }
    pub(crate) fn set_pie1_if_none(&mut self, value: impl Into<String>) {
        if self.pie1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie1 = Some(value.into());
        }
    }
    pub(crate) fn set_pie2_if_none(&mut self, value: impl Into<String>) {
        if self.pie2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie2 = Some(value.into());
        }
    }
    pub(crate) fn set_pie3_if_none(&mut self, value: impl Into<String>) {
        if self.pie3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie3 = Some(value.into());
        }
    }
    pub(crate) fn set_pie4_if_none(&mut self, value: impl Into<String>) {
        if self.pie4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie4 = Some(value.into());
        }
    }
    pub(crate) fn set_pie5_if_none(&mut self, value: impl Into<String>) {
        if self.pie5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie5 = Some(value.into());
        }
    }
    pub(crate) fn set_pie6_if_none(&mut self, value: impl Into<String>) {
        if self.pie6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie6 = Some(value.into());
        }
    }
    pub(crate) fn set_pie7_if_none(&mut self, value: impl Into<String>) {
        if self.pie7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie7 = Some(value.into());
        }
    }
    pub(crate) fn set_pie8_if_none(&mut self, value: impl Into<String>) {
        if self.pie8.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie8 = Some(value.into());
        }
    }
    pub(crate) fn set_pie9_if_none(&mut self, value: impl Into<String>) {
        if self.pie9.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie9 = Some(value.into());
        }
    }
    pub(crate) fn set_pie10_if_none(&mut self, value: impl Into<String>) {
        if self.pie10.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie10 = Some(value.into());
        }
    }
    pub(crate) fn set_pie11_if_none(&mut self, value: impl Into<String>) {
        if self.pie11.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie11 = Some(value.into());
        }
    }
    pub(crate) fn set_pie12_if_none(&mut self, value: impl Into<String>) {
        if self.pie12.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.pie12 = Some(value.into());
        }
    }
    pub(crate) fn set_venn1_if_none(&mut self, value: impl Into<String>) {
        if self.venn1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn1 = Some(value.into());
        }
    }
    pub(crate) fn set_venn2_if_none(&mut self, value: impl Into<String>) {
        if self.venn2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn2 = Some(value.into());
        }
    }
    pub(crate) fn set_venn3_if_none(&mut self, value: impl Into<String>) {
        if self.venn3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn3 = Some(value.into());
        }
    }
    pub(crate) fn set_venn4_if_none(&mut self, value: impl Into<String>) {
        if self.venn4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn4 = Some(value.into());
        }
    }
    pub(crate) fn set_venn5_if_none(&mut self, value: impl Into<String>) {
        if self.venn5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn5 = Some(value.into());
        }
    }
    pub(crate) fn set_venn6_if_none(&mut self, value: impl Into<String>) {
        if self.venn6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn6 = Some(value.into());
        }
    }
    pub(crate) fn set_venn7_if_none(&mut self, value: impl Into<String>) {
        if self.venn7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn7 = Some(value.into());
        }
    }
    pub(crate) fn set_venn8_if_none(&mut self, value: impl Into<String>) {
        if self.venn8.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.venn8 = Some(value.into());
        }
    }
    pub(crate) fn set_git0_if_none(&mut self, value: impl Into<String>) {
        if self.git0.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git0 = Some(value.into());
        }
    }
    pub(crate) fn set_git1_if_none(&mut self, value: impl Into<String>) {
        if self.git1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git1 = Some(value.into());
        }
    }
    pub(crate) fn set_git2_if_none(&mut self, value: impl Into<String>) {
        if self.git2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git2 = Some(value.into());
        }
    }
    pub(crate) fn set_git3_if_none(&mut self, value: impl Into<String>) {
        if self.git3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git3 = Some(value.into());
        }
    }
    pub(crate) fn set_git4_if_none(&mut self, value: impl Into<String>) {
        if self.git4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git4 = Some(value.into());
        }
    }
    pub(crate) fn set_git5_if_none(&mut self, value: impl Into<String>) {
        if self.git5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git5 = Some(value.into());
        }
    }
    pub(crate) fn set_git6_if_none(&mut self, value: impl Into<String>) {
        if self.git6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git6 = Some(value.into());
        }
    }
    pub(crate) fn set_git7_if_none(&mut self, value: impl Into<String>) {
        if self.git7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git7 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv0_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv0.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv0 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv1_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv1.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv1 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv2_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv2.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv2 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv3_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv3.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv3 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv4_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv4.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv4 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv5_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv5.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv5 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv6_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv6.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv6 = Some(value.into());
        }
    }
    pub(crate) fn set_git_inv7_if_none(&mut self, value: impl Into<String>) {
        if self.git_inv7.as_ref().is_none_or(|s| s.trim().is_empty()) {
            self.git_inv7 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label0_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label0
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label0 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label1_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label1
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label1 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label2_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label2
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label2 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label3_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label3
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label3 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label4_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label4
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label4 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label5_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label5
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label5 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label6_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label6
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label6 = Some(value.into());
        }
    }
    pub(crate) fn set_git_branch_label7_if_none(&mut self, value: impl Into<String>) {
        if self
            .git_branch_label7
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
        {
            self.git_branch_label7 = Some(value.into());
        }
    }

    pub(crate) fn set_border_color_array_if_none(&mut self, value: Vec<String>) {
        if self.border_color_array.as_ref().is_none() {
            self.border_color_array = Some(value);
        }
    }
}
