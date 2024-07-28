use std::{fs::{self, File}, io::Write};

use plotters::{chart::{ChartBuilder, ChartContext}, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, drawing::IntoDrawingArea, element::{Cross, IntoDynElement, Polygon, Text}, style::{IntoFont, ShapeStyle}};
use plotters_svg::SVGBackend;
use regex::Regex;

use crate::backend::setup::{game::Game, node::Node, shape::get_n_node_rings, tile::Tile};

use super::{board_parameters::UIBoardParameters, coords::{add_cart_coords_nodes, add_cart_coords_tiles, add_conc_coords_nodes}};

pub fn create_svg(
    game: &Game, 
    ui_parameters: &UIBoardParameters
) {
    let fig_scale = ui_parameters.size;
    let x_dim = ui_parameters.x_dim;
    let y_dim = ui_parameters.y_dim;

    let x_lims = ui_parameters.x_lims;
    let y_lims = ui_parameters.y_lims;

    let n_tile_rings = game.parameters.n_tile_rings.clone();

    let board = &game.round.board;

    let tile_shape = &game.parameters.tile_shape;
    let n_node_rings = get_n_node_rings(n_tile_rings, tile_shape);

    let mut nodes = board.nodes.clone();
    // println!("{:#?}", nodes);
    nodes = add_conc_coords_nodes(nodes, n_node_rings, &tile_shape);
    nodes = add_cart_coords_nodes(nodes, n_node_rings, &tile_shape);
    // println!("{:#?}", nodes);

    let mut tiles = board.tiles.clone();
    tiles = add_cart_coords_tiles(tiles, n_tile_rings, &tile_shape);

    let root = SVGBackend::new("data/assets/tiles.svg", ((fig_scale * x_dim as f64) as u32, (fig_scale * y_dim as f64) as u32)).into_drawing_area();
        root.fill(&ui_parameters.bg_color).expect("Error filling the drawing area");

    // Create a chart
    let mut chart = ChartBuilder::on(&root)
        .caption(ui_parameters.caption, (ui_parameters.caption_font, (fig_scale * ui_parameters.caption_fontsize as f64 * (n_tile_rings as f64 / 3.0)) as u32).into_font().color(&ui_parameters.caption_color))
        .margin(ui_parameters.margin)
        .x_label_area_size(ui_parameters.x_label_size)
        .y_label_area_size(ui_parameters.y_label_size)
        .build_cartesian_2d(x_lims.0..x_lims.1, y_lims.0..y_lims.1)
        .expect("Error building the chart");


    draw(&game, &mut chart, &ui_parameters, &tiles, &nodes);

    root.present().expect("Error presenting the drawing area");
}


pub fn modify_svg() -> Result<(), Box<dyn std::error::Error>> {
    let svg_data = fs::read_to_string("data/assets/tiles.svg")?;

    // Define the regex pattern to match polygon elements
    let re_polygon = Regex::new(r"(<polygon[^>]*>)").unwrap();
    // Define the regex pattern to match id attribute
    let re_id = Regex::new(r#"\bid\s*=\s*"[^"]*""#).unwrap();

    // Counter for unique IDs
    let mut counter = 0;

    // Replace each polygon element with a modified version including a unique ID
    let modified_svg_data = re_polygon.replace_all(svg_data.as_str(), |caps: &regex::Captures| {
        let element = &caps[0];
        if re_id.is_match(element) {
            // If the element already has an id attribute, return it unchanged
            element.to_string()
        } else {
            
            let id_attr = format!(r#" id="tile-{}""#, counter);
            // Otherwise, add a unique id attribute
            counter += 1;
            // Find position before the closing '>'
            if let Some(pos) = element.find(format!(r#"/>"#).as_str()) {
                let mut new_element = String::with_capacity(element.len() + id_attr.len());
                new_element.push_str(&element[..pos]);
                new_element.push_str(&id_attr);
                new_element.push_str(&element[pos..]);
                new_element
            } else {
                element.to_string()
            }
        }
    });

    // Save the modified SVG data to a file
    let mut file = File::create("data/assets/tiles.svg").expect("Unable to create file");
    file.write_all(modified_svg_data.as_bytes()).expect("Unable to write data");

    Ok(())
}


pub fn draw(
    game: &Game,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    tiles: &Vec<Tile>, 
    nodes: &Vec<Node>
) {

    for tile in tiles.iter() {

        let mut v_coords: Vec<(f64, f64)> = vec![];
        for i_node in &tile.nodes {
            match &nodes[*i_node as usize].coords_cart {
                Some(coords) => v_coords.push(*coords),
                None => continue,
            }
        }

        let resource_id = match tile.resource {
            Some(r) => r,
            None => game.parameters.n_resources,
        };

        let tile_color = ui_parameters.v_resource_colors[resource_id as usize];
        // println!("{:#?}", tile_color);

        chart.draw_series(std::iter::once(Polygon::new(v_coords, ShapeStyle::from(tile_color).filled()))).expect("Error drawing polygons.");

    }

}


pub fn draw_labels(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points_tiles: &Vec<(f64, f64)>,
    tiles: &Vec<Tile>,
    n_tiles: usize,
    n_tile_rings: u32,
) {
    let fig_scale = ui_parameters.size;
    let tile_label_font = ui_parameters.tile_label_font;
    let tile_label_fontsize = ui_parameters.tile_label_fontsize;

    for i_tile in 0..n_tiles {
        let x = &points_tiles[i_tile as usize].0;
        let y = &points_tiles[i_tile as usize].1;

        let tile = &tiles[i_tile as usize];

        let resource: String;
        if let Some(r) = &tile.resource {
            resource = ui_parameters.v_resource_names[*r as usize].clone()
        } else {
            continue
        }

        let rng: u32;
        if let Some(r) = &tile.rng {
            rng = *r
        } else {
            continue
        }
        
        // Create the label text
        let label = format!("{:.2}:({:.2}, {:.2})", i_tile, resource, rng);

        // Draw the label
        if x > &0.0 {
            if y > &0.0 {
                chart.draw_series(std::iter::once(
                    Text::new(label, (*x - 0.05, y+0.035), (tile_label_font, (fig_scale * tile_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            } else {
                chart.draw_series(std::iter::once(
                    Text::new(label, (*x - 0.05, y-0.05), (tile_label_font, (fig_scale * tile_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            }
            
        } else {
            if y > &0.0 {
                chart.draw_series(std::iter::once(
                    Text::new(label, (*x - 0.05, y+0.035), (tile_label_font, (fig_scale * tile_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            } else {
                chart.draw_series(std::iter::once(
                    Text::new(label, (*x - 0.05, y-0.05), (tile_label_font, (fig_scale * tile_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            }
        }

    }
}


pub fn draw_nodes(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points_tiles: &Vec<(f64, f64)>,
) {
    let fig_scale = ui_parameters.size;
    let tile_node_scale = ui_parameters.tile_node_scale;
    let tile_node_color = ui_parameters.tile_node_color;

    for (i, point) in points_tiles.iter().enumerate() {
        // Draw the point
        chart.draw_series(std::iter::once(
            Cross::new(*point, (fig_scale * tile_node_scale as f64) as u32, ShapeStyle::from(&tile_node_color).filled())
        )).expect("Error drawing the point");
    }
}