//! A module for plotting graphs

use std;
use std::collections::HashMap;

use crate::axis;
use crate::repr;
use crate::style;
use crate::utils::PairWise;

// Given a value like a tick label or a bin count,
// calculate how far from the x-axis it should be plotted
fn value_to_axis_cell_offset(value: f64, axis: &axis::ContinuousAxis, face_cells: u32) -> i32 {
    let data_per_cell = (axis.max() - axis.min()) / f64::from(face_cells);
    ((value - axis.min()) / data_per_cell).round() as i32
}

/// Given a list of ticks to display,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to tick value
fn tick_offset_map(axis: &axis::ContinuousAxis, face_width: u32) -> HashMap<i32, f64> {
    axis.ticks()
        .iter()
        .map(|&tick| (value_to_axis_cell_offset(tick, axis, face_width), tick))
        .collect()
}

/// Given a histogram object,
/// the total scale of the axis
/// and the number of face cells to work with,
/// create a mapping of cell offset to bin bound
fn bound_cell_offsets(
    hist: &repr::Histogram,
    x_axis: &axis::ContinuousAxis,
    face_width: u32,
) -> Vec<i32> {
    hist.bin_bounds
        .iter()
        .map(|&bound| value_to_axis_cell_offset(bound, x_axis, face_width))
        .collect()
}

/// calculate for each cell which bin it is representing
/// Cells which straddle bins will return the bin just on the lower side of the centre of the cell
/// Will return a vector with (`face_width + 2`) entries to represent underflow and overflow cells
/// cells which do not map to a bin will return `None`.
fn bins_for_cells(bound_cell_offsets: &[i32], face_width: u32) -> Vec<Option<i32>> {
    let bound_cells = bound_cell_offsets;

    let bin_width_in_cells = bound_cells.pairwise().map(|(&a, &b)| b - a);
    let bins_cell_offset = bound_cells.first().unwrap();

    let mut cell_bins: Vec<Option<i32>> = vec![None]; // start with a prepended negative null
    for (bin, width) in bin_width_in_cells.enumerate() {
        // repeat bin, width times
        for _ in 0..width {
            cell_bins.push(Some(bin as i32));
        }
    }
    cell_bins.push(None); // end with an appended positive null

    if *bins_cell_offset < 0 {
        cell_bins = cell_bins
            .iter()
            .skip(bins_cell_offset.wrapping_abs() as usize)
            .cloned()
            .collect();
    } else if *bins_cell_offset > 0 {
        let mut new_bins = vec![None; (*bins_cell_offset) as usize];
        new_bins.extend(cell_bins.iter());
        cell_bins = new_bins;
    }

    if cell_bins.len() < face_width as usize + 2 {
        let deficit = face_width as usize + 2 - cell_bins.len();
        let mut new_bins = cell_bins;
        new_bins.extend(vec![None; deficit].iter());
        cell_bins = new_bins;
    } else if cell_bins.len() > face_width as usize + 2 {
        let new_bins = cell_bins;
        cell_bins = new_bins
            .iter()
            .take(face_width as usize + 2)
            .cloned()
            .collect();
    }

    cell_bins
}

/// An x-axis label for the text output renderer
#[derive(Debug)]
struct XAxisLabel {
    text: String,
    offset: i32,
}

impl XAxisLabel {
    fn len(&self) -> usize {
        self.text.len()
    }

    /// The number of cells the label will actually use
    /// We want this to always be an odd number
    fn footprint(&self) -> usize {
        if self.len() % 2 == 0 {
            self.len() + 1
        } else {
            self.len()
        }
    }

    /// The offset, relative to the zero-point of the axis where the label should start to be drawn
    fn start_offset(&self) -> i32 {
        self.offset as i32 - self.footprint() as i32 / 2
    }
}

fn create_x_axis_labels(x_tick_map: &HashMap<i32, f64>) -> Vec<XAxisLabel> {
    let mut ls: Vec<_> = x_tick_map
        .iter()
        .map(|(&offset, &tick)| XAxisLabel {
            text: tick.to_string(),
            offset,
        }).collect();
    ls.sort_by_key(|l| l.offset);
    ls
}

