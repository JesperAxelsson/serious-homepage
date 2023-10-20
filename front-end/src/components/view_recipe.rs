use yew::prelude::*;
// use comrak::{markdown_to_html, ComrakOptions};
// use crate::markdown;
use crate::utils::render_text_as_html;
use log::*;
// use pulldown_cmark::{html, Options, Parser};
use crate::fetch::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub content: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: i32,
}

pub struct ViewRecipe {
    id: i32,
    state: PageState,
}

pub enum PageState {
    Init,
    Fetching,
    Success(Recipe),
    Error,
}

pub enum Msg {
    StartFetching,
    SetFetchState(FetchState<Recipe>),
}

impl Component for ViewRecipe {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        //         ViewRecipe {
        //             markdown: r#"
        // # Banana pancakes!
        // ## fo real?
        // Hello, **世界**!
        // - Hello
        // - Bread
        // - Toilet

        // "#
        //             .to_string(),
        //         }
        info!("Hello yaya");
        ViewRecipe {
            id: ctx.props().id,
            state: PageState::Init,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("Hello");

        match msg {
            Msg::StartFetching => {
                info!("Fetch recipe!");

                let id = self.id;

                let future = async move {
                    match fetch_url2(&format!("/api/recipe/{}", id)).await {
                        Ok(md) => Msg::SetFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                };

                send_future(&ctx.link(), future);

                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));

                false
            }
            Msg::SetFetchState(fetch_state) => {
                info!("Set_fetch!");

                self.state = match fetch_state {
                    FetchState::NotFetching => PageState::Init,
                    FetchState::Fetching => PageState::Fetching,
                    FetchState::Success(val) => PageState::Success(val),
                    FetchState::Failed(_) => PageState::Error,
                };
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // self.state = PageState::Browsing;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.state {
            PageState::Init => {
                ctx.link().send_message(Msg::StartFetching);
                html!("Fetching...")
            }
            PageState::Fetching => html!("Fetching..."),
            PageState::Error => html!("Failed to fetch recipe..."),
            PageState::Success(recipe) => {
                html! {
                    <article class="markdown-body">
                        <div>
                            { &recipe.title }
                        </div>

                        <div>
                            { &recipe.description }
                        </div>

                        <div>
                            { render_text_as_html(&recipe.content) }
                        </div>
                        // render_text_as_html(&html_output);
                        // { render_mark(&self.markdown) }
                        // render( markdown_to_html(&self.markdown, &ComrakOptions::default()) )
                    </article>
                }
            }
        }

        // htmlMark(&renderMark(&self.markdown))
    }
}

// fn render_mark(mark: &str) -> Html {
//     // Set up options and parser. Strikethroughs are not part of the CommonMark standard
//     // and we therefore must enable it explicitly.
//     let options = Options::empty();
//     // options.insert(Options::ENABLE_STRIKETHROUGH);
//     let parser = Parser::new_ext(mark, options);

//     // Write to String buffer.
//     let mut html_output: String = String::with_capacity(mark.len() * 3 / 2);
//     html::push_html(&mut html_output, parser);

//     let node = render_text_as_html(&html_output);

//     node
// }

// fn htmlMark(html: &str) -> Html {
//     use web_sys::{console, Node};
//     use yew::virtual_dom::VNode;
//     use yew::{Component, ComponentLink, Html, ShouldRender};

//     let div = VNode::from(html);
//     let js_svg = {
//         let div = web_sys::window()
//             .unwrap()
//             .document()
//             .unwrap()
//             .create_element("div")
//             .unwrap();
//         div.set_inner_html(html);
//         console::log_1(&div);
//         div
//     };

//     let node = Node::from(html);
//     let vnode = VNode::VRef(node);
//     vnode
// }
