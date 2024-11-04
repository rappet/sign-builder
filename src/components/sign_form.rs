use std::rc::Rc;

use base64::prelude::*;
use ulid::Ulid;
use web_sys::{
    Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent, MouseEvent,
    SubmitEvent,
};
use yew::{classes, function_component, html, Callback, Html, Properties, TargetCast};
use yew_hooks::use_title;
use yew_router::prelude::*;

use crate::{models::*, route::Route, services::signs::add_sign};

turf::style_sheet!("src/components/sign_form.scss");

#[derive(Properties, PartialEq)]
pub struct SignFormProps {
    pub value: Rc<Sign>,
    pub on_change: Callback<Rc<Sign>>,
}

#[function_component]
pub fn SignForm(props: &SignFormProps) -> Html {
    let sign = &props.value;
    let navigator = use_navigator();
    use_title(if sign.title.trim().is_empty() {
        format!("<empty sign> - Edit Sign")
    } else {
        format!("{} - Edit Sign", sign.title.trim())
    });

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

        Callback::from(move |e: InputEvent| {
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

        Callback::from(move |e: InputEvent| {
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

        Callback::from(move |e: InputEvent| {
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

        Callback::from(move |e: InputEvent| {
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
        <form class={ClassName::FORM} onsubmit={on_submit}>
            <style>{STYLE_SHEET}</style>
            <div class={ClassName::FORM_COL}>
                <span class={ClassName::SCREEN_READER}><label for="room">{"Room:"}</label></span>
                <select type="text" id="room" name="room" class={ClassName::SCREEN_READER} value={sign.room.id()} onchange={on_room_change}>
                    { Room::all().iter().map(|v| html! {
                        <option key={v.id()} value={v.id()} selected={*v == sign.room}>{v.name(Language::En)}</option>
                    }).collect::<Html>() }
                </select>
                <div class={ClassName::ROOM_SELECTOR}>
                    { Room::all().iter().map(|v| {
                        let is_selected = sign.room == *v;
                        let on_change = props.on_change.clone();
                        let sign = sign.clone();
                        let cb = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            on_change.emit(Rc::new(Sign {
                                room: *v,
                                ..sign.as_ref().clone()
                            }));
                        });
                        html! {
                            <span class={v.color().accent_class()}>
                                <a href="#" onclick={cb} class={classes!(is_selected.then_some(ClassName::SELECTED))}>
                                    {Html::from_html_unchecked(v.icon().unwrap_or_default().into())}
                                    <span class={ClassName::ROOM_NAME}>{v.name(Language::En)}</span>
                                </a>
                            </span>
                        }
                    }).collect::<Html>() }
                </div>
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="title">{"Title:"}</label>
                <input type="text" id="title" name="title" value={sign.title.clone()} oninput={on_title_change} />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="subtitle">{"Subtitle:"}</label>
                <input type="text" id="subtitle" name="subtitle" value={sign.subtitle.clone()} oninput={on_subtitle_change} />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="url">{"Url:"}</label>
                <input type="text" id="url" name="url" value={sign.url.clone()} oninput={on_url_change} />
            </div>

            <div class={ClassName::FORM_COL}>
                <label for="content">{"Content:"}</label>
                <textarea rows="10" type="text" id="content" name="content" value={sign.content.clone()} oninput={on_content_change} >
                </textarea>
            </div>

            <input type="submit" value="Create Sign"/>
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
