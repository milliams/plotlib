use std::collections::btree_map::BTreeMap;

fn main() {
    let mut data = Vec::new();
    let message: &str = "This is a long message";
    let mut count = BTreeMap::new();

    for c in message.trim().to_lowercase().chars() {
        if c.is_alphabetic() {
            *count.entry(c).or_insert(0) += 1
        }
    }

    println!("Number of occurences per character");
    for (ch, count) in &count {
        println!("{:?}: {}", ch, count);
        let count = *count as f64;
        data.push(plotlib::repr::BarChart::new(count).label(ch.to_string()));
    }
    // Add data to the view
    let v = data
        .into_iter()
        .fold(plotlib::view::CategoricalView::new(), |view, datum| {
            view.add(datum)
        });

    plotlib::page::Page::single(&v)
        .save("barchart.svg")
        .expect("saving svg");
}
