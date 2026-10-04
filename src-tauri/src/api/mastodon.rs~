use mastodon_async::{Mastodon, Registration};
use mastodon_async::registration::Registered;
use mastodon_async::prelude::Status as MastodonStatus;
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub instance_url: String,
    pub access_token: String,
}

#[derive(Serialize, Clone)]
pub struct AccountInfo {
    pub username: String,
    pub display_name: String,
    pub avatar: String,
}

#[derive(Serialize, Clone)]
pub struct TimelineStatus {
    pub id: String,
    pub account: AccountInfo,
    pub content: String,
    pub created_at: String,
    pub reblogs_count: u64,
    pub favourites_count: u64,
}

#[derive(Serialize, Clone)]
pub struct InstanceInfo {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub email: String,
    pub users_count: u64,
}

pub async fn start_login(instance_url: String) -> Result<(String, Registered), String> {
    eprintln!("Mastodon instance: {}", instance_url);
    let registration = Registration::new(instance_url)
        .client_name("masskey")
        .scopes(mastodon_async::scopes::Scopes::all())
        .redirect_uris("urn:ietf:wg:oauth:2.0:oob")
        .build()
        .await
        .map_err(|e| {
            eprintln!("Mastodon registration error: {e:?}");
            e.to_string()
        })?;

    let url = registration.authorize_url().map_err(|e| e.to_string())?;
    Ok((url, registration))
}

pub async fn complete_login(registration: Registered, code: String) -> Result<Account, String> {
    eprintln!("complete_login called with code: {code}");
    let mastodon: Mastodon = registration.complete(code).await.map_err(|e| {
        eprintln!("complete_login error: {e:?}");
        e.to_string()
    })?;
    eprintln!("complete_login success: instance_url={}", mastodon.data.base);
    Ok(Account {
        instance_url: mastodon.data.base.to_string(),
        access_token: mastodon.data.token.to_string(),
    })
}

pub fn create_mastodon_client(instance_url: &str, access_token: &str) -> Result<Mastodon, String> {
    let data = mastodon_async::Data {
        base: Cow::Owned(instance_url.to_string()),
        client_id: Cow::Borrowed(""),
        client_secret: Cow::Borrowed(""),
        redirect: Cow::Borrowed(""),
        token: Cow::Owned(access_token.to_string()),
    };
    Ok(Mastodon::from(data))
}

pub async fn get_home_timeline(mastodon: Mastodon, limit: u32) -> Result<Vec<TimelineStatus>, String> {
    eprintln!("get_home_timeline called with limit={}", limit);
    let page = mastodon.get_home_timeline().await.map_err(|e| {
        eprintln!("get_home_timeline error: {e:?}");
        e.to_string()
    })?;
    
    eprintln!("  Page initial_items count: {}", page.initial_items.len());
    eprintln!("  Page next: {:?}", page.next.is_some());
    eprintln!("  Page prev: {:?}", page.prev.is_some());
    
    if page.initial_items.is_empty() {
        // ホームタイムラインが空の場合はパブリックタイムラインをフォールバックとして使用
        eprintln!("  Home timeline is empty, falling back to public timeline");
        return get_public_timeline(mastodon, limit, true).await;
    }
    
    convert_statuses(page.initial_items.into_iter().take(limit as usize))
}

pub async fn get_public_timeline(mastodon: Mastodon, limit: u32, local: bool) -> Result<Vec<TimelineStatus>, String> {
    eprintln!("get_public_timeline called with limit={}, local={}", limit, local);
    let statuses = mastodon.get_public_timeline(local).await.map_err(|e| {
        eprintln!("get_public_timeline error: {e:?}");
        e.to_string()
    })?;
    
    eprintln!("  Public timeline count: {}", statuses.len());
    convert_statuses(statuses.into_iter().take(limit as usize))
}

pub async fn get_tagged_timeline(mastodon: Mastodon, tag: String, local: bool, limit: u32) -> Result<Vec<TimelineStatus>, String> {
    eprintln!("get_tagged_timeline called with tag={}, local={}, limit={}", tag, local, limit);
    let statuses = mastodon.get_tagged_timeline(tag, local).await.map_err(|e| {
        eprintln!("get_tagged_timeline error: {e:?}");
        e.to_string()
    })?;
    
    eprintln!("  Tagged timeline count: {}", statuses.len());
    convert_statuses(statuses.into_iter().take(limit as usize))
}

pub async fn get_instance_info(mastodon: Mastodon) -> Result<InstanceInfo, String> {
    eprintln!("get_instance_info called");
    let instance = mastodon.instance().await.map_err(|e| {
        eprintln!("get_instance_info error: {e:?}");
        e.to_string()
    })?;
    
    eprintln!("  Instance: {} ({})", instance.uri, instance.title);
    eprintln!("  Description: {}", instance.description);
    
    // Stats fields are private in mastodon-async-entities 1.1.0
    // users_count is set to 0 as a placeholder
    Ok(InstanceInfo {
        uri: instance.uri,
        title: instance.title,
        description: instance.description,
        email: instance.email,
        users_count: 0,
    })
}

pub async fn get_account(mastodon: Mastodon, account_id: String) -> Result<AccountInfo, String> {
    eprintln!("get_account called with id={}", account_id);
    use mastodon_async::prelude::AccountId;
    
    let account = mastodon.get_account(&AccountId::new(&account_id)).await.map_err(|e| {
        eprintln!("get_account error: {e:?}");
        e.to_string()
    })?;
    
    Ok(AccountInfo {
        username: account.acct,
        display_name: account.display_name,
        avatar: account.avatar.to_string(),
    })
}

fn convert_statuses(statuses: impl Iterator<Item = MastodonStatus>) -> Result<Vec<TimelineStatus>, String> {
    let result: Vec<TimelineStatus> = statuses.map(|s| {
        let account = s.account;
        TimelineStatus {
            id: s.id.to_string(),
            account: AccountInfo {
                username: account.acct,
                display_name: account.display_name,
                avatar: account.avatar.to_string(),
            },
            content: strip_html(s.content),
            created_at: format!("{}", s.created_at.date()),
            reblogs_count: s.reblogs_count,
            favourites_count: s.favourites_count,
        }
    }).collect();
    
    Ok(result)
}

fn strip_html(html: String) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        if c == '<' {
            in_tag = true;
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag {
            result.push(c);
        }
    }
    result.trim().to_string()
}
