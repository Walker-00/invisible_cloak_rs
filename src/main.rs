use ndarray;
use opencv;
use opencv::core::{BorderTypes, Point, Scalar, ToInputArray, Vector, CV_8U};
use opencv::gapi::morphology_ex;
use opencv::imgproc::{
    contour_area, fill_poly, find_contours, MorphTypes, CHAIN_APPROX_SIMPLE, RETR_EXTERNAL,
};
use opencv::prelude::*;

const SENST: i8 = 20;
const H_VALUE: i8 = 20;
const KERNEL_SIZE: i32 = 10;

fn detect_blue(frame: Mat, background: Mat) {
    let mut hsv_image = Mat::default();
    opencv::imgproc::cvt_color(&frame, &mut hsv_image, opencv::imgproc::COLOR_RGB2HSV, 0);

    let light_blue = opencv::gapi::GScalar::new(opencv::core::VecN::new(
        (H_VALUE - SENST) as f64,
        60.,
        60.,
        0.,
    ))
    .unwrap();

    let dark_blue = opencv::gapi::GScalar::new(opencv::core::VecN::new(
        (H_VALUE + SENST) as f64,
        255.,
        255.,
        0.,
    ))
    .unwrap();
    unsafe {
        let mask = opencv::gapi::in_range(
            &mut (opencv::gapi::GMat::from_raw(hsv_image.as_raw_mut())),
            &light_blue,
            &dark_blue,
        )
        .unwrap();

        let kernel = Mat::new_size(
            opencv::core::Size_ {
                width: KERNEL_SIZE,
                height: KERNEL_SIZE,
            },
            CV_8U,
        )
        .unwrap();

        let mut closing = morphology_ex(
            &mask,
            MorphTypes::MORPH_CLOSE,
            &kernel,
            Point::new(-1, -1),
            1,
            BorderTypes::BORDER_CONSTANT,
            Scalar::default(),
        )
        .unwrap();

        let mut contours = Mat::default();

        find_contours(
            &Mat::from_raw(closing.as_raw_mut()),
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_SIMPLE,
            Point::default(),
        )
        .unwrap();

        let contours_vec = contours
            .iter()
            .map(|c: opencv::core::MatIter<'_, f64>| c.collect::<Vec<_>>())
            .unwrap();

        let cont_sorted = contours_vec.sort_by(|c1, c2| {
            let area1 = contour_area(&c1, false).unwrap();
            let area2 = contour_area(&c2, false).unwrap();
            area2.partial_cmp(&area1).unwrap()
        });

        let mut idk = Mat::new_nd_vec(&Vector::from_slice(&[500, 500, 3]), CV_8U).unwrap();
    }
}

fn main() {}
