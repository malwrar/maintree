use kdtree::{
    KdTree,
    distance::squared_euclidean,
};
use nalgebra as na;

type Point = [f32; 3];

#[derive(Debug)]
pub struct PointCloud {
    points: KdTree<f32, (Point, f32), Point>,
}

impl PointCloud {
    pub fn new() -> Self {
        Self {
            points: KdTree::new(3),
        }
    }

    pub fn size(&self) -> usize {
        self.points.size()
    }

    pub fn push(&mut self, point: Point, estimated_error: f32) {
        self.points.add(point, (point, estimated_error));
    }

    pub fn expand(&mut self, additional_points: &PointCloud) {

    }

    pub fn nearest<'a>(
        &'a self,
        point: &'a Point,
    ) -> impl Iterator<Item = (f32, Point, f32)> + 'a {
        self.points
            .iter_nearest(point, &squared_euclidean)
            .unwrap()
            .map(|(distance, (point, err))| (distance, *point, *err))
    }

    //pub fn within(
    //    &self,
    //    point: Point,
    //    radius: f32,
    //) -> Vec<(Point, f32)> {
    //    self.points
    //        .within(&point, radius, &squared_euclidean)
    //        .unwrap()
    //}
}