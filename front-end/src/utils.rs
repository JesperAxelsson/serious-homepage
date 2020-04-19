pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use yew::virtual_dom::VNode;
use yew::html::Html;

pub fn render_text_as_html(html: &str) -> Html {
    use web_sys::Node;

    let js_svg = {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(html);
        div
    };

    let node = Node::from(js_svg);
    use web_sys::{console};

   console::log_1(&node);

    VNode::VRef(node)
}
