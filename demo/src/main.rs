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
        
        div{
            class: "h-screen",

        
        
        nav {
            div{
                class: "max-w bg-white shadow-lg flex items-center h-14 mb-24",
                div {
                    class: "ml-4 text-xl flex-shrink-0 text-sky-600",
                    "🦀 Rust SDK for ResDB"
                }
                a {
                    href: "/",
                    class: "ml-20 text-lg text-sky-500",
                    "Home"
                }
                
                a {
                    href: "/",
                    class: "ml-4 text-lg text-sky-500",
                    "Documentation"
                    
                }
            }

        }

        section {
            class: "h-screen",
            div {
                //Turn this flexbox into a grid later on because post is gonna have to be on the same page tbh
                class: "flex flex-col items-start h-2/3 w-11/12 mx-auto bg-sky-300 shadow-lg bg-opacity-50",
                GetComp{

                }
            }
        }
        
    }
    })
}


/*
Each individual component to render each part of the ui will have its own function -- will probably just have basic input that specifies either ID or key -- and a textbox to take it in
Then theres a button that says Get, and then the json output will be displayed in a textbox
*Need to figure out blocks as well work in prog -- post will be more complicated but probably not terrible -- probably won't do transfers because of time crunch but GET and POST are musts
*Will have another mostly static page with some basic documentation -- so I do need a router -- no state changes tho for documentation
*/


pub fn GetComp (cx: Scope) -> Element {

    //Create 2 use-state hooks
    //1 For the selection of the Input 
    //2 For what the JSON will output

    let get_type = use_state(cx,|| "all".to_string());
    let get_text = use_state(cx, || String::new());
    let final_text = use_state(cx, || String::new());



    cx.render(rsx! {
        div{
            class: "ml-6 mt-6",
            div{
                
                class: "mt-3 text-sm",

                "Select an option: (get all by default)"
            }
                select{

                    class: "mt-8",
                    
                    onchange: move |event| {
                        get_type.set(event.data.value.clone());


                    },
                    option{
                        value: "all",
                        "Get all transactions"
                        
                    }
                    option{
                        value: "key",
                        "Get transaction by key"

                    }
                    option{
                        value: "id",
                        "Get transaction by id"
                    }
                }
                div {
                    class: "mt-10",
                    input {
                        onchange: move |e| {
                            get_text.set(e.data.value.clone());
                        },
                        id: "state-1",
                    }
                }

                div {
                    class: " mt-16 px-7 py-2.5 w-20 font-medium bg-blue-50 hover:bg-blue-100 hover:text-blue-600 text-blue-500 rounded-lg text-sm",
                    button{
                        onclick: move |_| {
                            final_text.set(get_text.to_string().clone());
                        },
                        "GET"
                    }
                }
                // Ignore this for right now -- it's just to test state
                div {
                    class: "mt-16 w-36",
                    input{
                        value: "{final_text}",
                    }
                }

                
            


            
    }

    })

}


