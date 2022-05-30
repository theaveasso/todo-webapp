use yew::prelude::*;
use stylist::yew::styled_component;
use crate::atoms::c_link::ComLink;
use crate::router::Route::{Home, CreateAccount};

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = css!(
        r#"
            border-bottom: .2em solid;
            padding-bottom: .2em;
            display: block;
            margin: 2em 5em;

            h2 {
                color: #E5383B;
                display: inline;
                padding-bottom: 5em;
            }


        
        "#
    );

    html! {
        <navbar class={stylesheet}>
            <ComLink 
                    text="todo-app 🦀"
                    data_test="logo"
                    route={Home} 
            />

            <ComLink 
                    text="create-account"
                    data_test="logo"
                    route={CreateAccount} 
            />
        </navbar>
    }
}