use serde::{Deserialize, Serialize};

/// Top-level enum to represent different types of Gitea webhook events.
///
/// This allows you to deserialize any incoming webhook JSON into a single enum,
/// then match on the variant to handle specific event types.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GiteaWebhookEvent {
    /// Represents a `push` event webhook payload.
    Push(PushEventPayload),
    /// Represents a `repository` event webhook payload (e.g., created, deleted).
    Repository(RepositoryEventPayload),
    // Add other Gitea event types here as needed
}

/// Represents the payload for a Gitea `push` webhook event.
/// Fields typically always present in a push event are made non-optional.
#[derive(Debug, Serialize, Deserialize)]
pub struct PushEventPayload {
    #[serde(rename = "ref")]
    pub git_ref: String, // Likely always present in a push
    pub before: String,              // Likely always present
    pub after: String,               // Likely always present
    pub compare_url: String,         // Likely always present
    pub commits: Vec<GiteaCommit>,   // A push usually has at least one commit
    pub total_commits: u32,          // Always present with commits
    pub head_commit: GiteaCommit,    // Always present with commits
    pub repository: GiteaRepository, // Essential for a push event
    pub pusher: GiteaUser,           // Essential for a push event
    pub sender: GiteaUser,           // Essential for a push event
}

/// Represents the payload for a Gitea `repository` webhook event.
/// Fields included in your JSON example are made non-optional.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryEventPayload {
    pub action: RepositoryAction, // 'action' is always present in a repository event
    pub repository: GiteaRepository, // 'repository' object is always present
    pub organization: GiteaUser,  // 'organization' was present in your example
    pub sender: GiteaUser,        // 'sender' was present in your example
}

/// Defines the possible actions within a `repository` webhook event.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryAction {
    Created,
    Deleted,
    // Add other repository actions as they appear
}

/// Represents a commit object in Gitea's webhook payloads.
/// Made `id`, `message`, `url`, `author`, `committer`, `timestamp` non-optional as they are core to a commit.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaCommit {
    pub id: String,                                    // Commit ID is fundamental
    pub message: String,                               // Commit message is fundamental
    pub url: String,                                   // URL to the commit is fundamental
    pub author: GiteaCommitUser,                       // Commit author is fundamental
    pub committer: GiteaCommitUser,                    // Commit committer is fundamental
    pub verification: Option<GiteaCommitVerification>, // Verification might not always be present/applicable
    pub timestamp: String,                             // Commit timestamp is fundamental
    pub added: Vec<String>,                            // Can be empty, but present
    pub removed: Vec<String>,                          // Can be empty, but present
    pub modified: Vec<String>,                         // Can be empty, but present
}

/// Represents a user associated with a commit (author or committer).
/// Made `name`, `email`, `username` non-optional as they are core user info in this context.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaCommitUser {
    pub name: String,     // Name is fundamental
    pub email: String,    // Email is fundamental
    pub username: String, // Username is fundamental
}

/// Represents commit verification information.
/// Keep fields optional as verification might not always be present or fully detailed.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaCommitVerification {
    // Add fields if needed and determine if they are always present.
    // For now, keeping them optional or leaving empty if not explicitly defined.
    // pub verified: Option<bool>,
    // pub reason: Option<String>,
    // pub signature: Option<String>,
    // pub payload: Option<String>,
}

/// Represents a repository object in Gitea's webhook payloads.
/// Many fields from your JSON are made non-optional as they were consistently present.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaRepository {
    pub id: u64,
    pub owner: GiteaUser,
    pub name: String,
    pub full_name: String,
    pub description: String,
    pub empty: bool,
    pub private: bool,
    pub fork: bool,
    pub template: bool,
    pub parent: Option<Box<GiteaRepository>>, // Parent is null if not a fork, so remains optional
    pub mirror: bool,
    pub size: u64,
    pub language: String,
    pub languages_url: String,
    pub html_url: String,
    pub url: String,
    pub link: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub original_url: String,
    pub website: String,
    pub stars_count: u64,
    pub forks_count: u64,
    pub watchers_count: u64,
    pub open_issues_count: u64,
    pub open_pr_counter: u64,
    pub release_counter: u64,
    pub default_branch: String,
    pub archived: bool,
    pub created_at: String,
    pub updated_at: String,
    pub archived_at: String, // Was "1970-01-01T01:00:00+01:00" in JSON, so it's always there
    pub permissions: GiteaPermissions,
    pub has_issues: bool,
    pub internal_tracker: GiteaInternalTracker,
    pub has_wiki: bool,
    pub wiki_branch: String,
    pub globally_editable_wiki: bool,
    pub has_pull_requests: bool,
    pub has_projects: bool,
    pub has_releases: bool,
    pub has_packages: bool,
    pub has_actions: bool,
    pub ignore_whitespace_conflicts: bool,
    pub allow_merge_commits: bool,
    pub allow_rebase: bool,
    pub allow_rebase_explicit: bool,
    pub allow_squash_merge: bool,
    pub allow_fast_forward_only_merge: bool,
    pub allow_rebase_update: bool,
    pub default_delete_branch_after_merge: bool,
    pub default_merge_style: String,
    pub default_allow_maintainer_edit: bool,
    pub default_update_style: String,
    pub avatar_url: String,
    pub internal: bool,
    pub mirror_interval: String,
    pub object_format_name: String,
    pub mirror_updated: String,
    pub repo_transfer: Option<String>, // Was null in JSON, so remains optional
    pub topics: Vec<String>,           // Was empty array in JSON, but always present
}

/// Represents a user object in Gitea's webhook payloads.
/// Many fields from your JSON are made non-optional as they were consistently present.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaUser {
    pub id: u64,
    pub login: String,
    pub login_name: String, // Was "" in JSON, but present
    pub source_id: u64,
    pub full_name: String,
    pub email: String,
    pub avatar_url: String,
    pub html_url: String,
    pub language: String, // Was "" in JSON, but present
    pub is_admin: bool,
    pub last_login: String,
    pub created: String,
    pub restricted: bool,
    pub active: bool,
    pub prohibit_login: bool,
    pub location: String,    // Was "" in JSON, but present
    pub pronouns: String,    // Was "" or "he/him" in JSON, but present
    pub website: String,     // Was "" or URL in JSON, but present
    pub description: String, // Was "" or value in JSON, but present
    pub visibility: String,
    pub followers_count: u64,
    pub following_count: u64,
    pub starred_repos_count: u64,
    pub username: String,
}

/// Represents repository permissions.
/// Made all fields non-optional as they were present in your JSON example.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaPermissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

/// Represents internal issue tracker settings for a repository.
/// Made all fields non-optional as they were present in your JSON example.
#[derive(Debug, Serialize, Deserialize)]
pub struct GiteaInternalTracker {
    pub enable_time_tracker: bool,
    pub allow_only_contributors_to_track_time: bool,
    pub enable_issue_dependencies: bool,
}
