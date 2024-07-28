use plotters::{chart::ChartContext, coord::{cartesian::Cartesian2d, types::RangedCoordf64}, element::{Circle, IntoDynElement, Text}, style::{text_anchor::{HPos, Pos, VPos}, Color, IntoFont, ShapeStyle, TextStyle}};
use plotters_svg::SVGBackend;

use crate::backend::setup::game::Game;

use super::board_parameters::UIBoardParameters;

pub fn draw(
    game: &Game,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points: &Vec<(f64, f64)>,
) {
    let fig_scale = ui_parameters.size;
    let harbor_scale = ui_parameters.harbor_scale;
    let harbor_color = ui_parameters.harbor_color;
    match &game.parameters.init_harbors {
        Some(harbors) => {
            for harbor in harbors {
                let first_node_id = harbor.nodes.0;
                let second_node_id = harbor.nodes.1;

                let x_coord = (points[first_node_id as usize].0 + points[second_node_id as usize].0) / 2.0;
                let y_coord = (points[first_node_id as usize].1 + points[second_node_id as usize].1) / 2.0;

                // Draw the point
            chart.draw_series(std::iter::once(
                Circle::new((x_coord, y_coord), (fig_scale * harbor_scale as f64) as u32, ShapeStyle::from(&harbor_color))
            )).expect("Error drawing the point");
            }
        },
        None => (),
    }
}


pub fn draw_labels(
    game: &Game,
    chart: &mut ChartContext<'_, SVGBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>, 
    ui_parameters: &UIBoardParameters, 
    points: &Vec<(f64, f64)>,
    n_tile_rings: u32
) {
    let fig_scale = ui_parameters.size;
    let harbor_label_fontsize = ui_parameters.harbor_label_fontsize;
    let harbor_label_font = ui_parameters.harbor_label_font;
    let harbor_label_color = ui_parameters.harbor_label_color;

    match &game.parameters.init_harbors {
        Some(harbors) => {
            for harbor in harbors {
                let first_node_id = harbor.nodes.0;
                let second_node_id = harbor.nodes.1;

                // Create the label text
                let label = ui_parameters.harbor_label.to_owned() + &format!(" {}", ui_parameters.v_harbor_symbols[harbor.harbor_type as usize]);

                let x = (points[first_node_id as usize].0 + points[second_node_id as usize].0) / 2.0;
                let y = (points[first_node_id as usize].1 + points[second_node_id as usize].1) / 2.0;

                if x > 0.0 {
                    if y > 0.0 {
    
                        let text_style = TextStyle {
                                        font: (harbor_label_font, (fig_scale * harbor_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font(),
                                        color: harbor_label_color.to_backend_color(),
                                        pos: Pos::new(HPos::Left, VPos::Center),
                                    };
    
                        chart.draw_series(std::iter::once(
                            Text::new(label, (x+0.01, y), text_style)
                                .into_dyn()
                        )).expect("Error drawing the label");
                    } else {
    
                        let text_style = TextStyle {
                            font: (harbor_label_font, (fig_scale * harbor_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font(),
                            color: harbor_label_color.to_backend_color(),
                            pos: Pos::new(HPos::Left, VPos::Top),
                        };
    
                        chart.draw_series(std::iter::once(
                            Text::new(label, (x+0.01, y), text_style)
                                .into_dyn()
                        )).expect("Error drawing the label");
                    }
                    
                } else {
                    if y > 0.0 {
    
                        let text_style = TextStyle {
                            font: (harbor_label_font, (fig_scale * harbor_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font(),
                            color: harbor_label_color.to_backend_color(),
                            pos: Pos::new(HPos::Right, VPos::Bottom),
                        };
    
                        chart.draw_series(std::iter::once(
                            Text::new(label, (x, y), text_style)
                                .into_dyn()
                        )).expect("Error drawing the label");
                    } else {
    
                        let text_style = TextStyle {
                            font: (harbor_label_font, (fig_scale * harbor_label_fontsize as f64 / (n_tile_rings as f64 / 3.0)) as u32).into_font(),
                            color: harbor_label_color.to_backend_color(),
                            pos: Pos::new(HPos::Right, VPos::Top),
                        };
    
                        chart.draw_series(std::iter::once(
                            Text::new(label, (x-0.01, y), text_style)
                                .into_dyn()
                        )).expect("Error drawing the label");
                    }
                }
            }
        },
        None => (),
    }
}