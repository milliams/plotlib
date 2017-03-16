use svg::Node;
use svg::node;

use histogram;
use scatter;
use axis;
use utils::PairWise;

fn value_to_face_offset(value: f64, axis: &axis::Axis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    (face_size * (value - axis.min())) / range
}

pub fn draw_x_axis(a: &axis::Axis, face_width: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", face_width)
        .set("y2", 0)
        .set("stroke", "black")
        .set("stroke-width", 1);

    let mut ticks = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_width);
        let tick_mark = node::element::Line::new()
            .set("x1", tick_pos)
            .set("y1", 0)
            .set("x2", tick_pos)
            .set("y2", 10)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);
    }

    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_width);
        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 20)
            .set("text-anchor", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
}

pub fn draw_y_axis(a: &axis::Axis, face_height: f64) -> node::element::Group {
    let axis_line = node::element::Line::new()
        .set("x1", 0)
        .set("y1", 0)
        .set("x2", 0)
        .set("y2", -face_height)
        .set("stroke", "black")
        .set("stroke-0", 1);

    let mut ticks = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_height);
        let tick_mark = node::element::Line::new()
            .set("x1", 0)
            .set("y1", -tick_pos)
            .set("x2", -10)
            .set("y2", -tick_pos)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);
    }

    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_height);
        let tick_label = node::element::Text::new()
            .set("x", -15)
            .set("y", -tick_pos)
            .set("text-anchor", "end")
            .set("dominant-baseline", "middle")
            .set("font-size", 12)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
}

pub fn draw_face_points(s: &scatter::Scatter,
                        x_axis: &axis::Axis,
                        y_axis: &axis::Axis,
                        face_width: f64,
                        face_height: f64,
                        style: &scatter::Style)
                        -> node::element::Group {
    let mut group = node::element::Group::new();

    for &(x, y) in s.data.iter() {
        let x_pos = value_to_face_offset(x, &x_axis, face_width);
        let y_pos = -value_to_face_offset(y, &y_axis, face_height);
        let radius = 5.;
        match style.get_marker() {
            scatter::Marker::Circle => {
                group.append(node::element::Circle::new()
                    .set("cx", x_pos)
                    .set("cy", y_pos)
                    .set("r", radius)
                    .set("fill", style.get_colour()));
            }
            scatter::Marker::Square => {
                group.append(node::element::Rectangle::new()
                    .set("x", x_pos - radius)
                    .set("y", y_pos - radius)
                    .set("width", 2. * radius)
                    .set("height", 2. * radius)
                    .set("fill", style.get_colour()));
            }
        };
    }

    group
}

pub fn draw_face_bars(h: &histogram::Histogram,
                      x_axis: &axis::Axis,
                      y_axis: &axis::Axis,
                      face_width: f64,
                      face_height: f64)
                      -> node::element::Group {
    let mut group = node::element::Group::new();

    for ((&l, &u), &count) in h.bin_bounds.pairwise().zip(h.bin_counts.iter()) {
        let l_pos = value_to_face_offset(l, &x_axis, face_width);
        let u_pos = value_to_face_offset(u, &x_axis, face_width);
        let width = u_pos - l_pos;
        let count_scaled = value_to_face_offset(count as f64, &y_axis, face_height);
        let circ = node::element::Rectangle::new()
            .set("x", l_pos)
            .set("y", -count_scaled)
            .set("width", width)
            .set("height", count_scaled)
            .set("fill", "burlywood")
            .set("stroke", "black");
        group.append(circ);
    }

    group
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn test_draw_scatter() {
        let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
        let s = scatter::Scatter::from_vec(&data);
        s.to_svg().save("scatter.svg");
    }

    #[test]
    fn test_draw_histogram() {
        use render::Render;
        use save::Save;

        let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
        let h = histogram::Histogram::from_vec(&data, 10);
        h.to_svg().save("histogram.svg");
    }*/
}
