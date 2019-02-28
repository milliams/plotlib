use plotlib::style::Point;

fn main() {
    let data = [
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];
    let s1 = plotlib::scatter::Scatter::from_slice(&data).style(
        plotlib::scatter::Style::new()
            .marker(plotlib::style::Marker::Square)
            .colour("burlywood")
            .size(2.),
    );
    let s2 = plotlib::scatter::Scatter::from_slice(&[(-1.4, 2.5), (7.2, -0.3)])
        .style(plotlib::scatter::Style::new().colour("darkseagreen"));
    let v = plotlib::view::ContinuousView::new()
        .add(&s1)
        .add(&s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");
    plotlib::page::Page::single(&v)
        .save("scatter.svg")
        .expect("saving svg");

    /*
    //
    // Create a scatter representation
    let s = Scatter::new()
        .data(data)
        .y_errors(errors)
        .colour(Colour::Red)
        .marker(Marker::Circle);

    // Create a histogram representation
    let h = Histogram::new()
        .data(data2)
        .error_bars(true)
        .fill(Colour::Green);

    // Create a function representation
    let l = Function::new()
        .function(|&x| sin(x/100.0))
        .colour(Colour::Blue);
        .stroke(Style::Dotted);

    // Create a grid. Maybe this should be part of the view directly?
    let g = Grid::new();

    // Create a view containing all the representations
    let v = ContinuousView::new()
        .add(&l)
        .add(&h)
        .add(&s)
        .add(&g)
        .x_range(-10, 90)
        .x_label("Age")
        );

    // put that view into a plot and save it to file
    let p = Plot::single(v);
    p.save("plot.svg");

    //
    // Copy of http://matplotlib.org/examples/lines_bars_and_markers/fill_demo_features.html
    let y1 = Function::new()
        .function(|&x| sin(x))
        .sampling(Array::linspace(0., 2. * pi, 500))
        .colour(Colour::Blue);
        .alpha(0.3)
        .fill(true);

    let y2 = Function::new()
        .function(|&x| sin(3 * x))
        .sampling(Array::linspace(0., 2. * pi, 500))
        .colour(Colour::Red);
        .alpha(0.3)
        .fill(true;

    let v = ContinuousView::new()
        .add(y1)
        .add(y2);

    Plot::single(v).save("plot.svg");
    */
}
