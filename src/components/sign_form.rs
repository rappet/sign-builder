use std::rc::Rc;

use base64::prelude::*;
use ulid::Ulid;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, SubmitEvent};
use yew::{function_component, html, Callback, Html, Properties, TargetCast};
use yew_hooks::use_title;
use yew_router::prelude::*;

use crate::{models::*, route::Route, services::signs::add_sign};

#[derive(Properties, PartialEq)]
pub struct SignFormProps {
    pub value: Rc<Sign>,
    pub on_change: Callback<Rc<Sign>>,
}

#[function_component]
pub fn SignForm(props: &SignFormProps) -> Html {
    let sign = &props.value;
    let navigator = use_navigator();
    use_title(format!("{} - Edit Sign", sign.title));

    let on_room_change = {
        let sign = Rc::clone(&props.value);
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            if let Ok(room) = input.value().parse() {
                on_change.emit(Rc::new(Sign {
                    room,
                    ..sign.as_ref().clone()
                }));
            }
        })
    };

    let on_title_change = {
        let sign = Rc::clone(&props.value);
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_change.emit(Rc::new(Sign {
                title: input.value(),
                ..sign.as_ref().clone()
            }));
        })
    };

    let on_subtitle_change = {
        let sign = Rc::clone(&props.value);
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_change.emit(Rc::new(Sign {
                subtitle: input.value(),
                ..sign.as_ref().clone()
            }));
        })
    };

    let on_url_change = {
        let sign = Rc::clone(&props.value);
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
            on_change.emit(Rc::new(Sign {
                url: input.value(),
                ..sign.as_ref().clone()
            }));
        })
    };

    let on_content_change = {
        let sign = Rc::clone(&props.value);
        let on_change = props.on_change.clone();

        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
            on_change.emit(Rc::new(Sign {
                content: input.value(),
                ..sign.as_ref().clone()
            }));
        })
    };

    let on_submit = {
        let sign = Rc::clone(&props.value);
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            spawn_add_sign(Rc::clone(&sign));

            let encoded_sign = encode_sign(&sign);
            if let Some(navigator) = &navigator {
                navigator.push(&Route::Display { data: encoded_sign });
            }
        })
    };

    html! {
        <form class="flex flex-col" onsubmit={on_submit}>
            <span class="inline-block py-4 pl-28 [&>svg]:w-24 [&>svg]:h-24 [&>svg]:stroke-black [&>svg]:fill-black">{sign.room.icon().map(|icon| Html::from_html_unchecked(icon.into()))}</span>
            <div class="form-row">
                <label for="room">{"Room:"}</label>
                <select type="text" id="room" name="room" class="text-xl" value={sign.room.id()} onchange={on_room_change}>
                    { Room::all().iter().map(|v| html! {
                        <option key={v.id()} value={v.id()} selected={*v == sign.room}>{v.name(Language::En)}</option>
                    }).collect::<Html>() }
                </select>
            </div>
            <div class="form-row">
                <label for="title">{"Title:"}</label>
                <input type="text" id="title" name="title" class="text-2xl" value={sign.title.clone()} onchange={on_title_change} />
            </div>
            <div class="form-row">
                <label for="subtitle">{"Subtitle:"}</label>
                <input type="text" id="subtitle" name="subtitle" class="text-xl" value={sign.subtitle.clone()} onchange={on_subtitle_change} />
            </div>
            <div class="form-row">
                <label for="url">{"Url:"}</label>
                <input type="text" id="url" name="url" class="text-xl" value={sign.url.clone()} onchange={on_url_change} />
            </div>

            <div class="form-col">
                <label for="content">{"Content:"}</label>
                <textarea rows="10" type="text" id="content" name="content" class="text-xl" value={sign.content.clone()} onchange={on_content_change} >
                </textarea>
            </div>

            <input type="submit" value="Create Sign" class="m-4 text-xl p-4"/>
        </form>
    }
}

fn encode_sign(sign: &Sign) -> String {
    let serialized = serde_json::to_vec(&sign).expect("serializing does not fail");
    let encoded = BASE64_URL_SAFE.encode(&serialized);
    encoded
}

fn spawn_add_sign(mut sign: Rc<Sign>) {
    Rc::make_mut(&mut sign).id = Some(Ulid::new());

    wasm_bindgen_futures::spawn_local(async move {
        add_sign(sign).await.unwrap();
    });
}
