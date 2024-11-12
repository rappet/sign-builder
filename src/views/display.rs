use leptos::{component, create_memo, view, IntoView, Show};
use leptos_router::{create_query_signal, NavigateOptions};

use crate::{components::*, models::*};

turf::style_sheet!("src/views/display.scss");

#[component]
pub fn DisplayView() -> impl IntoView {
    let mut nav_options = NavigateOptions::default();
    nav_options.replace = true;
    let (room, _set_room) = create_query_signal::<Room>("room");
    let (title, _set_title) = create_query_signal::<String>("title");
    let (subtitle, _set_subtitle) = create_query_signal::<String>("subtitle");
    let (url, _set_url) = create_query_signal::<String>("url");
    let (content, _set_content) = create_query_signal::<String>("content");
    let page_title = create_memo(move |_| {
        let title_val: String = title().unwrap_or_default();
        if title_val.is_empty() {
            format!("Unnamed Sign")
        } else {
            format!("{} - Sign", title_val.trim())
        }
    });

    view! {
        <div class={move || format!("wrapper {}", room().unwrap_or_default().color().accent_class())}>
            <style>{STYLE_SHEET}</style>
            <main class=ClassName::PRINTED>
                //<Show when=move || !url().unwrap_or_default().is_empty()><div class={ClassName::QR_CONTAINER}><QRCode url=move || url().unwrap_or_default()/></div></Show>
                <div class={ClassName::TEXT_CONTENT}>
                    <div class={ClassName::ICON_CIRCLE} inner_html=move || room().unwrap_or_default().icon().unwrap_or_default() />
                    <hgroup>
                        <h1>{move || title()}</h1>
                        <p>{move || subtitle()}</p>
                    </hgroup>
                    <p><a href={move || url().unwrap_or_default().trim().to_string()} target="blank">{move || url().unwrap_or_default().trim().to_string()}</a></p>
                    <p>{move ||content().unwrap_or_default()}</p>
                </div>
            </main>
        </div>
    }
}
