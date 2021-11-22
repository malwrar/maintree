//! Generic 3d graphics engine (3dge... get it?) that tries to keep the boring parts out of sight
//! as much as possible.

//pub mod camera;
//pub mod command;
//pub mod engine;
//pub mod surface;

//
//use wgpu::util::DeviceExt;
//
//
//    let event_loop = EventLoop::new();
//    let window = WindowBuilder::new().build(&event_loop).unwrap();

//struct Engine {
//
//}
//
//impl Engine {
//    pub fn new() -> Self {
//
//    }
//
//    pub fn add_surface(&self) {
//
//    }
//
//    pub fn add_entity(&self) {
//
//    }
//
//    pub fn send_command(&self) {
//
//    }
//
//    pub fn events(&self) {
//
//    }
//
//    pub fn run(self) {
//        for event in self.events() {
//            log::debug("Event: {:?}");
//        }
//    }
//}
//
//#[macro_export]
//macro_rules! easy_window {
//    ( $( $x:expr ),* ) => {jj
//        {
//            //let mut temp_vec = Vec::new();
//            //$(
//            //    temp_vec.push($x);
//            //)*
//            //temp_vec
//            log::debug!("Creating window.");
//        }
//    };
//}

//trait Pipeline {
//    fn bake(&self, surface: &Surface) -> Result<BakedPipeline, &'static str>;
//}
//
//struct MeshPipeline {
//    fn from_vertices() {
//
//    }
//
//    fn from_origin(attrs) {
//        from_vertices(&[
//            1.0, 1.0, 1.0
//        ])
//    }
//
//    fn bake(&self, surface: &Surface) -> Result<BakedPipeline, &'static str> {
//
//    }
//}
//
//struct BakedPipeline {
//
//}
//
//trait Surface {
//    fn repositioned(&self, x: u32, y: u32, w: u32, h: u32) { }
//    fn repositioned(&self, x: u32, y: u32, w: u32, h: u32) { }
//}
//
//struct WindowSurface {
//
//}
//
//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        assert_eq!(2 + 2, 4);
//    }
//}
