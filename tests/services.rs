use std::rc::Rc;

use sign_builder::{
    models::{Room, Sign},
    services::signs::{add_sign, delete_sign, get_signs},
};
use ulid::Ulid;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
async fn sign_storage() {
    assert_eq!(
        get_signs()
            .await
            .expect("fetching all signs should not fail"),
        vec![]
    );

    let sign = Sign {
        id: Some(Ulid::from_parts(1234567890, 2342)),
        room: Room::Hackcenter,
        title: "Example title".to_string(),
        subtitle: "Example subtitle".to_string(),
        content: "Example content".to_string(),
        url: "https://www.example.com".to_string(),
    };

    add_sign(Rc::new(sign.clone()))
        .await
        .expect("adding a sign should not fail");

    let got = get_signs()
        .await
        .expect("fetching all signs should not fail");

    assert_eq!(vec![sign.clone()], got);

    delete_sign(sign.id.unwrap().to_string()).await.unwrap();

    assert_eq!(
        get_signs()
            .await
            .expect("fetching all signs should not fail"),
        vec![]
    );
}
