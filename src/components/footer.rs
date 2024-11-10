use chrono::{Datelike, Local};
use yew::prelude::*;

turf::style_sheet!("src/components/footer.scss");

#[derive(Properties, Clone, PartialEq)]
pub struct FooterProps {
    #[prop_or_default]
    pub start_year: Option<u16>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let copyright = use_memo(props.start_year, |start_year| {
        let current = Local::now().year().try_into().unwrap_or(2024u16);
        let start = start_year.unwrap_or(current.try_into().unwrap_or(2024u16));
        if start < current {
            format!("© {start}–{current} rappet — ")
        } else {
            format!("© {start} rappet — ")
        }
    });

    html! {
        <footer class={ClassName::FOOTER}>
            <style>{STYLE_SHEET}</style>
            <p>
                {copyright}
                <a href="https://github.com/rappet/sign-builder/blob/main/LICENSE" target="_blank">{"MIT License"}</a>
            </p>
        </footer>
    }
}