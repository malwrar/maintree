use std::env;
use malicious::kitti::OdometryDatasetExtractor;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];  // TODO: check arg quantity

    let mut kitti = OdometryDatasetExtractor::construct()
        .velodyne_zip(filepath)
        .build();
    log::info!("Loaded odometry dataset extractor");

    for capture in kitti.captures() {
        log::info!("{} {} {}",
            capture.sequence_num,
            capture.capture_num,
            capture.points.len());
    }
}
