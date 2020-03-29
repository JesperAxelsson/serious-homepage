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
            page_state: PageState::Blog,
            // page_state: PageState::Home,
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
            <header class="sm:pl-6 h-12 sm:h-20 bg-gold shadow flex flex-shrink-0">
                <div class="self-center mx-auto sm:m-0">
                    <h1 class="text-center text-3xl sm:text-4xl font-display text-darkblue font-black leading-tight ">
                        { "Welcome my homepage!" }
                    </h1>
                    // <p class="text-sm font-subheader text-light text-darkblue">
                    //     { "Very nice site!" }
                    // </p>
                </div>
            </header>
            <div class="flex flex-col sm:flex-row h-full">
                <div class="w-auto flex-grow-0 sm:flex-grow-1 h-full flex flex-col justify-between">
                    <div class="px-2 sm:px-0 sm:py-3 shadow sm:flex-grow-1 sm:h-full inline-flex sm:flex-col" >
                        { self.render_link(PageState::Home, "Home") }
                        { self.render_link(PageState::Recipies, "Recipies") }
                        { self.render_link(PageState::Gallery, "Gallery") }
                        { self.render_link(PageState::Todo, "Todo") }
                        { self.render_link(PageState::Blog, "Blog") }
                    </div>
                    <div class="hidden sm:inline-flex sm:pl-3 sm:pb-3 sm:pt-2 bg-darkgreen text-sm sm:text-regular">
                        <p class="text-darkblue">{ "Written by " }<br/><div><a class="text-beige font-medium truncate" href="https://github.com/JesperAxelsson/" target="_blank">{ "Jesper Axelsson" }</a></div></p>
                   </div>
                </div>
                <div class="bg-darkgreen w-full px-4 pt-4 sm:px-6 sm:pt-6 flex-grow flex">
                    <section class="w-full bg-beige flex flex-grow shadow p-5 sm:mr-3">
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
            <a class="sm:pl-4 cursor-pointer sm:mb-2" onclick=self.link.callback(move |_| Msg::ChangePageState(page_state)) >
                <div class={link_class}><div class="text-sm pr-1 sm:pr-4 tracking-wider font-semibold " title={title} > {title} </div></div>
            </a>
        }
    }
}
