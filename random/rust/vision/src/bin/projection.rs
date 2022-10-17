//!

use opencv::{
    highgui,
    calib3d,
    core::{
        hconcat,
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
    pattern::{Chessboard, Pattern},
};

fn main() -> Result<()> {
    let window = "debug view";
    highgui::named_window(window, highgui::WINDOW_NORMAL)
        .expect("Failed to create debug window!");

    let mut file = videoio::VideoCapture::from_file("assets/office_calib_iphone/translate_left_right.mov", videoio::CAP_ANY)?;

    if !videoio::VideoCapture::is_opened(&file)? {
        panic!("Unable to open video file!");
    }

    let calib = CameraCalibration::from_file(String::from("./assets/office_calib_iphone/orbit_left_right.yaml"))
        .expect("Failed to open calibration file.");

    let tracker = Tracker::new(calib);

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

        if tracker.track(&frame, &mut rvec, &mut tvec) {
            let mut rmat = Mat::default();
            let mut jacobian = Mat::default();
            calib3d::rodrigues(&rvec, &mut rmat, &mut jacobian).unwrap();

            // [ r11 r12 r13 | tx ]
            // [ r21 r22 r23 | ty ]
            // [ r31 r32 r33 | tz ]
            let mut rt = Mat::default();
            let mut matrices = VectorOfMat::new();
            matrices.push(rmat);
            matrices.push(tvec.clone());
            hconcat(&matrices, &mut rt).unwrap();

            let points = VectorOfPoint3f::from_slice(&[
                Point3f::new(1.0, 1.0, 0.0),
                Point3f::new(2.0, 2.0, 0.0),
                Point3f::new(3.0, 3.0, 0.0),
                Point3f::new(4.0, 4.0, 0.0),
                Point3f::new(5.0, 5.0, 0.0),
            ]);
        
            let k = calib.k();
            let dist_coeffs = calib.dist_coeffs();

            let mut jacobian = Mat::default();
            let mut pts1 = VectorOfPoint2f::new();

            calib3d::project_points(&points, &rvec, &tvec, &k, &dist_coeffs,
                    &mut pts1, &mut jacobian, 0.0)
                .expect("Failed to project automatically undistorted points.");

            /*
            let points = points
                .iter()
                .map(|pt| {
                    let mut pt_mat = Mat::zeros(4, 1, CV_64F)
                        .unwrap()
                        .to_mat()
                        .unwrap();

                    *pt_mat.at_2d_mut(0, 0).unwrap() = pt.x as f64;
                    *pt_mat.at_2d_mut(1, 0).unwrap() = pt.y as f64;
                    *pt_mat.at_2d_mut(2, 0).unwrap() = pt.z as f64;
                    *pt_mat.at_2d_mut(3, 0).unwrap() = 1.0;


                    // TODO: do undistortion
                    let p = (&k * rt * &pt_mat)
                        .into_result()
                        .unwrap()
                        .to_mat()
                        .unwrap();

                    println!("k:      {:?}", k);
                    println!("pt_mat: {:?}", pt_mat);
                    println!("p:      {:?}", p);

                    Point3f::new(
                            *pt_mat.at_2d::<f64>(0, 0).unwrap() as f32,
                            *pt_mat.at_2d::<f64>(1, 0).unwrap() as f32,
                            *pt_mat.at_2d::<f64>(2, 0).unwrap() as f32)
                })
                .fold(VectorOfPoint3f::new(), |mut v, pt| { v.push(pt); v});

            let k = Mat::eye_size(Size::new(3, 3), CV_64F)
               .unwrap()
               .to_mat()
               .unwrap();

            let dist_coeffs = Mat::zeros(1, 5, CV_64F)
               .unwrap()
               .to_mat()
               .unwrap();

            let mut pts2 = VectorOfPoint2f::new();
            let mut jacobian = Mat::default();

            calib3d::project_points(&points, &rvec, &tvec, &k, &dist_coeffs,
                    &mut pts2, &mut jacobian, 0.0)
                .expect("Failed to project automatically undistorted points.");
            */

            let mut pts2 = VectorOfPoint2f::new();

            for pt in &points {
                let mut pt_mat = Mat::zeros(4, 1, CV_64F)
                    .unwrap()
                    .to_mat()
                    .unwrap();

                *pt_mat.at_2d_mut(0, 0).unwrap() = pt.x as f64;
                *pt_mat.at_2d_mut(1, 0).unwrap() = pt.y as f64;
                *pt_mat.at_2d_mut(2, 0).unwrap() = pt.z as f64;
                *pt_mat.at_2d_mut(3, 0).unwrap() = 1.0;


                // TODO: do undistortion
                let p = (&k * &rt * &pt_mat)
                    .into_result()
                    .unwrap()
                    .to_mat()
                    .unwrap();

                println!("p: {:?}", p);

                let z = *p.at_2d::<f64>(2, 0).unwrap() as f32;
                let p = Point2f::new(
                        *p.at_2d::<f64>(0, 0).unwrap() as f32 / z,
                        *p.at_2d::<f64>(1, 0).unwrap() as f32 / z);

                pts2.push(p);
            }

            println!("aaa {}", points.len());
            for (pt1, pt2) in pts1.iter().zip(pts2.iter()) { 
                println!("{:?} {:?}", pt1, pt2);
                draw_comparison(&mut frame_gray, pt1, pt2);
            }

            draw_axis(&mut frame_gray, &rvec, &tvec, calib);
        }

        let aspect_ratio = frame_gray.rows() as f32 / frame_gray.cols() as f32;
        let width = 1500.0;
        highgui::resize_window(window, width as i32, (width * aspect_ratio) as i32)
            .expect("Failed to resize window.");

        highgui::imshow(window, &frame_gray)?;
    }

    Ok(())
}

fn preprocess_image(image: &Mat) -> Mat {
    let mut gray = Mat::default();
    imgproc::cvt_color(&image, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    gray
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
    let c = Scalar::new(255.0, 255.0, 0.0, 1.0);

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