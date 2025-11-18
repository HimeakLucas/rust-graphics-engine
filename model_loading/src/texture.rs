use image::GenericImageView;
use gl;

pub struct Texture {
    pub id: u32,
    pub type_: String,
    pub path: String,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).expect("Failed to load texture");
        let data = img.flipv().into_rgb8();
        let (width, height) = img.dimensions();
        let mut id = 0; // Texture ID 

        // unsafe block to make a call to OpenGL(functions) 
        unsafe{
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB as i32, 
                width as i32, height as i32, 0, 
                gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self {id, type_:  "TextureDiffuse".into(), path: path.into()}

    }
}