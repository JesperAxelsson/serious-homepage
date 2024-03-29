#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::fetch::*;
use yew::{html::Scope, prelude::*};

use log::*;

pub struct Gallery {
    state: PageState,

    albums: FetchState<Vec<Album>>,
    images: FetchState<Vec<Image>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Album {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub preview_url: String,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PageState {
    Albums,
    Images(i64),
    ShowImage(i64, i64),
}

#[derive(Debug)]
pub enum Msg {
    FetchAlbum(FetchState<Vec<Album>>),
    StartFetchAlbum,
    FetchImages(FetchState<Vec<Image>>),
    StartFetchImages(i64),
    ChangePageState(PageState),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Gallery {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Gallery {
            state: PageState::Albums,
            albums: FetchState::NotFetching,
            images: FetchState::NotFetching,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartFetchAlbum => {
                info!("New fetch album!");
                self.images = FetchState::NotFetching;
                let future = async {
                    match fetch_url2("/api/gallery").await {
                        Ok(md) => Msg::FetchAlbum(FetchState::Success(md)),
                        Err(err) => Msg::FetchAlbum(FetchState::Failed(err)),
                    }
                };

                send_future(&ctx.link(), future);

                ctx.link()
                    .send_message(Msg::FetchAlbum(FetchState::Fetching));

                false
            }
            Msg::FetchAlbum(fetch_state) => {
                info!("Set_fetch!");

                if let FetchState::Success(ref val) = fetch_state {
                    info!("Coool! {:?}", val);
                }

                self.albums = fetch_state;

                true
            }

            Msg::StartFetchImages(album_id) => {
                info!("New fetch images!");
                // ConsoleService::log(&format!("Fetching album images: {:?}", album_id));

                let future = async move {
                    match fetch_url2(&format!("/api/gallery/{}", album_id)).await {
                        Ok(md) => Msg::FetchImages(FetchState::Success(md)),
                        Err(err) => Msg::FetchImages(FetchState::Failed(err)),
                    }
                };

                send_future(&ctx.link(), future);

                ctx.link()
                    .send_message(Msg::FetchImages(FetchState::Fetching));

                false
            }
            Msg::FetchImages(fetch_state) => {
                info!("Set_fetch images!");

                if let FetchState::Success(ref val) = fetch_state {
                    info!("Coool, got images! {:?}", val);
                }

                self.images = fetch_state;

                true
            }
            Msg::ChangePageState(new_state) => {
                // ConsoleService::log(&format!("New page state: {:?}", new_state));
                self.state = new_state;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.state = PageState::Albums;
        self.images = FetchState::NotFetching;
        // ConsoleService::log("Changing");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match self.state {
            PageState::Albums => html! {
                // <div class="flex flex-col overflow-hidden">
                    // <div class="h-full">
                <div class="flex flex-col overflow-hidden h-full p-3">
                    <div class="flex-1 overflow-y-auto">
                        <div class="flex flex-wrap overflow-y-visible  justify-between">
                        // <div class="overflow-y-auto overflow-x-hidden flex flex-wrap ">
                        {
                            match &self.albums {
                                FetchState::NotFetching => {
                                    ctx.link().send_message(Msg::StartFetchAlbum);
                                    html! {"Fetching albums..."}
                                },

                                FetchState::Fetching => html! {"Fetching getting it!"},
                                FetchState::Success(data) => html! {

                                    { data.iter().map(|rec| render_album_card(rec, &ctx.link())).collect::<Html>() }
                                },
                                FetchState::Failed(err) => html! {&err},
                            }
                        }

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

            PageState::Images(album_id) => html! {
                <div class="flex flex-col overflow-hidden h-full p-3">
                    <div class="flex-1 overflow-y-auto">
                        <div class="flex flex-wrap overflow-y-visible  justify-between">
                        {
                            match &self.images {
                                FetchState::NotFetching => {
                                    ctx.link().send_message(Msg::StartFetchImages(album_id));
                                    html! {"Fetching images..."}
                                },

                                FetchState::Fetching => html! {"Fetching getting it!"},
                                FetchState::Success(data) => html! {

                                    { data.iter().map(|rec| render_image_card(album_id, rec, &ctx.link())).collect::<Html>() }
                                },
                                FetchState::Failed(err) => html! {&err},
                            }
                        }

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

            PageState::ShowImage(album_id, image_id) => {
                let mut image: Option<&Image> = Option::None;

                if let FetchState::Success(images) = &self.images {
                    // let img = images.iter().find(|e| e.id == image_id).clone();
                    image = images.iter().find(|e| e.id == image_id);
                }

                if let Some(image) = image {
                    let url = create_url(&format!("/api/gallery/{}/{}", album_id, image.image_url));
                    return html! {
                    <div class="flex flex-col overflow-hidden h-full p-3">
                        <div class="flex-1 overflow-y-auto">
                            <div class="flex flex-wrap overflow-y-visible  justify-between">
                                <div class="shadow-xl bg-darkgreen mr-2 mb-4 pb-1 " >
                                    <img src={url.clone()} class="p-1  "/>

                                    <div class="text-sm text-beige-lighter font-medium text-center">
                                        {{ &image.title }}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                     };
                } else {
                    return html! {
                       <div class="shadow-xl bg-darkgreen   mr-2 mb-4 pb-1 flex-none" >
                           { "Failed to get image" }
                       </div>
                    };
                }
            }
        }
    }
}

fn render_album_card(album: &Album, link: &Scope<Gallery>) -> Html {
    let album_id = album.id;
    let url = create_url(&format!("/api/gallery/{}/{}", album.id, album.image_url));

    html! {
        <div class="shadow-xl bg-darkgreen h-40 w-32 mr-2 mb-4 pb-1 flex-none"
             onclick={link.callback(move |_| Msg::ChangePageState(PageState::Images(album_id)))}>
            <img src={url.clone() }class="p-1 h-32"/>

            <div class="text-sm text-beige-lighter font-medium text-center">
                {{ &album.title }}
            </div>
        </div>
    }
}

fn render_empty_card() -> Html {
    html! {
        <div class="h-1 w-32 mr-2 mb-2 pb-1 invisible">
            {"\u{00a0}" }
        </div>
    }
}

fn render_image_card(album_id: i64, image: &Image, link: &Scope<Gallery>) -> Html {
    let id = image.id;
    let url = create_url(&format!("/api/gallery/{}/{}", album_id, image.image_url));
    let title = image.title.clone();

    html! {
        <div class="shadow-xl bg-darkgreen h-40 w-32 mr-2 mb-4 pb-1 flex-none"
             onclick={link.callback(move |_| Msg::ChangePageState(PageState::ShowImage(album_id, id))) }>
            <img src={url.clone()} class="p-1 h-32"/>

            <div class="text-sm text-beige-lighter font-medium text-center">
                {{ title }}
            </div>
        </div>
    }
}
