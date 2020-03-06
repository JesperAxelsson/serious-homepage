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

    fn view(&self) -> Html {
        html! {
            <div>
                { "Everyone needs a Todo right?" }
            </div>
        }
    }
}
