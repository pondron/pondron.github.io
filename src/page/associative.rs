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
            p![C!["card-header-title"], "Associative Learning"],
        ],
        div![C!["card-content"],
            div![C!["content"], 
            "Organizational Encoding/Associative learning is the theory that ideas 
            and experiences reinforce each other and can be linked to one another. 
            Our brains are not designed to recall information in isolation. "
            ],
        ],
        footer![C!["card-footer"], 
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "try it out"],
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "contact us"],
        ],
    ]
}