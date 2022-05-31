use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub label: String,
    pub placeholder: Option<String>
}
#[styled_component(ComTextInput)]
pub fn com_text_input(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        label {
            font-size: 1em;
        }
        "#
    );
    
    let id = props.label.to_lowercase().replace(" ", "-");
    let placeholder = props.placeholder.clone().unwrap_or_default();

    html!(
        <>
            <div class={stylesheet}>
                <label for={id.clone()}>{&props.label}</label>
            </div>
            <div>
                <input type="text" id={id} {placeholder} data-test={props.data_test.clone()} />
            </div>

        </>
    )

}