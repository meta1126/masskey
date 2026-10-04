mod api;

use std::sync::Mutex;
use tauri::State;

struct AppState {
    pending_registration: Mutex<Option<mastodon_async::registration::Registered>>,
    mastodon_client: Mutex<Option<mastodon_async::Mastodon>>,
}

#[tauri::command]
async fn start_login(instance_url: String, state: State<'_, AppState>) -> Result<String, String> {
    let (url, registration) = api::mastodon::start_login(instance_url)
    .await
    .map_err(|e| {
        eprintln!("start_login error: {e}");
        e
    })?;
    *state.pending_registration.lock().unwrap() = Some(registration);
    Ok(url)
} 

#[tauri::command]
async fn complete_login(code: String, state: State<'_, AppState>) -> Result<api::mastodon::Account, String> {
    eprintln!("complete_login called, pending_registration state: {:?}", state.pending_registration.lock().unwrap().is_some());
    let registration = state.pending_registration.lock().unwrap().take()
        .ok_or("登録情報がありません。最初からやり直してください")?;
    let account = api::mastodon::complete_login(registration, code).await?;
    // Mastodon クライアントを保存
    let mastodon = api::mastodon::create_mastodon_client(&account.instance_url, &account.access_token)
        .map_err(|e| format!("クライアント作成エラー: {e}"))?;
    *state.mastodon_client.lock().unwrap() = Some(mastodon);
    Ok(account)
}

#[derive(serde::Serialize, Clone)]
pub struct Status {
    pub id: String,
    pub account: api::mastodon::AccountInfo,
    pub content: String,
    pub created_at: String,
    pub reblogs_count: u64,
    pub favourites_count: u64,
}

#[tauri::command]
async fn get_home_timeline(limit: u32, state: State<'_, AppState>) -> Result<Vec<Status>, String> {
    let mastodon = state.mastodon_client.lock().unwrap().clone()
        .ok_or("ログインしていません。まずログインしてください")?;
    let timeline_statuses = api::mastodon::get_home_timeline(mastodon, limit).await?;
    Ok(timeline_statuses.into_iter().map(|ts| Status {
        id: ts.id,
        account: ts.account,
        content: ts.content,
        created_at: ts.created_at,
        reblogs_count: ts.reblogs_count,
        favourites_count: ts.favourites_count,
    }).collect())
}

#[tauri::command]
async fn get_public_timeline(local: bool, limit: u32, state: State<'_, AppState>) -> Result<Vec<Status>, String> {
    let mastodon = state.mastodon_client.lock().unwrap().clone()
        .ok_or("ログインしていません。まずログインしてください")?;
    let timeline_statuses = api::mastodon::get_public_timeline(mastodon, limit, local).await?;
    Ok(timeline_statuses.into_iter().map(|ts| Status {
        id: ts.id,
        account: ts.account,
        content: ts.content,
        created_at: ts.created_at,
        reblogs_count: ts.reblogs_count,
        favourites_count: ts.favourites_count,
    }).collect())
}

#[tauri::command]
async fn get_tagged_timeline(tag: String, local: bool, limit: u32, state: State<'_, AppState>) -> Result<Vec<Status>, String> {
    let mastodon = state.mastodon_client.lock().unwrap().clone()
        .ok_or("ログインしていません。まずログインしてください")?;
    let timeline_statuses = api::mastodon::get_tagged_timeline(mastodon, tag, local, limit).await?;
    Ok(timeline_statuses.into_iter().map(|ts| Status {
        id: ts.id,
        account: ts.account,
        content: ts.content,
        created_at: ts.created_at,
        reblogs_count: ts.reblogs_count,
        favourites_count: ts.favourites_count,
    }).collect())
}

#[tauri::command]
async fn get_instance_info(state: State<'_, AppState>) -> Result<api::mastodon::InstanceInfo, String> {
    let mastodon = state.mastodon_client.lock().unwrap().clone()
        .ok_or("ログインしていません。まずログインしてください")?;
    api::mastodon::get_instance_info(mastodon).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            pending_registration: Mutex::new(None),
            mastodon_client: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            start_login, complete_login,
            get_home_timeline, get_public_timeline, get_tagged_timeline,
            get_instance_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
