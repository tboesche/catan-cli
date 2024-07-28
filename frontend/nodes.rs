use std::{fs::{self, File}, io::Write};

use plotters::{chart::{ChartBuilder, ChartContext}, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, drawing::IntoDrawingArea, element::{Circle, IntoDynElement, Text}, style::{IntoFont, ShapeStyle}};
use plotters_svg::SVGBackend;
use regex::Regex;

use crate::backend::setup::{game::Game, node::Node, shape::get_n_node_rings};

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

    let n_tiles = board.tiles.len();
    let n_nodes = board.nodes.len();

    let tile_shape = &game.parameters.tile_shape;
    let n_node_rings = get_n_node_rings(n_tile_rings, tile_shape);

    let v_tile_resource = &game.parameters.v_tile_resources;

    let mut nodes = board.nodes.clone();
    // println!("{:#?}", nodes);
    nodes = add_conc_coords_nodes(nodes, n_node_rings, &tile_shape);
    nodes = add_cart_coords_nodes(nodes, n_node_rings, &tile_shape);
    // println!("{:#?}", nodes);

    // let mut tiles = board.tiles.clone();
    // tiles = add_cart_coords_tiles(tiles, n_tile_rings, &tile_shape);

    let mut points: Vec<(f64, f64)> = vec![];
        // println!("{:#?}", nodes);
        for node in &nodes {
            match node.coords_cart {
                Some(point) => points.push(point),
                None => (),
            }
        }

    let root = SVGBackend::new("data/assets/nodes.svg", ((fig_scale * x_dim as f64) as u32, (fig_scale * y_dim as f64) as u32)).into_drawing_area();
        root.fill(&ui_parameters.bg_color).expect("Error filling the drawing area");

    // Create a chart
    let mut chart = ChartBuilder::on(&root)
        .caption(ui_parameters.caption, (ui_parameters.caption_font, (fig_scale * ui_parameters.caption_fontsize as f64 * (n_tile_rings as f64 / 3.0)) as u32).into_font().color(&ui_parameters.caption_color))
        .margin(ui_parameters.margin)
        .x_label_area_size(ui_parameters.x_label_size)
        .y_label_area_size(ui_parameters.y_label_size)
        .build_cartesian_2d(x_lims.0..x_lims.1, y_lims.0..y_lims.1)
        .expect("Error building the chart");


    draw(&mut chart, &ui_parameters, &points);

    root.present().expect("Error presenting the drawing area");
}


pub fn modify_svg() -> Result<(), Box<dyn std::error::Error>> {
    let svg_data = fs::read_to_string("data/assets/nodes.svg")?;

    // Define the regex pattern to match polygon elements
    let re_polygon = Regex::new(r"(<circle[^>]*>)").unwrap();
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
            
            let id_attr = format!(r#" id="node-{}""#, counter);
             // Otherwise, add a unique id attribute
             counter += 1;
            // Find position before the closing '>'
            if let Some(pos) = element.find(r#"/>"#.to_string().as_str()) {
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
    let mut file = File::create("data/assets/nodes.svg").expect("Unable to create file");
    file.write_all(modified_svg_data.as_bytes()).expect("Unable to write data");

    Ok(())
}



pub fn draw(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points: &Vec<(f64, f64)>,
) {
    let fig_scale = ui_parameters.size;
    let node_scale = ui_parameters.node_scale;
    let node_color = ui_parameters.node_color;

    for (i, point) in points.iter().enumerate() {
        // Draw the point
        chart.draw_series(std::iter::once(
            Circle::new(*point, (fig_scale * node_scale as f64) as u32, ShapeStyle::from(&node_color).filled())
        )).expect("Error drawing the point");
    }
}


pub fn draw_labels(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    nodes: &Vec<Node>,
    points: &Vec<(f64, f64)>,
    points_conc: &Vec<(u32, u32)>,
    n_tile_rings: u32) 
    {
    let fig_scale = ui_parameters.size;
    let node_label_fontsize = ui_parameters.node_label_fontsize.clone();
    let node_label_font = ui_parameters.node_label_font;
    let node_label_color = ui_parameters.node_label_color;

    for (point, conc) in points.iter().zip(points_conc.iter()) {
        // Convert Cartesian coordinates to polar coordinates
        let (x, y) = *point;

        // Create the label text
        let label = format!("({:.2}, {:.2})", conc.0, conc.1);

        // Draw the label
        if x > 0.0 {
            if y > 0.0 {
                chart.draw_series(std::iter::once(
                    Text::new(label, (x+0.02, y-0.01), (node_label_font, (fig_scale * node_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            } else {
                chart.draw_series(std::iter::once(
                    Text::new(label, (x+0.02, y+0.025), (node_label_font, (fig_scale * node_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            }
            
        } else {
            if y > 0.0 {
                chart.draw_series(std::iter::once(
                    Text::new(label, (x-0.075, y-0.01), (node_label_font, (fig_scale * node_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            } else {
                chart.draw_series(std::iter::once(
                    Text::new(label, (x-0.075, y+0.02), (node_label_font, (fig_scale * node_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font())
                        .into_dyn()
                )).expect("Error drawing the label");
            }
        }
        
    }
}