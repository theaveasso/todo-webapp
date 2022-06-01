use yew::prelude::*;
use stylist::yew::styled_component;
use gloo::console::log;
use crate::components::atoms::{c_text_input::{ComTextInput, InputType}, c_button::ComButton};

#[derive(Default)]
pub struct State {
    pub username: String,
    pub password: String
}

#[styled_component(Login)]
pub fn login() -> Html {
    let stylesheet = css!(
        r#"
            display: flex;
            justify-content: center;

            section > form {
                width: 75%;
            }
    "#);
    let username_onchange = Callback::from(|username: String|{
        log!(username);
    });

    let password_onchange = Callback::from(|password: String|{
        log!(password);
    });
    html! {
        <>
        <h1 style="text-align : center;">{ "Login" }</h1>
        <section class={stylesheet}>
            
            <form>
                <ComTextInput data_test="username" 
                label="Username:  " 
                inputtype = {InputType::Text}
                placeholder="Please Input your name..." 
                onchange = {username_onchange}/>
                
                <ComTextInput data_test="password" 
                label="Password:  "
                inputtype = {InputType::Password}
                placeholder="Please Input your password..." 
                onchange = {password_onchange}/>
                
                <ComButton data_test="summit"
                label="Sign in" />
            </form>
            
        </section>
        </>
    }
}