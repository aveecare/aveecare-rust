use std::time::Duration;

use aveecare::futures_util::{pin_mut, StreamExt};
use aveecare::{
    AveeCare, Error, PatientCreate, PatientLifecycleStage, PatientListParams, PatientUpdate,
    RequestOptions, VisitListParams,
};
use serde_json::{json, Value};
use wiremock::matchers::{header, header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

fn patient(id: &str) -> Value {
    json!({
        "object": "patient",
        "id": id,
        "firstName": "Margaret",
        "lastName": "Alvarez",
        "status": "Active",
        "lifecycleStage": "Client",
        "address": "1 Main St", "city": "Phoenix", "state": "AZ", "zipCode": "85015",
        "createdAt": "2026-09-12T16:04:11Z",
        "updatedAt": "2026-09-30T09:12:45Z",
        "someFutureField": 1
    })
}

fn client(server: &MockServer) -> AveeCare {
    AveeCare::builder()
        .api_key("ak_test_secret")
        .base_url(server.uri())
        .build()
        .unwrap()
}

fn error_body(code: &str) -> Value {
    json!({"error": {"type": "x_error", "code": code, "message": format!("boom {code}"), "request_id": "req_abc"}})
}

#[tokio::test]
async fn sends_auth_and_user_agent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/me"))
        .and(header("authorization", "Bearer ak_test_secret"))
        .and(header(
            "user-agent",
            concat!("aveecare-rust/", env!("CARGO_PKG_VERSION")),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "me", "id": "key_1", "name": "k", "scopes": ["read", "write", "admin"], "role": "User", "email": "avery@sunrisehomecare.example", "expiresAt": "2027-10-05T00:00:00.000Z",
            "organization": {"id": "o", "name": "Demo Agency"}
        })))
        .expect(1)
        .mount(&server)
        .await;
    let me = client(&server).me().retrieve().await.unwrap();
    assert_eq!(me.organization.name.as_deref(), Some("Demo Agency"));
    // Unknown enum values decode to the catch-all instead of failing.
    assert_eq!(me.scopes.len(), 3);
    assert_eq!(me.scopes[2], aveecare::MeScope::Unknown);
    let requests = server.received_requests().await.unwrap();
    assert!(requests[0].headers.get("idempotency-key").is_none());
}

#[tokio::test]
async fn list_sends_wire_params_and_decodes_models() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/patients"))
        .and(query_param("status", "Active"))
        .and(query_param("lifecycle_stage", "Client"))
        .and(query_param("updated_after", "2026-01-01T00:00:00Z"))
        .and(query_param("limit", "25"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "object": "list", "data": [patient("pat_1")], "next_cursor": null, "has_more": false
        })))
        .expect(1)
        .mount(&server)
        .await;
    let page = client(&server)
        .patients()
        .list(
            PatientListParams::new()
                .status("Active")
                .lifecycle_stage(PatientLifecycleStage::Client)
                .updated_after("2026-01-01T00:00:00Z")
                .limit(25),
        )
        .await
        .unwrap();
    assert!(!page.has_next_page());
    let p = &page.data[0];
    assert_eq!(p.first_name.as_deref(), Some("Margaret"));
    assert_eq!(p.status.as_deref(), Some("Active"));
    assert_eq!(p.lifecycle_stage, Some(PatientLifecycleStage::Client));
    assert_eq!(p.zip_code.as_deref(), Some("85015"));
    assert_eq!(p.middle_name, None);
}

#[tokio::test]
async fn visit_list_requires_window() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/visits"))
        .and(query_param("start_from", "2026-10-01T00:00:00Z"))
        .and(query_param("start_to", "2026-10-08T00:00:00Z"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"object": "list", "data": [], "next_cursor": null, "has_more": false}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let page = client(&server)
        .visits()
        .list(VisitListParams::new(
            "2026-10-01T00:00:00Z",
            "2026-10-08T00:00:00Z",
        ))
        .await
        .unwrap();
    assert!(page.data.is_empty());
}

struct TwoPages;
impl Respond for TwoPages {
    fn respond(&self, req: &Request) -> ResponseTemplate {
        let cursor = req
            .url
            .query_pairs()
            .find(|(k, _)| k == "cursor")
            .map(|(_, v)| v.into_owned());
        let body = if cursor.as_deref() == Some("cur_2") {
            json!({"object": "list", "data": [patient("pat_3")], "next_cursor": null, "has_more": false})
        } else {
            json!({"object": "list", "data": [patient("pat_1"), patient("pat_2")], "next_cursor": "cur_2", "has_more": true})
        };
        ResponseTemplate::new(200).set_body_json(body)
    }
}

