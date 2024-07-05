use dioxus::html::input_data::keyboard_types::{Key, Modifiers};
use dioxus::prelude::*;
use dioxus::desktop::Config;

fn main() {
    let cfg: Config;
    cfg = dioxus::desktop::Config::new().with_menu(None);
    
    
    
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}
// The #[component] attribute streamlines component creation.
// It's not required, but highly recommended. For example, UpperCamelCase components will not generate a warning.
#[component]
fn App() -> Element {
    let mut disabled = use_signal(|| false);
    rsx!{
        body {
            section { class: "flex items-center justify-center min-h-screen bg-gray-50",
                div{ 
                    class:"flex items-center max-w-3xl p-5 bg-gray-100 shadow-lg rounded-2xl"
                    }
                div { class: "px-8 md:w-1/2 md:px-16",
                    h2 { class: "text-2xl font-bold text-green-600",
                        "Регистрация в системе"
                    }
                    p { class: "text-sl mt-4 text-[#002D74]",
                        "Зарегистрировались на сервере, просто зайдите:)"
                    }
                    p { class: "text-sl mt-4 font-serif underline underline-offset-2 text-[#002D74]"
                    }
                    form { action: "", class: "flex flex-col gap-4",
                        input {
                            name: "family",
                            r#type: "family",
                            placeholder: "Фамилия",
                            class: "p-2 mt-8 border rounded-xl"
                        }
                        div { class: "relative",
                            input {
                                r#type: "name",
                                name: "name",
                                placeholder: "Имя",
                                class: "w-full p-2 border rounded-xl"
                            }
                            input {
                                r#type: "Group",
                                name: "Group",
                                placeholder: "Курс, группа",
                                class: "w-full p-2 mt-8 border rounded-xl"
                            }
                        }
                        button { class: "py-2 text-white duration-300 bg-green-600 rounded-xl hover:scale-105",
                            "Войти"
                            
                        }
                        button { class: "py-2 text-white duration-300 bg-green-600 rounded-xl hover:scale-105",
                            "Закрыть"
                            //button { onclick: move |event| log::info!("Clicked! Event: {event:?}"), "click me!" }
                            button { onclick: move |event |   }
                        }
                    }
                }
                div { class: "hidden w-1/2 md:block",
                    img {
                        src: "https://images.unsplash.com/photo-1616606103915-dea7be788566?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1887&q=80",
                        class: "rounded-2xl"
                    }
                }
            }
        }
    }
}

#[component]
fn PopUp(message: String) -> Element{
    rsx!{
        body {
            section { class: "flex items-center justify-center min-h-screen bg-gray-50",
                   div {
                    class: "flex items-center max-w-3xl p-5 bg-gray-100 shadow-lg rounded-2xl"
                   }
                div { class: "px-4 md:w md:px-16",
                    p { class: "text-start mt-4 text-[#002D74]",
                        "Одно и несколько полей не заполнены, заполните пж:)"
                    }
                    form { action: "", class: "flex flex-col gap-4" }
                }
            }
        }
    }
}