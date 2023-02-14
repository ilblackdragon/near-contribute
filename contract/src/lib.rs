use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::UnorderedMap;
use near_sdk::{env, near_bindgen, require, sys, AccountId, BorshStorageKey, Gas, PanicOnDefault};

use crate::contribution::{
    Contribution, VersionedContribution, VersionedContributionInvite, VersionedContributionNeed,
    VersionedContributionRequest,
};
use crate::contributor::VersionedContributor;
use crate::entity::{Permission, VersionedEntity};

mod contribution;
mod contributor;
mod dec_serde;
mod entity;
mod events;

const MAX_DESCRIPTION_LENGTH: usize = 420;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKeys {
    Entities,
    Contributions,
    Requests,
    Contributors,
    Needs,
    Invites,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    moderator_id: AccountId,
    entities: UnorderedMap<AccountId, VersionedEntity>,
    contributions: UnorderedMap<(AccountId, AccountId), VersionedContribution>,
    requests: UnorderedMap<(AccountId, AccountId), VersionedContributionRequest>,
    contributors: UnorderedMap<AccountId, VersionedContributor>,
    needs: UnorderedMap<(AccountId, String), VersionedContributionNeed>,
    invites: UnorderedMap<(AccountId, AccountId), VersionedContributionInvite>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(moderator_id: AccountId) -> Self {
        Self {
            moderator_id,
            entities: UnorderedMap::new(StorageKeys::Entities),
            contributions: UnorderedMap::new(StorageKeys::Contributions),
            requests: UnorderedMap::new(StorageKeys::Requests),
            contributors: UnorderedMap::new(StorageKeys::Contributors),
            needs: UnorderedMap::new(StorageKeys::Needs),
            invites: UnorderedMap::new(StorageKeys::Invites),
        }
    }

    pub fn set_moderator(&mut self, moderator_id: AccountId) {
        self.assert_moderator();
        self.moderator_id = moderator_id;
    }

    /// Assertions.

    /// Checks if transaction was performed by moderator account.
    fn assert_moderator(&self) {
        // Errors::OnlyModerator.into()
        require!(
            self.moderator_id == env::predecessor_account_id(),
            "ERR_ONLY_MODERATOR"
        );
    }

    /// Checks if given account has permissions of a manager or higher for given entity.
    fn assert_manager_or_higher(&self, entity_id: &AccountId, account_id: &AccountId) {
        require!(
            self.check_is_manager_or_higher(entity_id, account_id),
            "ERR_NO_PERMISSION"
        );
    }

    /// Checks if given account is registered as a contributor.
    #[allow(dead_code)]
    fn assert_is_registered(&self, account_id: &AccountId) {
        require!(
            self.contributors.contains_key(account_id),
            "ERR_NOT_REGISTERED"
        );
    }

    /// Views

    /// Check if given account ID is moderator.
    pub fn check_is_moderator(&self, account_id: AccountId) -> bool {
        self.moderator_id == account_id
    }

    /// Check if given account ID is manager or higher for given entity.
    pub fn check_is_manager_or_higher(
        &self,
        entity_id: &AccountId,
        account_id: &AccountId,
    ) -> bool {
        if account_id == &self.moderator_id {
            return true;
        }
        let Some(contribution) = self.contributions.get(&(entity_id.clone(), account_id.clone())) else {
            return false;
        };
        let contribution = Contribution::from(contribution.clone());
        contribution.permissions.contains(&Permission::Admin)
    }

    /// Should only be called by this contract on migration.
    /// This is NOOP implementation. KEEP IT if you haven't changed contract state.
    /// This method is called from `upgrade()` method.
    /// For next version upgrades, change this function.
    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let this: Contract = env::state_read().expect("Contract is not initialized.");
        this
    }
}

