use ndarray as nd;
use opencv as cv2;
use opencv::prelude::*;

const SENST: i8 = 20;
const H_VALUE: i8 = 20;

fn detect_blue(frame: Mat, background: Mat) {
    let mut hsv_image = cv2::gapi::GMat::default().unwrap();
    cv2::imgproc::cvt_color(
        &frame,
        &mut hsv_image.into(),
        cv2::imgproc::COLOR_RGB2HSV,
        0,
    );

    let light_blue = nd::array![H_VALUE - SENST, 60, 60];
    let dark_blue = nd::array![H_VALUE + SENST, 255, 255];

    let mask = cv2::gapi::in_range(&mut hsv_image, light_blue, dark_blue);
}

fn main() {}
