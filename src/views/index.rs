use std::rc::Rc;
use codee::string::JsonSerdeCodec;
use leptos::{component, view, IntoView};
use leptos_use::storage::use_local_storage;
use crate::{
    components::*,
    models::Sign,
};

turf::style_sheet!("src/views/index.scss");

#[component]
pub fn IndexView() -> impl IntoView {
    let (sign, set_sign, _) = use_local_storage::<Rc<Sign>, JsonSerdeCodec>("last-sign");

    /*
    let pre_anim = use_state(|| true);
    {
        let pre_anim = pre_anim.clone();
        use_timeout(move || pre_anim.set(false), 1);
    }

    let pre_sign_anim = use_state(|| true);
    {
        let pre_sign_anim = pre_sign_anim.clone();
        use_timeout(move || pre_sign_anim.set(false), 200);
    }

     */

    view! {
        <div class=move || format!("wrapper {}", sign().as_ref().room.color().accent_class())>
            <style>{STYLE_SHEET}</style>
            <main class=ClassName::SIGN_FORM>
                <h1>{ "Create Sign" }</h1>
                <SignForm sign=sign on_change=set_sign />
            </main>
            <aside class=ClassName::SIDEBAR>
                <h2 class={ClassName::SIGN_LIST_TITLE}>{"History"}</h2>
                <SignList sign={sign} on_change={set_sign} />
            </aside>
        </div>
    }
}
