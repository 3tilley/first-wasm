use std::collections::hash_map::Entry;
use std::collections::HashMap;
use seed::{prelude::*, *};

struct Model {
    inventory: HashMap<String, u32>,
}

impl Model {
    fn new() -> Self {
        Self {
            inventory: HashMap::new(),
        }
    }
}

enum Msg {
    AddItem(String),
    RemoveItem(String),
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::new()
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddItem(item) => {
            let count = model.inventory.entry(item.to_uppercase()).or_insert(0);
            *count += 1;
        }
        Msg::RemoveItem(item) => {
            let count = model.inventory.entry(item.to_uppercase());
            match count {
                //Entry::Occupied(c) if *c. == 0 => panic!("No items to remove"),
                Entry::Occupied(c) => {
                    let mut count = c.get();
                    //*count += 1
                }
                Entry::Vacant(_) => panic!("No items found in inventory")
            }
        }
    }
}



fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![
            "Add",
            ev(Ev::Click, |_| Msg::AddItem("Dog".to_string())),
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}

fn main() {}