#[tokio::test]
async fn auto_paging_stream_walks_every_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/patients"))
        .respond_with(TwoPages)
        .expect(2)
        .mount(&server)
        .await;
    let stream = client(&server)
        .patients()
        .list_auto_paging(PatientListParams::new().limit(2));
    pin_mut!(stream);
    let mut ids = Vec::new();
    while let Some(p) = stream.next().await {
        ids.push(p.unwrap().id);
    }
    assert_eq!(ids, vec!["pat_1", "pat_2", "pat_3"]);
    let requests = server.received_requests().await.unwrap();
    assert!(requests[1].url.query().unwrap().contains("cursor=cur_2"));
    assert!(requests[1].url.query().unwrap().contains("limit=2"));
}

#[tokio::test]
async fn create_update_delete() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/patients"))
        .and(header_exists("idempotency-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(patient("pat_9")))
        .mount(&server)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v1/patients/pat_9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(patient("pat_9")))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path("/v1/patients/pat%201%2Fx"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"object": "patient", "id": "pat 1/x", "deleted": true})),
        )
        .mount(&server)
        .await;
    let avee = client(&server);
    let created = avee
        .patients()
        .create(
            PatientCreate::new("Margaret", "Alvarez")
                .address("1 Main St")
                .city("Phoenix")
                .zip_code("85015")
                .clear_email(),
        )
        .await
        .unwrap();
    assert_eq!(created.id, "pat_9");
    avee.patients()
        .create_with_options(
            PatientCreate::new("A", "B"),
            RequestOptions::new().idempotency_key("my-key-1"),
        )
        .await
        .unwrap();
    avee.patients()
        .update(
            "pat_9",
            PatientUpdate::new()
                .scheduling_notes("Prefers morning visits")
                .clear_date_of_birth(),
        )
        .await
        .unwrap();
    let deleted = avee.patients().delete("pat 1/x").await.unwrap();
    assert!(deleted.deleted);

    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        body,
        json!({
            "firstName": "Margaret", "lastName": "Alvarez", "email": null,
            "address": "1 Main St", "city": "Phoenix", "zipCode": "85015"
        })
    );
    let key = requests[0]
        .headers
        .get("idempotency-key")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(uuid_version(key), Some('4'));
    assert_eq!(
        requests[1].headers.get("idempotency-key").unwrap(),
        "my-key-1"
    );
    let patch: Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(
        patch,
        json!({"schedulingNotes": "Prefers morning visits", "dateOfBirth": null})
    );
    assert_eq!(requests[2].method.as_str(), "PATCH");
}

fn uuid_version(s: &str) -> Option<char> {
    let parts: Vec<&str> = s.split('-').collect();
    (parts.len() == 5 && parts[0].len() == 8)
        .then(|| parts[2].chars().next())
        .flatten()
}

#[tokio::test]
async fn errors_are_typed() {
    type IsVariant = fn(&Error) -> bool;
    let cases: [(u16, &str, IsVariant); 5] = [
        (400, "unknown_field", |e| {
            matches!(e, Error::InvalidRequest(_))
        }),
        (401, "unauthorized", |e| {
            matches!(e, Error::Authentication(_))
        }),
        (403, "insufficient_scope", |e| {
            matches!(e, Error::Permission(_))
        }),
        (404, "not_found", |e| matches!(e, Error::NotFound(_))),
        (409, "idempotency_conflict", |e| {
            matches!(e, Error::Conflict(_))
        }),
    ];
    for (status, code, is_variant) in cases {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(status).set_body_json(error_body(code)))
            .expect(1)
            .mount(&server)
            .await;
        let err = client(&server)
            .patients()
            .retrieve("pat_1")
            .await
            .unwrap_err();
        assert!(is_variant(&err), "{status} -> {err:?}");
        assert_eq!(err.status(), Some(status));
        assert_eq!(err.code(), Some(code));
        assert_eq!(err.request_id(), Some("req_abc"));
        assert_eq!(
            err.details().unwrap().error_type.as_deref(),
            Some("x_error")
        );
        assert!(err.to_string().contains(&format!("boom {code}")));
    }
}

#[tokio::test]
async fn request_id_header_fallback() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(502)
                .set_body_string("oops")
                .insert_header("request-id", "req_hdr"),
        )
        .mount(&server)
        .await;
    let avee = AveeCare::builder()
        .api_key("k")
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .unwrap();
    let err = avee.me().retrieve().await.unwrap_err();
    assert!(matches!(err, Error::Api(_)));
    assert_eq!(err.request_id(), Some("req_hdr"));
}

