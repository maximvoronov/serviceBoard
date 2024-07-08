
use crate dioxus::event::*;


pub struct ToastMessage{
    pub message: String,
    pub toast_type: String,
    pub visible: bool,
};



[#component]
pub fn ToastMessageApp() -> Element
{
    
}

#[derive(PartialEq, Clone, Props)]
pub struct FancyButtonProps {
    onclick: EventHandler<MouseEvent>,
}

pub fn FancyButton(props: FancyButtonProps) -> Element {
    rsx! {
        button {
            class: "fancy-button",
            onclick: move |evt| props.onclick.call(evt),
            "click me pls."
        }
    }
}
