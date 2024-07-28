use plotters::style::{full_palette::{BLACK, BROWN}, Color, RGBAColor, RGBColor};

use super::io::read_parameters::{read_colors_csv, read_csv_to_vector};



#[derive(Debug, Clone)]
pub struct UIBoardParameters {
    pub size: f64,
    pub x_dim: u32,
    pub y_dim: u32,

    pub bg_color: RGBColor,

    pub caption: &'static str,
    pub caption_fontsize: u32,
    pub  caption_font: &'static str,
    pub caption_color: RGBAColor,

    pub margin: u32,
    pub x_label_size: u32,
    pub y_label_size: u32,
    pub x_lims: (f64, f64),
    pub y_lims: (f64, f64),

    pub n_circle_steps: u32,
    pub circle_color: RGBAColor,
    pub circle_width: u32,

    pub node_scale: u32,
    pub node_color: RGBColor,

    pub tile_node_scale: u32,
    pub tile_node_color: RGBColor,

    pub harbor_scale: u32,
    pub harbor_color: RGBColor,

    pub edge_color: RGBColor,

    pub node_label_font: &'static str,
    pub node_label_color: RGBColor,
    pub node_label_fontsize: u32,

    pub tile_label_font: &'static str,
    pub tile_label_color: RGBColor,
    pub tile_label_fontsize: u32,

    pub harbor_label: &'static str, 
    pub harbor_label_font: &'static str,
    pub harbor_label_color: RGBColor,
    pub harbor_label_fontsize: u32,

    pub road_width: u32,

    pub settlement_scale: u32,
    pub settlement_width: u32,

    pub city_scale: u32,

    pub v_resource_colors: Vec<RGBColor>,
    pub v_player_colors: Vec<RGBColor>,

    pub v_resource_names: Vec<String>,

    pub v_harbor_symbols: Vec<String>,

    pub svg_nodes: Option<String>,
    pub svg_tiles: Option<String>
}

impl Default for UIBoardParameters {
    fn default() -> Self {

        let color_path = "data/parameters/tile_colors.csv";
        let v_resource_colors = read_colors_csv(color_path).expect("Error while reading colours.");

        let player_color_path = "data/parameters/player_colors.csv";
        let v_player_colors = read_colors_csv(&player_color_path).expect("Error while reading player colors.");

        let resource_path = "data/parameters/resources.csv";
        let v_resource_names: Vec<String> = read_csv_to_vector(resource_path).expect("An error occurred while reading resource file.");

        let harbor_sym_path = "data/parameters/harbor_symbols.csv";
        let v_harbor_symbols: Vec<String> = read_csv_to_vector(harbor_sym_path).expect("Error while reading harbor symbols.");

        Self {
            size: 1.0,
            x_dim: 1500,
            y_dim: 1400,

            bg_color: RGBColor(230, 226, 213),

            caption: "カタンアイランド",
            caption_fontsize: 100,
            caption_font: "Dela Gothic One",
            caption_color: BROWN.mix(0.2),

            margin: 100,
            x_label_size: 30,
            y_label_size: 30,
            x_lims: (-1.2, 1.2),
            y_lims: (-1.1, 1.1),

            n_circle_steps: 200,
            circle_color: BROWN.mix(0.1),
            circle_width: 5,

            node_scale: 5,
            node_color: BLACK,
    
            tile_node_scale: 5,
            tile_node_color: BLACK,

            harbor_scale: 5,
            harbor_color: BLACK,

            edge_color: BLACK,

            node_label_font: "sans-serif",
            node_label_color: BLACK,
            node_label_fontsize: 18,

            tile_label_font: "monospace",
            tile_label_color: BLACK,
            tile_label_fontsize: 20,

            harbor_label: "港",
            harbor_label_font: "Noto Serif JP",
            harbor_label_color:BLACK,
            harbor_label_fontsize: 20,

            road_width: 15,

            settlement_scale: 20,
            settlement_width: 5,

            city_scale: 20,

            v_resource_colors,
            v_player_colors,

            v_resource_names,

            v_harbor_symbols,

            svg_nodes: None,
            svg_tiles: None
        }

    }
}
