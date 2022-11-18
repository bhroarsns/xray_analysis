mod peak;
mod simple;
mod peak_2;

use std::env;
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let mode = match args[1].as_str() {
        "simple" => Some(true),
        "peak" => Some(false),
        _ => None
    };

    match mode {
        Some(b) => if b {
            let data_name = &args[2];
            let xstart = args[3].trim().parse::<f64>().unwrap();
            let xend = args[4].trim().parse::<f64>().unwrap();
            let ystart = args[5].trim().parse::<f64>().unwrap();
            let yend = args[6].trim().parse::<f64>().unwrap();

            simple::simple_plot(data_name, Range{start: xstart, end: xend}, Range{start: ystart, end:yend})?;
        } else {
            let data_name = &args[2];
            let peak_width = args[3].trim().parse::<usize>().unwrap();
            let threshold = args[4].trim().parse::<f64>().unwrap();
            let start_index = args[5].trim().parse::<usize>().unwrap();
            let mirror = args[6].trim().parse::<u64>().unwrap();
            let ray = args[7].trim().parse::<usize>().unwrap();
            peak::find_peak_and_plot(data_name, peak_width, threshold, start_index, mirror, ray)?;
        }
        None => {
            peak_2::find_peak_and_plot()?;
        }
    }

    Ok(())
}
