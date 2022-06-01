use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::Route;
use stylist::{yew::styled_component, css, StyleSource};

#[derive(Clone, PartialEq)]
pub enum LinkType {
    Link,
    Button
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    pub route: Route,
    pub link_type: Option<LinkType>
}
impl Default for LinkType{
    fn default() -> Self {
        Self::Link
    }    
}
#[styled_component(ComLink)]
pub fn com_link(props: &Props) -> Html{
    

    let link_type = props.link_type.clone().unwrap_or_default();
    let stylesheet = select_style(link_type);
    html!(
        <span data_test={props.data_test.clone()}>
        <Link<Route> 
        to={props.route.clone()} 
        classes={classes!(stylesheet)}>{props.text.clone()}</Link<Route>>
        </span>
    )
}

fn select_style(link_type: LinkType) -> StyleSource<'static> {
    let link_stylesheet = css!(
        r#"
            color: #D3D3D3;
            text-decoration: None;
            font-size: 20px;

    "#);
    let button_stylesheet =  css!(
        r#"
            text-decoration: None;
            border-radius: 20px;
            color: #161A1D;
            background-color: #E5383B;
            padding: 0 1em;
            font-size: 18px;
            margin: 0 0.5em;
    "#);
    match link_type {
        LinkType::Link => link_stylesheet,
        LinkType::Button => button_stylesheet
    }
}