use plotters::{chart::ChartContext, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, series::LineSeries};
use plotters_svg::SVGBackend;

use crate::backend::setup::game::Game;

use super::board_parameters::UIBoardParameters;



pub fn draw(
    game: &Game,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points: &Vec<(f64, f64)>,
    n_nodes: usize,
) {

    let edge_color = ui_parameters.edge_color;
    for (i, conns) in game.parameters.node_adjacency.iter().enumerate() {
        for &conn_index in conns {
            match conn_index {
                Some(conn_id) => {
                    if conn_id < n_nodes as u32 {
                        let start = points[i];
                        let end = points[conn_id as usize];
                        chart.draw_series(
                            LineSeries::new(
                                vec![start, end],
                                &edge_color,
                            )
                        ).expect("Error drawing the connection lines");
                    }
                },
                None => (),
            }  
        }
    }
}