use yew::prelude::*;
use stylist::yew::styled_component;
use crate::components::atoms::c_text_input::{ComTextInput, InputType};

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = css!(
        r#"
            display:flex;
            flex-direction: column;
    "#);
    html! {
        <>
        <h1>{ "Create Account" }</h1>
        <section class={stylesheet}>
            <ComTextInput data_test="username" 
            label="Username:  " 
            inputtype = {InputType::Text}
            placeholder="Please Input your name..." />
            <ComTextInput data_test="password" 
            label="Password:  "
            inputtype = {InputType::Password}
            placeholder="Please Input your password..." />
        </section>
        </>
    }
}