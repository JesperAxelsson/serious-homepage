#![allow(dead_code)]
use log::*;
// use serde_derive::{Deserialize, Serialize};
// use yew::format::Json;
use yew::prelude::*;

use crate::views::Blog;
use crate::views::Gallery;
use crate::views::Home;
use crate::views::Recipies;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

// #[derive(Serialize, Deserialize)]
pub struct State {
    page_state: PageState,
}

// #[derive(Serialize, Deserialize)]
pub enum PageState {
    Home,
    Recipies,
    Gallery,
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
        <div class="max-h-screen antialiased" style="height: 100%;">
            <header class="pl-2 bg-gold shadow-xl">
                    <h1 class="text-2xl text-dark font-medium">
                        { "Welcome my homepage!" }
                    </h1>
                    <p class="text-sm text-darkblue">
                        { "Very nice site!" }
                    </p>
            </header>
            <div class="flex bg-beige">
                <div class="pl-2 shadow">
                    <aside class="pr-4 my-3" >
                        <ul>
                            <li><a class="text-darkblue" onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Home))>{"Home"}</a></li>
                            <li><a class="text-darkblue" onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Recipies))>{"Recipies"}</a></li>
                            <li><a class="text-darkblue" onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Gallery))>{"Gallery"}</a></li>
                            <li><a class="text-darkblue" onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Blog))>{"Todo"}</a></li>
                            <li><a class="text-darkblue" onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Blog))>{"Blog"}</a></li>
                        </ul>
                    </aside>
                </div>
                <section class="w-full bg-darkgreen">
                    { self.render_body() }
                </section>
            </div>
            <footer class="flex text-sm w-full bg-beige">
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
                <h3 class="title is-4 is-spaced">
                    {
                        match self.state.page_state {
                            PageState::Home => "Home!",
                            PageState::Recipies => "Recipies!",
                            PageState::Gallery => "Gallery!",
                            PageState::Blog => "Blog!"
                        }
                    }
                </h3>
                {
                    match self.state.page_state {
                        PageState::Home => html!{<Home /> },
                        PageState::Recipies => html!{<Recipies /> },
                        PageState::Gallery => html!{<Gallery /> },
                        PageState::Blog => html!{<Blog /> },
                    }
                }
            </section>
        }
    }
}
