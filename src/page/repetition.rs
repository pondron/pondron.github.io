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
    div![C!["card"],
        header![C!["card-header"],
            p![C!["card-header-title"], "Spaced Repetition"],
        ],
        div![C!["card-content"],
            div![C!["content"], 
            "One effect of repeatedly practicing an action is increasing myelin 
            around the network leading to faster and more efficient processing 
            of cell signals. Myelination is a slow process that works over days 
            and weeks, usually flashcards, you have to walk the graph to get to 
            our unit every day. As you learn and practice new information, intricate 
            circuits of knowledge and memory are built and reinforced in the brain."
            ],
        ],
        footer![C!["card-footer"], 
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "try it out"],
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "contact us"],
        ],
    ]
}