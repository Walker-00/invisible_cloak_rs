use ndarray;
use opencv;
use opencv::prelude::*;

const SENST: i8 = 20;
const H_VALUE: i8 = 20;
const KERNEL_SIZE: i8 = 10;

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
        );
    }
}

fn main() {}
