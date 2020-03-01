
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
// use serde_json::Result;

use log::*;
use yew::prelude::*;
use crate::fetch::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub id: i64,
    pub text: String,
    pub completed: bool,
}

pub struct Recipies {
    link: ComponentLink<Self>,
    markdown: FetchState<Vec<Recipe>>,
    // markdown: FetchState<String>,
}

pub enum Msg {
    SetMarkdownFetchState(FetchState<Vec<Recipe>>),
    // SetMarkdownFetchState(FetchState<String>),
    GetMarkdown,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Recipies {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Recipies {
            link,
            markdown: FetchState::NotFetching,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetMarkdown => {
                info!("New fetch!");
                let future = async {
                    match fetch_url2("http://localhost:3030/todos").await {
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
                info!("SEt_fetch!");
                
                if let FetchState::Success(ref val) = fetch_state {
                    info!("Coool! {:?}", val);
                }

                self.markdown = fetch_state;
              
                true
            }
        }
    }

    // fn change(&mut self, props: Self::Properties) -> ShouldRender {
    //     self.title = props.title;
    //     self.onsignal = props.onsignal;
    //     true
    // }

    fn view(&self) -> Html {
        info!("View!");
        html! {
            <div>
                { "Recipies!" }
                <br/>
                <div>
                {
                    match &self.markdown {
                        FetchState::NotFetching => html! {
                            <button onclick=self.link.callback(|_| Msg::GetMarkdown)>
                                {"Get Markdown"}
                            </button>
                        },
                        FetchState::Fetching => html! {"Fetching"},
                        FetchState::Success(data) => html! {
                            data.iter().map(|rec| html! {
                                <div class="columns">
                                    <div class="column">{rec.id }</div>
                                    <div class="column">{&rec.text }</div>
                                    <div class="column">{rec.completed }</div>
                                </div>
                            }).collect::<Html>()
                        }
                        ,
                        FetchState::Failed(err) => html! {&err},
                    }
                }
                </div>

            </div>
        }
    }
}
