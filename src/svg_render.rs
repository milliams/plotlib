use std;

use svg::Node;
use svg::node;

use histogram;
use axis;
use style;
use utils;
use utils::PairWise;

pub fn value_to_face_offset(value: f64, axis: &axis::ContinuousAxis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    (face_size * (value - axis.min())) / range
}

pub fn draw_x_axis(a: &axis::ContinuousAxis, face_width: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", face_width)
        .set("y2", 0)
        .set("stroke", "black")
        .set("stroke-width", 1);

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, a, face_width);
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", 0)
            .set("x2", tick_pos)
            .set("y2", 10)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", face_width / 2.)
        .set("y", 30)
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_y_axis(a: &axis::ContinuousAxis, face_height: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", 0)
        .set("y2", -face_height)
        .set("stroke", "black")
        .set("stroke-0", 1);

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, a, face_height);
        let tick_mark = node::element::Line::new()
            .set("x1", 0)
            .set("y1", -tick_pos)
            .set("x2", -10)
            .set("y2", -tick_pos)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", -15)
            .set("y", -tick_pos)
            .set("text-anchor", "end")
            .set("dominant-baseline", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", -30)
        .set("y", -(face_height / 2.))
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .set(
            "transform",
            format!("rotate(-90 {} {})", -30, -(face_height / 2.)),
        )
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_discrete_x_axis(a: &axis::DiscreteAxis, face_width: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", face_width)
        .set("y2", 0)
        .set("stroke", "black")
        .set("stroke-width", 1);

    let mut ticks = node::element::Group::new();
    let mut labels = node::element::Group::new();

    let space_per_tick = face_width / a.ticks().len() as f64;

    for (i, tick) in a.ticks().iter().enumerate() {
        let tick_pos = (i as f64 * space_per_tick) + (0.5 * space_per_tick);
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", 0)
            .set("x2", tick_pos)
            .set("y2", 10)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);

        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_owned()));
        labels.append(tick_label);
    }

    let label = node::element::Text::new()
        .set("x", face_width / 2.)
        .set("y", 30)
        .set("text-anchor", "middle")
        .set("font-size", 12)
        .add(node::Text::new(a.get_label()));

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
        .add(label)
}

pub fn draw_face_points<S>(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &S,
) -> node::element::Group
where
    S: style::Point,
{
    let mut group = node::element::Group::new();

    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width);
        let y_pos = -value_to_face_offset(y, y_axis, face_height);
        let radius = style.get_size().clone().unwrap_or(5.) as f64;
        match style.get_marker().clone().unwrap_or(style::Marker::Circle) {
            style::Marker::Circle => {
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
            style::Marker::Square => {
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
            style::Marker::Cross => {
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

use nalgebra::{Affine2, Point2};

pub fn draw_face_points2<S>(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    transform: Affine2<f64>,
    style: &S,
) -> node::element::Group
where
    S: style::Point,
{
    let mut group = node::element::Group::new();

    for &(x, y) in s {
        if x < x_axis.min() || x > x_axis.max() || y < y_axis.min() || y > y_axis.max() {
            continue;
        }
        let p = transform * Point2::new(x, y);
        let x_pos = p.x;
        let y_pos = p.y;
        let radius = style.get_size().clone().unwrap_or(5.) as f64;
        match style.get_marker().clone().unwrap_or(style::Marker::Circle) {
            style::Marker::Circle => {
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
            style::Marker::Square => {
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
            style::Marker::Cross => {
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

pub fn draw_face_bars<S>(
    h: &histogram::Histogram,
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &S,
) -> node::element::Group
where
    S: style::Bar,
{
    let mut group = node::element::Group::new();

    for ((&l, &u), &count) in h.bin_bounds.pairwise().zip(h.bin_counts.iter()) {
        let l_pos = value_to_face_offset(l, x_axis, face_width);
        let u_pos = value_to_face_offset(u, x_axis, face_width);
        let width = u_pos - l_pos;
        let count_scaled = value_to_face_offset(count as f64, y_axis, face_height);
        let rect = node::element::Rectangle::new()
            .set("x", l_pos)
            .set("y", -count_scaled)
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

pub fn draw_face_line<S>(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &S,
) -> node::element::Group
where
    S: style::Line,
{
    let mut group = node::element::Group::new();

    let mut d: Vec<node::element::path::Command> = vec![];
    let &(first_x, first_y) = s.first().unwrap();
    let first_x_pos = value_to_face_offset(first_x, x_axis, face_width);
    let first_y_pos = -value_to_face_offset(first_y, y_axis, face_height);
    d.push(node::element::path::Command::Move(
        node::element::path::Position::Absolute,
        (first_x_pos, first_y_pos).into(),
    ));
    for &(x, y) in s {
        let x_pos = value_to_face_offset(x, x_axis, face_width);
        let y_pos = -value_to_face_offset(y, y_axis, face_height);
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
            .set("d", path),
    );

    group
}

pub fn draw_face_boxplot<L, S>(
    d: &[f64],
    label: &L,
    x_axis: &axis::DiscreteAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: f64,
    face_height: f64,
    style: &S,
) -> node::element::Group
where
    S: style::BoxPlot,
    L: Into<String>,
    String: std::cmp::PartialEq<L>,
{
    let mut group = node::element::Group::new();

    let tick_index = x_axis.ticks().iter().position(|t| t == label).unwrap(); // TODO this should raise an error
    let space_per_tick = face_width / x_axis.ticks().len() as f64;
    let tick_pos = (tick_index as f64 * space_per_tick) + (0.5 * space_per_tick);

    let box_width = space_per_tick / 2.;

    let (q1, median, q3) = utils::quartiles(d);

    let box_start = -value_to_face_offset(q3, y_axis, face_height);
    let box_end = -value_to_face_offset(q1, y_axis, face_height);

    group.append(
        node::element::Rectangle::new()
            .set("x", tick_pos - (box_width / 2.))
            .set("y", box_start)
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
            .set("y1", mid_line)
            .set("x2", tick_pos + (box_width / 2.))
            .set("y2", mid_line)
            .set("stroke", "black"),
    );

    let (min, max) = utils::range(d);

    let whisker_bottom = -value_to_face_offset(min, y_axis, face_height);
    let whisker_top = -value_to_face_offset(max, y_axis, face_height);

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", whisker_bottom)
            .set("x2", tick_pos)
            .set("y2", box_end)
            .set("stroke", "black"),
    );

    group.append(
        node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", whisker_top)
            .set("x2", tick_pos)
            .set("y2", box_start)
            .set("stroke", "black"),
    );

    group
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_to_face_offset() {
        let axis = axis::ContinuousAxis::new(-2., 5.);
        assert_eq!(value_to_face_offset(-2.0, &axis, 14.0), 0.0);
        assert_eq!(value_to_face_offset(5.0, &axis, 14.0), 14.0);
        assert_eq!(value_to_face_offset(0.0, &axis, 14.0), 4.0);
        assert_eq!(value_to_face_offset(-4.0, &axis, 14.0), -4.0);
        assert_eq!(value_to_face_offset(7.0, &axis, 14.0), 18.0);
    }
}
