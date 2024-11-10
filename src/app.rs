use std::rc::Rc;

use base64::prelude::*;
use leptos::{component, view, IntoView};
use crate::{models::*, views::*};
use crate::components::Footer;

#[component]
pub fn App() -> impl IntoView {
    let favicon_url = format!(
        "data:image/svg+xml;base64,{}",
        BASE64_STANDARD.encode(include_bytes!("icons/sign-right.svg"))
    );
    //use_favicon(favicon_url);

    view! {
        <>
            //<HashRouter>
            //    <Switch<Route> render={switch} />
            //</HashRouter>
            <IndexView />
            <Footer />
        </>
    }
}

/*fn switch(routes: Route) -> impl IntoView {
    match routes {
        Route::Home => view! { <IndexView /> },
        _ => panic!()
        //Route::Display { data } => {
        //    if let Ok(sign) = decode_sign(&data) {
        //        view! { <DisplayView {sign} /> }
        //    } else {
        //        view! { <p>{"Decoding provided sign failed"}</p> }
        //    }
        //}
    }
}*/

fn decode_sign(data: &str) -> Result<Rc<Sign>, ()> {
    let decoded = BASE64_URL_SAFE.decode(data).map_err(|_err| ())?;
    let sign = serde_json::from_slice(&decoded).map_err(|_err| ())?;
    Ok(Rc::new(sign))
}
