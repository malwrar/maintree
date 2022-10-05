use std::os::raw::c_void;
use gl;
use gl::types::{GLuint, GLint, GLenum, GLsizei};

fn get_pixel_size(pixel_format: GLenum) -> usize {
    match pixel_format {
        gl::RGB => 3,
        gl::RGBA => 4,
        _ => panic!("Tried to get pixel size for unhandled format.")
    }
}

struct TextureParam {
    name: GLenum,
    value: GLenum,
}

/// Carries contextual info for creating a new texture.
pub struct TextureBuilder2D {
    target: GLenum,
    width: GLsizei,
    height: GLsizei,
    params: Vec<TextureParam>,
    pixel_format: GLenum,
    pixel_data: Vec<u8>,
}

impl TextureBuilder2D {
    pub fn new(target: GLenum) -> Self {
        Self {
            target: target,
            width: 0,
            height: 0,
            params: Vec::new(),
            pixel_format: gl::RGBA,
            pixel_data: Vec::new(),
        }
    }

    pub fn from_data<'a>(
        &'a mut self,
        width: GLsizei,
        height: GLsizei,
        pixel_format: GLenum,
        pixel_data: Vec<u8>,
    ) -> &'a mut Self {
        self.width = width;
        self.height = height;
        self.pixel_format = pixel_format;

        self.pixel_data.clear();
        self.pixel_data.extend(pixel_data.iter());

        self
    }

    pub fn set_param<'a>(
        &'a mut self,
        name: GLenum,
        value: GLenum
    ) -> &'a mut Self {
        self.params.push(TextureParam {
            name,
            value,
        });

        self
    }

    pub fn flip_horizontally<'a>(&'a mut self) -> &'a mut Self {
        let row_amount = self.height as usize;
        let row_width = self.pixel_data.len() / self.height as usize;

        // Flip array over the X axis
        let mut new_data = Vec::with_capacity(self.pixel_data.len());

        for row in (0..row_amount).rev() {
            let row_start = row * row_width;
            let row_end = row_start + row_width;
            new_data.extend(&self.pixel_data[row_start..row_end]);
        }

        self.pixel_data = new_data;

        self
    }

    pub fn flip_y<'a>(&'a mut self) -> &'a mut Self {
        // TODO: implement
        self 
    }

    pub fn build<'a>(&'a mut self) -> Texture {
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
        }

        let texture = Texture {
            id: texture_id, 
            target: self.target,
        };

        // Start setting up texture
        texture.activate();

        for param in self.params.iter() {
            unsafe {
                gl::TexParameteri(self.target, param.name,
                        param.value as GLint);
            }
        }

        // TODO: fail if width/height are 0 or won't fit provided data?
        unsafe {
            gl::TexImage2D(self.target, 0, gl::RGBA as GLint,
                    self.width, self.height, 0,
                    self.pixel_format, gl::UNSIGNED_BYTE,
                    self.pixel_data.as_ptr() as *const c_void);
        }

        // Finish setting up texture
        texture.deactivate(); 

        texture
    }
}

/// Represents a texture.
pub struct Texture {
    id: GLuint,
    target: GLenum,
}

impl Texture {
    pub fn new() -> TextureBuilder2D {
        TextureBuilder2D::new(gl::TEXTURE_2D)
    }

    pub fn activate(&self) {
        unsafe {
            gl::BindTexture(self.target, self.id);
        }
    }

    pub fn deactivate(&self) {

    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