pub fn render_y_axis_strings(y_axis: &axis::ContinuousAxis, face_height: u32) -> (String, i32) {
    // Get the strings and offsets we'll use for the y-axis
    let y_tick_map = tick_offset_map(y_axis, face_height);

    // Find a minimum size for the left gutter
    let longest_y_label_width = y_tick_map
        .values()
        .map(|n| n.to_string().len())
        .max()
        .expect("ERROR: There are no y-axis ticks");

    let y_axis_label = format!(
        "{: ^width$}",
        y_axis.get_label(),
        width = face_height as usize + 1
    );
    let y_axis_label: Vec<_> = y_axis_label.chars().rev().collect();

    // Generate a list of strings to label the y-axis
    let y_label_strings: Vec<_> = (0..=face_height)
        .map(|line| match y_tick_map.get(&(line as i32)) {
            Some(v) => v.to_string(),
            None => "".to_string(),
        }).collect();

    // Generate a list of strings to tick the y-axis
    let y_tick_strings: Vec<_> = (0..=face_height)
        .map(|line| match y_tick_map.get(&(line as i32)) {
            Some(_) => "-".to_string(),
            None => " ".to_string(),
        }).collect();

    // Generate a list of strings to be the y-axis line itself
    let y_axis_line_strings: Vec<String> = std::iter::repeat('+')
        .take(1)
        .chain(std::iter::repeat('|').take(face_height as usize))
        .map(|s| s.to_string())
        .collect();

    let iter = y_axis_label
        .iter()
        .zip(y_label_strings.iter())
        .zip(y_tick_strings.iter())
        .zip(y_axis_line_strings.iter())
        .map(|(((a, x), y), z)| (a, x, y, z));

    let axis_string: Vec<String> = iter
        .rev()
        .map(|(l, ls, t, a)| {
            format!(
                "{} {:>num_width$}{}{}",
                l,
                ls,
                t,
                a,
                num_width = longest_y_label_width
            )
        }).collect();

    let axis_string = axis_string.join("\n");

    (axis_string, longest_y_label_width as i32)
}

pub fn render_x_axis_strings(x_axis: &axis::ContinuousAxis, face_width: u32) -> (String, i32) {
    // Get the strings and offsets we'll use for the x-axis
    let x_tick_map = tick_offset_map(x_axis, face_width as u32);

    // Create a string which will be printed to give the x-axis tick marks
    let x_axis_tick_string: String = (0..=face_width)
        .map(|cell| match x_tick_map.get(&(cell as i32)) {
            Some(_) => '|',
            None => ' ',
        }).collect();

    // Create a string which will be printed to give the x-axis labels
    let x_labels = create_x_axis_labels(&x_tick_map);
    let start_offset = x_labels
        .iter()
        .map(|label| label.start_offset())
        .min()
        .expect("ERROR: Could not compute start offset of x-axis");

    // This string will be printed, starting at start_offset relative to the x-axis zero cell
    let mut x_axis_label_string = "".to_string();
    for label in (&x_labels).iter() {
        let spaces_to_append =
            label.start_offset() - start_offset - x_axis_label_string.len() as i32;
        if spaces_to_append.is_positive() {
            for _ in 0..spaces_to_append {
                x_axis_label_string.push(' ');
            }
        } else {
            for _ in 0..spaces_to_append.wrapping_neg() {
                x_axis_label_string.pop();
            }
        }
        let formatted_label = format!("{: ^footprint$}", label.text, footprint = label.footprint());
        x_axis_label_string.push_str(&formatted_label);
    }

    // Generate a list of strings to be the y-axis line itself
    let x_axis_line_string: String = std::iter::repeat('+')
        .take(1)
        .chain(std::iter::repeat('-').take(face_width as usize))
        .collect();

    let x_axis_label = format!(
        "{: ^width$}",
        x_axis.get_label(),
        width = face_width as usize
    );

    let x_axis_string = if start_offset.is_positive() {
        let padding = (0..start_offset).map(|_| " ").collect::<String>();
        format!(
            "{}\n{}\n{}{}\n{}",
            x_axis_line_string, x_axis_tick_string, padding, x_axis_label_string, x_axis_label
        )
    } else {
        let padding = (0..start_offset.wrapping_neg())
            .map(|_| " ")
            .collect::<String>();
        format!(
            "{}{}\n{}{}\n{}\n{}{}",
            padding,
            x_axis_line_string,
            padding,
            x_axis_tick_string,
            x_axis_label_string,
            padding,
            x_axis_label
        )
    };

    (x_axis_string, start_offset)
}

