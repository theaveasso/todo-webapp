use yew::prelude::*;
use stylist::yew::styled_component;
use super::super::atoms::c_link::{ComLink, LinkType};
use crate::router::Route::{Home, CreateAccount, Login};

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let stylesheet = css!(
        r#"
            border-bottom: .2em solid;
            padding-bottom: .2em;
            display: flex;
            justify-content: space-between;
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
                    text="nostal 🦀"
                    data_test="logo"
                    route={Home} 
            />
            <div>
                <ComLink 
                        text="Create Account"
                        data_test="logo"
                        route={CreateAccount} 
                        link_type={LinkType::Button}
                />
                <ComLink 
                        text="Login"
                        data_test="login"
                        route={Login}
                        link_type={LinkType::Button}

            
                />
            </div> 
        </navbar>
    }
}