use plotters::{chart::ChartContext, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, element::{Circle, TriangleMarker}, style::ShapeStyle};
use plotters_svg::SVGBackend;

use crate::backend::setup::node::Node;

use super::board_parameters::UIBoardParameters;

pub fn draw(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    nodes: &Vec<Node>,
) {
    let fig_scale = ui_parameters.size;
    let v_player_colors = &ui_parameters.v_player_colors;
    let settlement_scale = ui_parameters.settlement_scale;
    let settlement_width = ui_parameters.settlement_width;
    let city_scale = ui_parameters.city_scale;
    

    for node in nodes {
        match &node.settlement {
            Some(settlement) => {
                let i_player = settlement.player_id;
                let coords = match node.coords_cart {
                    Some(c) => c,
                    None => (0.0, 0.0),
                };

                chart.draw_series(std::iter::once(
                    Circle::new(coords, (fig_scale * settlement_scale as f64) as u32, ShapeStyle::from(v_player_colors[i_player as usize]).stroke_width((fig_scale * settlement_width as f64) as u32))
                )).expect("Error drawing the point");
            },
            None => continue,
        }

        match &node.city {
            Some(city) => {
                let i_player = city.player_id;
                let coords = match node.coords_cart {
                    Some(c) => c,
                    None => (0.0, 0.0),
                };

                chart.draw_series(std::iter::once(
                    Circle::new(coords, (fig_scale * city_scale as f64) as u32, ShapeStyle::from(v_player_colors[i_player as usize]).filled())
                )).expect("Error drawing the point");
            },
            None => continue,
        }
    }
}