/// Stores persistent slam state.

extern crate sqlite;

use sqlite;

struct SlamState {
    db: sqlite::Connection;
}

struct SlamStateData {

impl SlamState {
    pub fn new() -> Self {
        Self::open(":memory:")
    }

    pub fn open(path: String) -> Self {
        let db = sqlite::open(path).unwrap();

        //db.execute(
        //    "
        //    CREATE TABLE calibration
        //    CREATE TABLE keyframes

        //    CREATE TABLE keypoints
        //    CREATE TABLE keypoint_covisibilities (relate features to poses)

        //    CREATE TABLE markers
        //    CREATE TABLE marker_covisibilities (relate markers to poses)

        //    CREATE TABLE objects
        //    CREATE TABLE planes
        //    "
        //);

        Self {
            db
        }
    }
}