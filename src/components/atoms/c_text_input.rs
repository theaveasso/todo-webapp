use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, PartialEq)]
pub enum InputType {
    Text,
    Password
}
impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Text => "text".to_owned(),
            InputType::Password => "password".to_owned()
        }
    }
    
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub label: String,
    pub placeholder: Option<String>,
    pub inputtype: InputType
}
#[styled_component(ComTextInput)]
pub fn com_text_input(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        margin: 1em 0 1em 0;
        display:flex;
        justify-content:center;

        label {
            font-size: 1em;
            margin-right: 1em;

        }
        input {
            top: 1em;
            width: 50vw;
            border: none;
            border-bottom: 2px solid #B1A7A6;
            background-color: #161A1D;
            color: white;
        }
        "#
    );
    
    let id = props.label.to_lowercase().replace(" ", "-");
    let placeholder = props.placeholder.clone().unwrap_or_default();

    html!(
        <>
            <div class={stylesheet}>
                <div>
                    <label for={id.clone()}>{&props.label}</label>
                </div>
                <div>
                    <input type={props.inputtype.to_string()}
                    id={id} {placeholder} 
                    data-test={props.data_test.clone()}
                     />
                </div>
            </div>
        </>
    )
}
