use std::rc::Rc;

use qrencode::{render::svg, EcLevel, QrCode};
use yew::{classes, function_component, html, Html, Properties};
use yew_hooks::use_title;

use crate::models::*;

#[derive(Properties, PartialEq)]
pub struct DisplayViewProps {
    pub sign: Rc<Sign>,
}

#[function_component]
pub fn DisplayView(props: &DisplayViewProps) -> Html {
    let sign = Rc::clone(&props.sign);

    use_title(format!("{} - Sign", sign.title));

    let qr = if !sign.url.trim().is_empty() {
        QrCode::with_error_correction_level(sign.url.trim(), EcLevel::L)
            .ok()
            .map(|qr| {
                qr.render::<svg::Color>()
                    .light_color(svg::Color("transparent"))
                    .build()
            })
            .map(|svg| Html::from_html_unchecked(svg.into()))
    } else {
        None
    };

    html! {
        <div class={classes!("wrapper", "bg-accent", sign.room.color().accent_class())}>
            <main class={classes!("flex", "flex-row", "items-center", "relative", sign.url.trim().is_empty().then_some("w-[70ch]"))}>
                if qr.is_some() {
                    <div class="shrink-0 w-80 h-80 [&>svg]:h-full [&>svg]:w-full self-center">
                         {qr}
                    </div>
                }
                <div class="grow pr-16">
                    if sign.room.icon().is_some() {
                        <div class="absolute -top-8 shadow -right-8 [&>svg]:w-20 [&>svg]:h-20 p-4 rounded-full [&>svg]:stroke-black [&>svg]:fill-black bg-white">
                            {Html::from_html_unchecked(sign.room.icon().unwrap_or_default().into())}
                        </div>
                    }
                    <h1 class="pb-0 font-bold">{sign.title.trim()}</h1>
                    if !sign.subtitle.trim().is_empty() {
                        <p class="text-2xl px-4 font-bold">{sign.subtitle.clone()}</p>
                    }
                    if !sign.url.trim().is_empty() {
                        <p class="px-4 text-lg"><a class="hover:underline text-accent" href={sign.url.trim().to_string()} target="blank">{sign.url.trim().to_string()}</a></p>
                    }
                    if !sign.content.trim().is_empty() {
                        <p class="text-lg px-4 py-2">{sign.content.clone()}</p>
                    }
                </div>
            </main>
        </div>
    }
}
