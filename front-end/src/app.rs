#![allow(dead_code)]
use log::*;
// use serde_derive::{Deserialize, Serialize};
// use yew::format::Json;
use yew::prelude::*;

use crate::views::Blog;
use crate::views::Gallery;
use crate::views::Home;
use crate::views::Recipies;
use crate::views::Todo;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

// #[derive(Serialize, Deserialize)]
pub struct State {
    page_state: PageState,
}

// #[derive(Serialize, Deserialize)]
#[derive(PartialEq, Copy, Clone)]
pub enum PageState {
    Home,
    Recipies,
    Gallery,
    Todo,
    Blog,
}

pub enum Msg {
    ChangePageState(PageState),
    Nope,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            page_state: PageState::Home,
        };
        App { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangePageState(page_state) => self.state.page_state = page_state,
            Msg::Nope => {}
        }
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");

        let link_class = "py-1 pl-2 text-darkblue font-medium";
        let active_link_class = "py-1 pl-2 text-darkblue font-medium";

        html! {
        <div class="antialiased h-full ">
            <header class="pl-3 bg-gold shadow-xl h-24 flex">
                <div class="self-center">
                    <h1 class="text-3xl text-dark font-bold">
                        { "Welcome my homepage!" }
                    </h1>
                    <p class="text-sm text-darkblue">
                        { "Very nice site!" }
                    </p>
                </div>
            </header>
            <div class="flex bg-beige h-full">
                <div class="shadow h-full">
                    <aside class="py-3 flex flex-col" >
                        { self.render_link(PageState::Home, "Home") }
                        { self.render_link(PageState::Recipies, "Recipies") }
                        { self.render_link(PageState::Gallery, "Gallery") }
                        { self.render_link(PageState::Todo, "Todo") }
                        { self.render_link(PageState::Blog, "Blog") }
                    </aside>
                </div>
                <section class="w-full bg-darkgreen p-5">
                    { self.render_body() }
                </section>
            </div>
            <footer class="flex text-sm  bg-beige">
                <div class="items-center justify-center">
                    <p>{ "Written by " }<a class="text-indigo-900" href="https://github.com/JesperAxelsson/" target="_blank">{ "Jesper Axelsson" }</a></p>
                </div>
            </footer>
        </div>
        }
    }
}

impl App {
    fn render_body(&self) -> Html {
        html! {
            <section>
                <h3 class="">
                    {
                        match self.state.page_state {
                            PageState::Home => "Home!",
                            PageState::Recipies => "Recipies!",
                            PageState::Gallery => "Gallery!",
                            PageState::Todo =>  "What todo!",
                            PageState::Blog => "Blog!",
                        }
                    }
                </h3>
                {
                    match self.state.page_state {
                        PageState::Home => html!{<Home /> },
                        PageState::Recipies => html!{<Recipies /> },
                        PageState::Gallery => html!{<Gallery /> },
                        PageState::Todo => html!{<Todo /> },
                        PageState::Blog => html!{<Blog /> },
                    }
                }
            </section>
        }
    }

    fn render_link(&self, page_state: PageState, title: &str) -> Html {
        let link_class = if self.state.page_state == page_state {
            "py-1 pl-2  bg-darkgreen text-beige font-medium"
        } else {
            "py-1 pl-2 text-darkblue font-medium"
        };

        html! {
            <div class="pl-1">
                <div class={link_class}><a class="pr-4" onclick=self.link.callback(move |_| Msg::ChangePageState(page_state))>{title}</a></div>
            </div>
        }
    }
}
