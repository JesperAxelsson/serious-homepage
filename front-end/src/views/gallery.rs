#![allow(dead_code)]

use yew::prelude::*;
use crate::fetch::*;

pub struct Gallery {
    link: ComponentLink<Self>,
    state: GalleryState,
}

pub enum GalleryState {
    Albums,
    Images,   
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Gallery {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gallery {
            link,
            state: GalleryState::Albums,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        // match msg {
        //     Msg::Clicked => {
        //         self.onsignal.emit(());
        //     }
        // }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.title = props.title;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        let album = Album {
            id: 1, 
            title: "Jutenhaim 2021".to_string(),
            description: "Things that happened".to_string(),
            image_url: "https://images.unsplash.com/photo-1573134806390-786f85c2e828?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=300&h=300&q=80".to_string(),
        };

        let album2 = Album {
            id: 1, 
            title: "Spain".to_string(),
            description: "Things that happened".to_string(),
            image_url: "https://images.unsplash.com/photo-1573134806390-786f85c2e828?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=300&h=300&q=80".to_string(),
        };

        match self.state {
            GalleryState::Albums => html! {
                // <div class="flex flex-col overflow-hidden">
                    // <div class="h-full">
                <div class="flex flex-col overflow-hidden h-full p-3">
                    <div class="flex-1 overflow-y-auto">
                        <div class="flex flex-wrap overflow-y-visible  justify-between">
                        // <div class="overflow-y-auto overflow-x-hidden flex flex-wrap ">
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_album_card(&album) }}
                            {{ render_album_card(&album2) }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                            {{ render_empty_card() }}
                        </div>
                    </div>
                </div>
            },
            GalleryState::Images => html! {
                <div>{ "Neat" } </div>
            },
        }
    }
}

fn render_album_card(album: &Album) -> Html {
    html!{
        <div class="shadow-xl bg-darkgreen h-40 w-32 mr-2 mb-2 pb-1 flex-none  ">
            <img src=&album.image_url class="p-1 h-32"/>

            <div class="text-sm text-beige-lighter font-medium text-center">
                {{ &album.title }}
            </div>
        </div>
    }
}

fn render_empty_card() -> Html {
    html!{
        <div class="h-1 w-32 mr-2 mb-2 pb-1 invisible">
            {"\u{00a0}" }
        </div>
    }
}