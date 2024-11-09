use std::{ops::Deref, rc::Rc};

use yew::{platform::spawn_local, prelude::*};

use crate::{
    models::{Language, Sign},
    services::signs::{delete_sign, get_signs},
};

turf::style_sheet!("src/components/sign_list.scss");

#[derive(Properties, Clone, PartialEq)]
pub struct SignListProps {
    pub sign: Rc<Sign>,
    pub on_sign_select: Callback<Rc<Sign>>,
}

#[function_component(SignList)]
pub fn sign_list(props: &SignListProps) -> Html {
    let stored_signs_handle = use_state(|| None);
    let stored_signs = stored_signs_handle.deref().clone().unwrap_or_default();
    {
        let stored_signs_handle = stored_signs_handle.clone();
        use_effect(move || {
            spawn_local(async move {
                if let Ok(signs) = get_signs().await {
                    stored_signs_handle.set(Some(signs));
                }
            })
        });
    }

    let mut sign_list = stored_signs.iter().map(|sign| {
        let on_sign_select = props.on_sign_select.clone();
        let load_sign_callback = {
            let sign: Rc<Sign> = Rc::new(sign.clone());
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();
                on_sign_select.emit(Rc::clone(&sign));
            })
        };

        let delete_sign_callback = {
            let sign_id = sign.id.clone();
            let stored_signs_handle = stored_signs_handle.clone();
            Callback::from(move |event: MouseEvent| {
                event.prevent_default();

                if let Some(id) = sign_id.clone() {
                    let stored_signs_handle = stored_signs_handle.clone();
                    spawn_local(async move {
                        let _ = delete_sign(id.to_string()).await;
                        if let Ok(signs) = get_signs().await {
                            stored_signs_handle.set(Some(signs));
                        } else {
                            stored_signs_handle.set(None);
                        }
                    });
                }
            })
        };

        let show_subtitle = !sign.subtitle.trim().is_empty();
        let show_url = !sign.url.trim().is_empty();
        let show_content = !sign.content.trim().is_empty();

        html! {
            <li>
                <a href="#" onclick={load_sign_callback} class={classes!(ClassName::SIGN_SELECT_BUTTON, sign.room.color().accent_class())}>
                    <p class={ClassName::SIGN_SELECT_BUTTON_TITLE}>
                        <span>{sign.title.clone()}</span>
                        <span>{" - "}</span>
                        <span class={ClassName::SIGN_SELECT_BUTTON_ROOM_NAME}>{sign.room.name(Language::En)}</span>
                    </p>
                    if show_subtitle || show_url {
                        if show_subtitle { <span class={ClassName::SIGN_SELECT_BUTTON_SUBTITLE}>{sign.subtitle.clone()}</span> }
                        if show_subtitle && show_url { <span>{" - "}</span> }
                        if show_url { <span class={ClassName::SIGN_SELECT_BUTTON_URL}>{sign.url.clone()}</span> }
                    }
                    if show_content { <p class={ClassName::SIGN_SELECT_BUTTON_SUBTITLE}>{sign.content.clone()}</p> }
                </a>
                <a href="#" onclick={delete_sign_callback}>{"Delete above"}</a>
            </li>
        }
    }).collect::<Vec<_>>();
    sign_list.reverse();

    html! {
        <ul class={ClassName::SIGN_SELECT_LIST}>
            <style>{STYLE_SHEET}</style>
            {sign_list}
        </ul>
    }
}
