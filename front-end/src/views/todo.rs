use yew::prelude::*;

pub struct Todo {}

impl Component for Todo {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Todo {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // self.title = props.title;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { "Everyone needs a Todo right?" }
            </div>
        }
    }
}
