use std::sync::{Arc, Mutex, RwLock};
use crate::buffer::buffer::Buffer;
use crate::camera::camera::{StandardCamera};
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::ray::tracer::Tracer;
use crate::scene::scene::Scene;

pub struct Renderer {
    scene:      Arc<RwLock<Scene>>,
    camera: String,
    rt: Tracer
}




impl Renderer {
    pub fn new(scene: Arc<RwLock<Scene>>) -> Self {
        Renderer{scene, camera: "".to_string(), rt: Tracer::default() }
    }

    fn get_selected_camera(&mut self) -> Option<StandardCamera> {
        let scene = self.scene.read().unwrap();
        let c = scene.get_default_camera();
        if let Some(camera) = c {
            self.camera = camera.id.clone();
            Some(camera.clone())
        } else {
            None
        }
    }

    pub fn render(&mut self) -> Result<(), SysError> {
        let cam = self.get_selected_camera().clone();
        let mut buffer: Buffer;
        let output_filename: String;
        {
            let scene = self.scene.read().unwrap();
            buffer = Buffer::new(scene.render_settings.width, scene.render_settings.height);
            output_filename = scene.render_settings.get_output_file_name();
        }
        match cam {
            Some(c) => {
                let rt = &mut self.rt;
                match rt.trace_from_camera_to_scene(&mut buffer, &c, self.scene.clone()) {
                    Ok(output) => {
                        if let Err(e) = output.save_as_jpeg(&output_filename) {
                            println!("error saving image: {:?}", e);
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            },
            None => {
                return Err(SysError::new(ErrorKind::CameraNotFound, "no camera found for rendering".to_string()))
            }
        }


        Ok(())
    }

    pub fn per_pixel_render(&self) -> Result<(), SysError> {

        Ok(())
    }
}
