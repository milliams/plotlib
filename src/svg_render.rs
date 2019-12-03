use std;

use svg::node;
use svg::Node;

use crate::axis;
use crate::grid::GridType;
use crate::repr;
use crate::style;
use crate::utils;
use crate::utils::PairWise;

fn value_to_face_offset(value: f64, axis: &axis::ContinuousAxis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    (face_size * (value - axis.min())) / range
}

fn value_to_face_offset_y(value: f64, axis: &axis::ContinuousAxis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    -(face_size * (value - axis.max())) / range
}

fn vertical_line<S>(xpos: f64, ymin: f64, ymax: f64, color: S, arrow: bool) -> node::element::Line
where
    S: AsRef<str>,
{
    let mut node = node::element::Line::new()
        .set("x1", xpos)
        .set("x2", xpos)
        .set("y1", ymin)
        .set("y2", ymax)
        .set("stroke", color.as_ref())
        .set("stroke-width", 1);

    if arrow {
        node = node.set("marker-end", "url(#arrowhead)");
    };

    node
}

fn horizontal_line<S>(ypos: f64, xmin: f64, xmax: f64, color: S, arrow: bool) -> node::element::Line
where
    S: AsRef<str>,
{
    let mut node = node::element::Line::new()
        .set("x1", xmin)
        .set("x2", xmax)
        .set("y1", ypos)
        .set("y2", ypos)
        .set("stroke", color.as_ref())
        .set("stroke-width", 1);

    if arrow {
        node = node.set("marker-end", "url(#arrowhead)");
    };

    node
}

use crate::view::{
    ContinuousViewAxisLabelStyle, ContinuousViewAxisLineStyle, ContinuousViewTickStyle,
};

