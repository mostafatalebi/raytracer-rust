use serde::{Deserialize, Serialize};
use crate::camera::types::AntiAliasingMethod;

#[derive(Clone, Deserialize, Serialize)]
pub struct RenderSettings {
    pub width:          usize,
    pub height:         usize,
    pub anti_aliasing:   u8,
    pub anti_aliasing_method:   AntiAliasingMethod,
    pub rt_reflection_max_samples: u16,
    pub shadow_enabled: bool,
    pub output_dir:     String,
    pub file_name:      String,
    pub file_ext:       String,
    pub camera: String,
    pub mt_num_of_threads: usize,
    pub mt_num_of_render_workers_per_thread: usize,
}

impl RenderSettings {
    pub fn get_output_file_name(&self) -> String {
        let mut increment = 0;
        let mut path = format!("{dir}{file}{ext}", dir=self.output_dir, file=self.file_name.replace("{#}", &format!("{d}", d=increment)), ext=self.file_ext);
        let mut res = std::fs::exists(&path);

        if res.is_ok() {
            let mut exists = res.unwrap();
            while exists {
                increment += 1;
                path = format!("{dir}{file}{ext}", dir=self.output_dir, file=self.file_name.replace("{#}", &format!("{d}", d=increment)), ext=self.file_ext);
                res = std::fs::exists(&path);
                if res.is_ok() {
                    exists = res.unwrap();
                } else {
                    break
                }
            }
        }

        path
    }
}


impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings{
            width: 100,
            height: 100,
            rt_reflection_max_samples: 1,
            anti_aliasing: 1,
            anti_aliasing_method: AntiAliasingMethod::Uniform,
            shadow_enabled: false,
            output_dir: "../tmp/output/".to_string(),
            file_name: "test_{#}".to_string(),
            file_ext: ".jpg".to_string(),
            camera: "".to_string(),
            mt_num_of_threads: 1,
            mt_num_of_render_workers_per_thread: 1,
        }
    }


}