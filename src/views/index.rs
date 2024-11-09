use std::rc::Rc;

use yew::{classes, function_component, html, use_state, Callback, Html};
use yew_hooks::{use_local_storage, use_timeout};

use crate::{
    components::{SignForm, SignList},
    models::Sign,
};

turf::style_sheet!("src/views/index.scss");

#[function_component]
pub fn IndexView() -> Html {
    let sign_storage = use_local_storage("last-sign".to_string());

    let sign_handle = {
        let sign_storage = sign_storage.clone();
        use_state(move || {
            if let Some(sign_data) = (*sign_storage).clone() {
                Rc::new(sign_data)
            } else {
                Rc::new(Sign::default())
            }
        })
    };
    let sign = Rc::clone(&sign_handle);

    let on_sign_change: Callback<Rc<Sign>> = {
        let sign_handle = sign_handle.clone();

        Callback::from(move |sign: Rc<Sign>| {
            sign_handle.set(Rc::clone(&sign));
            sign_storage.set(sign.as_ref().clone());
        })
    };

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

    html! {
        <div class={classes!("wrapper", sign.as_ref().room.color().accent_class())}>
            <style>{STYLE_SHEET}</style>
            <main class={classes!(ClassName::SIGN_FORM, pre_anim.then(||"pre-anim"))}>
                <h1>{ "Create Sign" }</h1>
                <SignForm value={sign.clone()} on_change={on_sign_change.clone()} />
            </main>
            <aside class={classes!(ClassName::SIDEBAR, pre_sign_anim.then(||ClassName::PRE_ANIM))}>
                <h2 class={ClassName::SIGN_LIST_TITLE}>{"History"}</h2>
                <SignList sign={sign} on_sign_select={on_sign_change} />
            </aside>
        </div>
    }
}
