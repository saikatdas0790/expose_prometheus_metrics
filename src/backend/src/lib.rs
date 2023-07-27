use candid::{export_service, CandidType, Deserialize};
use ic_cdk::api;

#[cfg(test)]
mod test;

#[derive(CandidType, Deserialize, Debug)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub upgrade: Option<bool>,
}

#[ic_cdk::query]
#[candid::candid_method(query)]
fn http_request() -> HttpResponse {
    let cycle_balance = api::canister_balance128();

    HttpResponse {
        status_code: 200,
        headers: vec![],
        body: format!(
            r"
# TYPE cycle_balance gauge
cycle_balance {cycle_balance}
"
        )
        .into_bytes()
        .to_vec(),
        upgrade: None,
    }
}

#[ic_cdk::update]
#[candid::candid_method(update)]
fn http_request_update(request: HttpRequest) -> HttpResponse {
    let HttpRequest {
        method,
        url,
        headers,
        body,
    } = request;

    HttpResponse {
        status_code: 200,
        headers: vec![],
        body: format!(
            r#"Update call
{method}
{url}
{:?}
{:?}"#,
            headers, body
        )
        .into_bytes()
        .to_vec(),
        upgrade: Some(true),
    }
}

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}
