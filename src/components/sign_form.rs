use std::rc::Rc;
use std::str::FromStr;

use base64::prelude::*;
use leptos::{component, create_memo, event_target_value, view, IntoView, Signal, WriteSignal};
use leptos_router::{create_query_signal_with_options, Form, NavigateOptions};
use leptos_meta::Title;
use ulid::Ulid;
use url::Url;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

use crate::{models::*, services::signs::add_sign};

turf::style_sheet!("src/components/sign_form.scss");

#[component]
pub fn SignForm(
    #[prop(into)]
    sign: Signal<Rc<Sign>>,
    #[prop(into)]
    on_change: WriteSignal<Rc<Sign>>,
) -> impl IntoView {
    let mut nav_options = NavigateOptions::default();
    nav_options.replace = true;
    let (room, set_room) = create_query_signal_with_options::<Room>("room", nav_options.clone());
    let (title, set_title) = create_query_signal_with_options("title", nav_options.clone());
    let (subtitle, set_subtitle) = create_query_signal_with_options("subtitle", nav_options.clone());
    let (url, set_url) = create_query_signal_with_options::<String>("url", nav_options.clone());
    let (content, set_content) = create_query_signal_with_options("content", nav_options);
    let page_title = create_memo(move |_| {
        let title_val: String = title().unwrap_or_default();
        if title_val.is_empty() {
            format!("<empty sign> - Edit Sign")
        } else {
            format!("{} - Edit Sign", title_val.trim())
        }
    });

    let url_warning = move || {
        if let Some(url) = url() {
            if let Ok(url) = Url::parse(&url) {
                if url.scheme() != "https" {
                    "Not HTTPS"
                } else {
                    ""
                }
            } else {
                "Not a valid URL"
            }
        } else {
            ""
        }
    };

    view! {
        <Form class={ClassName::FORM} method="GET" action="sign">
            <Title text=page_title />
            <style>{STYLE_SHEET}</style>
            <div class={ClassName::FORM_COL}>
                <span class={ClassName::SCREEN_READER}><label for="room">{"Room:"}</label></span>
                <select type="text" id="room" name="room" class={ClassName::SCREEN_READER} value=move || room().unwrap_or_default().id() on:change=move |e| set_room(Room::from_str(&event_target_value(&e)).ok())>
                    { Room::all().iter().map(|v| view! {
                        <option key={v.id()} value={v.id()} selected={move || *v == room().unwrap_or_default()}>{v.name(Language::En)}</option>
                    }).collect::<Vec<_>>() }
                </select>
                <div class={ClassName::ROOM_SELECTOR}>
                    { Room::all().iter().map(|v| {
                        let is_selected = move || room().unwrap_or_default() == *v;
                        let sign = sign.clone();
                        let cb = move |e: MouseEvent| {
                            e.prevent_default();
                            set_room(Some(*v));
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
                <input type="text" id="title" name="title" value=move ||title().unwrap_or_default() on:change=move |e| set_title(event_target_value_opt(&e)) />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="subtitle">{"Subtitle:"}</label>
                <input type="text" id="subtitle" name="subtitle" value=move ||subtitle().unwrap_or_default() on:change=move |e| set_subtitle(event_target_value_opt(&e)) />
            </div>
            <div class={ClassName::FORM_ROW}>
                <label for="url">{"Url:"}</label>
                <input type="text" id="url" name="url" value=move ||url().unwrap_or_default() on:change=move |e| set_url(event_target_value_opt(&e)) />
                <span>{url_warning}</span>
            </div>

            <div class={ClassName::FORM_COL}>
                <label for="content">{"Content:"}</label>
                <textarea rows="10" type="text" id="content" name="content" prop:value=move ||content().unwrap_or_default() on:change=move |e| set_content(event_target_value_opt(&e)) >
                </textarea>
            </div>

            <input type="submit" value="Create Sign"/>
        </Form>
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

fn event_target_value_opt<T>(event: &T) -> Option<String>
where
    T: JsCast,
{
    let s = event_target_value(event);
    if s.is_empty() {
        None
    } else {
        Some(s.trim().to_string())
    }
}