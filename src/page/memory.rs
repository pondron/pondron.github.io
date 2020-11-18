use seed::{prelude::*, *};

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

pub type Model = i32;


#[derive(Copy, Clone)]
pub enum Msg {
    Increment,
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div!["Memory View"]
}