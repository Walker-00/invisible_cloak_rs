use ndarray;
use opencv;
use opencv::core::{BorderTypes, Point, Scalar, CV_8U};
use opencv::gapi::morphology_ex;
use opencv::imgproc::{find_contours, MorphTypes};
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

        let closing = morphology_ex(
            &mask,
            MorphTypes::MORPH_CLOSE,
            &kernel,
            Point::default(),
            0,
            BorderTypes::BORDER_REFLECT,
            Scalar::default(),
        )
        .unwrap();

        let mut contours = Mat::default();

        find_contours(
            &Mat::from_raw(closing.as_raw_mut()),
            &mut contours,
            mode,
            method,
            offset,
        );
    }
}

fn main() {}
