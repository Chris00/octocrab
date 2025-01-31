// Tests for calls to the /projects/{project_id} API.
mod mock_error;

use mock_error::setup_error_handler;
use octocrab::{models::Project, Octocrab};
use serde::{Deserialize, Serialize};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

const PROJECT_ID: u32 = 1002605;

#[derive(Serialize, Deserialize)]
struct FakeProject(Project);

async fn setup_api(template: ResponseTemplate) -> MockServer {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(format!("/projects/{PROJECT_ID}")))
        .respond_with(template)
        .mount(&mock_server)
        .await;
    setup_error_handler(
        &mock_server,
        &format!("DELETE on /projects/{PROJECT_ID} was not received"),
    )
    .await;
    mock_server
}

fn setup_octocrab(uri: &str) -> Octocrab {
    Octocrab::builder().base_uri(uri).unwrap().build().unwrap()
}

#[tokio::test]
async fn should_delete_project() {
    let org_project: Project =
        serde_json::from_str(include_str!("resources/project.json")).unwrap();

    let page_response = FakeProject(org_project);

    let template = ResponseTemplate::new(200).set_body_json(&page_response);
    let mock_server = setup_api(template).await;

    let client = setup_octocrab(&mock_server.uri());
    let project = client
        .projects()
        .delete_project(PROJECT_ID)
        .send()
        .await
        .unwrap();

    assert_eq!(project.creator.login, "octocat");
}
