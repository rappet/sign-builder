use chrono::{Datelike, Local};
use leptos::{component, create_memo, view, IntoView, MaybeSignal};

turf::style_sheet!("src/components/footer.scss");

#[component]
pub fn Footer(
    #[prop(optional, into)]
    start_year: MaybeSignal<Option<u16>>
) -> impl IntoView {
    let copyright = create_memo(move |_| {
        let current = Local::now().year().try_into().unwrap_or(2024u16);
        let start = start_year().unwrap_or(current.try_into().unwrap_or(2024u16));
        if start < current {
            format!("© {}–{current} rappet — ", start)
        } else {
            format!("© {} rappet — ", start)
        }
    });

    view! {
        <footer class={ClassName::FOOTER}>
            <style>{STYLE_SHEET}</style>
            <p>
                {copyright}
                <a href="https://github.com/rappet/sign-builder/blob/main/LICENSE" target="_blank">{"MIT License"}</a>
            </p>
        </footer>
    }
}