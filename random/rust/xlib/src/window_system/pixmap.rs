use std::convert::TryInto;
use std::rc::Rc;
use x11::xlib;
use crate::window_system::Display;
use crate::error::Result;
use crate::render::texture::Texture;

pub type NativePixmap = xlib::Pixmap;

pub struct PixmapHandle {
    display: Display,
    native: NativePixmap,
}

impl PixmapHandle {
    pub fn manage_native(
        display: &Display,
        native: NativePixmap)
    -> Self {
        Self {
            display: display.clone(),
            native,
        }
    }

    pub unsafe fn native(&self) -> NativePixmap {
        self.native
    }
}

impl Drop for PixmapHandle {
    fn drop(&mut self) {
        unsafe {
            xlib::XFreePixmap(self.display.native(), self.native);
        }
        // TODO: panic on error
    }
}

pub struct Pixmap {
    handle: Rc<PixmapHandle>,
}

impl Pixmap {
    pub fn from_native(
        display: &Display,
        native: NativePixmap
    ) -> Self {
        Self {
            handle: Rc::new(PixmapHandle::manage_native(display, native))
        }
    }

    pub fn to_texture(
        &self,
        width: usize,
        height: usize
    ) -> Result<Texture> {
        // Get window Pixmap's pixel data.
        //
        // Pixmap pixel data is stored as a series of `height` rows in the final
        // Pixmap plane, each row being `width` pixels wide. The rows are
        // organized left to right, topmost to bottommost. A pixel is 4 bytes
        // wide, with each byte representing the `R`, `G`, `B`, and `A` channels
        // respectively.
        let texture_data = unsafe {
            let image = xlib::XGetImage(self.handle.display.native(),
                    self.handle.native(), 0, 0, width.try_into().unwrap(),
                    height.try_into().unwrap(),
                    xlib::XAllPlanes(), xlib::ZPixmap);

            // `width*height` rows of 4-byte pixels.
            let data_size = width * height * 4;

            let texture_data = std::slice::from_raw_parts(
                    (*image).data as *mut u8, data_size).to_vec();

            xlib::XDestroyImage(image);

            texture_data
        };

        // Build final texture from raw data.
        let texture = Texture::new()
                .from_data(width.try_into().unwrap(),
                        height.try_into().unwrap(), gl::RGBA, texture_data)
                .set_param(gl::TEXTURE_WRAP_S, gl::REPEAT)
                .set_param(gl::TEXTURE_WRAP_T, gl::REPEAT)
                .set_param(gl::TEXTURE_MAG_FILTER, gl::LINEAR)
                .set_param(gl::TEXTURE_MIN_FILTER, gl::LINEAR)
                .flip_horizontally()
                .build();

        Ok(texture)
    }

    pub unsafe fn native(&self) -> NativePixmap {
        self.handle.native()
    }
}
