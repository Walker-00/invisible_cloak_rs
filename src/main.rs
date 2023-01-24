use ndarray;
use opencv;
use opencv::core::{
    in_range, BorderTypes, Point, Scalar, Scalar_, Size, ToInputArray, Vec3d, VecN, Vector,
    BORDER_CONSTANT, CV_8U,
};
use opencv::gapi::{bitwise_and, bitwise_or, resize, GMat};
use opencv::highgui::{destroy_all_windows, imshow, wait_key};
use opencv::imgproc::{
    contour_area, fill_poly, find_contours, morphology_default_border_value, morphology_ex,
    MorphTypes, CHAIN_APPROX_SIMPLE, INTER_LINEAR, LINE_8, MORPH_CLOSE, RETR_EXTERNAL,
};
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, VideoWriter, CAP_ANY};

const SENST: i8 = 20;
const H_VALUE: i8 = 20;
const KERNEL_SIZE: i32 = 10;

unsafe fn detect_blue(frame: &mut Mat, background: &mut Mat) -> GMat {
    let mut hsv_image = Mat::default();
    opencv::imgproc::cvt_color(frame, &mut hsv_image, opencv::imgproc::COLOR_RGB2HSV, 0);

    let light_blue = opencv::core::VecN::new((H_VALUE - SENST) as f64, 60., 60., 0.);

    let dark_blue = opencv::core::VecN::new((H_VALUE + SENST) as f64, 255., 255., 0.);

    let mut frame = (&mut (frame.clone())).clone();
    let mut mask = Mat::default();

    in_range(&mut hsv_image, &light_blue, &dark_blue, &mut mask).unwrap();

    let kernel = Mat::ones(KERNEL_SIZE, KERNEL_SIZE, CV_8U).unwrap();

    let mut closing = Mat::default();

    morphology_ex(
        &mask,
        &mut closing,
        MORPH_CLOSE,
        &kernel,
        Point::new(0, 0),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    let mut contours = Mat::default();

    find_contours(
        &closing,
        &mut contours,
        RETR_EXTERNAL,
        CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )
    .unwrap();

    let cont_sorted_vec: Vec<opencv::core::Point_<i32>> =
        contours.iter::<f32>().unwrap().map(|c| c.0).collect();
    let mut contour_mask = Mat::zeros_nd(&[500, 500, 3], CV_8U)
        .unwrap()
        .to_mat()
        .unwrap();

    let mat_cont_sorted = Mat::from_slice(&cont_sorted_vec).unwrap();

    fill_poly(
        &mut contour_mask,
        &mat_cont_sorted,
        Scalar_::from((255., 255., 255.)),
        LINE_8,
        0,
        Point::default(),
    )
    .unwrap();

    fill_poly(
        &mut frame,
        &mat_cont_sorted,
        Scalar_::from((0., 0., 0.)),
        LINE_8,
        0,
        Point::default(),
    )
    .unwrap();

    let obj_mask = &mut frame.clone();

    let background_mask = bitwise_and(
        &GMat::from_raw(contour_mask.as_raw_mut()),
        &GMat::from_raw(background.as_raw_mut()),
    )
    .unwrap();

    let final_img = bitwise_or(&GMat::from_raw(obj_mask.as_raw_mut()), &background_mask).unwrap();

    return final_img;
}

fn main() {
    let mut cap = VideoCapture::new(0, CAP_ANY).unwrap();

    let mut background = Mat::default();

    let ret = cap.read(&mut background);

    unsafe {
        let mut res_background = resize(
            &GMat::from_raw(background.as_raw_mut()),
            Size::new(500, 500),
            0.,
            0.,
            INTER_LINEAR,
        )
        .unwrap();

        imshow("Background", &Mat::from_raw(res_background.as_raw_mut())).unwrap();
        wait_key(0).unwrap();
        destroy_all_windows().unwrap();

        let fourcc = VideoWriter::fourcc('X', 'V', 'I', 'D').unwrap();

        let mut out =
            VideoWriter::new("output.mp4", fourcc, 30.0, Size::new(500, 500), true).unwrap();

        loop {
            let mut frame = Mat::default();
            cap.read(&mut frame).unwrap();
            let mut res_frame = resize(
                &GMat::from_raw(frame.as_raw_mut()),
                Size::new(500, 500),
                0.,
                0.,
                INTER_LINEAR,
            )
            .unwrap();

            let mut image = detect_blue(
                &mut Mat::from_raw(res_frame.as_raw_mut()),
                &mut Mat::from_raw(res_background.as_raw_mut()),
            );

            out.write(&Mat::from_raw(image.as_raw_mut())).unwrap();

            imshow("Image", &Mat::from_raw(image.as_raw_mut())).unwrap();

            if wait_key(1).unwrap() == b'q' as i32 {
                break;
            }
        }

        cap.release().unwrap();
        out.release().unwrap();
        destroy_all_windows().unwrap();
    }
}
