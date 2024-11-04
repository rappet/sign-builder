use std::rc::Rc;

use qrencode::{render::svg, EcLevel, QrCode};
use yew::{classes, function_component, html, Html, Properties};
use yew_hooks::use_title;

use crate::models::*;

turf::style_sheet!("src/views/display.scss");

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
        <div class={classes!("wrapper", sign.room.color().accent_class())}>
            <style>{STYLE_SHEET}</style>
            <main class={classes!(ClassName::PRINTED, sign.url.trim().is_empty().then_some(ClassName::PRINTED_SMALL))}>
                if qr.is_some() {<div class={ClassName::QR_CONTAINER}>{qr}</div>}
                <div class={ClassName::TEXT_CONTENT}>
                    if sign.room.icon().is_some() {
                        <div class={ClassName::ICON_CIRCLE}>
                            {Html::from_html_unchecked(sign.room.icon().unwrap_or_default().into())}
                        </div>
                    }
                    <hgroup>
                        <h1>{sign.title.trim()}</h1>
                        if !sign.subtitle.trim().is_empty() {
                            <p>{sign.subtitle.clone()}</p>
                        }
                    </hgroup>
                    if !sign.url.trim().is_empty() {
                        <p><a href={sign.url.trim().to_string()} target="blank">{sign.url.trim().to_string()}</a></p>
                    }
                    if !sign.content.trim().is_empty() {
                        <p>{sign.content.clone()}</p>
                    }
                </div>
            </main>
        </div>
    }
}