pub fn draw_x_axis(
    a: &axis::ContinuousAxis,
    x: f64,
    y: f64,
    face_width: f64,
    face_height: f64,
    show_zero: bool,
    line_style: ContinuousViewAxisLineStyle,
    label: (ContinuousViewAxisLabelStyle, u32),
    tick_style: (ContinuousViewTickStyle, f32, u32),
) -> node::element::Group {
    let (axis_line, size, skew) = match line_style {
        ContinuousViewAxisLineStyle::Arrow => (
            horizontal_line(y, x, face_width, "black", true),
            face_width - x,
            x,
        ),
        ContinuousViewAxisLineStyle::Line => (
            horizontal_line(y, x, face_width, "black", false),
            face_width - x,
            x,
        ),
    };

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    let (tick_min_y, tick_max_y, tick_label_y) = match tick_style.0 {
        ContinuousViewTickStyle::OnLine => (
            y + (tick_style.1 / 2.0) as f64,
            y - (tick_style.1 / 2.0) as f64,
            y + tick_style.2 as f64 + tick_style.1 as f64 / 2.,
        ),
        ContinuousViewTickStyle::OverLine => {
            (y - tick_style.1 as f64, y, (y - tick_style.1 as f64) - 4.0)
        }
        ContinuousViewTickStyle::UnderLine => (
            y + tick_style.1 as f64,
            y,
            y + tick_style.2 as f64 + tick_style.1 as f64,
        ),
    };

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, a, size) + skew;

        if !show_zero && tick == 0.0 {
            continue;
        }

        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", tick_max_y)
            .set("x2", tick_pos)
            .set("y2", tick_min_y)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", tick_label_y)
            .set("text-anchor", "middle")
            .set("font-size", tick_style.2)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    let label = match (tick_style.0, label.0) {
        (_, ContinuousViewAxisLabelStyle::Border) => node::element::Text::new()
            .set("x", (face_width - x) / 2. + x)
            .set("y", face_height)
            .set("text-anchor", "middle")
            .set("font-size", label.1)
            .add(node::Text::new(a.get_label())),
        (ContinuousViewTickStyle::OverLine, ContinuousViewAxisLabelStyle::NextToAxis) => {
            node::element::Text::new()
                .set("x", face_width)
                .set("y", y + label.1 as f64)
                .set("text-anchor", "end")
                .set("font-size", label.1)
                .add(node::Text::new(a.get_label()))
        }
        (_, ContinuousViewAxisLabelStyle::NextToAxis) => node::element::Text::new()
            .set("x", face_width)
            .set("y", y - 8.0)
            .set("text-anchor", "end")
            .set("font-size", label.1)
            .add(node::Text::new(a.get_label())),
    };

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_y_axis(
    a: &axis::ContinuousAxis,
    x: f64,
    y: f64,
    _face_height: f64,
    show_zero: bool,
    line_style: ContinuousViewAxisLineStyle,
    label: (ContinuousViewAxisLabelStyle, u32),
    tick_style: (ContinuousViewTickStyle, f32, u32),
) -> node::element::Group {
    let (axis_line, size, skew) = match line_style {
        ContinuousViewAxisLineStyle::Arrow => (vertical_line(x, y, 0., "black", true), y, 0.0),
        ContinuousViewAxisLineStyle::Line => (vertical_line(x, 0.0, y, "black", false), y, 0.0),
    };

    let (tick_min_x, tick_max_x, tick_label_x) = match tick_style.0 {
        ContinuousViewTickStyle::OnLine => (
            x - (tick_style.1 / 2.0) as f64,
            x + (tick_style.1 / 2.0) as f64,
            x - tick_style.1 as f64 / 2.0 - 4.0,
        ),
        ContinuousViewTickStyle::OverLine => (
            x + tick_style.1 as f64,
            x,
            (x + tick_style.1 as f64) + tick_style.2 as f64,
        ),
        ContinuousViewTickStyle::UnderLine => {
            (x - tick_style.1 as f64, x, x - tick_style.1 as f64 - 4.0)
        }
    };

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset_y(tick, a, size) + skew;

        if tick != 0.0 || show_zero {
            let tick_mark = node::element::Line::new()
                .set("x1", tick_max_x)
                .set("y1", tick_pos)
                .set("x2", tick_min_x)
                .set("y2", tick_pos)
                .set("stroke", "black")
                .set("stroke-width", 1);
            ticks.append(tick_mark);
        }

        let y = if tick != 0.0 || show_zero {
            tick_pos
        } else {
            tick_pos + 10.0
        };

        let x = if tick != 0.0 || show_zero {
            tick_label_x
        } else {
            x - 10.0
        };

        let tick_label = match tick_style.0 {
            ContinuousViewTickStyle::OverLine => node::element::Text::new()
                .set("x", x)
                .set("y", y)
                .set("text-anchor", "start")
                .set("dominant-baseline", "middle")
                .set("font-size", tick_style.2)
                .add(node::Text::new(tick.to_string())),
            _ => node::element::Text::new()
                .set("x", x)
                .set("y", y)
                .set("text-anchor", "end")
                .set("dominant-baseline", "middle")
                .set("font-size", tick_style.2)
                .add(node::Text::new(tick.to_string())),
        };
        labels.append(tick_label);
    }

    let label = match (tick_style.0, label.0) {
        (_, ContinuousViewAxisLabelStyle::Border) => node::element::Text::new()
            .set("x", -y / 2.)
            .set("y", label.1 as f64 / 2.)
            .set("text-anchor", "middle")
            .set("font-size", label.1)
            .set("transform", "rotate(-90 0 0)")
            .add(node::Text::new(a.get_label())),
        (ContinuousViewTickStyle::OverLine, ContinuousViewAxisLabelStyle::NextToAxis) => {
            node::element::Text::new()
                .set("x", x - 8.0)
                .set("y", 0. + label.1 as f64)
                .set("text-anchor", "end")
                .set("font-size", label.1)
                .add(node::Text::new(a.get_label()))
        }
        (_, ContinuousViewAxisLabelStyle::NextToAxis) => node::element::Text::new()
            .set("x", x + 8.0)
            .set("y", 0. + label.1 as f64)
            .set("text-anchor", "start")
            .set("font-size", label.1)
            .add(node::Text::new(a.get_label())),
    };

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_categorical_x_axis(
    a: &axis::CategoricalAxis,
    x: f64,
    y: f64,
    face_width: f64,
    face_height: f64,
) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", x)
        .set("y1", y)
        .set("x2", face_width)
        .set("y2", y)
        .set("stroke", "black")
        .set("stroke-width", 1);

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    let space_per_tick = (face_width - x) / a.ticks().len() as f64;

    for (i, tick) in a.ticks().iter().enumerate() {
        let tick_pos = (i as f64 * space_per_tick) + (0.5 * space_per_tick) + x;
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", y)
            .set("x2", tick_pos)
            .set("y2", y + 10.)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", y + 20.)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_owned()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", (face_width - x) / 2. + x)
        .set("y", face_height)
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_face_points(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    skew_x: f64,
    skew_y: f64,
    face_width: f64,
    face_height: f64,
    style: &style::PointStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width) + skew_x;
        let y_pos = -value_to_face_offset(y, y_axis, face_height) + skew_y;
        let radius = f64::from(style.get_size().clone().unwrap_or(5.));
        match style
            .get_marker()
            .clone()
            .unwrap_or(style::PointMarker::Circle)
        {
            style::PointMarker::Circle => {
                group.append(
                    node::element::Circle::new()
                        .set("cx", x_pos)
                        .set("cy", y_pos)
                        .set("r", radius)
                        .set(
                            "fill",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        ),
                );
            }
            style::PointMarker::Square => {
                group.append(
                    node::element::Rectangle::new()
                        .set("x", x_pos - radius)
                        .set("y", y_pos - radius)
                        .set("width", 2. * radius)
                        .set("height", 2. * radius)
                        .set(
                            "fill",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        ),
                );
            }
            style::PointMarker::Cross => {
                let path = node::element::path::Data::new()
                    .move_to((x_pos - radius, y_pos - radius))
                    .line_by((radius * 2., radius * 2.))
                    .move_by((-radius * 2., 0))
                    .line_by((radius * 2., -radius * 2.))
                    .close();
                group.append(
                    node::element::Path::new()
                        .set("fill", "none")
                        .set(
                            "stroke",
                            style.get_colour().clone().unwrap_or_else(|| "".into()),
                        )
                        .set("stroke-width", 2)
                        .set("d", path),
                );
            }
        };
    }

    group
}

