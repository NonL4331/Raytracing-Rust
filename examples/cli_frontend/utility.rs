extern crate chrono;
extern crate cpu_raytracer;
extern crate image;

use crate::parameters::Parameters;
use chrono::Local;
use cpu_raytracer::acceleration::bvh::Bvh;
use cpu_raytracer::material::Scatter;
use cpu_raytracer::ray_tracing::intersection::Primitive;
use cpu_raytracer::*;
use std::{
    convert::TryInto,
    io::{stdout, Write},
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

const BAR_LENGTH: u32 = 30;

pub fn progress_bar(percentage: f64) {
    print!("\r[");
    for i in 1..=BAR_LENGTH {
        if percentage >= i as f64 / BAR_LENGTH as f64
            && percentage < (i + 1) as f64 / BAR_LENGTH as f64
            && i != BAR_LENGTH
        {
            print!(">");
        } else if percentage >= i as f64 / BAR_LENGTH as f64 {
            print!("=");
        } else {
            print!("-");
        }
    }
    print!("]");
}

pub fn line_break() {
    println!("--------------------------------");
}

pub fn get_readable_duration(duration: Duration) -> String {
    let days = duration.as_secs() / 86400;

    let days_string = match days {
        0 => "".to_string(),
        1 => format!("{} day, ", days),
        _ => format!("{} days, ", days),
    };

    let hours = (duration.as_secs() - days * 86400) / 3600;
    let hours_string = match hours {
        0 => "".to_string(),
        1 => format!("{} hour, ", hours),
        _ => format!("{} hours, ", hours),
    };

    let minutes = (duration.as_secs() - days * 86400 - hours * 3600) / 60;
    let minutes_string = match minutes {
        0 => "".to_string(),
        1 => format!("{} minute, ", minutes),
        _ => format!("{} minutes, ", minutes),
    };

    let seconds = duration.as_secs() % 60;
    let seconds_string = match seconds {
        0 => "~0 seconds".to_string(),
        1 => format!("{} second", seconds),
        _ => format!("{} seconds", seconds),
    };
    days_string + &hours_string + &minutes_string + &seconds_string
}

pub fn save_u8_to_image(width: u64, height: u64, image: Vec<u8>, filename: String) {
    image::save_buffer(
        filename,
        &image,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        image::ColorType::Rgb8,
    )
    .unwrap();
}

pub fn get_progress_output(
    options: &Parameters,
    progresses: &Vec<Arc<RwLock<SamplerProgress>>>,
) -> Vec<u8> {
    let mut exit = false;
    while !exit {
        let mut samples_sum = 0;
        for progress in progresses.iter() {
            samples_sum += progress.read().unwrap().samples_completed;
        }

        progress_bar(samples_sum as f64 / options.samples as f64);
        print!(" ({}/{}) samples", samples_sum, options.samples);
        stdout().flush().unwrap();

        if samples_sum == options.samples {
            exit = true;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let progresses: Vec<Vec<Float>> = progresses
        .iter()
        .map(|prog| prog.read().unwrap().current_image.clone())
        .collect();

    let image = progresses.iter().fold(
        vec![0.0; (options.width * options.height * 3) as usize],
        |acc, image| acc.iter().zip(image).map(|(&a, &b)| a + b).collect(),
    );
    let image: Vec<Float> = image
        .iter()
        .map(|pixel_val| pixel_val / progresses.len() as Float)
        .collect();

    image
        .iter()
        .map(|value| (value.sqrt() * 255.0) as u8)
        .collect()
}

pub fn create_bvh_with_info<P: Primitive<M>, M: Scatter>(
    primitives: Vec<P>,
    bvh_type: SplitType,
) -> Arc<Bvh<P, M>> {
    let time = Local::now();

    println!("\n{} - Bvh construction started at", time.format("%X"));

    let start = Instant::now();
    let bvh = bvh!(primitives, bvh_type);
    let end = Instant::now();
    let duration = end.checked_duration_since(start).unwrap();

    println!("\tBvh construction finished in: {}ms", duration.as_millis());
    println!("\tNumber of BVH nodes: {}\n", bvh.number_nodes());

    bvh
}
