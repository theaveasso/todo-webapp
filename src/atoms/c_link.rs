use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use stylist::{yew::styled_component, css};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub route: Route
}

#[styled_component(ComLink)]
pub fn com_link(props: &Props) -> Html{
    let stylesheet = css!(
        r#"
            color: #E5383B;
            text-decoration: None;

    "#);

    html!(
        <span data_test={props.data_test.clone()}>
        <Link<Route> 
        to={props.route.clone()} 
        classes={classes!(stylesheet)}>{props.text.clone()}</Link<Route>>
        </span>
    )
}