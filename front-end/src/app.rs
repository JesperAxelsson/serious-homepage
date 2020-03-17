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

        html! {
        <div class="antialiased h-screen bg-beige flex flex-col">
            <header class="pl-6 pb-2 bg-gold shadow  h-24 flex">
                <div class="self-center">
                    <h1 class="text-3xl text-dark font-bold leading-tight tracking-wide">
                        { "Welcome my homepage!" }
                    </h1>
                    <p class="text-sm text-darkblue  ">
                        { "Very nice site!" }
                    </p>
                </div>
            </header>
            <div class="flex h-full">
                <div class="w-auto h-full flex flex-col">
                    <aside class="py-3 shadow h-full flex flex-col" >
                        { self.render_link(PageState::Home, "Home") }
                        { self.render_link(PageState::Recipies, "Recipies") }
                        { self.render_link(PageState::Gallery, "Gallery") }
                        { self.render_link(PageState::Todo, "Todo") }
                        { self.render_link(PageState::Blog, "Blog") }
                    </aside>
                    <div class="pl-3 pb-3 pt-2 bg-darkgreen text-sm">
                        <p class="text-darkblue">{ "Written by " }<br/><div><a class="text-beige font-medium truncate" href="https://github.com/JesperAxelsson/" target="_blank">{ "Jesper Axelsson" }</a></div></p>
                   </div>
                </div>
                <div class="bg-darkgreen w-full px-6 pt-6 ">
                    <section class="w-full bg-beige shadow h-full p-5 mr-3">
                        { self.render_body() }
                    </section>
                </div>
            </div>
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
            "py-1 pl-2 shadow bg-darkgreen text-beige-lighter font-medium "
        } else {
            "py-1 pl-2 text-darkblue "
        };

        html! {
            <a class="pl-4 cursor-pointer mb-2" onclick=self.link.callback(move |_| Msg::ChangePageState(page_state)) >
                <div class={link_class}><div class="pr-4 tracking-wider font-semibold" >{title}</div></div>
            </a>
        }
    }
}
