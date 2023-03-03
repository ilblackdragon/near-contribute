use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Timestamp};
use std::collections::{HashMap, HashSet};

use crate::contribution::{
    Contribution, ContributionDetail, ContributionInvite, ContributionRequest,
    VersionedContribution, VersionedContributionInvite, VersionedContributionRequest,
};
use crate::contributor::{ContributionType, VersionedContributor};
use crate::dec_serde::{option_u64_dec_format, u64_dec_format};
use crate::events::Events;
use crate::{Contract, ContractExt};

/// An entity can be in different states because it can potentially have an end (through different
/// ways - legal issues, no funding...).
/// This is represented by the EntityStatus.
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum EntityStatus {
    Active,
    Flagged,
}

/// An entity can take different shapes, and currently we can categorize them in these types.
#[allow(clippy::upper_case_acronyms)]
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum EntityKind {
    Project,
    Organization,
    DAO,
}

/// Entity is something that is beyond a single person.
/// Something that has a start and potentially an end.
/// Note, that all the basic information like name, description and social information is stored in the `socialdb`.
#[derive(BorshSerialize, BorshDeserialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Entity {
    /// Name of the entity.
    name: String,
    /// Status of the entity.
    status: EntityStatus,
    /// The type of the entity.
    kind: EntityKind,
    /// The start date of the entity.
    #[serde(with = "u64_dec_format")]
    start_date: Timestamp,
    /// The end date of the entity. (optional)
    #[serde(with = "option_u64_dec_format")]
    end_date: Option<Timestamp>,
}

/// Permissions table for interaction between a contributor and an entity.
#[derive(
    BorshSerialize, BorshDeserialize, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Hash, Clone,
)]
#[serde(crate = "near_sdk::serde")]
pub enum Permission {
    Admin,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum VersionedEntity {
    Current(Entity),
}

impl From<VersionedEntity> for Entity {
    fn from(value: VersionedEntity) -> Self {
        match value {
            VersionedEntity::Current(e) => e,
        }
    }
}

#[near_bindgen]
impl Contract {
    /// Add new entity and user as founding contributor.
    pub fn admin_add_entity(
        &mut self,
        account_id: AccountId,
        founder_id: AccountId,
        name: String,
        kind: EntityKind,
        start_date: U64,
    ) {
        self.assert_moderator();
        self.entities.insert(
            account_id.clone(),
            VersionedEntity::Current(Entity {
                name,
                status: EntityStatus::Active,
                kind,
                start_date: start_date.into(),
                end_date: None,
            }),
        );
        self.contributors
            .entry(founder_id.clone())
            .or_insert(VersionedContributor::Current(Default::default()));
        self.contributions.insert(
            (account_id, founder_id),
            VersionedContribution::Current(Contribution {
                permissions: HashSet::from([Permission::Admin]),
                current: ContributionDetail {
                    description: "".to_string(),
                    start_date: start_date.into(),
                    contribution_type: ContributionType::Founding,
                    end_date: None,
                    need: None,
                },
                history: vec![],
            }),
        );
    }

    /// Add new entity and given user as founding contributor.
    pub fn add_entity(
        &mut self,
        account_id: AccountId,
        name: String,
        kind: EntityKind,
        start_date: U64,
    ) {
        if self.entities.contains_key(&account_id) {
            env::panic_str("ERR_ENTITY_EXISTS");
        }
        self.entities.insert(
            account_id.clone(),
            VersionedEntity::Current(Entity {
                name,
                status: EntityStatus::Active,
                kind,
                start_date: start_date.into(),
                end_date: None,
            }),
        );
        self.contributors
            .entry(env::predecessor_account_id())
            .or_insert(VersionedContributor::Current(Default::default()));
        self.contributions.insert(
            (account_id.clone(), env::predecessor_account_id()),
            VersionedContribution::Current(Contribution {
                permissions: HashSet::from([Permission::Admin]),
                current: ContributionDetail {
                    description: "".to_string(),
                    start_date: start_date.into(),
                    contribution_type: ContributionType::Founding,
                    end_date: None,
                    need: None,
                },
                history: vec![],
            }),
        );
        Events::AddEntity {
            entity_id: account_id,
        }
        .emit();
    }