pub fn draw_face_bars(
    h: &repr::Histogram,
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    x: f64,
    y: f64,
    face_width: f64,
    _face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    for ((&l, &u), &count) in h.bin_bounds.pairwise().zip(h.get_values()) {
        let l_pos = value_to_face_offset(l, x_axis, face_width) + x;
        let u_pos = value_to_face_offset(u, x_axis, face_width) + x;
        let width = u_pos - l_pos;
        let count_scaled = value_to_face_offset(count, y_axis, y);
        let rect = node::element::Rectangle::new()
            .set("x", l_pos)
            .set("y", y - count_scaled)
            .set("width", width)
            .set("height", count_scaled)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black");
        group.append(rect);
    }

    group
}

pub fn draw_face_line(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    skew_x: f64,
    skew_y: f64,
    face_width: f64,
    _face_height: f64,
    style: &style::LineStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    let mut d: Vec<node::element::path::Command> = vec![];
    let &(first_x, first_y) = s.first().unwrap();
    let first_x_pos = value_to_face_offset(first_x, x_axis, face_width) + skew_x;
    let first_y_pos = -value_to_face_offset(first_y, y_axis, skew_y) + skew_y;
    d.push(node::element::path::Command::Move(
        node::element::path::Position::Absolute,
        (first_x_pos, first_y_pos).into(),
    ));
    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width) + skew_x;
        let y_pos = -value_to_face_offset(y, y_axis, skew_y) + skew_y;
        d.push(node::element::path::Command::Line(
            node::element::path::Position::Absolute,
            (x_pos, y_pos).into(),
        ));
    }

    let path = node::element::path::Data::from(d);

    group.append(
        node::element::Path::new()
            .set("fill", "none")
            .set(
                "stroke",
                style.get_colour().clone().unwrap_or_else(|| "".into()),
            )
            .set("stroke-width", style.get_width().clone().unwrap_or(2.))
            .set(
                "stroke-linejoin",
                match style
                    .get_linejoin()
                    .clone()
                    .unwrap_or(style::LineJoin::Round)
                {
                    style::LineJoin::Miter => "miter",
                    style::LineJoin::Round => "round",
                },
            )
            .set("d", path),
    );

    group
}

