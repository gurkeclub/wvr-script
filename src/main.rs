use std::rc::Rc;
use std::sync::Mutex;
use std::{collections::HashMap, path::PathBuf};
use wvr_script::Script;

fn main() {
    let mut stage_index_list = HashMap::new();
    stage_index_list.insert(String::from("Fb1"), 0);

    let mut script = Script::new(PathBuf::from("main.rhai")).unwrap();

    script.update().unwrap();

    let event_list = script
        .execute(
            stage_index_list.clone(),
            Rc::new(Mutex::new(HashMap::new())),
            89.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0,
        )
        .unwrap();
    println!("{:?}", event_list);
    let event_list = script
        .execute(
            stage_index_list.clone(),
            Rc::new(Mutex::new(HashMap::new())),
            89.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0,
        )
        .unwrap();
    println!("{:?}", event_list);
}
