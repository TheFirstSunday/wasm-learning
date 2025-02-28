use wasm_bindgen::prelude::*;
use ssvm_tensorflow_interface;
use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;
use std::time::{Instant};

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
    let start = Instant::now();
    let mut img = image::load_from_memory(image_data).unwrap();
    let resized = image::imageops::thumbnail(&img, 416, 416);
    println!("Resized image in ... {:?}", start.elapsed());
    let mut flat_img: Vec<f32> = Vec::new();
    for rgb in resized.pixels() {
        flat_img.push(rgb[0] as f32 / 255.);
        flat_img.push(rgb[1] as f32 / 255.);
        flat_img.push(rgb[2] as f32 / 255.);
    }
    println!("Loaded image in ... {:?}", start.elapsed());

    let model_data: &[u8] = include_bytes!("yolov4-416.tflite");

    let mut session = ssvm_tensorflow_interface::Session::new(model_data, ssvm_tensorflow_interface::ModelType::TensorFlowLite);
    session.add_input("input_1", &flat_img, &[1,416,416,3]);
    println!("Input added ...");
    session.add_output("Identity");
    println!("Output added ... Identity");
    session.add_output("Identity_1");
    println!("Output added ... Identity_1");
    println!("All preparation completed in ... {:?}", start.elapsed());
    session.run();
    println!("Session successfully ran in ... {:?}", start.elapsed());
    let res_vec: Vec<f32> = session.get_output("Identity");
    let res_vec_1: Vec<f32> = session.get_output("Identity_1");
    println!("Output obtained in ... {:?}", start.elapsed());
    println!("Identity:");
    println!("{:?}", res_vec);
    println!("Identity_1:");
    println!("{:?}", res_vec_1);
    /*
    // Parse results.
    let mut iter = 0;
    let mut box_vec: Vec<[f32; 4]> = Vec::new();
    while (iter * 4) < res_vec.len() {
        box_vec.push([
            res_vec[4 * iter + 1], // x1
            res_vec[4 * iter],     // y1
            res_vec[4 * iter + 3], // x2
            res_vec[4 * iter + 2], // y2
        ]);
        iter += 1;
    }
    println!("Parsed results in ... {:?}", start.elapsed());

    println!("Drawing box: {} results ...", box_vec.len());
    let line = Pixel::from_slice(&[0, 255, 0, 0]);
    for i in 0..box_vec.len() {
        let xy = box_vec[i];
        let x1: i32 = xy[0] as i32;
        let y1: i32 = xy[1] as i32;
        let x2: i32 = xy[2] as i32;
        let y2: i32 = xy[3] as i32;
        let rect = Rect::at(x1, y1).of_size((x2 - x1) as u32, (y2 - y1) as u32);
        draw_hollow_rect_mut(&mut img, rect, *line);
    }
    
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(80u8)).expect("Unable to write");
    println!("Drawn on image in ... {:?}", start.elapsed());

    return buf;
    */

    // Temporary blank vector
    let mut vec = Vec::new();
    vec.push(1);
    return vec;
}
