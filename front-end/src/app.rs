use log::*;
// use serde_derive::{Deserialize, Serialize};
// use yew::format::Json;
use yew::prelude::*;

use crate::views::home::Home;
use crate::views::gallery::Gallery;
use crate::views::recipies::Recipies;

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

        <div class="container ">

            <header class="section is-desktop">
                    <h1 class="title">
                        { "Welcome my homepage!" }
                    </h1>
                    <p class="subtitle">
                        { "Very nice site!" }
                    </p>
            </header>
            <div class="columns is-tablet">
                <div class="column is-2">
                    <aside class="menu" >
                        <p class="menu-label">
                            {"Pick your poison"}
                        </p>
                        <ul>
                            <li><a onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Home))>{"Home"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Recipies))>{"Recipies"}</a></li>
                            <li><a onclick=self.link.callback(|_| Msg::ChangePageState(PageState::Gallery))>{"Gallery"}</a></li>
                        </ul>
                    </aside>
                </div>
                <section class="column is-pulled-right">
                    { self.render_body() }
                </section>
            </div>
            <footer class="section">
                <div class="content has-text-centered">
                    <p>{ "Written by " }<a href="https://github.com/JesperAxelsson/" target="_blank">{ "Jesper Axelsson" }</a></p>
                </div>
            </footer>
        </div>
        }
    }
}

impl App {
    fn render_body(&self) -> Html {
        match self.state.page_state {
            PageState::Home => html!{<Home /> },
            PageState::Recipies => html!{<Recipies /> },
            PageState::Gallery => html!{<Gallery /> },
        }
    }
}

