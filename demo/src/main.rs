#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that will render all the basic divs -- initiliaze state -- call other components 
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        }
    })
}


/*
Each individual component to render each part of the ui will have its own function -- will probably just have basic input that specifies either ID or key -- and a textbox to take it in
Then theres a button that says Get, and then the json output will be displayed in a textbox
*Need to figure out blocks as well work in prog -- post will be more complicated but probably not terrible -- probably won't do transfers because of time crunch but GET and POST are musts
*Will have another mostly static page with some basic documentation -- so I do need a router -- no state changes tho tbh 
*/



