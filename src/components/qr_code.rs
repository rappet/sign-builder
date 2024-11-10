use qrencode::{render::svg, EcLevel, QrCode};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct QRCodeProps {
    pub url: String,
}

#[function_component(QRCode)]
pub fn qr_code(props: &QRCodeProps) -> impl IntoView {
    QrCode::with_error_correction_level(props.url.trim(), EcLevel::L)
        .ok()
        .map(|qr| {
            qr.render::<svg::Color>()
                .light_color(svg::Color("transparent"))
                .build()
        })
        .map(|svg| Html::from_html_unchecked(svg.into()))
        .unwrap_or_default()
        .into()
}
