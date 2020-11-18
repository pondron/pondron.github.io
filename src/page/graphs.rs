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
            p![C!["card-header-title"], "Graphs & The Brain"],
        ],
        div![C!["card-content"],
            div![C!["content"], 
            "The human brain demonstrates a network structure that is commonly represented 
            using graphs with pseudonym connectome. Traditionally, connectomes encode only 
            inter-regional connectivity as edges, while regional information, such as centrality 
            of a node that may be crucial to the analysis, is usually handled as statistical 
            covariates. This results in an incomplete encoding of valuable information. 
            In order to alleviate such problems, we propose an enriched connectome encoding 
            regional properties of the brain network, such as structural node degree, strength, 
            and centrality, as node features in addition to representing structural connectivity 
            between regions as weighted edges. We further present an efficient graph matching 
            algorithm, providing two measures to quantify similarity between enriched connectomes. 
            We demonstrate the utility of our graph representation and similarity measures on 
            classifying a traumatic brain injury dataset. Our results show that the enriched 
            representation combining nodal features and structural connectivity information with 
            the graph matching based similarity measures is able to differentiate the groups better
             than the traditional connectome representation."
            ],
        ],
        footer![C!["card-footer"], 
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "try it out"],
            a![C!["card-footer-item"], attrs!{At::Href => "#"}, "contact us"],
        ],
    ]
}