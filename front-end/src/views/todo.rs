use yew::prelude::*;

pub struct Todo {}

impl Component for Todo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Todo {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // self.title = props.title;
        // self.onsignal = props.onsignal;
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { "Everyone needs a Todo right?" }
            </div>
        }
    }
}
