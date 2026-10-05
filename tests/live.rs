//! Live tests against a real AveeCare API.
//!
//! Ignored by default. Run them with both AVEECARE_API_KEY and
//! AVEECARE_BASE_URL set; they create, update and delete one test patient. Use
//! a sandbox or local API, never production data:
//!
//!     AVEECARE_API_KEY=... AVEECARE_BASE_URL=... cargo test --test live -- --ignored

use aveecare::{AveeCare, Error, PatientCreate, PatientListParams, PatientUpdate, RequestOptions};

fn live_client() -> AveeCare {
    for var in ["AVEECARE_API_KEY", "AVEECARE_BASE_URL"] {
        assert!(
            std::env::var(var).is_ok(),
            "{var} must be set to run live tests"
        );
    }
    AveeCare::from_env().expect("client")
}

#[tokio::test]
#[ignore = "live API test: set AVEECARE_API_KEY and AVEECARE_BASE_URL, run with --ignored"]
async fn live_me() {
    let avee = live_client();
    assert!(!avee.me().retrieve().await.unwrap().id.is_empty());
}

#[tokio::test]
#[ignore = "live API test: set AVEECARE_API_KEY and AVEECARE_BASE_URL, run with --ignored"]
async fn live_list_page_twice_and_retrieve() {
    let avee = live_client();
    let first = avee
        .patients()
        .list(PatientListParams::new().limit(1))
        .await
        .unwrap();
    assert_eq!(first.object, "list");
    if let (true, Some(cursor)) = (first.has_next_page(), first.next_cursor.clone()) {
        let second = avee
            .patients()
            .list(PatientListParams::new().limit(1).cursor(cursor))
            .await
            .unwrap();
        if let (Some(a), Some(b)) = (first.data.first(), second.data.first()) {
            assert_ne!(a.id, b.id);
        }
    }
    if let Some(p) = first.data.first() {
        assert_eq!(avee.patients().retrieve(&p.id).await.unwrap().id, p.id);
    }
}

#[tokio::test]
#[ignore = "live API test: set AVEECARE_API_KEY and AVEECARE_BASE_URL, run with --ignored"]
async fn live_create_idempotent_update_delete() {
    let avee = live_client();
    let key = format!(
        "live-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let last = format!("Livetest {key}");
    let created = avee
        .patients()
        .create_with_options(
            PatientCreate::new("Sdk", last.clone()),
            RequestOptions::new().idempotency_key(key.clone()),
        )
        .await
        .unwrap();
    let replay = avee
        .patients()
        .create_with_options(
            PatientCreate::new("Sdk", last),
            RequestOptions::new().idempotency_key(key),
        )
        .await
        .unwrap();
    assert_eq!(
        replay.id, created.id,
        "same Idempotency-Key returns the same patient"
    );
    let updated = avee
        .patients()
        .update(&created.id, PatientUpdate::new().first_name("Sdk2"))
        .await;
    avee.patients().delete(&created.id).await.unwrap();
    assert_eq!(updated.unwrap().first_name, "Sdk2");
    assert!(matches!(
        avee.patients().retrieve(&created.id).await,
        Err(Error::NotFound(_))
    ));
}
