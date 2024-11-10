use std::rc::Rc;

use base64::prelude::*;
use leptos::{component, event_target_value, view, IntoView, Signal, SignalSet, WriteSignal};
use ulid::Ulid;
use web_sys::{
    Event, MouseEvent,
    SubmitEvent,
};

use crate::{models::*, services::signs::add_sign};

turf::style_sheet!("src/components/sign_form.scss");

#[component]
pub fn SignForm(
    #[prop(into)]
    sign: Signal<Rc<Sign>>,
    #[prop(into)]
    on_change: WriteSignal<Rc<Sign>>,
) -> impl IntoView {
    /*use_title(if sign.title.trim().is_empty() {
        format!("<empty sign> - Edit Sign")
    } else {
        format!("{} - Edit Sign", sign.title.trim())
    });*/

    let on_room_change = move |e: Event| {
        let input = event_target_value(&e);
        if let Ok(room) = input.parse() {
            on_change.set(Rc::new(Sign {
                room,
                ..sign().as_ref().clone()
            }));
        }
    };

    let on_title_change = move |e: Event| {
        let input = event_target_value(&e);
        on_change.set(Rc::new(Sign {
            title: input,
            ..sign().as_ref().clone()
        }));
    };

    let on_subtitle_change = move |e: Event| {
        let input = event_target_value(&e);
        on_change.set(Rc::new(Sign {
            subtitle: input,
            ..sign().as_ref().clone()
        }));
    };

    let on_url_change = move |e: Event| {
        let input = event_target_value(&e);
        on_change.set(Rc::new(Sign {
            url: input,
            ..sign().as_ref().clone()
        }));
    };

    let on_content_change = move |e: Event| {
        let input = event_target_value(&e);
        on_change.set(Rc::new(Sign {
            content: input,
            ..sign().as_ref().clone()
        }));
    };

    let on_submit = {
        let sign = Rc::clone(&sign());
        //let navigator = navigator.clone();

        move |e: SubmitEvent| {
            e.prevent_default();
            spawn_add_sign(Rc::clone(&sign));

            let encoded_sign = encode_sign(&sign);
            //if let Some(navigator) = &navigator {
            //    navigator.push(&Route::Display { data: encoded_sign });
            //}
        }
    };

    view! {
        <form class={ClassName::FORM} on:submit={on_submit}>
            <style>{STYLE_SHEET}</style>
            <div class={ClassName::FORM_COL}>
                <span class={ClassName::SCREEN_READER}><label for="room">{"Room:"}</label></span>
                <select type="text" id="room" name="room" class={ClassName::SCREEN_READER} value={move || sign().room.id()} on:change={on_room_change}>
                    { Room::all().iter().map(|v| view! {
                        <option key={v.id()} value={v.id()} selected={move || *v == sign().room}>{v.name(Language::En)}</option>
                    }).collect::<Vec<_>>() }
                </select>
                <div class={ClassName::ROOM_SELECTOR}>
                    { Room::all().iter().map(|v| {
                        let is_selected = move || sign().room == *v;
                        let sign = sign.clone();
                        let cb = move |e: MouseEvent| {
                            e.prevent_default();
                            on_change.set(Rc::new(Sign {
                                room: *v,
                                ..sign().as_ref().clone()
                            }));
                        };
                        view! {
                            <span class={v.color().accent_class()}>
                                <a href="#" on:click={cb} class=move || if is_selected() { Some(ClassName::SELECTED) } else { None} >
                                    <span inner_html=move || v.icon().unwrap_or_default() />
                                    <span class={ClassName::ROOM_NAME}>{v.name(Language::En)}</span>
                                </a>
                            </span>
                        }
                    }).collect::<Vec<_>>() }
                </div>
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="title">{"Title:"}</label>
                <input type="text" id="title" name="title" value={move || sign().title.clone()} on:input={on_title_change} />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="subtitle">{"Subtitle:"}</label>
                <input type="text" id="subtitle" name="subtitle" value={move || sign().subtitle.clone()} on:input={on_subtitle_change} />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="url">{"Url:"}</label>
                <input type="text" id="url" name="url" value={move || sign().url.clone()} on:input={on_url_change} />
            </div>

            <div class={ClassName::FORM_COL}>
                <label for="content">{"Content:"}</label>
                <textarea rows="10" type="text" id="content" name="content" prop:value=move ||sign().content.clone() on:input={on_content_change} >
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
