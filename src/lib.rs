use std::collections::HashMap;
use std::io::Error;
use std::io::Read;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::time::Duration;
use std::{fs::File, io::ErrorKind};

use anyhow::{Context, Result};

use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use rhai::{Engine, Map, Scope, AST, FLOAT, INT};

use wvr_com::data::Message;
use wvr_data::types::InputProvider;

pub mod link;

use link::WvrLink;

pub struct Script {
    engine: Engine,
    file_path: PathBuf,
    script_source: String,
    ast: Option<AST>,
    _file_watcher: RecommendedWatcher,
    file_change_rx: Receiver<DebouncedEvent>,
    global_variable_list: Map,
}

impl Script {
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to open script file: {:?} ({:?})", &file_path, &e),
            )
        })?;

        let (tx, file_change_rx) = channel();
        let mut _file_watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(1))?;

        _file_watcher.watch(&file_path, RecursiveMode::NonRecursive)?;

        let mut script_source = String::new();
        file.read_to_string(&mut script_source).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to retrieve script content: {:?} ({:?})",
                    &file_path, &e
                ),
            )
        })?;

        let mut engine = Engine::new();

        engine
            .register_type::<WvrLink>()
            .register_fn("rnd_float", WvrLink::rnd_float)
            .register_fn("rnd_int", WvrLink::rnd_int)
            .register_fn("set_stage_texture_input", WvrLink::set_stage_texture_input)
            .register_fn("set_bpm", WvrLink::set_bpm)
            .register_fn("set_provider_float", WvrLink::set_provider_float)
            .register_fn("get_provider_float", WvrLink::get_provider_float)
            .register_fn("set_provider_int", WvrLink::set_provider_int)
            .register_fn("get_provider_int", WvrLink::get_provider_int)
            .register_fn("set_provider_bool", WvrLink::set_provider_bool)
            .register_fn("get_provider_bool", WvrLink::get_provider_bool)
            .register_fn("set_stage_float", WvrLink::set_stage_float)
            .register_fn("set_stage_float2", WvrLink::set_stage_float2)
            .register_fn("set_stage_float3", WvrLink::set_stage_float3)
            .register_fn("set_stage_float4", WvrLink::set_stage_float4)
            .register_fn("set_stage_int", WvrLink::set_stage_int)
            .register_fn("set_stage_int2", WvrLink::set_stage_int2)
            .register_fn("set_stage_int3", WvrLink::set_stage_int3)
            .register_fn("set_stage_int4", WvrLink::set_stage_int4)
            .register_fn("set_stage_bool", WvrLink::set_stage_bool);

        let ast = match engine.compile(&script_source) {
            Ok(ast) => Some(ast),
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        };

        Ok(Self {
            engine,
            file_path,
            script_source,
            ast,
            _file_watcher,
            file_change_rx,
            global_variable_list: Map::new(),
        })
    }

    pub fn update(&mut self) -> Result<()> {
        if self.file_change_rx.try_recv().is_ok() {
            if let Ok(mut file) = File::open(&self.file_path) {
                let mut text = String::new();
                file.read_to_string(&mut text)
                    .context("Failed to retrieve script content")?;

                self.ast = match self.engine.compile(&text) {
                    Ok(ast) => Some(ast),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        None
                    }
                };

                self.script_source = text;
            }
        }

        Ok(())
    }

    pub fn execute(
        &mut self,
        stage_index_list: HashMap<String, usize>,
        input_list: Rc<Mutex<HashMap<String, Box<dyn InputProvider>>>>,
        bpm: f64,
        beat: f64,
        d_beat: f64,
        time: f64,
        d_time: f64,
        frame_count: usize,
    ) -> Result<Vec<Message>> {
        let event_generator = WvrLink {
            stage_index_list,
            input_list,
            event_list: Vec::new(),
            rng: rand::thread_rng(),
        };
        let mut scope = Scope::new();
        scope.set_value("event_list", event_generator);
        scope.set_value("beat", beat as FLOAT);
        scope.set_value("bpm", bpm as FLOAT);
        scope.set_value("time", time as FLOAT);
        scope.set_value("d_beat", d_beat as FLOAT);
        scope.set_value("d_time", d_time as FLOAT);
        scope.set_value("frame_count", frame_count as INT);
        scope.set_value("globals", self.global_variable_list.clone());

        if let Some(ast) = &self.ast {
            self.engine
                .eval_ast_with_scope::<()>(&mut scope, ast)
                .map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to execute script: {:?} ({:?})", &self.file_path, &e),
                    )
                })?;
        }

        self.global_variable_list = scope.get_value::<Map>("globals").unwrap();

        Ok(scope.get_value::<WvrLink>("event_list").unwrap().event_list)
    }
}
