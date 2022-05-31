mod pages;
mod router;
mod components;
// components

use yew::prelude::*;
use yew_router::prelude::*;
use router::{Route, switch};

use components::organisms::navbar::Navbar;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render = {Switch::render(switch)} />
        </BrowserRouter>
    }
}