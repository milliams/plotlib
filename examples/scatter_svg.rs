extern crate plotlib;

fn main() {
    //let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    //let s = plotlib::scatter::Scatter::from_vec(&data);
    //s.to_svg().save("scatter.svg");

    //
    // Or, simplest example:
    let data = vec![(-3.0, 2.3), (-1.6, 5.3), (0.3, 0.7), (4.3, -1.4), (6.4, 4.3), (8.5, 3.7)];
    let s = plotlib::scatter::Scatter::from_vec(&data);
    let v = plotlib::view::View::new().add(&s);
    plotlib::plot::Plot::single(&v).save("scatter.svg");

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
    let v = View::new()
        .add(&l).
        .add(&h).
        .add(&s).
        .add(&g).
        x_axis(Axis::new()
            .range(-10, 90)
            .label("Age")
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

    let v = View::new()
        .add(y1)
        .add(y2);

    Plot::single(v).save("plot.svg");
    */
}
