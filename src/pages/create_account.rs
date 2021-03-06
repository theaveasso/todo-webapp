use yew::prelude::*;
use gloo::console::log;
use stylist::yew::styled_component;
use crate::components::atoms::{c_text_input::{ComTextInput, InputType}, c_button::ComButton};

#[derive(Default)]
pub struct State {
    pub username: String,
    pub password: String
}

#[styled_component(CreateAccount)]
pub fn create_account() -> Html {
    let stylesheet = css!(
        r#"
            display: flex;
            justify-content: center;

            section > form {
                width: 75%;
            }
    "#);



    let state = use_state(State::default);
    // event on summit
    let onsubmit = Callback::from(|event: FocusEvent| {
        event.prevent_default();
    });
    let username_onchange = Callback::from(|username: String|{
        log!(username);
    });

    let password_onchange = Callback::from(|password: String|{
        log!(password);
    });

    html! {
        <>
        <h1 style="text-align : center;">{ "Create Account" }</h1>
        <section class={stylesheet}>
            
            <form {onsubmit}>
                <ComTextInput data_test="username" 
                label="Username:  " 
                inputtype = {InputType::Text}
                placeholder="Please Input your name..." 
                onchange={username_onchange}/>
                
                <ComTextInput data_test="password" 
                label="Password:  "
                inputtype = {InputType::Password}
                placeholder="Please Input your password..." 
                onchange={password_onchange}/>
                
                <ComButton data_test="summit"
                label="Create Account" />
            </form>
            
            
        </section>
        </>
    }

}