#![allow(dead_code)]

use yew::prelude::*;

pub struct Gallery {
    link: ComponentLink<Self>,
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for Gallery {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Gallery {
            link,
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

    // fn change(&mut self, props: Self::Properties) -> ShouldRender {
    //     self.title = props.title;
    //     self.onsignal = props.onsignal;
    //     true
    // }

    fn view(&self) -> Html {
        html! {
            <div>
                { "Gallery!" }
            </div>
        }
    }
}