//!

use opencv::{
    highgui,
    calib3d,
    core::{
        hconcat,
        vconcat,
        invert,
        DecompTypes,
        Mat,
        Point2f,
        Point3f,
        Point2i,
        Scalar,
        Size,
        CV_64F,
    },
    imgproc,
    prelude::*,
    types::{
        VectorOfPoint3f,
        VectorOfPoint2f,
        VectorOfMat,
    },
    videoio,
    Result,
};

use vision::{
    calibration::CameraCalibration,
    tracking::Tracker,
};

fn main() -> Result<()> {
    let window = "debug view";
    highgui::named_window(window, highgui::WINDOW_NORMAL)
        .expect("Failed to create debug window!");

    let mut file = videoio::VideoCapture::from_file("assets/office_calib_iphone/orbit_left_right.mov", videoio::CAP_ANY)?;

    if !videoio::VideoCapture::is_opened(&file)? {
        panic!("Unable to open video file!");
    }

    let calib = CameraCalibration::from_file(String::from("./assets/office_calib_iphone/orbit_left_right.yaml"))
        .expect("Failed to open calibration file.");
    let k = calib.k();
    let dist_coeffs = calib.dist_coeffs();

    // Instrinsic parameters become just another 4x4 matrix
    let mut k2 = Mat::eye(4, 4, CV_64F)
        .unwrap()
        .to_mat()
        .unwrap();

    *k2.at_2d_mut(0, 0).unwrap() = calib.fx;
    *k2.at_2d_mut(1, 1).unwrap() = calib.fy;
    *k2.at_2d_mut(0, 2).unwrap() = calib.cx;
    *k2.at_2d_mut(1, 2).unwrap() = calib.cy;

    let tracker = Tracker::new(calib);

    let points = VectorOfPoint3f::from_slice(
        (0..9)
            .map(|x| Point3f::new(x as f32, x as f32, 0.0))
            .collect::<Vec<Point3f>>()
            .as_slice());

    loop {
        if highgui::wait_key(10)? > 0 { break; }

        let mut frame = Mat::default();
        if !file.read(&mut frame).expect("Failed to read next frame.") {
            break;
        }

        let mut tmp = Mat::default();
        let mut frame_gray = Mat::default();
        imgproc::cvt_color(&frame, &mut tmp, imgproc::COLOR_BGR2GRAY, 0).unwrap();  // color -> grayscale
        imgproc::cvt_color(&tmp, &mut frame_gray, imgproc::COLOR_GRAY2BGR, 0).unwrap();   // technically still no color, but adds BGR channels back for colored drawing

        let mut tvec = Mat::default();
        let mut rvec = Mat::default();

        let mut debug_view = Mat::default();
        let mut _k = Mat::default();  // New camera matrix after undistort, useless to us
        calib3d::undistort(&frame_gray, &mut debug_view, &k, &dist_coeffs, &mut _k)
            .unwrap();

        if tracker.track(&frame, &mut rvec, &mut tvec) {
            // rotation vector -> 3x3 rotation matrix
            let mut rmat = Mat::default();
            let mut jacobian = Mat::default();
            calib3d::rodrigues(&rvec, &mut rmat, &mut jacobian).unwrap();

            // Will eventually be 4x4, see below
            let mut rt = Mat::default();

            // [ r11 r12 r13 | tx ]
            // [ r21 r22 r23 | ty ]
            // [ r31 r32 r33 | tz ]
            // ...
            let mut matrices = VectorOfMat::new();
            matrices.push(rmat);
            matrices.push(tvec.clone());
            hconcat(&matrices, &mut rt).unwrap();

            // ...
            // [ 0   0   0   | 1  ]
            let mut matrices = VectorOfMat::new();
            matrices.push(rt.clone());
            matrices.push(Mat::from_slice(&[0.0, 0.0, 0.0, 1.0]).unwrap());
            vconcat(&matrices, &mut rt).unwrap();

            // for each point (x,y,z), do:
            //
            // [ fx 0  cx 0 ] [ r11 r12 r13 tx ]   [x]
            // [ 0  fy cy 0 ] [ r21 r22 r23 ty ]   [y]
            // [ 0  0  1  0 ] [ r31 r32 r33 tz ] * [z]
            // [ 0  0  0  1 ] [ 0   0   0   1  ]   [1]
            let mut pts1 = VectorOfPoint2f::new();
            for pt in &points {
                // Point3f -> Mat
                let mut pt_mat = Mat::zeros(4, 1, CV_64F)
                    .unwrap()
                    .to_mat()
                    .unwrap();

                // euclidian -> homogenous
                *pt_mat.at_2d_mut(0, 0).unwrap() = pt.x as f64;
                *pt_mat.at_2d_mut(1, 0).unwrap() = pt.y as f64;
                *pt_mat.at_2d_mut(2, 0).unwrap() = pt.z as f64;
                *pt_mat.at_2d_mut(3, 0).unwrap() = 1.0;


                // 3d -> 2d. Result will be 4x1 vector, but we can ignore its w
                // component during euclidian conversion as it algebraically
                // does not matter.
                let p = (&k2 * &rt * &pt_mat)
                    .into_result()
                    .unwrap()
                    .to_mat()
                    .unwrap();

                // homogenous -> euclidian
                let z = *p.at_2d::<f64>(2, 0).unwrap() as f32;
                let p = Point2f::new(
                        *p.at_2d::<f64>(0, 0).unwrap() as f32 / z,
                        *p.at_2d::<f64>(1, 0).unwrap() as f32 / z);

                pts1.push(p);
            }

            // OpenCV 3d -> 2d projection
            let mut jacobian = Mat::default();
            let mut pts2 = VectorOfPoint2f::new();

            calib3d::project_points(&points, &rvec, &tvec, &k, &dist_coeffs,
                    &mut pts2, &mut jacobian, 0.0)
                .expect("Failed to project automatically undistorted points.");

            // Visually compare point positions and display any drift
            for (pt1, pt2) in pts1.iter().zip(pts2.iter()) { 
                draw_comparison(&mut debug_view, pt1, pt2);
            }

            // Also draw axis, to sanity-check tvec and rvec
            draw_axis(&mut debug_view, &rvec, &tvec, calib);
        }


        let aspect_ratio = debug_view.rows() as f32 / debug_view.cols() as f32;
        let width = 1500.0;
        highgui::resize_window(window, width as i32, (width * aspect_ratio) as i32)
            .expect("Failed to resize window.");

        highgui::imshow(window, &debug_view)?;
    }

    Ok(())
}