/// Given a histogram,
/// the x ands y-axes
/// and the face height and width,
/// create the strings to be drawn as the face
pub fn render_face_bars(
    h: &repr::Histogram,
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: u32,
    face_height: u32,
) -> String {
    let bound_cells = bound_cell_offsets(h, x_axis, face_width);

    let cell_bins = bins_for_cells(&bound_cells, face_width);

    // counts per bin converted to rows per column
    let cell_heights: Vec<_> = cell_bins
        .iter()
        .map(|&bin| match bin {
            None => 0,
            Some(b) => value_to_axis_cell_offset(h.get_values()[b as usize], y_axis, face_height),
        }).collect();

    let mut face_strings: Vec<String> = vec![];

    for line in 1..=face_height  {
        let mut line_string = String::new();
        for column in 1..=face_width as usize {
            // maybe use a HashSet for faster `contains()`?
            line_string.push(if bound_cells.contains(&(column as i32)) {
                // The value of the column _below_ this one
                let b = cell_heights[column - 1].cmp(&(line as i32));
                // The value of the column _above_ this one
                let a = cell_heights[column + 1].cmp(&(line as i32));
                match b {
                    std::cmp::Ordering::Less => {
                        match a {
                            std::cmp::Ordering::Less => ' ',
                            std::cmp::Ordering::Equal => '-', // or 'r'-shaped corner
                            std::cmp::Ordering::Greater => '|',
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        match a {
                            std::cmp::Ordering::Less => '-',    // or backwards 'r'
                            std::cmp::Ordering::Equal => '-',   // or 'T'-shaped
                            std::cmp::Ordering::Greater => '|', // or '-|'
                        }
                    }
                    std::cmp::Ordering::Greater => {
                        match a {
                            std::cmp::Ordering::Less => '|',
                            std::cmp::Ordering::Equal => '|', // or '|-'
                            std::cmp::Ordering::Greater => '|',
                        }
                    }
                }
            } else {
                let bin_height_cells = cell_heights[column];

                if bin_height_cells == line as i32 {
                    '-' // bar cap
                } else {
                    ' ' //
                }
            });
        }
        face_strings.push(line_string);
    }
    let face_strings: Vec<String> = face_strings.iter().rev().cloned().collect();
    face_strings.join("\n")
}

/// Given a scatter plot,
/// the x ands y-axes
/// and the face height and width,
/// create the strings to be drawn as the face
pub fn render_face_points(
    s: &[(f64, f64)],
    x_axis: &axis::ContinuousAxis,
    y_axis: &axis::ContinuousAxis,
    face_width: u32,
    face_height: u32,
    style: &style::PointStyle,
) -> String
{
    let points: Vec<_> = s
        .iter()
        .map(|&(x, y)| {
            (
                value_to_axis_cell_offset(x, x_axis, face_width),
                value_to_axis_cell_offset(y, y_axis, face_height),
            )
        }).collect();

    let marker = match style.get_marker().clone().unwrap_or(style::PointMarker::Circle) {
        style::PointMarker::Circle => '●',
        style::PointMarker::Square => '■',
        style::PointMarker::Cross => '×',
    };

    let mut face_strings: Vec<String> = vec![];
    for line in 1..=face_height {
        let mut line_string = String::new();
        for column in 1..=face_width as usize {
            line_string.push(if points.contains(&(column as i32, line as i32)) {
                marker
            } else {
                ' '
            });
        }
        face_strings.push(line_string);
    }
    let face_strings: Vec<String> = face_strings.iter().rev().cloned().collect();
    face_strings.join("\n")
}

/// Given two 'rectangular' strings, overlay the second on the first offset by `x` and `y`
pub fn overlay(under: &str, over: &str, x: i32, y: i32) -> String {
    let split_under: Vec<_> = under.split('\n').collect();
    let under_width = split_under.iter().map(|s| s.len()).max().unwrap();
    let under_height = split_under.len();

    let split_over: Vec<String> = over.split('\n').map(|s| s.to_string()).collect();
    let over_width = split_over.iter().map(|s| s.len()).max().unwrap();

    // Take `over` and pad it so that it matches `under`'s dimensions

    // Trim/add lines at beginning
    let split_over: Vec<String> = if y.is_negative() {
        split_over.iter().skip(y.abs() as usize).cloned().collect()
    } else if y.is_positive() {
        (0..y)
            .map(|_| (0..over_width).map(|_| ' ').collect())
            .chain(split_over.iter().map(|s| s.to_string()))
            .collect()
    } else {
        split_over
    };

    // Trim/add chars at beginning
    let split_over: Vec<String> = if x.is_negative() {
        split_over
            .iter()
            .map(|l| l.chars().skip(x.abs() as usize).collect())
            .collect()
    } else if x.is_positive() {
        split_over
            .iter()
            .map(|s| (0..x).map(|_| ' ').chain(s.chars()).collect())
            .collect()
    } else {
        split_over
    };

    // pad out end of vector
    let over_width = split_over.iter().map(|s| s.len()).max().unwrap();
    let over_height = split_over.len();
    let lines_deficit = under_height as i32 - over_height as i32;
    let split_over: Vec<String> = if lines_deficit.is_positive() {
        let new_lines: Vec<String> = (0..lines_deficit)
            .map(|_| (0..over_width).map(|_| ' ').collect::<String>())
            .collect();
        let mut temp = split_over.clone();
        for new_line in new_lines {
            temp.push(new_line);
        }
        temp
    } else {
        split_over
    };

    // pad out end of each line
    let line_width_deficit = under_width as i32 - over_width as i32;
    let split_over: Vec<String> = if line_width_deficit.is_positive() {
        split_over
            .iter()
            .map(|l| {
                l.chars()
                    .chain((0..line_width_deficit).map(|_| ' '))
                    .collect()
            }).collect()
    } else {
        split_over
    };

    // Now that the dimensions match, overlay them
    let mut out: Vec<String> = vec![];
    for (l, ol) in split_under.iter().zip(split_over.iter()) {
        let mut new_line = "".to_string();
        for (c, oc) in l.chars().zip(ol.chars()) {
            new_line.push(if oc == ' ' { c } else { oc });
        }
        out.push(new_line);
    }

    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bins_for_cells() {
        let face_width = 10;
        let n = i32::max_value();
        let run_bins_for_cells = |bound_cell_offsets: &[i32]| -> Vec<_> {
            bins_for_cells(&bound_cell_offsets, face_width)
                .iter()
                .map(|&a| a.unwrap_or(n))
                .collect()
        };

        assert_eq!(
            run_bins_for_cells(&vec![-4, -1, 4, 7, 10]),
            [1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 3, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![0, 2, 4, 8, 10]),
            [n, 0, 0, 1, 1, 2, 2, 2, 2, 3, 3, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![3, 5, 7, 9, 10]),
            [n, n, n, n, 0, 0, 1, 1, 2, 2, 3, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![0, 2, 4, 6, 8]),
            [n, 0, 0, 1, 1, 2, 2, 3, 3, n, n, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![0, 3, 6, 9, 12]),
            [n, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3]
        );

        assert_eq!(
            run_bins_for_cells(&vec![-5, -4, -3, -1, 0]),
            [3, n, n, n, n, n, n, n, n, n, n, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![10, 12, 14, 16, 18]),
            [n, n, n, n, n, n, n, n, n, n, n, 0]
        );

        assert_eq!(
            run_bins_for_cells(&vec![15, 16, 17, 18, 19]),
            [n, n, n, n, n, n, n, n, n, n, n, n]
        );
        assert_eq!(
            run_bins_for_cells(&vec![-19, -18, -17, -16, -1]),
            [n, n, n, n, n, n, n, n, n, n, n, n]
        );
    }

    #[test]
    fn test_value_to_axis_cell_offset() {
        assert_eq!(
            value_to_axis_cell_offset(3.0, &axis::ContinuousAxis::new(5.0, 10.0, 6), 10),
            -4
        );
    }

    #[test]
    fn test_x_axis_label() {
        let l = XAxisLabel {
            text: "3".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 1);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 2);

        let l = XAxisLabel {
            text: "34".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 2);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 1);

        let l = XAxisLabel {
            text: "345".to_string(),
            offset: 2,
        };
        assert_eq!(l.len(), 3);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), 1);

        let l = XAxisLabel {
            text: "3454".to_string(),
            offset: 1,
        };
        assert_eq!(l.len(), 4);
        assert!(l.footprint() % 2 != 0);
        assert_eq!(l.start_offset(), -1);
    }

    #[test]
    fn test_render_y_axis_strings() {
        let y_axis = axis::ContinuousAxis::new(0.0, 10.0, 6);

        let (y_axis_string, longest_y_label_width) = render_y_axis_strings(&y_axis, 10);

        assert!(y_axis_string.contains(&"0".to_string()));
        assert!(y_axis_string.contains(&"6".to_string()));
        assert!(y_axis_string.contains(&"10".to_string()));
        assert_eq!(longest_y_label_width, 2);
    }

    #[test]
    fn test_render_x_axis_strings() {
        let x_axis = axis::ContinuousAxis::new(0.0, 10.0, 6);

        let (x_axis_string, start_offset) = render_x_axis_strings(&x_axis, 20);

        assert!(x_axis_string.contains("0 "));
        assert!(x_axis_string.contains(" 6 "));
        assert!(x_axis_string.contains(" 10"));
        assert_eq!(x_axis_string.chars().filter(|&c| c == '|').count(), 6);
        assert_eq!(start_offset, 0);
    }

    #[test]
    fn test_render_face_bars() {
        let data = vec![0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
        let h = repr::Histogram::from_slice(&data, repr::HistogramBins::Count(10));
        let x_axis = axis::ContinuousAxis::new(0.3, 7.5, 6);
        let y_axis = axis::ContinuousAxis::new(0., 3., 6);
        let strings = render_face_bars(&h, &x_axis, &y_axis, 20, 10);
        assert_eq!(strings.lines().count(), 10);
        assert!(strings.lines().all(|s| s.chars().count() == 20));

        let comp = vec![
            "       ---          ",
            "       | |          ",
            "       | |          ",
            "--     | |          ",
            " |     | |          ",
            " |     | |          ",
            " |     | |          ",
            " |     | |---- -----",
            " |     | | | | | | |",
            " |     | | | | | | |",
        ].join("\n");

        assert_eq!(&strings, &comp);
    }

    #[test]
    fn test_render_face_points() {
        use crate::repr;
        use crate::style::PointStyle;
        let data = vec![
            (-3.0, 2.3),
            (-1.6, 5.3),
            (0.3, 0.7),
            (4.3, -1.4),
            (6.4, 4.3),
            (8.5, 3.7),
        ];
        let s = repr::Scatter::from_slice(&data);
        let x_axis = axis::ContinuousAxis::new(-3.575, 9.075, 6);
        let y_axis = axis::ContinuousAxis::new(-1.735, 5.635, 6);
        let style = PointStyle::new();
        let strings = render_face_points(&s.data, &x_axis, &y_axis, 20, 10, &style);
        assert_eq!(strings.lines().count(), 10);
        assert!(strings.lines().all(|s| s.chars().count() == 20));

        let comp = vec![
            "  ●                 ",
            "                    ",
            "               ●    ",
            "                  ● ",
            "                    ",
            "●                   ",
            "                    ",
            "     ●              ",
            "                    ",
            "                    ",
        ].join("\n");

        assert_eq!(&strings, &comp);
    }

    #[test]
    fn test_overlay() {
        let a = " ooo ";
        let b = "  #  ";
        let r = " o#o ";
        assert_eq!(overlay(a, b, 0, 0), r);

        let a = " o o o o o o o o o o ";
        let b = "# # # # #";
        let r = " o#o#o#o#o#o o o o o ";
        assert_eq!(overlay(a, b, 2, 0), r);

        let a = "     \n   o \n o  o\nooooo\no o o";
        let b = "  #  \n   # \n     \n  ## \n   ##";
        let r = "  #  \n   # \n o  o\noo##o\no o##";
        assert_eq!(overlay(a, b, 0, 0), r);

        let a = "     \n   o \n o  o\nooooo\no o o";
        let b = "  #\n## ";
        let r = "     \n   o \n o #o\no##oo\no o o";
        assert_eq!(overlay(a, b, 1, 2), r);

        let a = "     \n   o \n o  o\nooooo\no o o";
        let b = "###\n###\n###";
        let r = "##   \n## o \n o  o\nooooo\no o o";
        assert_eq!(overlay(a, b, -1, -1), r);

        let a = "oo\noo";
        let b = "    \n  # \n #  \n    ";
        let r = "o#\n#o";
        assert_eq!(overlay(a, b, -1, -1), r);
    }
}
