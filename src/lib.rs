mod pages;
mod router;

// components
mod atoms;
mod organisms;

use yew::prelude::*;
use yew_router::prelude::*;
use router::{Route, switch};

use organisms::navbar::Navbar;
use atoms::c_link;


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Navbar />
                <Switch<Route> render = {Switch::render(switch)} />
            </BrowserRouter>
        </>
    }
}