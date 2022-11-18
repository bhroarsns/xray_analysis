use plotters::prelude::*;
use std::ops::Range;
use std::fs::File;
use std::io::Read;

#[allow(dead_code)]
pub fn simple_plot(name: &String, xrange: Range<f64>, yrange: Range<f64>) -> Result<(), Box<dyn std::error::Error>> {

    let mut input_name = String::from("./data/");
    input_name.push_str(name);
    input_name.push_str(".txt");

    let mut output_name = String::from("./output/simple/");
    output_name.push_str(name);
    output_name.push_str(".png");

    let mut data = String::new();
    let mut f = File::open(&input_name)?;
    f.read_to_string(&mut data)?;

    let data: Vec<(f64, f64)> = data.split("\n").flat_map(|str| {
        let (theta, count) = str.split_once(",").unwrap_or(("", ""));
        match theta.trim().parse::<f64>() {
            Ok(i) => match count.trim().parse::<f64>() {
                Ok(j) => Ok((i, j)),
                Err(_) => Err("not a number")
            }
            Err(_) => Err("not a number")
        }
    }).collect();

    let root = BitMapBackend::new(&output_name, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&root)
        .set_all_label_area_size(80)
        .build_cartesian_2d(xrange, yrange)?;
    
    cc.configure_mesh()
        .x_desc("2theta / degree")
        .y_desc("count")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;

    cc.draw_series(LineSeries::new(
        data.into_iter(),
        &BLUE
    ))?;

    Ok(())
}