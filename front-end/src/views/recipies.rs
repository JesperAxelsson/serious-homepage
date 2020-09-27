#![allow(dead_code)]
use serde::{Deserialize, Serialize};
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
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        info!("recipie create!");
        Recipies {
            link,
            markdown: FetchState::NotFetching,
            state: PageState::Browsing,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetMarkdown => {
                info!("New fetch!");
                let future = async {
                    match fetch_url2("http://localhost:3030/recipe").await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                };

                send_future(&self.link, future);

                self.link
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        self.state = PageState::Browsing;
        true
    }

    fn view(&self) -> Html {
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
                                        self.link.send_message(Msg::GetMarkdown);
                                        html! {"Fetching"}
                                    },
                                    FetchState::Fetching => html! {"Fetching"},
                                    FetchState::Success(data) => html! {
                                        <div>
                                            <h3 class="pb-3 ">
                                                { "Pick your poison!" }
                                            </h3>
                                            { data.iter().map(|rec| rec.render(&self.link) ).collect::<Html>() }
                                        </div>
                                    },
                                    FetchState::Failed(err) => html! {&err},
                                }
                            }
                            </div>
                        ),
                        PageState::Viewing(idd) => html!(
                            <div>
                                <ViewRecipe id=idd />
                            </div>
                        ),
                    }
                }
            </div>
        }
    }
}

impl RecipeListItem {
    fn render(&self, link: &ComponentLink<Recipies>) -> Html {
        let id = self.id;

        html! {
            <div class="pb-3">
                <a href="#" onclick=link.callback(move |_| Msg::ChangePageState(PageState::Viewing(id)))>
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
