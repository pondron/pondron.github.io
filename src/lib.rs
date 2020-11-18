use seed::{prelude::*, *};

mod page;

// ------ Url path parts ------
const GRAPHS: &str = "graphs";
const ASSOCIATIVE: &str = "associative";
const REPETITION: &str = "repetition";
const NET: &str = "net";
const MEMORY: &str = "memory";
const CORE: &str = "core";
const LOGIN: &str = "login";
const SIGNUP: &str = "signup";
const SETTINGS: &str = "settings";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        ctx: Context {
            user: None,
            token: None,
        },
        base_url: url.to_base_url(),
        page: Page::init(url, orders),
        menu_visable: false,
    }
}

struct Model {
    ctx: Context,
    base_url: Url,
    page: Page,
    menu_visable: bool,
}

struct Context {
    user: Option<User>,
    token: Option<String>,
}

struct User {
    username: String,
    email: String,
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn graphs(self) -> Url {
        self.base_url().add_path_part(GRAPHS)
    }
    pub fn associative(self) -> Url {
        self.base_url().add_path_part(ASSOCIATIVE)
    }
    pub fn repetition(self) -> Url {
        self.base_url().add_path_part(REPETITION)
    }
    pub fn net(self) -> Url {
        self.base_url().add_path_part(NET)
    }
    pub fn memory(self) -> Url {
        self.base_url().add_path_part(MEMORY)
    }
    pub fn core(self) -> Url {
        self.base_url().add_path_part(CORE)
    }
    pub fn settings(self) -> Url {
        self.base_url().add_path_part(SETTINGS)
    }
    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }
    pub fn signup(self) -> Url {
        self.base_url().add_path_part(SIGNUP)
    }
}

enum Page {
    Home,
    Graphs(page::graphs::Model),
    Associative(page::associative::Model),
    Repetition(page::repetition::Model),
    Net(page::net::Model),
    Memory(page::memory::Model),
    Core(page::core::Model),
    Settings(page::settings::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Self::Home,
            [GRAPHS] => Self::Graphs(
                page::graphs::init(url, &mut orders.proxy(Msg::GraphMsg))
            ),
            [ASSOCIATIVE] => Self::Associative(
                page::associative::init(url, &mut orders.proxy(Msg::AssociativeMsg))
            ),
            [REPETITION] => Self::Repetition(
                page::repetition::init(url, &mut orders.proxy(Msg::RepetitionMsg))
            ),
            [NET] => Self::Net(
                page::net::init(url, &mut orders.proxy(Msg::NetMsg))
            ),
            [MEMORY] => Self::Memory(
                page::memory::init(url, &mut orders.proxy(Msg::MemoryMsg))
            ),
            [CORE] => Self::Core(
                page::core::init(url, &mut orders.proxy(Msg::CoreMsg))
            ),
            [SETTINGS] => Self::Settings(
                page::settings::init(url, &mut orders.proxy(Msg::SettingsMsg))
            ),
            _ => Self::NotFound,
        }
    }
}

enum Msg {
    UrlChanged(subs::UrlChanged),
    ToggleMenu,