struct Sequence(std::sync::Mutex<Vec<ResponseTemplate>>);
impl Respond for Sequence {
    fn respond(&self, _: &Request) -> ResponseTemplate {
        let mut q = self.0.lock().unwrap();
        if q.len() > 1 {
            q.remove(0)
        } else {
            q[0].clone()
        }
    }
}

#[tokio::test]
async fn retries_429_with_retry_after_and_same_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(Sequence(std::sync::Mutex::new(vec![
            ResponseTemplate::new(429)
                .set_body_json(error_body("rate_limited"))
                .insert_header("retry-after", "1"),
            ResponseTemplate::new(201).set_body_json(patient("pat_1")),
        ])))
        .expect(2)
        .mount(&server)
        .await;
    let started = std::time::Instant::now();
    let created = client(&server)
        .patients()
        .create(PatientCreate::new("A", "B"))
        .await
        .unwrap();
    assert_eq!(created.id, "pat_1");
    assert!(
        started.elapsed() >= Duration::from_millis(950),
        "honored Retry-After"
    );
    let requests = server.received_requests().await.unwrap();
    assert_eq!(
        requests[0].headers.get("idempotency-key"),
        requests[1].headers.get("idempotency-key")
    );
}

#[tokio::test]
async fn gives_up_after_max_retries() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(429)
                .set_body_json(error_body("rate_limited"))
                .insert_header("retry-after", "0"),
        )
        .expect(3)
        .mount(&server)
        .await;
    let err = client(&server)
        .patients()
        .retrieve("pat_1")
        .await
        .unwrap_err();
    match err {
        Error::RateLimit(d) => assert_eq!(d.retry_after, Some(Duration::ZERO)),
        other => panic!("expected RateLimit, got {other:?}"),
    }
}

#[tokio::test]
async fn retries_5xx_then_succeeds_and_skips_4xx() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/patients/pat_1"))
        .respond_with(Sequence(std::sync::Mutex::new(vec![
            ResponseTemplate::new(503).set_body_json(error_body("internal_error")),
            ResponseTemplate::new(200).set_body_json(patient("pat_1")),
        ])))
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/patients/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(error_body("not_found")))
        .expect(1)
        .mount(&server)
        .await;
    let avee = client(&server);
    assert_eq!(avee.patients().retrieve("pat_1").await.unwrap().id, "pat_1");
    assert!(matches!(
        avee.patients().retrieve("missing").await,
        Err(Error::NotFound(_))
    ));
}

#[tokio::test]
async fn timeouts_and_connection_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(patient("p"))
                .set_delay(Duration::from_millis(500)),
        )
        .mount(&server)
        .await;
    let avee = AveeCare::builder()
        .api_key("k")
        .base_url(server.uri())
        .timeout(Duration::from_millis(50))
        .max_retries(0)
        .build()
        .unwrap();
    assert!(matches!(
        avee.patients().retrieve("p").await,
        Err(Error::Timeout)
    ));

    let dead = AveeCare::builder()
        .api_key("k")
        .base_url("http://127.0.0.1:9")
        .max_retries(1)
        .build()
        .unwrap();
    assert!(matches!(
        dead.me().retrieve().await,
        Err(Error::Connection(_))
    ));
}

#[test]
fn builder_reads_environment() {
    // One test owns these variables to avoid races with other tests.
    std::env::set_var("AVEECARE_API_KEY", "ak_env_key");
    std::env::set_var("AVEECARE_BASE_URL", "http://env.local/");
    let c = AveeCare::from_env().unwrap();
    assert_eq!(c.base_url(), "http://env.local");
    std::env::remove_var("AVEECARE_BASE_URL");
    assert_eq!(
        AveeCare::from_env().unwrap().base_url(),
        "https://api.aveecare.com"
    );
    std::env::remove_var("AVEECARE_API_KEY");
    assert!(matches!(AveeCare::from_env(), Err(Error::Config(_))));
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_client_lists_and_auto_pages() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/patients"))
            .respond_with(TwoPages)
            .mount(&server)
            .await;
        server
    });
    let avee = aveecare::blocking::AveeCare::from_async(
        AveeCare::builder()
            .api_key("k")
            .base_url(server.uri())
            .build()
            .unwrap(),
    );
    let page = avee.patients().list(PatientListParams::new()).unwrap();
    assert_eq!(page.data.len(), 2);
    let ids: Vec<String> = avee
        .patients()
        .list_auto_paging(PatientListParams::new())
        .map(|p| p.unwrap().id)
        .collect();
    assert_eq!(ids, vec!["pat_1", "pat_2", "pat_3"]);
    drop(server);
}
