use leptos::{component, create_memo, view, IntoView, Signal};
use qrencode::{render::svg, EcLevel, QrCode};

#[component]
pub fn QRCode(
    #[prop(into)]
    url: Signal<String>
) -> impl IntoView {
    let raw_svg = create_memo(move |_| QrCode::with_error_correction_level(url().trim(), EcLevel::L)
        .ok()
        .map(|qr| {
            qr.render::<svg::Color>()
                .light_color(svg::Color("transparent"))
                .build()
        }));
    view! {
        <span inner_html={move ||raw_svg().unwrap_or_default()} />
    }
}
