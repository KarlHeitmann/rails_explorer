use glob::glob;

use crate::associations::model::Model;

pub mod model;

pub struct Associations {
    application_root_path: String,
    models: Vec<Model>,
}

impl Associations {
    pub fn new(application_root_path: String) -> Self {
        
        let model_files = glob(&format!("{}/app/models/*.rb", application_root_path));
        let mut models: Vec<Model> = vec![];
        for e in model_files.expect("Failed to read glob pattern") { // FIXME: upon error, load a special component instead of this one
            let tmp = e.unwrap();
            // log::debug!("{}", e.unwrap().display());
            // log::debug!("{}", tmp.display());
            let model = Model::new(tmp);
            models.push(model);
        }
        Self {
            application_root_path,
            models,
        }
    }

    pub fn get_model_name(&self, i: usize) -> String {

        match self.models.get(i) {
            Some(model) => model.name.clone(),
            None => String::new(),
        }
    }

    pub fn get_models_iter(&self) -> std::slice::Iter<'_, Model> {
        self.models.iter()
    }
}
