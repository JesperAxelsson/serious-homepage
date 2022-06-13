#![allow(dead_code)]
use yew::prelude::*;

pub struct Home {}

pub enum Msg {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        // match msg {
        //     Msg::Clicked => {
        //         self.onsignal.emit(());
        //     }
        // }
        false
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        // self.title = props.title;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { "Welcome to my new page! Works?
                    I will put some sort of image gallery here at some point..
                    Ah, yes. The colors and layout is horrible." }
            </div>
        }
    }
}
