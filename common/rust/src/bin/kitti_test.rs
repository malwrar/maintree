use malicious::{
    parsing::kitti::KittiParser,
};

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    log::info!("Starting up...");
    log::warn!("Starting up...");
    log::debug!("Starting up...");
    log::error!("Starting up...");

    let parser = KittiParser::new(&[
            "../../../../datasets/kitti/2011_09_29_calib.zip",
            //"../../../../datasets/kitti/2011_09_29_drive_0004_extract.zip",
            "../../../../datasets/kitti/2011_09_29_drive_0004_sync.zip",
        ]);
}
