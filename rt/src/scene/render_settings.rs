use crate::scene::deserializers::deserialize_environment;
use serde::{Deserialize, Serialize};
use crate::camera::types::AntiAliasingMethod;
use crate::scene::environment::Environment;

#[derive(Clone, Deserialize, Serialize)]
pub struct RenderSettings {
    pub width:          usize,
    pub height:         usize,
    pub anti_aliasing:   AntiAliasingSetting,
    pub shadow_enabled: bool,
    pub output_dir:     String,
    pub file_name:      String,
    pub file_ext:       String,
    pub camera: String,
    pub multi_threading: MultiThreadingSetting,
    pub smooth_shading: bool,

    #[serde(deserialize_with = "deserialize_environment")]
    pub environment: Option<Environment>,

    pub disable_reflections: bool,
    pub disable_shadows: bool,
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

    pub fn set_environment(&mut self, env: Environment) {
        self.environment = Some(env);
    }
}


impl Default for RenderSettings {
    fn default() -> Self {
        RenderSettings{
            width: 100,
            height: 100,
            anti_aliasing: AntiAliasingSetting::default(),
            shadow_enabled: false,
            output_dir: "../tmp/output/".to_string(),
            file_name: "test_{#}".to_string(),
            file_ext: ".jpg".to_string(),
            camera: "".to_string(),
            multi_threading: MultiThreadingSetting::default(),
            smooth_shading: true,
            environment: None,
            disable_reflections: false,
            disable_shadows: false,
        }
    }


}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct AntiAliasingSetting {
    pub method: AntiAliasingMethod,
    pub sample: usize,
}

#[derive(Default, Clone, Deserialize, Serialize)]
pub struct MultiThreadingSetting {
    pub enabled: bool,
    pub count: usize,
}

