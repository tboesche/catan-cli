use std::{fs::{self, File}, io::Write};

use plotters::{backend::SVGBackend, chart::ChartBuilder, drawing::IntoDrawingArea, style::IntoFont};
use regex::Regex;

use crate::backend::setup::{board, game::Game, shape::get_n_node_rings};

use super::{board_parameters::UIBoardParameters, buildings, circles, coords::{add_cart_coords_nodes, add_cart_coords_tiles, add_conc_coords_nodes}, edges, harbors, nodes, roads, tiles};



impl Game {

    pub fn draw_board(&self, ui_parameters: UIBoardParameters, board_name: String) -> Result<(), Box<dyn std::error::Error>> {

        tiles::create_svg(self, &ui_parameters);

        tiles::modify_svg()?;

        nodes::create_svg(self, &ui_parameters);

        nodes::modify_svg()?;

        self.draw_raw_board(ui_parameters);

        add_tiles_nodes_svg(board_name)?;

        Ok(())
    }

    fn draw_raw_board(&self, ui_parameters: UIBoardParameters) -> () {
        
        let fig_scale = ui_parameters.size;
        let x_dim = ui_parameters.x_dim;
        let y_dim = ui_parameters.y_dim;

        let x_lims = ui_parameters.x_lims;
        let y_lims = ui_parameters.y_lims;

        let n_tile_rings = self.parameters.n_tile_rings.clone();

        let board = &self.round.board;

        let n_tiles = board.tiles.len();
        let n_nodes = board.nodes.len();

        let tile_shape = &self.parameters.tile_shape;
        let n_node_rings = get_n_node_rings(n_tile_rings, tile_shape);

        let v_tile_resource = &self.parameters.v_tile_resources;

        let mut nodes = board.nodes.clone();
        // println!("{:#?}", nodes);
        nodes = add_conc_coords_nodes(nodes, n_node_rings, &tile_shape);
        nodes = add_cart_coords_nodes(nodes, n_node_rings, &tile_shape);
        // println!("{:#?}", nodes);

        let mut tiles = board.tiles.clone();
        tiles = add_cart_coords_tiles(tiles, n_tile_rings, &tile_shape);
        // println!("{:#?}", tiles);

        let mut points: Vec<(f64, f64)> = vec![];
        // println!("{:#?}", nodes);
        for node in &nodes {
            match node.coords_cart {
                Some(point) => points.push(point),
                None => (),
            }
        }

        let mut points_conc: Vec<(u32, u32)> = vec![];
        for node in &nodes {
            match node.coords_conc {
                Some(point) => points_conc.push(point),
                None => (),
            }
        }

        let mut points_tiles: Vec<(f64, f64)> = vec![];
        for tile in &tiles {
            match tile.coords_cart {
                Some(point) => points_tiles.push(point),
                None => (),
            }
        }

        // Create a drawing area
        let root = SVGBackend::new("data/assets/raw_board.svg", ((fig_scale * x_dim as f64) as u32, (fig_scale * y_dim as f64) as u32)).into_drawing_area();
        root.fill(&ui_parameters.bg_color).expect("Error filling the drawing area");
        // 179, 179, 179

        // Create a chart
        let mut chart = ChartBuilder::on(&root)
            .caption(ui_parameters.caption, (ui_parameters.caption_font, (fig_scale * ui_parameters.caption_fontsize as f64 * (n_tile_rings as f64 / 3.0)) as u32).into_font().color(&ui_parameters.caption_color))
            .margin(ui_parameters.margin)
            .x_label_area_size(ui_parameters.x_label_size)
            .y_label_area_size(ui_parameters.y_label_size)
            .build_cartesian_2d(x_lims.0..x_lims.1, y_lims.0..y_lims.1)
            .expect("Error building the chart");

        //chart.configure_mesh().draw().expect("Error configuring the mesh");

        // Draw tiles 
        // tiles::draw(&self, &mut chart, &ui_parameters, &tiles, &nodes);

        // Draw the edges
        edges::draw(&self, &mut chart, &ui_parameters, &points, n_nodes);

        // draw tile nodes
        tiles::draw_nodes(&mut chart, &ui_parameters, &points_tiles);

        // Add tile labels with resource and RNG
        tiles::draw_labels(&mut chart, &ui_parameters, &points_tiles, &tiles, n_tiles, n_tile_rings);

        // Draw circles
        circles::draw(&mut chart, &ui_parameters, &n_node_rings);

        // Draw roads
        roads::draw(&mut chart, &ui_parameters, &points, board);

        // Draw settlements and cities
        buildings::draw(&mut chart, &ui_parameters, &nodes);

        // draw harbors
        harbors::draw(&self, &mut chart, &ui_parameters, &points);

        // Add harbour labels
        harbors::draw_labels(&self, &mut chart, &ui_parameters, &points, n_tile_rings);

        // Draw nodes
        // nodes::draw(&mut chart, &ui_parameters, &points);

        // Add labels with polar coordinates
        nodes::draw_labels(&mut chart, &ui_parameters, &nodes, &points, &points_conc, n_tile_rings);

        // Save the plot
        root.present().expect("Error presenting the drawing area");
    }

}

fn add_tiles_nodes_svg(board_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let tiles = fs::read_to_string("data/assets/tiles.svg")?;

    let board = fs::read_to_string("data/assets/raw_board.svg")?;

    let nodes = fs::read_to_string("data/assets/nodes.svg")?;

    // Define the regex pattern to match polygon elements
    let re_polygon = Regex::new(r"(<polygon[^>]*>)").unwrap();
    // Define the regex pattern to find the first text element
    let re_text = Regex::new(r"(<text[^>]*>[^<]*</text>)").unwrap();
    // Define the regex pattern to match circle elements
    let re_circle = Regex::new(r"(<circle[^>]*>)").unwrap();


    // Extract all polygon elements from the first SVG data
    let polygon_elements: Vec<String> = re_polygon
        .find_iter(tiles.as_str())
        .map(|mat| mat.as_str().to_string())
        .collect();

    // Find the position right after the first text element in the second SVG data
    let modified_board = if let Some(text_match) = re_text.find(board.as_str()) {
        let pos = text_match.end();
        let mut new_svg = String::new();
        new_svg.push_str(&board[..pos]);
        for polygon in &polygon_elements {
            new_svg.push_str(polygon);
        }
        new_svg.push_str(&board[pos..]);
        new_svg
    } else {
        board.to_string()
    };

    // Extract all circle elements from the third SVG
    let circle_elements: Vec<String> = re_circle
        .captures_iter(nodes.as_str())
        .map(|caps| caps[0].to_string())
        .collect();

    // Find the position of the closing </svg> tag in the second SVG
    if let Some(pos) = modified_board.find("</svg>") {
        let mut new_modified_board = String::with_capacity(modified_board.len() + circle_elements.join("\n").len());
        new_modified_board.push_str(&modified_board[..pos]);
        for circle in &circle_elements {
            new_modified_board.push_str(circle);
        }
        new_modified_board.push_str(&modified_board[pos..]);

        // Save the modified SVG data to a file
        let board_name = board_name + ".svg";
        let mut file = File::create(board_name).expect("Unable to create file");
        file.write_all(new_modified_board.as_bytes()).expect("Unable to write data");

        // println!("SVG data has been written to modified_svg_2.svg");
    } else {
        println!("Error writing combined SVG file.");
    }

    Ok(())
}