use plotters::{chart::ChartContext, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, series::LineSeries, style::Color};
use plotters_svg::SVGBackend;

use super::{aux_functions::linspace, board_parameters::UIBoardParameters};



pub fn draw(
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    n_node_rings: &u32,
) {
    let radii: Vec<f64> = (0..*n_node_rings).map(|x| (1.0 + x as f64) / *n_node_rings as f64).collect();
    let circle_color = ui_parameters.circle_color;
    let circle_width = ui_parameters.circle_width;
    for radius in radii {
        let x_coord: Vec<f64> = linspace(-radius, radius, ui_parameters.n_circle_steps as usize + 1);


        chart.draw_series(LineSeries::new(x_coord.iter().map(|&x| (x, (radius.powf(2.0) - x.powf(2.0)).sqrt())), circle_color.stroke_width(circle_width))).expect("Error drawing upper circle.");
        chart.draw_series(LineSeries::new(x_coord.iter().rev().map(|&x| (x, (radius.powf(2.0) - x.powf(2.0)).sqrt())), circle_color.stroke_width(circle_width))).expect("Error drawing upper circle.");
        chart.draw_series(LineSeries::new(x_coord.iter().map(|&x| (x, -(radius.powf(2.0) - x.powf(2.0)).sqrt())), circle_color.stroke_width(circle_width))).expect("Error drawing lower circle.");
        chart.draw_series(LineSeries::new(x_coord.iter().rev().map(|&x| (x, -(radius.powf(2.0) - x.powf(2.0)).sqrt())), circle_color.stroke_width(circle_width))).expect("Error drawing lower circle.");
    }
}