use std::{ops::Deref, rc::Rc};
use codee::string::JsonSerdeCodec;
use leptos::{component, create_signal, view, For, IntoView, Signal, WriteSignal};
use leptos_use::storage::use_local_storage;
use web_sys::MouseEvent;
use crate::{
    models::{Language, Sign},
    services::signs::{delete_sign, get_signs},
};

turf::style_sheet!("src/components/sign_list.scss");

#[component]
pub fn SignList(
    #[prop(into)]
    sign: Signal<Rc<Sign>>,
    #[prop(into)]
    on_change: WriteSignal<Rc<Sign>>,
) -> impl IntoView {
    let (stored_signs, set_stored_signs, _) = use_local_storage::<Vec<Rc<Sign>>, JsonSerdeCodec>("stored-signs");

    view! {
        <ul class={ClassName::SIGN_SELECT_LIST}>
            <style>{STYLE_SHEET}</style>
            <For
                each=move || stored_signs()
                key=|sign| sign.id
                children=move |sign| {
                    view!{ <li>Foo</li> }
                }
            />
        </ul>
    }
}
