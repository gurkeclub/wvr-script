use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use rhai::{FLOAT, INT};

use rand::{rngs::ThreadRng, Rng};

use wvr_com::data::SetInfo;
use wvr_com::data::{Message, RenderStageUpdate};
use wvr_data::types::data::DataHolder;
use wvr_data::types::InputProvider;
use wvr_data::types::InputSampler;

#[derive(Clone)]
pub struct WvrLink {
    pub event_list: Vec<Message>,
    pub stage_index_list: HashMap<String, usize>,
    pub input_list: Rc<Mutex<HashMap<String, Box<dyn InputProvider>>>>,
    pub rng: ThreadRng,
}

impl WvrLink {
    pub fn rnd_float(&mut self) -> FLOAT {
        self.rng.gen()
    }

    pub fn rnd_int(&mut self) -> INT {
        self.rng.gen()
    }

    pub fn set_bpm(&mut self, bpm: FLOAT) {
        self.event_list.push(Message::Set(SetInfo::Bpm(bpm)));
    }

    pub fn set_provider_float(
        &mut self,
        provider_name: String,
        variable_name: String,
        value: FLOAT,
    ) {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            provider.set_property(&variable_name, &DataHolder::Float(value as f32));
        }
    }
    pub fn get_provider_float(&mut self, provider_name: String, variable_name: String) -> FLOAT {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            if let Some(DataHolder::Float(variable_value)) = provider.get(&variable_name, false) {
                return variable_value as FLOAT;
            }
        }

        0.0 as FLOAT
    }
    pub fn set_provider_int(&mut self, provider_name: String, variable_name: String, value: INT) {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            provider.set_property(&variable_name, &DataHolder::Int(value as i32));
        }
    }
    pub fn get_provider_int(&mut self, provider_name: String, variable_name: String) -> INT {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            if let Some(DataHolder::Int(variable_value)) = provider.get(&variable_name, false) {
                return variable_value as INT;
            }
        }

        0 as INT
    }

    pub fn set_provider_bool(&mut self, provider_name: String, variable_name: String, value: bool) {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            provider.set_property(&variable_name, &DataHolder::Bool(value));
        }
    }
    pub fn get_provider_bool(&mut self, provider_name: String, variable_name: String) -> bool {
        if let Some(provider) = self.input_list.lock().unwrap().get_mut(&provider_name) {
            if let Some(DataHolder::Bool(variable_value)) = provider.get(&variable_name, false) {
                return variable_value;
            }
        }

        false
    }

    pub fn set_stage_texture_input(
        &mut self,
        stage: String,
        texture_name: String,
        sampling: String,
        source_name: String,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            if let Some(sampled_input) = match sampling.as_str() {
                "Nearest" => Some(InputSampler::Nearest(source_name)),
                "Linear" => Some(InputSampler::Linear(source_name)),
                "Mipmaps" => Some(InputSampler::Mipmaps(source_name)),
                _ => None,
            } {
                self.event_list.push(Message::UpdateRenderStage(
                    *stage_index,
                    RenderStageUpdate::Input(texture_name, sampled_input),
                ));
            }
        }
    }

    pub fn set_stage_float(&mut self, stage: String, variable_name: String, value: FLOAT) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(variable_name, DataHolder::Float(value as f32)),
            ));
        }
    }

    pub fn set_stage_float2(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: FLOAT,
        value_y: FLOAT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Float2([value_x as f32, value_y as f32]),
                ),
            ));
        }
    }

    pub fn set_stage_float3(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: FLOAT,
        value_y: FLOAT,
        value_z: FLOAT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Float3([value_x as f32, value_y as f32, value_z as f32]),
                ),
            ));
        }
    }

    pub fn set_stage_float4(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: FLOAT,
        value_y: FLOAT,
        value_z: FLOAT,
        value_w: FLOAT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Float4([
                        value_x as f32,
                        value_y as f32,
                        value_z as f32,
                        value_w as f32,
                    ]),
                ),
            ));
        }
    }

    pub fn set_stage_int(&mut self, stage: String, variable_name: String, value: INT) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(variable_name, DataHolder::Int(value as i32)),
            ));
        }
    }

    pub fn set_stage_int2(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: INT,
        value_y: INT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Int2([value_x as i32, value_y as i32]),
                ),
            ));
        }
    }

    pub fn set_stage_int3(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: INT,
        value_y: INT,
        value_z: INT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Int3([value_x as i32, value_y as i32, value_z as i32]),
                ),
            ));
        }
    }

    pub fn set_stage_int4(
        &mut self,
        stage: String,
        variable_name: String,
        value_x: INT,
        value_y: INT,
        value_z: INT,
        value_w: INT,
    ) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(
                    variable_name,
                    DataHolder::Int4([
                        value_x as i32,
                        value_y as i32,
                        value_z as i32,
                        value_w as i32,
                    ]),
                ),
            ));
        }
    }

    pub fn set_stage_bool(&mut self, stage: String, variable_name: String, value: INT) {
        if let Some(stage_index) = self.stage_index_list.get(&stage) {
            self.event_list.push(Message::UpdateRenderStage(
                *stage_index,
                RenderStageUpdate::Variable(variable_name, DataHolder::Bool(value == 1)),
            ));
        }
    }
}
