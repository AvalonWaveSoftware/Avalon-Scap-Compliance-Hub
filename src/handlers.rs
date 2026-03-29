use askama::Template;
use crate::models::*;
use axum::{response::Html, extract::{State, Form, Path}};
use sqlx::PgPool;

// ── HANDLERS ──
pub async fn dashboard_handler(_pool: State<PgPool>) -> Html<String> {
    let template = DashboardTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<h1 style='color:red'>Template error</h1>".to_string()),
    }
}

pub async fn orgs_handler(State(pool): State<PgPool>) -> Html<String> {
    let orgs: Vec<OrgCard> = sqlx::query_as(r#"
        SELECT o.id as _id, o.name,
               COALESCE(ROUND(AVG(s.compliance_percent), 0), 0)::int as compliance,
               COUNT(DISTINCT h.id) as host_count,
               COUNT(DISTINCT g.id) as group_count
        FROM organizations o
        LEFT JOIN hosts h ON h.organization_id = o.id
        LEFT JOIN host_groups g ON g.organization_id = o.id
        LEFT JOIN scans s ON s.host_id = h.id
        GROUP BY o.id, o.name
        ORDER BY o.name
    "#)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let fragment = OrgsFragment { orgs };
    match fragment.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<p class='text-red-400'>Error loading organizations</p>".to_string()),
    }
}

pub async fn groups_handler(_pool: State<PgPool>) -> Html<String> {
    Html(r##"
        <div class="mb-6">
            <div class="flex justify-between items-center mb-3">
                <h3 class="text-lg font-medium text-gray-400">Host Groups</h3>
                <button 
                    hx-get="/api/dashboard/new-group-form"
                    hx-target="#new-group-modal"
                    hx-on::after-request="document.getElementById('new-group-modal').classList.remove('hidden')"
                    class="flex items-center gap-2 px-5 py-2 bg-emerald-600 hover:bg-emerald-500 rounded-2xl text-sm font-medium">
                    <span class="text-xl">+</span> New Host Group
                </button>
            </div>
            <div id="new-group-modal" class="fixed inset-0 bg-black/70 hidden flex items-center justify-center z-50"></div>
            <div class="text-2xl font-medium text-gray-400 p-8">Host Groups — real data coming next</div>
        </div>
    "##.to_string())
}

pub async fn hosts_handler(_pool: State<PgPool>) -> Html<String> {
    Html(r##"
        <div class="mb-6">
            <div class="flex justify-between items-center mb-3">
                <h3 class="text-lg font-medium text-gray-400">Hosts</h3>
                <button 
                    hx-get="/api/dashboard/new-host-form"
                    hx-target="#new-host-modal"
                    hx-on::after-request="document.getElementById('new-host-modal').classList.remove('hidden')"
                    class="flex items-center gap-2 px-5 py-2 bg-emerald-600 hover:bg-emerald-500 rounded-2xl text-sm font-medium">
                    <span class="text-xl">+</span> New Host
                </button>
            </div>
            <div id="new-host-modal" class="fixed inset-0 bg-black/70 hidden flex items-center justify-center z-50"></div>
            <div class="text-2xl font-medium text-gray-400 p-8">Hosts — real data coming next</div>
        </div>
    "##.to_string())
}

pub async fn reports_handler(_pool: State<PgPool>) -> Html<String> {
    Html(r#"<div class="text-2xl font-medium text-gray-400 p-8">📋 Compliance Reports — real data coming next</div>"#.to_string())
}

// New Organization handlers (unchanged from before)
pub async fn new_org_form_handler() -> Html<String> {
    let template = NewOrgFormTemplate;
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<p class='text-red-400'>Form render error</p>".to_string()),
    }
}

pub async fn create_org_handler(State(pool): State<PgPool>, Form(form): Form<NewOrgForm>) -> Html<String> {
    let result = sqlx::query("INSERT INTO organizations (name) VALUES ($1)")
        .bind(&form.name)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            tracing::info!("✅ Created organization: {}", form.name);
            let refreshed = orgs_handler(State(pool)).await;
            Html(format!(
                r#"{}
                <script>document.getElementById('new-org-modal').classList.add('hidden');</script>"#,
                refreshed.0
            ))
        }
        Err(e) => {
            tracing::error!("❌ Failed to create organization '{}': {}", form.name, e);
            if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
                Html("<p class='text-red-400 p-8'>❌ Organization name already exists. Choose a different name.</p>".to_string())
            } else {
                Html(format!("<p class='text-red-400 p-8'>❌ Error creating organization: {}</p>", e))
            }
        }
    }
}