pub fn draw_face_vector(
    s: ((f64, f64), (f64, f64)),
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    skew_x: f64,
    skew_y: f64,
    face_width: f64,
    _face_height: f64,
    style: &style::LineStyle,
) -> node::element::Group {
    let mut group = node::element::Group::new();

    let (first_x_skew, second_x_skew, first_y_skew, second_y_skew) =
        if (s.0).0 < (s.1).0 && (s.0).1 < (s.1).1 {
            (skew_x, skew_x - 10., skew_y, skew_y + 3.5)
        } else if (s.0).0 > (s.1).0 && (s.0).1 > (s.1).1 {
            (skew_x, skew_x + 5., skew_y, skew_y - 7.)
        } else if (s.0).0 < (s.1).0 && (s.0).1 > (s.1).1 {
            (skew_x, skew_x - 10., skew_y, skew_y - 3.5)
        } else {
            (skew_x, skew_x + 5., skew_y, skew_y + 7.)
        };

    let node = node::element::Line::new()
        .set(
            "x1",
            value_to_face_offset((s.0).0, x_axis, face_width) + first_x_skew,
        )
        .set(
            "x2",
            value_to_face_offset((s.1).0, x_axis, face_width) + second_x_skew,
        )
        .set(
            "y1",
            -value_to_face_offset((s.0).1, y_axis, skew_y) + first_y_skew,
        )
        .set(
            "y2",
            -value_to_face_offset((s.1).1, y_axis, skew_y) + second_y_skew,
        )
        .set(
            "stroke",
            style.get_colour().clone().unwrap_or_else(|| "".into()),
        )
        .set("stroke-width", style.get_width().unwrap_or(1.))
        .set("marker-end", "url(#arrowhead)");

    group.append(node);

    group
}

pub fn draw_face_boxplot<L>(
    d: &[f64],
    label: &L,
    x_axis: &axis::CategoricalAxis,
    y_axis: &axis::ContinuousAxis,
    y: f64,
    x: f64,
    face_width: f64,
    face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group
where
    L: Into<String>,
    String: std::cmp::PartialEq<L>,
{
    let mut group = node::element::Group::new();

    let tick_index = x_axis.ticks().iter().position(|t| t == label).unwrap(); // TODO this should raise an error
    let space_per_tick = face_width / x_axis.ticks().len() as f64;
    let tick_pos = (tick_index as f64 * space_per_tick) + (0.5 * space_per_tick) + x;

    let box_width = space_per_tick / 2.;

    let (q1, median, q3) = utils::quartiles(d);

    let box_start = -value_to_face_offset(q3, y_axis, face_height);
    let box_end = -value_to_face_offset(q1, y_axis, face_height);

    group.append(
        node::element::Rectangle::new()
            .set("x", tick_pos - (box_width / 2.))
            .set("y", y + box_start)
            .set("width", box_width)
            .set("height", box_end - box_start)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black"),
    );

    let mid_line = -value_to_face_offset(median, y_axis, face_height);

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos - (box_width / 2.))
            .set("y1", y + mid_line)
            .set("x2", tick_pos + (box_width / 2.))
            .set("y2", y + mid_line)
            .set("stroke", "black"),
    );

    let (min, max) = utils::range(d);

    let whisker_bottom = -value_to_face_offset(min, y_axis, face_height);
    let whisker_top = -value_to_face_offset(max, y_axis, face_height);

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", y + whisker_bottom)
            .set("x2", tick_pos)
            .set("y2", y + box_end)
            .set("stroke", "black"),
    );

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", y + whisker_top)
            .set("x2", tick_pos)
            .set("y2", y + box_start)
            .set("stroke", "black"),
    );

    group
}

