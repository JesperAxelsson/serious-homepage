use yew::prelude::*;
// use comrak::{markdown_to_html, ComrakOptions};
// use crate::markdown;
use pulldown_cmark::{html, Options, Parser};

// pub struct Recipe {
//     pub id: i64,
//     pub title: String,
//     pub text: String,
// }

pub struct ViewRecipe {
    markdown: String,
}

impl Component for ViewRecipe {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ViewRecipe {
            markdown: r#"
# Banana pancakes!
## fo real?            
Hello, **世界**!"#.to_string(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <article class="markdown-body">
            { render_mark(&self.markdown) }
            // render( markdown_to_html(&self.markdown, &ComrakOptions::default()) )
            </article>
        }
        // htmlMark(&renderMark(&self.markdown))
    }
}

fn render_mark(mark: &str) -> Html {

    use web_sys::{console, Node};
    use yew::virtual_dom::VNode;
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let options = Options::empty();
    // options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(mark, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(mark.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    let elem = {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&html_output);
        console::log_1(&div);
        div
    };
    
    let node = Node::from(elem);
    let vnode = VNode::VRef(node);
    vnode
}

// fn htmlMark(html: &str) -> Html {
//     use web_sys::{console, Node};
//     use yew::virtual_dom::VNode;
//     use yew::{Component, ComponentLink, Html, ShouldRender};
 
//     let div = VNode::from(html);
//     let js_svg = {
//         let div = web_sys::window()
//             .unwrap()
//             .document()
//             .unwrap()
//             .create_element("div")
//             .unwrap();
//         div.set_inner_html(html);
//         console::log_1(&div);
//         div
//     };
    
//     let node = Node::from(html);
//     let vnode = VNode::VRef(node);
//     vnode
// }
