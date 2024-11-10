use std::rc::Rc;

use base64::prelude::*;
use yew::prelude::*;
use yew_hooks::use_favicon;
use yew_router::prelude::*;

use crate::{models::*, route::Route, views::*};
use crate::components::Footer;

#[function_component]
pub fn App() -> Html {
    let favicon_url = format!(
        "data:image/svg+xml;base64,{}",
        BASE64_STANDARD.encode(include_bytes!("icons/sign-right.svg"))
    );
    use_favicon(favicon_url);

    html! {
        <>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
            <Footer />
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <IndexView /> },
        Route::Display { data } => {
            if let Ok(sign) = decode_sign(&data) {
                html! { <DisplayView {sign} /> }
            } else {
                html! { <p>{"Decoding provided sign failed"}</p> }
            }
        }
    }
}

fn decode_sign(data: &str) -> Result<Rc<Sign>, ()> {
    let decoded = BASE64_URL_SAFE.decode(data).map_err(|_err| ())?;
    let sign = serde_json::from_slice(&decoded).map_err(|_err| ())?;
    Ok(Rc::new(sign))
}
