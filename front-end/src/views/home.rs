use yew::prelude::*;

pub struct Home {
    link: ComponentLink<Self>,
}

pub enum Msg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
}

impl Component for Home {
    type Message = Msg;
    type Properties = Props;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Home {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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
            <section>
                <h2> {"Home"} </h2>
                { "home!" }
            </section>
        }
    }
}