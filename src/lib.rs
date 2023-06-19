#![deny(clippy::all)]

use napi_derive::napi;

use dicom::object::open_file;
use dicom_pixeldata::PixelDecoder;
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn convert(preview_path: String) -> String {
  let mut img_base64 = "data:image/png;base64, ".to_string();
  if let Ok(obj) = open_file(preview_path) {
    if let Ok(image) = obj.decode_pixel_data() {
      if let Ok(dynamic_image) = image.to_dynamic_image(0) {
        let mut image_data: Vec<u8> = Vec::new();
        dynamic_image.write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png).unwrap();
        let res_base64 = base64::encode(image_data);
        img_base64 = format!("data:image/png;base64,{}", res_base64)
      }
    }

  }
  img_base64
}
