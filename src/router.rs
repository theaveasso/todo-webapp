use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{create_account::CreateAccount, home::Home};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create-account")]
    CreateAccount
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!{ <Home /> },
        Route::CreateAccount => html!{ <CreateAccount /> }
    }
}