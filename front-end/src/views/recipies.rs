#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use yew::html::Scope;
// use serde_json::Result;

use crate::components::ViewRecipe;
use crate::fetch::*;
use log::*;
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecipeListItem {
    pub id: i32,
    pub title: String,
    pub description: String,
}

pub struct Recipies {
    markdown: FetchState<Vec<RecipeListItem>>,
    // markdown: FetchState<String>,
    state: PageState,
}

pub enum PageState {
    Browsing,
    Viewing(i32),
}

pub enum Msg {
    SetMarkdownFetchState(FetchState<Vec<RecipeListItem>>),
    // SetMarkdownFetchState(FetchState<String>),
    GetMarkdown,
    ChangePageState(PageState),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Recipies {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        info!("recipie create!");
        Recipies {
            markdown: FetchState::NotFetching,
            state: PageState::Browsing,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetMarkdown => {
                info!("New fetch!");
                let future = async {
                    match fetch_url2("/api/recipe").await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                };

                send_future(&ctx.link(), future);

                ctx.link()
                    .send_message(Msg::SetMarkdownFetchState(FetchState::Fetching));

                false
            }
            Msg::SetMarkdownFetchState(fetch_state) => {
                info!("Set_fetch!");

                if let FetchState::Success(ref val) = fetch_state {
                    info!("Coool! {:?}", val);
                }

                self.markdown = fetch_state;

                true
            }
            Msg::ChangePageState(new_state) => {
                self.state = new_state;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.state = PageState::Browsing;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        info!("recipie view!");

        html! {
            <div>
                {
                    match self.state {
                        PageState::Browsing => html!(
                            <div>
                            {
                                match &self.markdown {
                                    FetchState::NotFetching => {
                                        ctx.link().send_message(Msg::GetMarkdown);
                                        html! {"Fetching"}
                                    },
                                    FetchState::Fetching => html! {"Fetching"},
                                    FetchState::Success(data) => html! {
                                        <div>
                                            <h3 class="pb-3 ">
                                                { "Pick your poison!" }
                                            </h3>
                                            { data.iter().map(|rec| rec.render(&ctx.link()) ).collect::<Html>() }
                                        </div>
                                    },
                                    FetchState::Failed(err) => html! {&err},
                                }
                            }
                            </div>
                        ),
                        PageState::Viewing(idd) => html!(
                            <div>
                                <ViewRecipe id={idd} />
                            </div>
                        ),
                    }
                }
            </div>
        }
    }
}

impl RecipeListItem {
    fn render(&self, link: &Scope<Recipies>) -> Html {
        let id = self.id;

        html! {
            <div class="pb-3">
                <a href="#" onclick={link.callback(move |_| Msg::ChangePageState(PageState::Viewing(id)))}>
                    <div>
                        { &self.title }
                    </div>
                    <div>
                        { &self.description }
                    </div>
                </a>
            </div>
        }
    }
}
