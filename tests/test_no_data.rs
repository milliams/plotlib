use plotlib::page::Page;
use plotlib::repr::Scatter;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

#[test]
fn test_data_with_one_length() {
    // Scatter plots expect a list of pairs
    let data1 = [(-3.0, 2.3)];

    // We create our scatter plot from the data
    let s1 = Scatter::from_slice(&data1).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v)
        .save("/tmp/scatter_one_length.svg")
        .unwrap();
}

#[test]
fn test_data_with_no_length() {
    // Scatter plots expect a list of pairs
    let data1 = [];

    // We create our scatter plot from the data
    let s1 = Scatter::from_slice(&data1).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v)
        .save("/tmp/scatter_zero_length.svg")
        .unwrap();
}

#[test]
#[should_panic(expected = "Invalid x_range")]
fn test_data_with_one_length_and_autoscaling_axes_limits() {
    // Scatter plots expect a list of pairs
    let data1 = [(-3.0, 2.3)];

    // We create our scatter plot from the data
    let s1 = Scatter::from_slice(&data1).style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // // A page with a single view is then saved to an SVG file
    Page::single(&v)
        .save("/tmp/scatter_one_length.svg")
        .unwrap();
}
