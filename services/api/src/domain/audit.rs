use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::tenant::TenantContext;

#[derive(Debug, Clone, PartialEq)]
pub struct AuditEvent {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub actor_user_id: Uuid,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub metadata: Value,
    pub occurred_at: DateTime<Utc>,
}

impl AuditEvent {
    pub fn security_sensitive(
        tenant: &TenantContext,
        action: impl Into<String>,
        target_type: impl Into<String>,
        target_id: Option<Uuid>,
        metadata: Value,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            organization_id: tenant.organization_id,
            actor_user_id: tenant.actor_user_id,
            action: action.into(),
            target_type: target_type.into(),
            target_id,
            metadata,
            occurred_at: Utc::now(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("failed to persist audit event: {0}")]
    Persist(String),
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde_json::json;
    use uuid::Uuid;

    use crate::domain::tenant::{Role, TenantContext};

    use super::AuditEvent;

    #[test]
    fn security_sensitive_event_uses_tenant_context_ownership() {
        let tenant = TenantContext {
            actor_user_id: Uuid::new_v4(),
            organization_id: Uuid::new_v4(),
            roles: BTreeSet::from([Role::Auditor]),
        };

        let event = AuditEvent::security_sensitive(
            &tenant,
            "authorization.policy_updated",
            "role_binding",
            Some(Uuid::new_v4()),
            json!({"organization_id":"client-supplied-value"}),
        );

        assert_eq!(event.organization_id, tenant.organization_id);
        assert_eq!(event.actor_user_id, tenant.actor_user_id);
        assert_eq!(event.action, "authorization.policy_updated");
    }
}