// (new-group, new-host, create-group, create-host, org_detail_handler handlers go here — they are identical to what you already have)

pub async fn new_group_form_handler(State(pool): State<PgPool>) -> Html<String> {
    let orgs: Vec<OrgSelect> = sqlx::query_as("SELECT id, name FROM organizations ORDER BY name")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let template = NewGroupFormTemplate { orgs };
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<p class='text-red-400'>Form render error</p>".to_string()),
    }
}

pub async fn create_group_handler(State(pool): State<PgPool>, Form(form): Form<NewGroupForm>) -> Html<String> {
    let result = sqlx::query("INSERT INTO host_groups (organization_id, name, description) VALUES ($1, $2, $3)")
        .bind(form.organization_id)
        .bind(&form.name)
        .bind(&form.description)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => groups_handler(State(pool)).await,
        Err(e) => Html(format!("<p class='text-red-400 p-8'>❌ Error creating host group: {}</p>", e)),
    }
}

pub async fn new_host_form_handler(State(pool): State<PgPool>) -> Html<String> {
    let orgs: Vec<OrgSelect> = sqlx::query_as("SELECT id, name FROM organizations ORDER BY name")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let template = NewHostFormTemplate { orgs };
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<p class='text-red-400'>Form render error</p>".to_string()),
    }
}

pub async fn create_host_handler(State(pool): State<PgPool>, Form(form): Form<NewHostForm>) -> Html<String> {
    let result = sqlx::query("INSERT INTO hosts (organization_id, hostname, ip_address, description) VALUES ($1, $2, $3, $4)")
        .bind(form.organization_id)
        .bind(&form.hostname)
        .bind(&form.ip_address)
        .bind(&form.description)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => hosts_handler(State(pool)).await,
        Err(e) => Html(format!("<p class='text-red-400 p-8'>❌ Error creating host: {}</p>", e)),
    }
}

pub async fn org_detail_handler(State(pool): State<PgPool>, Path(org_id): Path<i32>) -> Html<String> {
    let org: OrgDetail = sqlx::query_as(r#"
        SELECT o.id, o.name, COUNT(DISTINCT h.id) as host_count
        FROM organizations o
        LEFT JOIN hosts h ON h.organization_id = o.id
        WHERE o.id = $1
        GROUP BY o.id, o.name
    "#)
    .bind(org_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(OrgDetail { id: org_id, name: "Unknown".to_string(), host_count: 0 });

    let groups: Vec<GroupCard> = sqlx::query_as(r#"
        SELECT g.id, g.name, COUNT(DISTINCT m.host_id) as host_count
        FROM host_groups g
        LEFT JOIN host_group_memberships m ON m.host_group_id = g.id
        WHERE g.organization_id = $1
        GROUP BY g.id, g.name
    "#)
    .bind(org_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let hosts: Vec<HostCard> = sqlx::query_as(r#"
        SELECT id, hostname, ip_address, description
        FROM hosts
        WHERE organization_id = $1
    "#)
    .bind(org_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let template = OrgDetailTemplate { org, groups, hosts };
    match template.render() {
        Ok(html) => Html(html),
        Err(_) => Html("<p class='text-red-400'>Error loading organization detail</p>".to_string()),
    }
}