    /// Claim an entity.
    pub fn request_claim_entity(&mut self, account_id: AccountId) {
        self.requests.insert(
            (account_id, env::predecessor_account_id()),
            VersionedContributionRequest::Current(ContributionRequest {
                description: "".to_string(),
                contribution_type: ContributionType::Founding,
                need: None,
            }),
        );
    }

    /// Approve a claim request.
    pub fn approve_claim_entity(
        &mut self,
        entity_id: AccountId,
        contributor_id: AccountId,
        start_date: U64,
        remove_current: bool,
    ) {
        self.assert_manager_or_higher(&entity_id, &env::predecessor_account_id());
        let Some(request) = self.requests.remove(&(entity_id.clone(), contributor_id.clone())) else {
            env::panic_str("ERR_NO_REQUEST");
        };
        let request = ContributionRequest::from(request);
        let contribution_detail = ContributionDetail {
            description: request.description,
            contribution_type: request.contribution_type,
            need: None,
            start_date: start_date.into(),
            end_date: None,
        };
        let permissions = HashSet::from_iter([Permission::Admin]);
        self.contributions
            .entry((entity_id.clone(), contributor_id.clone()))
            .and_modify(|v_old| {
                let old = Contribution::from(v_old.clone());
                *v_old = VersionedContribution::Current(old.add_detail(
                    start_date.into(),
                    contribution_detail.clone(),
                    Some(permissions.clone()),
                ));
            })
            .or_insert(VersionedContribution::Current(Contribution {
                permissions,
                current: contribution_detail,
                history: vec![],
            }));
        if remove_current {
            self.contributions
                .remove(&(entity_id.clone(), env::predecessor_account_id()));
        }
        Events::ClaimEntity {
            entity_id,
            contributor_id,
            approver_id: env::predecessor_account_id(),
            start_date: start_date.into(),
        }
        .emit();
    }

    /// Admin or moderator updates the entity details.
    pub fn set_entity(&mut self, account_id: AccountId, entity: Entity) {
        self.assert_manager_or_higher(&account_id, &env::predecessor_account_id());
        self.entities
            .insert(account_id, VersionedEntity::Current(entity));
    }

    /// Invite a user as a contributor to an entity.
    pub fn invite_contributor(
        &mut self,
        entity_id: AccountId,
        contributor_id: AccountId,
        description: String,
        contribution_type: ContributionType,
        start_date: U64,
        permissions: HashSet<Permission>,
    ) {
        if self
            .invites
            .contains_key(&(entity_id.clone(), contributor_id.clone()))
        {
            env::panic_str("ERR_INVITE_EXISTS");
        }
        self.assert_manager_or_higher(&entity_id, &env::predecessor_account_id());
        self.contributors
            .entry(contributor_id.clone())
            .or_insert(VersionedContributor::Current(Default::default()));
        self.invites.insert(
            (entity_id.clone(), contributor_id.clone()),
            VersionedContributionInvite::Current(ContributionInvite {
                permissions,
                description: description.clone(),
                contribution_type: contribution_type.clone(),
                start_date: start_date.into(),
            }),
        );
        Events::InviteContributor {
            entity_id,
            contributor_id,
            description,
            contribution_type,
            start_date: start_date.into(),
        }
        .emit()
    }

