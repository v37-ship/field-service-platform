use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Dispatcher,
    Technician,
    Auditor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedActorIdentity {
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub roles: BTreeSet<Role>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntrustedAccessRequest {
    pub organization_id: Option<Uuid>,
    pub requested_roles: BTreeSet<Role>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TenantContext {
    pub actor_user_id: Uuid,
    pub organization_id: Uuid,
    pub roles: BTreeSet<Role>,
}

impl TenantContext {
    pub fn from_identity(
        identity: &TrustedActorIdentity,
        _request: &UntrustedAccessRequest,
    ) -> TenantContext {
        TenantContext {
            actor_user_id: identity.user_id,
            organization_id: identity.organization_id,
            roles: identity.roles.clone(),
        }
    }

    pub fn has_role(&self, role: &Role) -> bool {
        self.roles.contains(role)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use super::{Role, TenantContext, TrustedActorIdentity, UntrustedAccessRequest};

    #[test]
    fn tenant_context_uses_only_trusted_identity_values() {
        let trusted_org_id = Uuid::new_v4();
        let trusted_user_id = Uuid::new_v4();

        let identity = TrustedActorIdentity {
            user_id: trusted_user_id,
            organization_id: trusted_org_id,
            roles: BTreeSet::from([Role::Technician]),
        };

        let untrusted_request = UntrustedAccessRequest {
            organization_id: Some(Uuid::new_v4()),
            requested_roles: BTreeSet::from([Role::Owner, Role::Auditor]),
        };

        let context = TenantContext::from_identity(&identity, &untrusted_request);

        assert_eq!(context.organization_id, trusted_org_id);
        assert_eq!(context.actor_user_id, trusted_user_id);
        assert_eq!(context.roles, BTreeSet::from([Role::Technician]));
        assert!(!context.has_role(&Role::Owner));
    }
}
