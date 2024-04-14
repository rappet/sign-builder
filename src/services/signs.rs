use std::rc::Rc;

use gloo_utils::format::JsValueSerdeExt;
use indexed_db_futures::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{js_sys::JsString, DomException};

use crate::models::*;

pub async fn open_db() -> anyhow::Result<IdbDatabase, DomException> {
    // Open my_db v1
    let mut db_req: OpenDbRequest = IdbDatabase::open_u32("sign", 1)?;
    db_req.set_on_upgrade_needed(Some(
        |evt: &IdbVersionChangeEvent| -> anyhow::Result<(), JsValue> {
            // Check if the object store exists; create it if it doesn't
            if let None = evt.db().object_store_names().find(|n| n == "sign") {
                let store = evt.db().create_object_store_with_params(
                    "sign",
                    &IdbObjectStoreParameters::default().key_path(Some(&IdbKeyPath::from("id"))),
                )?;

                store.create_index_with_params(
                    "content_index",
                    &IdbKeyPath::str_sequence(&["room", "title", "subtitle", "content", "url"]),
                    &IdbIndexParameters::new().unique(true),
                )?;
            }
            Ok(())
        },
    ));

    let db: IdbDatabase = db_req.await?;
    Ok(db)
}

pub async fn add_sign(other_sign: Rc<Sign>) -> anyhow::Result<(), DomException> {
    let db = open_db().await?;

    // Insert/overwrite a record
    let tx: IdbTransaction =
        db.transaction_on_one_with_mode("sign", IdbTransactionMode::Readwrite)?;
    let store: IdbObjectStore = tx.object_store("sign")?;

    let value_to_put: JsValue = JsValue::from_serde(other_sign.as_ref()).unwrap();
    store.put_val_owned(&value_to_put)?;

    // IDBTransactions can have an Error or an Abort event; into_result() turns both into a
    // DOMException
    tx.await.into_result()?;

    Ok(())
}

pub async fn get_signs() -> anyhow::Result<Vec<Sign>, DomException> {
    let db = open_db().await?;

    let tx: IdbTransaction =
        db.transaction_on_one_with_mode("sign", IdbTransactionMode::Readonly)?;
    let store: IdbObjectStore = tx.object_store("sign")?;

    let signs: Vec<Sign> = store
        .get_all()?
        .await?
        .iter()
        .map(|v| v.into_serde().unwrap())
        .collect();
    Ok(signs)
}

pub async fn delete_sign(id: String) -> anyhow::Result<(), DomException> {
    let db = open_db().await?;

    let tx: IdbTransaction =
        db.transaction_on_one_with_mode("sign", IdbTransactionMode::Readwrite)?;
    let store: IdbObjectStore = tx.object_store("sign")?;

    store.delete(&JsString::from(id.as_str()))?.await?;

    Ok(())
}