    /// Accept a contribution invite from an entity with the given account ID.
    pub fn accept_invite(&mut self, account_id: AccountId) {
        let invite = ContributionInvite::from(
            self.invites
                .remove(&(account_id.clone(), env::predecessor_account_id()))
                .expect("ERR_NO_INVITE"),
        );
        let contribution_detail = ContributionDetail {
            description: invite.description.clone(),
            contribution_type: invite.contribution_type.clone(),
            start_date: invite.start_date,
            end_date: None,
            need: None,
        };
        self.contributions
            .entry((account_id.clone(), env::predecessor_account_id()))
            .and_modify(|v_old| {
                let old = Contribution::from(v_old.clone());
                *v_old = VersionedContribution::Current(old.add_detail(
                    invite.start_date,
                    contribution_detail.clone(),
                    None,
                ));
            })
            .or_insert(VersionedContribution::Current(Contribution {
                permissions: invite.permissions.clone(),
                current: contribution_detail.clone(),
                history: vec![],
            }));
        Events::AcceptInvite {
            entity_id: account_id,
            contributor_id: env::predecessor_account_id(),
            description: invite.description,
            contribution_type: invite.contribution_type,
            start_date: invite.start_date,
        }
        .emit();
    }

    /// Reject a contribution inivte from an entity with the given account ID.
    pub fn reject_invite(&mut self, account_id: AccountId) {
        self.invites
            .remove(&(account_id, env::predecessor_account_id()))
            .expect("ERR_NO_INVITE");
    }

    /// Views

    /// List out entities. By default list all of them.
    pub fn get_entities(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> HashMap<AccountId, Entity> {
        let from_index = from_index.unwrap_or(0);
        let limit = limit.unwrap_or(self.entities.len().into());
        self.entities
            .into_iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|(key, entity)| (key.clone(), entity.clone().into()))
            .collect()
    }

    /// List out entities that account ID is admin for.
    pub fn get_admin_entities(&self, account_id: AccountId) -> HashMap<AccountId, Entity> {
        self.contributions
            .into_iter()
            .filter_map(|((entity_id, contributor_id), contribution)| {
                (contributor_id == &account_id
                    && Contribution::from(contribution.clone())
                        .permissions
                        .contains(&Permission::Admin))
                .then_some((
                    entity_id.clone(),
                    self.entities.get(entity_id).unwrap().clone().into(),
                ))
            })
            .collect()
    }

    /// List single entity details.
    pub fn get_entity(&self, account_id: AccountId) -> Entity {
        self.entities
            .get(&account_id)
            .expect("ERR_NO_ENTITY")
            .clone()
            .into()
    }

    /// List entity founders.
    pub fn get_founders(&self, account_id: AccountId) -> HashSet<AccountId> {
        self.contributions
            .into_iter()
            .filter_map(|((entity_id, contributor_id), contribution)| {
                (entity_id == &account_id && {
                    let contribution = Contribution::from(contribution.clone());
                    let founding_type = vec![
                        ContributionType::Founding,
                        ContributionType::Other("Founding".to_string()),
                    ];
                    founding_type.contains(&contribution.current.contribution_type)
                        || contribution
                            .history
                            .into_iter()
                            .any(|detail| founding_type.contains(&detail.contribution_type))
                })
                .then_some(contributor_id.clone())
            })
            .collect()
    }

    /// Check if account ID is an entity.
    pub fn check_is_entity(&self, account_id: AccountId) -> bool {
        self.entities.contains_key(&account_id)
    }

    /// List invites sent by entity with given account ID.
    pub fn get_entity_invites(
        &self,
        account_id: AccountId,
    ) -> HashMap<AccountId, ContributionInvite> {
        self.invites
            .into_iter()
            .filter_map(|((entity_id, contributor_id), invite)| {
                (entity_id == &account_id)
                    .then_some((contributor_id.clone(), invite.clone().into()))
            })
            .collect()
    }

    /// List invites sent to contributor with given account ID.
    pub fn get_contributor_invites(
        &self,
        account_id: AccountId,
    ) -> HashMap<AccountId, ContributionInvite> {
        self.invites
            .into_iter()
            .filter_map(|((entity_id, contributor_id), invite)| {
                (contributor_id == &account_id)
                    .then_some((entity_id.clone(), invite.clone().into()))
            })
            .collect()
    }

    /// Get invite details for entity and contributor with given IDs.
    pub fn get_invite(
        &self,
        entity_id: AccountId,
        contributor_id: AccountId,
    ) -> Option<ContributionInvite> {
        self.invites
            .get(&(entity_id, contributor_id))
            .map(|invite| invite.clone().into())
    }
}