pub fn draw_face_barchart<L>(
    d: f64,
    label: &L,
    x_axis: &axis::CategoricalAxis,
    y_axis: &axis::ContinuousAxis,
    y: f64,
    x: f64,
    face_width: f64,
    face_height: f64,
    style: &style::BoxStyle,
) -> node::element::Group
where
    L: Into<String>,
    String: std::cmp::PartialEq<L>,
{
    let mut group = node::element::Group::new();

    let tick_index = x_axis.ticks().iter().position(|t| t == label).unwrap(); // TODO this should raise an error
    let space_per_tick = face_width / x_axis.ticks().len() as f64;
    let tick_pos = (tick_index as f64 * space_per_tick) + (0.5 * space_per_tick);

    let box_width = space_per_tick / 2.;

    let box_start = -value_to_face_offset(d, y_axis, face_height);
    let box_end = -value_to_face_offset(0.0, y_axis, face_height);

    group.append(
        node::element::Rectangle::new()
            .set("x", tick_pos - (box_width / 2.) + x)
            .set("y", y + box_start)
            .set("width", box_width)
            .set("height", box_end - box_start)
            .set(
                "fill",
                style
                    .get_fill()
                    .clone()
                    .unwrap_or_else(|| "burlywood".into()),
            )
            .set("stroke", "black"),
    );

    group
}

pub(crate) fn draw_grid(
    grid: GridType,
    skew_x: f64,
    skew_y: f64,
    face_width: f64,
    face_height: f64,
) -> node::element::Group {
    match grid {
        GridType::HorizontalOnly(grid) => {
            let (ymin, ymax) = (0f64, face_height);
            let y_step = (ymax - ymin) / f64::from(grid.ny);
            let mut lines = node::element::Group::new();

            for iy in 0..=grid.ny {
                let y = f64::from(iy) * y_step + ymin;
                let line =
                    horizontal_line(y, skew_x, face_width + skew_x, grid.color.as_str(), false);
                lines = lines.add(line);
            }

            lines
        }
        GridType::Both(grid) => {
            let (xmin, xmax) = (0f64, face_width);
            let (ymin, ymax) = (0f64, face_height);

            let x_step = (xmax - xmin) / f64::from(grid.nx);
            let y_step = (ymax - ymin) / f64::from(grid.ny);

            let mut lines = node::element::Group::new();

            for iy in 0..=grid.ny {
                let y = f64::from(iy) * y_step + ymin;
                let line =
                    horizontal_line(y, skew_x, face_width + skew_x, grid.color.as_str(), false);
                lines = lines.add(line);
            }

            for ix in 0..=grid.nx {
                let x = f64::from(ix) * x_step + skew_x;
                let line =
                    vertical_line(x, skew_y, face_height + skew_y, grid.color.as_str(), false);
                lines = lines.add(line);
            }

            lines
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_face_offset() {
        let axis = axis::ContinuousAxis::new(-2., 5., 6);
        assert_eq!(value_to_face_offset(-2.0, &axis, 14.0), 0.0);
        assert_eq!(value_to_face_offset(5.0, &axis, 14.0), 14.0);
        assert_eq!(value_to_face_offset(0.0, &axis, 14.0), 4.0);
        assert_eq!(value_to_face_offset(-4.0, &axis, 14.0), -4.0);
        assert_eq!(value_to_face_offset(7.0, &axis, 14.0), 18.0);
    }
}
