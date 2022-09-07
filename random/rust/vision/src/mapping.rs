//!
//! 
use opencv::{
	prelude::*,
	core::{
		Point2f,
		Point2i,
		Point3d,
		Point3f,
		Scalar,
		Size,
		CV_64F,
		TermCriteria,
		TermCriteria_Type,
        Vector,
	},
	calib3d,
	highgui,
	imgproc,
	Result,
	types::{
		VectorOfPoint3f,
		VectorOfPoint2f,
		VectorOfVectorOfPoint3f,
		VectorOfVectorOfPoint2f,
	},
	videoio,
};

use petgraph::{
	Graph,
	graph::NodeIndex,
	//dot::Dot,
};

#[derive(Debug)]
pub struct Pose {
}

#[derive(Debug)]
enum MetricMapNode {
	KeyFrame(Pose),
	Point(Point3f),
	Plane,
}

#[derive(Debug)]
enum MetricMapEdge {
	KeyFrameObservation(Point2f),
	PlaneMembership,
}

pub struct MetricMap {
    map: Graph<MetricMapNode, MetricMapEdge>, // Stuff we've seen at the places we've been.
}

impl MetricMap {
    pub fn new() -> Self {
        Self {
            map: Graph::new(),
        }
    }
}