#[no_mangle]
pub fn upgrade() {
    env::setup_panic_hook();

    let contract: Contract = env::state_read().expect("Contract is not initialized");
    contract.assert_moderator();

    const MIGRATE_METHOD_NAME: &[u8; 7] = b"migrate";
    const UPGRADE_GAS_LEFTOVER: Gas = Gas(5_000_000_000_000);

    unsafe {
        // Load code into register 0 result from the input argument if factory call or from promise if callback.
        sys::input(0);
        // Create a promise batch to upgrade current contract with code from register 0.
        let promise_id = sys::promise_batch_create(
            env::current_account_id().as_bytes().len() as u64,
            env::current_account_id().as_bytes().as_ptr() as u64,
        );
        // Deploy the contract code from register 0.
        sys::promise_batch_action_deploy_contract(promise_id, u64::MAX, 0);
        // Call promise to migrate the state.
        // Batched together to fail upgrade if migration fails.
        sys::promise_batch_action_function_call(
            promise_id,
            MIGRATE_METHOD_NAME.len() as u64,
            MIGRATE_METHOD_NAME.as_ptr() as u64,
            0,
            0,
            0,
            (env::prepaid_gas() - env::used_gas() - UPGRADE_GAS_LEFTOVER).0,
        );
        sys::promise_return(promise_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::contribution::{
        Contribution, VersionedContribution, VersionedContributionInvite,
        VersionedContributionNeed, VersionedContributionRequest,
    };
    use crate::contributor::{ContributionType, VersionedContributor};
    use crate::entity::{Entity, EntityKind, EntityStatus, Permission, VersionedEntity};
    use near_sdk::json_types::U64;
    use near_sdk::{test_utils::VMContextBuilder, testing_env, AccountId};

    use crate::Contract;

    #[test]
    fn test_moderator_init() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id)
            .build());

        contract.assert_moderator();
    }

    #[test]
    fn test_check_moderator() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.check_is_moderator(owner_id);
    }

    #[test]
    fn test_set_moderator() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        let new_owner_id: AccountId = "new_owner".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.set_moderator(new_owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(new_owner_id.clone())
            .build());

        contract.check_is_moderator(new_owner_id);
    }

    #[test]
    fn test_add_entity() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity.near".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = near_sdk::json_types::U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind.clone(),
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert_eq!(entity.name, entity_name);
        assert_eq!(entity.kind, entity_kind);
        assert_eq!(entity.start_date, 0);
        assert_eq!(entity.status, EntityStatus::Active);
    }

    #[test]
    fn test_add_multiple_entities() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        let entity_id_proj: AccountId = "project".parse().unwrap();
        let entity_name_proj = "project name".to_string();
        let entity_kind_proj: EntityKind = EntityKind::Project;
        let start_date_proj: U64 = U64(1);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );
        contract.add_entity(
            entity_id_proj.clone(),
            entity_name_proj.clone(),
            entity_kind_proj,
            start_date_proj,
        );

        let entities = contract.get_entities(Some(0), Some(2));

        let entity = entities[0].clone();
        let entity_p = entities[1].clone();

        assert!(entity.0 == entity_id);
        assert!(entity.1.start_date == 0);
        assert!(entity.1.kind == EntityKind::Organization);
        assert!(entity.1.status == EntityStatus::Active);

        assert!(entity_p.0 == entity_id_proj);
        assert!(entity_p.1.start_date == 1);
        assert!(entity_p.1.kind == EntityKind::Project);
        assert!(entity_p.1.status == EntityStatus::Active);
    }

    #[test]
    fn test_request_contribution() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type.clone(),
            None,
        );

        let contribution_req =
            contract.get_contribution_request(entity_id.clone(), contributor_id.clone());

        assert!(contribution_req.unwrap().description == description);
    }

    #[test]
    fn test_approve_contribution() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type.clone(),
            None,
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.approve_contribution(
            entity_id.clone(),
            contributor_id.clone(),
            Some(description.clone()),
            Some(U64(0)),
        );

        let contribution = contract.get_contribution(entity_id.clone(), contributor_id.clone());

        assert!(!contribution.is_none());
    }

    #[test]
    fn test_finish_contribution() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.name == entity_name);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type,
            None,
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.approve_contribution(
            entity_id.clone(),
            contributor_id.clone(),
            Some(description.clone()),
            Some(U64(0)),
        );

        contract.finish_contribution(entity_id.clone(), contributor_id.clone(), U64(1));

        let contribution = contract
            .get_contribution(entity_id.clone(), contributor_id.clone())
            .unwrap();

        assert!(contribution.current.description == description.clone());
        assert!(contribution.current.start_date == 0);
        assert!(contribution.current.end_date.unwrap() == 1);
    }

    #[test]
    fn test_get_contributors() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.name == entity_name);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type,
            None,
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.approve_contribution(
            entity_id.clone(),
            contributor_id.clone(),
            Some(description.clone()),
            Some(U64(0)),
        );

        contract.finish_contribution(entity_id.clone(), contributor_id.clone(), U64(1));

        let contributors = contract.get_contributors();

        // Check if contributors show owner of entity and contributer
        assert_eq!(contributors.len(), 2);
        assert!(contributors.contains(&contributor_id));
        assert!(contributors.contains(&owner_id));
    }

    #[test]
    fn test_assert_manager_or_higher() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.name == entity_name);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type,
            None,
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.approve_contribution(
            entity_id.clone(),
            contributor_id.clone(),
            Some(description.clone()),
            Some(U64(0)),
        );

        contract.finish_contribution(entity_id.clone(), contributor_id.clone(), U64(1));

        contract.assert_manager_or_higher(&entity_id, &owner_id);
    }

    #[test]
    #[should_panic(expected = "ERR_NO_PERMISSION")]
    fn test_assert_manager_or_higher_fail() {
        let owner_id: AccountId = "owner".parse().unwrap();

        let mut contract = Contract::new(owner_id.clone());

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        let entity_id: AccountId = "entity".parse().unwrap();
        let entity_name = "entity name".to_string();
        let entity_kind: EntityKind = EntityKind::Organization;
        let start_date: U64 = U64(0);

        contract.add_entity(
            entity_id.clone(),
            entity_name.clone(),
            entity_kind,
            start_date,
        );

        let entity = contract.get_entity(entity_id.clone());

        assert!(entity.start_date == 0);
        assert!(entity.name == entity_name);
        assert!(entity.kind == EntityKind::Organization);
        assert!(entity.status == EntityStatus::Active);

        let contributor_id: AccountId = "contributor".parse().unwrap();

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(contributor_id.clone())
            .build());

        let description = "test description".to_string();
        let contribution_type = ContributionType::Development;

        contract.request_contribution(
            entity_id.clone(),
            description.clone(),
            contribution_type,
            None,
        );

        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(owner_id.clone())
            .build());

        contract.approve_contribution(
            entity_id.clone(),
            contributor_id.clone(),
            Some(description.clone()),
            Some(U64(0)),
        );

        contract.finish_contribution(entity_id.clone(), contributor_id.clone(), U64(1));

        contract.assert_manager_or_higher(&entity_id, &contributor_id);
    }
}
