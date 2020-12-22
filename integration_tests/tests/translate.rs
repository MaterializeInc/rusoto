#![cfg(feature = "translate")]

extern crate rusoto_core;
extern crate rusoto_translate;

use rusoto_core::Region;
use rusoto_translate::{Translate, TranslateClient, TranslateTextRequest};

#[tokio::test]
async fn should_translate_to_english() {
    let client = TranslateClient::new(Region::UsEast1);
    let request = TranslateTextRequest {
        source_language_code: "de".to_owned(),
        target_language_code: "en".to_owned(),
        text: "guten tag".to_owned(),
        ..Default::default()
    };

    let result = client.translate_text(request).await.unwrap();
    assert_eq!("good day", result.translated_text.to_lowercase());
}
