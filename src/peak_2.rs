use std::fs::File;
use std::io::{BufWriter, Write};

#[allow(dead_code)]
pub fn find_peak_and_plot() -> Result<(), Box<dyn std::error::Error>> {

    let peaks: Vec<(f64, f64)> = vec![
        (21.92, 4325.0),
        (31.05, 10275.0),
        (38.17,1783.33),
        (44.3, 358.333),
        (49.8, 2100.0),
        (54.99, 2516.67),
        (64.22, 1091.67),
        (68.6, 916.667),
        (72.87, 1491.67),
        (77.01, 625.0),
        (81.11, 516.667),
        (85.17, 450.0),
        (89.09, 1225.0),
        (97.25, 333.333),
        (101.16, 550.0),
        (105.26, 783.333),
        (109.43, 400.0),
        (113.8, 666.667),
        (118.2, 550.0),
    ];

    let mut writer = BufWriter::new(File::create("./output/peak/log_koyoutai.txt")?);
    let a = 1.54178 / 2.0 / (peaks[0].0 * std::f64::consts::PI / 360.0).sin();

    for p in peaks.iter() {
        let sint = (p.0 * std::f64::consts::PI / 360.0).sin();
        let dsint = std::f64::consts::PI / 360.0 * (p.0 * std::f64::consts::PI / 360.0).cos() * 0.05;
        let d_0 = 1.54178 / 2.0 / sint;
        let d_1 = 1.54050 / 2.0 / sint;
        let d_2 = 1.54433 / 2.0 / sint;
        let m_0 = (a / d_0).powi(2).round();
        let m_1 = (a / d_1).powi(2).round();
        let m_2 = (a / d_2).powi(2).round();

        let out_str = format!(
            "{} & {} & {} & {} & {} & {} & {} & {} & {} & {} & {} & {} \\\\\n",
            p.0,
            return_uncertainty_form(sint, dsint),
            p.1,
            return_uncertainty_form(d_0, d_0 / sint * dsint),
            return_uncertainty_form(d_1, d_1 / sint * dsint),
            return_uncertainty_form(d_2, d_2 / sint * dsint),
            m_0,
            m_1,
            m_2,
            return_uncertainty_form(d_0 * m_0.sqrt(), d_0 / sint * dsint * m_0.sqrt()),
            return_uncertainty_form(d_1 * m_1.sqrt(), d_1 / sint * dsint * m_1.sqrt()),
            return_uncertainty_form(d_2 * m_2.sqrt(), d_2 / sint * dsint * m_2.sqrt())
        );
        writer.write_all(out_str.as_bytes())?;
    }

    Ok(())
}

fn return_uncertainty_form(value: f64, uncertainty: f64) -> String {
    if uncertainty.abs() != 0.0 {
        let count = -(uncertainty.abs().log10().floor()) as i32;
        if count >= 0 {
            let count = count as usize;
            format!("{:.*}+-{:.*}", count, value, count, uncertainty)
        } else {
            let dig = value.log10().floor() as usize;
            let count = - count as usize;
            format!("{:.*}+-{:.*}e{}", dig - count, value * 10_f64.powi(-(dig as i32)), dig - count, uncertainty * 10_f64.powi(-(dig as i32)), dig)
        }
    } else {
        String::from("ERR!!")
    }
}