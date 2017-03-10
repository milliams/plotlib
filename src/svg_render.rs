use svg;
use svg::Node;
use svg::Document;
use svg::node;

use histogram;
use scatter;
use axis;

fn value_to_face_offset(value: f64, axis: &axis::Axis, face_size: f64) -> f64 {
    let range = axis.max() - axis.min();
    (face_size * (value - axis.min())) / range
}

fn draw_x_axis(a: &axis::Axis, face_width: f64) -> node::element::Group {
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
            .set("y2", 3)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);
    }

    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_width);
        let tick_label = node::element::Text::new()
            .set("x", tick_pos)
            .set("y", 8)
            .set("text-anchor", "middle")
            .set("font-size", 5)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
}

fn draw_y_axis(a: &axis::Axis, face_height: f64) -> node::element::Group {
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
            .set("x2", -3)
            .set("y2", -tick_pos)
            .set("stroke", "black")
            .set("stroke-width", 1);
        ticks.append(tick_mark);
    }

    let mut labels = node::element::Group::new();

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_height);
        let tick_label = node::element::Text::new()
            .set("x", -8)
            .set("y", -tick_pos)
            .set("text-anchor", "right")
            .set("dominant-baseline", "middle")
            .set("font-size", 5)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    for &tick in a.ticks().iter() {
        let tick_pos = value_to_face_offset(tick, &a, face_height);
        let tick_label = node::element::Text::new()
            .set("x", -8)
            .set("y", -tick_pos)
            .set("text-anchor", "right")
            .set("dominant-baseline", "middle")
            .set("font-size", 5)
            .add(node::Text::new(tick.to_string()));
        labels.append(tick_label);
    }

    node::element::Group::new()
        .add(ticks)
        .add(axis_line)
        .add(labels)
}

fn draw_face_points(s: &scatter::Scatter,
                    face_width: f64,
                    face_height: f64)
                    -> node::element::Group {
    let mut group = node::element::Group::new();

    for &(x, y) in s.data.iter() {
        let x_pos = value_to_face_offset(x, &s.x_axis, face_width);
        let y_pos = -value_to_face_offset(y, &s.y_axis, face_height);
        let circ = node::element::Circle::new()
            .set("cx", x_pos)
            .set("cy", y_pos)
            .set("r", 1.0);
        group.append(circ);
    }

    group
}

pub fn draw_scatter(s: &scatter::Scatter) {
    let face_width = 70.0;
    let face_height = 50.0;

    let face_x_pos = 20.0;
    let face_y_pos = 5.0;

    let view_box_width = 100; // Overall width of the document
    let view_box_height = 70; // Overall height of the document

    let face_background = node::element::Rectangle::new()
        .set("x", 0)
        .set("y", -face_height)
        .set("width", face_width)
        .set("height", face_height)
        .set("fill", "lightgrey");

    let x_axis = draw_x_axis(&s.x_axis, face_width);
    let y_axis = draw_y_axis(&s.y_axis, face_height);
    let face = draw_face_points(&s, face_width, face_height);

    let components = node::element::Group::new()
        .add(face_background)
        .add(x_axis)
        .add(y_axis)
        .add(face)
        .set("transform",
             format!("translate({}, {})", face_x_pos, face_y_pos + face_height));

    let document_background = node::element::Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "#EAEAEA");

    let document = Document::new()
        .set("viewBox", (0, 0, view_box_width, view_box_height))
        .add(document_background)
        .add(components);

    svg::save("plot.svg", &document).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_scatter() {
        let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
        let s = scatter::Scatter::from_vec(&data);
        draw_scatter(&s);
    }
}
