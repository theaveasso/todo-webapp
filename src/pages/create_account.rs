use yew::prelude::*;
use crate::components::atoms::c_text_input::ComTextInput;

#[function_component(CreateAccount)]
pub fn create_account() -> Html {
    html! {
        <>
        <h1>{ "Create Account" }</h1>
        <section>
            <ComTextInput data_test="username" label="Username" placeholder="Please Input your name..." />
        </section>
        </>
    }
}