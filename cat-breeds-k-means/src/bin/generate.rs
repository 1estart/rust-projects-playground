use rusty_machine::prelude::Matrix;

extern crate rand;
extern crate rusty_machine;

use rusty_machine::linalg::BaseMatrix;

use rand::distributions::Distribution;
use rand::thread_rng; // for using .sample()
use rand_distr::Normal;
use serde::Deserialize;
use std::fs::read_to_string;
use std::io;
use std::vec::Vec;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short = "c", long = "config-file", parse(from_os_str))]
    /// Configuration file TOML
    config_file_path: std::path::PathBuf,
}

fn generate_data(centroids: &Matrix<f64>, points_per_centroid: usize, noise: f64) -> Matrix<f64> {
    assert!(centroids.cols() > 0, "centroids cannot be empty");
    assert!(centroids.rows() > 0, "centroids cannot be empty");
    assert!(noise >= 0f64, "noise must be non negative");
    let mut raw_cluster_data: Vec<f64> =
        Vec::with_capacity(centroids.rows() * points_per_centroid * centroids.cols());
    let mut rng = thread_rng();
    let normal_rv = Normal::new(0f64, noise).unwrap();

    for _ in 0..points_per_centroid {
        // generate points from each centroid
        for centroid in centroids.iter_rows() {
            // generate a point randomly around the centroid
            let mut point = Vec::with_capacity(centroids.cols());
            for feature in centroid.iter() {
                point.push(feature + normal_rv.sample(&mut rng));
            }
            // push point to raw cluster data
            raw_cluster_data.extend(point);
        }
    }
    Matrix::new(
        centroids.rows() * points_per_centroid,
        centroids.cols(),
        raw_cluster_data,
    )
}

#[derive(Deserialize)]
struct Config {
    centroids: [f64; 6],
    noise: f64,
    samples_per_centroid: usize,
}

fn main() -> Result<(), std::io::Error> {
    let options = Options::from_args();
    let toml_config_str = read_to_string(options.config_file_path)?;
    let config: Config = toml::from_str(&toml_config_str)?;

    let centroids = Matrix::new(3, 2, config.centroids.to_vec());

    let samples = generate_data(&centroids, config.samples_per_centroid, config.noise);

    let mut writer = csv::Writer::from_writer(io::stdout());

    writer.write_record(&["height", "length"])?;
    for sample in samples.iter_rows() {
        writer.serialize(sample)?;
    }
    Ok(())
}
