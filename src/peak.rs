use plotters::prelude::*;
use std::fs::File;
use std::io::{BufWriter, Read, Write};

#[allow(dead_code)]
pub fn find_peak_and_plot(name: &String, peak_width: usize, threshold: f64, start_index: usize, mirror: u64, ray: usize) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut data = String::new();
    let mut input = String::from("./data/");
    input.push_str(name);
    input.push_str(".txt");
    let mut f = File::open(&input)?;
    f.read_to_string(&mut data)?;

    let mut ymax = 0.0;
    let data: Vec<(f64, f64)> = data.split('\n').flat_map(|str| {
        let (theta, count) = str.split_once(',').unwrap_or(("", ""));
        match theta.trim().parse() {
            Ok(i) => match count.trim().parse() {
                Ok(j) => {
                    if ymax < j {
                        ymax = j;
                    }
                    Ok((i, j))
                }
                Err(_) => Err("not a number")
            }
            Err(_) => Err("not a number")
        }
    }).collect();

    let mut diff: Vec<f64> = Vec::new();
    for i in 0..(data.len() - 1) {
        diff.push(data[i + 1].1 - data[i].1)
    }
    
    let mut index: Vec<usize> = Vec::new();
    for i in 0..(diff.len() - 1) {
        if diff[i + 1] <= 0.0 && diff[i] >= 0.0 {
            let background = (0..peak_width).map(|j|
                if i > j + peak_width / 2 {
                    data[i - j - peak_width / 2].1 / peak_width as f64
                } else {
                    0.0
                }
            ).sum::<f64>().min(
                (0..peak_width).map(|j|
                    if i + j + peak_width / 2 < data.len() {
                        data[i + j + peak_width / 2].1 / peak_width as f64
                    } else {
                        0.0
                    }
                ).sum::<f64>(),
            );
            if data[i + 1].1 / background > threshold {
                index.push(i + 1);
            }
        }
    }
    /*
    let mut index: Vec<f64> = Vec::new();
    for i in 0..(diff.len() - 1) {
        index.push(diff[i + 1] * diff[i]);
    }
    let index: Vec<usize> = index.into_iter().enumerate().filter(|(i, v)| v < &0.0 && data[*i + 1].1 > threshold).map(|(i, _)| i + 1).collect();
    */

    let mut peaks: Vec<(f64, f64)> = Vec::new();

    /**/
    peaks.push(data[index[0]]);
    for k in 1..index.len()-1 {
        if index[k - 1] + peak_width > index[k] && index[k] + peak_width > index[k + 1] {
            if data[index[k - 1]].1 < data[index[k]].1 && data[index[k]].1 >= data[index[k + 1]].1 {
                peaks.push(data[index[k]]);
            }
        } else if index[k - 1] + peak_width > index[k] && !(index[k] + peak_width > index[k + 1]) {
            if data[index[k - 1]].1 < data[index[k]].1 {
                peaks.push(data[index[k]]);
            }
        } else if !(index[k - 1] + peak_width > index[k]) && index[k] + peak_width > index[k + 1] {
            if data[index[k]].1 >= data[index[k + 1]].1 {
                peaks.push(data[index[k]]);
            }
        } else {
            peaks.push(data[index[k]]);
        }
    }
    /**/
    /*
    for i in index {
        peaks.push(data[i]);
    }
    */

    let mut peak_log = String::from("./output/peak/log_");
    peak_log.push_str(name);
    peak_log.push_str(".txt");
    let mut writer = BufWriter::new(File::create(&peak_log)?);
    let a = vec![
        1.54178 / 2.0 / (peaks[start_index].0 * std::f64::consts::PI / 360.0).sin() * (mirror as f64).sqrt(),
        1.54050 / 2.0 / (peaks[start_index].0 * std::f64::consts::PI / 360.0).sin() * (mirror as f64).sqrt(),
        1.54433 / 2.0 / (peaks[start_index].0 * std::f64::consts::PI / 360.0).sin() * (mirror as f64).sqrt()
    ];

    let lattice = format!(
        "peak width: {}, threshold: {} -> peak No.{} represents mirror index (h, k, l) which satisfy sqrt(h^2 + k^2 + l^2) = {} -> lattice constant: {}\n",
        peak_width,
        threshold,
        start_index + 1,
        mirror,
        a[ray]
    );
    writer.write_all(lattice.as_bytes())?;

    for p in peaks.iter() {
        let sint = (p.0 * std::f64::consts::PI / 360.0).sin();
        let d_0 = 1.54178 / 2.0 / sint;
        let d_1 = 1.54050 / 2.0 / sint;
        let d_2 = 1.54433 / 2.0 / sint;
        let m_0 = (a[ray] / d_0).powi(2).round();
        let m_1 = (a[ray] / d_1).powi(2).round();
        let m_2 = (a[ray] / d_2).powi(2).round();
        
        let out_str = format!(
            "{} & {} & {} & {} & {} & {} & {} & {} & {} & {} & {} & {} \\\\\n",
            p.0,
            sint,
            p.1,
            d_0,
            d_1,
            d_2,
            m_0,
            m_1,
            m_2,
            d_0 * m_0.sqrt(),
            d_1 * m_1.sqrt(),
            d_2 * m_2.sqrt()
        );
        writer.write_all(out_str.as_bytes())?;
    }

    let mut output = String::from("./output/peak/");
    output.push_str(name);
    output.push_str(".png");

    let root = BitMapBackend::new(&output, (2048, 1536)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut cc = ChartBuilder::on(&root)
        .set_all_label_area_size(50)
        .build_cartesian_2d(9.9..120.1, 0.0..ymax * 1.01)?;

    cc.configure_mesh()
        .draw()?;

    cc.draw_series(PointSeries::of_element(
        peaks.into_iter(), 
        5, 
        &RED,
        &|c, s, st| {
            EmptyElement::at(c) + Circle::new((0, 0), s, st.filled()) + Text::new(format!("{}, {}", c.0, c.1), (5, -5), ("sans-serif", 20))
        }
    ))?;
    
    cc.draw_series(LineSeries::new(
        data.into_iter(), 
        &BLUE
    ))?;

    Ok(())
}