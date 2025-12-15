use gl;
use image::GenericImageView;
use std::path::Path;

pub struct Texture {
    pub id:u32,
}

impl Texture {

    pub fn new(path: &str) -> Self {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }

        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let img = img.flipv();
        let data = img.as_bytes();

        unsafe {
            
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            let format = match img {

                image::DynamicImage::ImageRgb8(_) => gl::RGB,
                image::DynamicImage::ImageRgba8(_) => gl::RGBA,
                _ => gl::RGB,
            };

            gl::TexImage2D(

                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _

            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self {id: texture_id}
    }

        

}