    GraphMsg(page::graphs::Msg),
    AssociativeMsg(page::associative::Msg),
    CoreMsg(page::core::Msg),
    NetMsg(page::net::Msg),
    MemoryMsg(page::memory::Msg),
    RepetitionMsg(page::repetition::Msg),
    SettingsMsg(page::settings::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(url, orders),
        Msg::ToggleMenu => model.menu_visable = not(model.menu_visable),
        Msg::GraphMsg(msg) => {
            if let Page::Graphs(model) = &mut model.page {
                page::graphs::update(msg, model, &mut orders.proxy(Msg::GraphMsg))
            }
        },
        Msg::AssociativeMsg(msg) => {
            if let Page::Associative(model) = &mut model.page {
                page::associative::update(msg, model, &mut orders.proxy(Msg::AssociativeMsg))
            }
        },
        Msg::RepetitionMsg(msg) => {
            if let Page::Repetition(model) = &mut model.page {
                page::repetition::update(msg, model, &mut orders.proxy(Msg::RepetitionMsg))
            }
        },
        Msg::NetMsg(msg) => {
            if let Page::Net(model) = &mut model.page {
                page::net::update(msg, model, &mut orders.proxy(Msg::NetMsg))
            }
        },
        Msg::MemoryMsg(msg) => {
            if let Page::Memory(model) = &mut model.page {
                page::memory::update(msg, model, &mut orders.proxy(Msg::MemoryMsg))
            }
        },
        Msg::CoreMsg(msg) => {
            if let Page::Core(model) = &mut model.page {
                page::core::update(msg, model, &mut orders.proxy(Msg::CoreMsg))
            }
        },
        Msg::SettingsMsg(msg) => {
            if let Page::Settings(model) = &mut model.page {
                page::settings::update(msg, model, &mut orders.proxy(Msg::SettingsMsg))
            }
        },
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        view_navbar(&model.page, &model.base_url, model.ctx.user.as_ref()),
        view_content(&model.page),
    ]
}

fn view_content(page: &Page) -> Node<Msg> {
    div![
        C!["column", "main"],
        match page {
            Page::Home => page::home::view(),
            Page::Graphs(model) => page::graphs::view(model).map_msg(Msg::GraphMsg),
            Page::Associative(model) => page::associative::view(model).map_msg(Msg::AssociativeMsg),
            Page::Repetition(model) => page::repetition::view(model).map_msg(Msg::RepetitionMsg),
            Page::Net(model) => page::net::view(model).map_msg(Msg::NetMsg),
            Page::Memory(model) => page::memory::view(model).map_msg(Msg::MemoryMsg),
            Page::Core(model) => page::core::view(model).map_msg(Msg::CoreMsg),
            Page::Settings(model) => page::settings::view(model).map_msg(Msg::SettingsMsg),
            Page::NotFound => page::not_found::view(),
        }
    ]
}

fn view_navbar(page: &Page, base_url: &Url, user: Option<&User>) -> Vec<Node<Msg>> {
    nodes![
        div![C!["column", "is-2", "sidey"],
            aside![C!["menu"],
                div![C!["has-text-centered"],
                    img![attrs!{At::Src => "static/images/pondron6.png"}],
                ],
                br![],
                p![C!["menu-label"], "Overview"],
                hr![],
                ul![
                    C!["menu-list"],
                    li![a![C![IF!(matches!(page, Page::Graphs(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).graphs()},
                        "Graphs & The Brain",
                    ],],
                    li![a![C![IF!(matches!(page, Page::Associative(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).associative()},
                        "Associative Learning",
                    ],],
                    li![a![C![IF!(matches!(page, Page::Repetition(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).repetition()},
                        "Spaced Repetition",
                    ],],
                ],
                br![],
                p![C!["menu-label"], "Navigation"],
                hr![],
                ul![
                    C!["menu-list"],
                    li![a![C![IF!(matches!(page, Page::Home) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).home()},
                        "Home",
                    ],],
                    li![a![C![IF!(matches!(page, Page::Net(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).net()},
                        "Neural Net",
                    ],],
                ],
                div![C!["field", "has-addons"], 
                    div![C!["control"],
                        input![C!["input", "siri"],
                        attrs!{
                            At::Type => "text",
                            At::Placeholder => "find a node..."
                        }
                        ],
                    ],
                    div![C!["control"], 
                        a![C!["button", "is-info", "siri"], "Search"],
                    ],
                ],
                br![],
                p![C!["menu-label"], "Learning Path"],
                hr![],
                ul![
                    C!["menu-list"],
                    C!["menu-list"],
                    li![a![C![IF!(matches!(page, Page::Memory(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).memory()},
                        "Memory Trace",
                    ],],
                    li![a![C![IF!(matches!(page, Page::Core(_)) => "is-active"),],
                        attrs!{At::Href => Urls::new(base_url).core()},
                        "Core Concepts",
                    ],],
                ],
            ],
        ],
    ]
}


#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}