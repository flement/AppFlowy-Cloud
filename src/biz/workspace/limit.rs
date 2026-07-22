use app_error::ErrorCode;
use shared_entity::dto::billing_dto::WorkspaceUsageAndLimit;
use shared_entity::response::AppResponseError;

pub const SAFE_UNLIMITED_LIMIT: i64 = i64::MAX / 2;

pub fn apply_self_host_unlimited_mode(
  usage: &mut WorkspaceUsageAndLimit,
  self_host_unlimited: bool,
) {
  if !self_host_unlimited {
    return;
  }

  usage.member_count_limit = SAFE_UNLIMITED_LIMIT;
  usage.storage_bytes_unlimited = true;
  usage.storage_bytes_limit = SAFE_UNLIMITED_LIMIT;
  usage.single_upload_unlimited = true;
  usage.single_upload_limit = SAFE_UNLIMITED_LIMIT;
  usage.ai_responses_unlimited = true;
  usage.ai_responses_count_limit = SAFE_UNLIMITED_LIMIT;
  usage.ai_image_responses_count_limit = SAFE_UNLIMITED_LIMIT;
}

pub fn enforce_workspace_member_limit(
  current_member_count: i64,
  member_count_limit: i64,
  self_host_unlimited: bool,
) -> Result<(), AppResponseError> {
  if self_host_unlimited {
    return Ok(());
  }
  if current_member_count >= member_count_limit {
    return Err(AppResponseError::new(
      ErrorCode::WorkspaceMemberLimitExceeded,
      "workspace member limit exceeded".to_string(),
    ));
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keep_usage_limits_when_unlimited_mode_disabled() {
    let mut usage = WorkspaceUsageAndLimit {
      member_count: 10,
      member_count_limit: 20,
      storage_bytes: 1_000,
      storage_bytes_limit: 2_000,
      storage_bytes_unlimited: false,
      single_upload_limit: 200,
      single_upload_unlimited: false,
      ai_responses_count: 1,
      ai_responses_count_limit: 2,
      ai_image_responses_count: 3,
      ai_image_responses_count_limit: 4,
      local_ai: false,
      ai_responses_unlimited: false,
    };

    apply_self_host_unlimited_mode(&mut usage, false);

    assert_eq!(usage.member_count_limit, 20);
    assert!(!usage.storage_bytes_unlimited);
    assert!(!usage.single_upload_unlimited);
    assert!(!usage.ai_responses_unlimited);
    assert_eq!(usage.ai_image_responses_count_limit, 4);
  }

  #[test]
  fn set_usage_limits_to_unlimited_when_enabled() {
    let mut usage = WorkspaceUsageAndLimit {
      member_count: 10,
      member_count_limit: 20,
      storage_bytes: 1_000,
      storage_bytes_limit: 2_000,
      storage_bytes_unlimited: false,
      single_upload_limit: 200,
      single_upload_unlimited: false,
      ai_responses_count: 1,
      ai_responses_count_limit: 2,
      ai_image_responses_count: 3,
      ai_image_responses_count_limit: 4,
      local_ai: false,
      ai_responses_unlimited: false,
    };

    apply_self_host_unlimited_mode(&mut usage, true);

    assert_eq!(usage.member_count_limit, SAFE_UNLIMITED_LIMIT);
    assert!(usage.storage_bytes_unlimited);
    assert_eq!(usage.storage_bytes_limit, SAFE_UNLIMITED_LIMIT);
    assert!(usage.single_upload_unlimited);
    assert_eq!(usage.single_upload_limit, SAFE_UNLIMITED_LIMIT);
    assert!(usage.ai_responses_unlimited);
    assert_eq!(usage.ai_responses_count_limit, SAFE_UNLIMITED_LIMIT);
    assert_eq!(usage.ai_image_responses_count_limit, SAFE_UNLIMITED_LIMIT);
  }

  #[test]
  fn enforce_member_limit_when_unlimited_mode_disabled() {
    let err = enforce_workspace_member_limit(10, 10, false).unwrap_err();
    assert_eq!(err.code, ErrorCode::WorkspaceMemberLimitExceeded);
  }

  #[test]
  fn bypass_member_limit_when_unlimited_mode_enabled() {
    let result = enforce_workspace_member_limit(10_000, 1, true);
    assert!(result.is_ok());
  }
}
