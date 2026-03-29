use askama::Template;
use sqlx::FromRow;
use serde::Deserialize;

// ── TEMPLATES ──
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate;

#[derive(Template)]
#[template(path = "fragments/orgs.html", escape = "none")]
pub struct OrgsFragment {
    pub orgs: Vec<OrgCard>,
}

#[derive(Template)]
#[template(path = "fragments/new-org-form.html", escape = "none")]
pub struct NewOrgFormTemplate;

#[derive(Template)]
#[template(path = "fragments/new-group-form.html", escape = "none")]
pub struct NewGroupFormTemplate {
    pub orgs: Vec<OrgSelect>,
}

#[derive(Template)]
#[template(path = "fragments/new-host-form.html", escape = "none")]
pub struct NewHostFormTemplate {
    pub orgs: Vec<OrgSelect>,
}

#[derive(Template)]
#[template(path = "fragments/org-detail.html", escape = "none")]
pub struct OrgDetailTemplate {
    pub org: OrgDetail,
    pub groups: Vec<GroupCard>,
    pub hosts: Vec<HostCard>,
}

// ── DATA MODELS ──
#[derive(FromRow)]
pub struct OrgCard {
    pub _id: i32,
    pub name: String,
    pub compliance: i32,
    pub host_count: i64,
    pub group_count: i64,
}

#[derive(FromRow)]
pub struct OrgDetail {
    pub id: i32,
    pub name: String,
    pub host_count: i64,
}

#[derive(FromRow)]
pub struct GroupCard {
    pub _id: i32,
    pub name: String,
    pub host_count: i64,
}

#[derive(FromRow)]
pub struct HostCard {
    pub _id: i32,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub description: Option<String>,
}

#[derive(FromRow)]
pub struct OrgSelect {
    pub id: i32,
    pub name: String,
}

// ── FORM MODELS ──
#[derive(Deserialize)]
pub struct NewOrgForm {
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewGroupForm {
    pub organization_id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct NewHostForm {
    pub organization_id: i32,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub description: Option<String>,
}