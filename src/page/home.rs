use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
    div![C!["card"],
        header![C!["card-header"],
            p![C!["card-header-title"], "Where the Magic Happens"],
        ],
        div![C!["card-content"],
            div![C!["content"], 
                img![attrs!{At::Src => "static/images/graph-vis.png"}]
            ],
        ],
    ]
}