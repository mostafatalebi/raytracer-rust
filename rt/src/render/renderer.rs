use std::io::Write;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;
use std::{io, thread};
use std::time::Duration;
use crossterm::cursor::{MoveTo, MoveToColumn};
use crossterm::ExecutableCommand;
use crate::buffer::buffer::Buffer;
use crate::camera::camera::{StandardCamera};
use crate::common::stats::Stats;
use crate::error::error::SysError;
use crate::error::kinds::ErrorKind;
use crate::ray::tracer::Tracer;
use crate::scene::render_settings::RenderSettings;
use crate::scene::scene::Scene;

pub struct Renderer {
    scene:      Arc<RwLock<Scene>>,
    camera: String,
    rt: Tracer,

    stats: Stats,
}




impl Renderer {
    pub fn new(scene: Arc<RwLock<Scene>>) -> Self {
        Renderer{scene, camera: "".to_string(), rt: Tracer::default(), stats: Stats::default() }
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
        let render_settings: RenderSettings;
        {
            let scene = self.scene.read().unwrap();
            render_settings = scene.render_settings.clone()
        }
        buffer = Buffer::new(render_settings.width, render_settings.height);
        match cam {
            Some(c) => {
                let rt = &mut self.rt;
                self.stats.set_num_of_threads(render_settings.mt_num_of_threads);
                self.stats.record_start_time();
                rt.set_anti_aliasing(render_settings.anti_aliasing, render_settings.anti_aliasing_method);
                rt.set_num_of_threads(render_settings.mt_num_of_threads);
                Self::show_render_progression(rt.total_rays_to_process.clone(), rt.rays_processed_sofar.clone());
                match rt.trace_from_camera_to_scene(&mut buffer, &c, self.scene.clone()) {
                    Ok(output) => {
                        self.stats.record_end_time();
                        if let Err(e) = output.save_as_jpeg(&render_settings.get_output_file_name()) {
                            println!("error saving image: {:?}", e);
                        }
                        self.stats.print_stats();
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


    pub fn show_render_progression(total_rays: Arc<AtomicU64>, progress_sofar: Arc<AtomicU64>) {
        thread::spawn(move || {
            let mut last_percentage = 0u8;
            let mut writer = std::io::stdout();
            let heading = "progress: ";
            writer.write(heading.as_ref()).unwrap();
            writer.flush().unwrap();
            let mut cursor_pos = heading.len() as u16;
            writer.execute(MoveToColumn(cursor_pos)).unwrap();
            thread::sleep(Duration::from_millis(1));
            loop {
                let ps = progress_sofar.load(SeqCst);
                let tr = total_rays.load(SeqCst);
                if ps > 0 && tr > 0 {
                    let progress_in_percentage: f64;
                    {
                        progress_in_percentage = ((ps as f64 / tr as f64) * 100.0);
                    }
                    let curr =  (progress_in_percentage as u8) % 5;
                    if curr == 0 && (progress_in_percentage as u8) > last_percentage {
                        last_percentage = progress_in_percentage as u8;
                        cursor_pos += 5;
                        _ = writer.execute(MoveToColumn(cursor_pos)).unwrap();
                        writer.write(format!("■ [{}%]", progress_in_percentage as u8).as_bytes()).unwrap();
                        io::stdout().flush().unwrap();
                    }
                }


                thread::sleep(Duration::from_millis(200));
            }
        });
    }
}
