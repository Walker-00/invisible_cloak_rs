use ndarray;
use opencv;
use opencv::core::{BorderTypes, Point, Scalar, ToInputArray, Vector, CV_8U, in_range};
use opencv::gapi::morphology_ex;
use opencv::imgproc::{
    contour_area, fill_poly, find_contours, morphology_default_border_value, MorphTypes,
    CHAIN_APPROX_SIMPLE, RETR_EXTERNAL, morphology_ex,
};
use opencv::prelude::*;
use opencv::ximgproc::morphology_ex;

const SENST: i8 = 20;
const H_VALUE: i8 = 20;
const KERNEL_SIZE: i32 = 10;

fn detect_blue(frame: Mat, background: Mat) {
    let mut hsv_image = Mat::default();
    opencv::imgproc::cvt_color(&frame, &mut hsv_image, opencv::imgproc::COLOR_RGB2HSV, 0);

    let light_blue = opencv::core::VecN::new(
        (H_VALUE - SENST) as f64,
        60.,
        60.,
        0.,
    );

    let dark_blue = opencv::core::VecN::new(
        (H_VALUE + SENST) as f64,
        255.,
        255.,
        0.,
    );

    unsafe {
        let mut mask = Mat::default();

        in_range(&mut hsv_image, &light_blue, &dark_blue, &mut mask) ;

        let kernel = Mat::ones(KERNEL_SIZE, KERNEL_SIZE, CV_8U).unwrap();

        let mut closing = morphology_ex(
            &mask,
            MorphTypes::MORPH_CLOSE,
            &kernel.a(),
            Point::new(-1, -1),
            1,
            BorderTypes::BORDER_CONSTANT,
            Scalar::default(),
        )
        .unwrap();

        morphology_ex(&mask, , op, kernel, anchor, iterations, border_type, border_value)

        let mut contours = Mat::default();

        find_contours(image, contours, mode, method, offset);
        find_contours(&mut closing, &mut contours, mode, method, offset);

        find_contours(
            &Mat::from_raw(closing.as_raw_mut()),
            &mut contours,
            RETR_EXTERNAL,
            CHAIN_APPROX_SIMPLE,
            Point::default(),
        )
        .unwrap();

        let mut c: Vec<_> = contours.iter::<f32>().into_iter().collect();

        let mut idk = Mat::new_nd_vec(&Vector::from_slice(&[500, 500, 3]), CV_8U).unwrap();
    }
}

fn main() {}
