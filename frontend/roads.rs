use plotters::{chart::ChartContext, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, series::LineSeries, style::Color};
use plotters_svg::SVGBackend;

use crate::backend::setup::board::Board;

use super::board_parameters::UIBoardParameters;

pub fn draw(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points: &Vec<(f64, f64)>,
    board: &Board,
) {
    let fig_scale = ui_parameters.size;
    let v_player_colors = &ui_parameters.v_player_colors;
    let road_width = ui_parameters.road_width;
    match &board.roads {
        Some(v_roads) => {
            for road in v_roads {
                let start = &points[road.nodes.0 as usize];
                let end = &points[road.nodes.1 as usize];
    
                chart.draw_series(
                    LineSeries::new(
                        vec![(start.0, start.1), (end.0, end.1)],
                        v_player_colors[road.player as usize].stroke_width((fig_scale * road_width as f64) as u32),
                    )
                ).expect("Error drawing roads");
            }
        },
        None => (),
    }
}