fn draw_comparison(
    image: &mut Mat,
    p1: Point2f,
    p2: Point2f,
) {
    let color_red = Scalar::new(0.0, 0.0, 255.0, 255.0);
    let color_blue = Scalar::new(255.0, 0.0, 0.0, 255.0);
    let color_green = Scalar::new(0.0, 255.0, 0.0, 255.0);

    //let p2 = p2 + Point2f::new(2.0, 2.0);

    draw_circle(image, p2, 6, color_red);
    draw_dot(image, p1, color_green);
    draw_line(image, p1, p2, color_blue);
}

fn draw_circle(
    image: &mut Mat,
    point: Point2f,
    radius: i32,
    color: Scalar
) {
    imgproc::circle(image, Point2i::new(point.x as i32, point.y as i32),
            radius, color, 1, imgproc::LineTypes::LINE_AA as i32, 0)
        .expect("Failed to draw circle.");
}

fn draw_dot(
    image: &mut Mat,
    point: Point2f,
    color: Scalar
) {

    imgproc::circle(image,
            Point2i::new(point.x as i32, point.y as i32), 1,
            color, 3, imgproc::LineTypes::FILLED as i32, 0)
        .expect("Failed to draw dot.");
}

fn draw_line(
    image: &mut Mat,
    p1: Point2f,
    p2: Point2f,
    color: Scalar
) {
    imgproc::line(image, Point2i::new(p1.x as i32, p1.y as i32),
            Point2i::new(p2.x as i32, p2.y as i32), color, 1,
            imgproc::LineTypes::LINE_AA as i32, 0)
        .expect("Failed to draw line.")
}

fn draw_axis(
    image: &mut Mat,
    r: &Mat,
    t: &Mat,
    calib: CameraCalibration,
) {
    // Define the virual 3d axis lines.
    let points = VectorOfPoint3f::from_slice(&[
        Point3f::new(1.0, 0.0, 0.0),  // [0]: x
        Point3f::new(0.0, 1.0, 0.0),  // [1]: y
        Point3f::new(0.0, 0.0, 1.0),  // [2]: z
        Point3f::new(0.0, 0.0, 0.0),  // [3]: origin
    ]);

    let mut image_points = VectorOfPoint2f::new();
    let mut jacobian = Mat::default();
    calib3d::project_points(&points, &r, &t, &calib.k(), &calib.dist_coeffs(),
            &mut image_points, &mut jacobian, 0.0)
        .unwrap();
    // TODO: inverse of PnP?? Make a test program with artificial data.

    // Palette
    let r = Scalar::new(255.0, 0.0, 0.0, 1.0);
    let g = Scalar::new(0.0, 255.0, 0.0, 1.0);
    let b = Scalar::new(0.0, 0.0, 255.0, 1.0);

    // Points
    let image_points = image_points.as_slice();
    let p0 = image_points[0];
    let p1 = image_points[1];
    let p2 = image_points[2];
    let p3 = image_points[3];

    let x = Point2i::new(p0.x as i32, p0.y as i32);
    let y = Point2i::new(p1.x as i32, p1.y as i32);
    let z = Point2i::new(p2.x as i32, p2.y as i32);
    let origin = Point2i::new(p3.x as i32, p3.y as i32);

    imgproc::line(image, origin, x, r, 3, 0, 0).unwrap();
    imgproc::line(image, origin, y, g, 3, 0, 0).unwrap();
    imgproc::line(image, origin, z, b, 3, 0, 0).unwrap();
}