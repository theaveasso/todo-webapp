use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub label: String
}
#[styled_component(ComButton)]
pub fn com_button(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
            
            width: 100%;
            
            margin: 1em 0;
            background-color: #E5383B;
            border: None;
            border-radius: 20px;
            height: 30px;
            font-size: 1em;
            line-height: 20px;
        "#
    );
    
    html!(
        <button data-test={props.data_test.clone()} class={stylesheet}> {&props.label}</button>
    )
}