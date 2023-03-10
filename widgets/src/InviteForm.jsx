const createDate = () => {
  const date = new Date();
  const year = date.getFullYear();
  const month = (date.getMonth() + 1).toString().padStart(2, "0");
  const day = date.getDate().toString().padStart(2, "0");

  return `${year}-${month}-${day}`;
};

const ownerId = "contribut3.near";
const accountId = props.accountId ?? "";
const kind = props.kind ? [{ name: props.kind }] : [];
const startDate = props.startDate ?? createDate();
const id = props.id;

const allContributionTypes = (
  Near.view(ownerId, "get_contribution_types", {}, "final", true) ?? []
).map((name) => ({ name }));

const convertType = (contributionType) => {
  if (allContributionTypes.some(({ name }) => name === contributionType.name)) {
    return contributionType.name;
  }

  return { Other: contributionType.name };
};

State.init({
  entityId: [],
  accountId,
  permissions: [],
  description: props.description ?? "",
  contributionType: [],
  accountIdValid: true,
  startDate,
  forbiddenIds: new Set(),
});

const accountIdInput = accountId ? (
  <div>
    <label htmlFor="account-id" className="text-muted fw-semibold">
      You're inviting:
    </label>
    <div
      className="rounded-3 bg-light"
      style={{ height: "5em" }}
      id="account-id"
    >
      <Widget
        src={`${ownerId}/widget/ProfileLine`}
        props={{ accountId, imageSize: "4em" }}
      />
    </div>
  </div>
) : (
  <div className="col-lg-12 mb-2">
    <Widget
      src={`${ownerId}/widget/ValidatedAccountIdInput`}
      props={{
        label: "Account ID of contributor:",
        value: state.accountId,
        update: (accountId, accountIdValid) =>
          State.update({ accountId, accountIdValid }),
        forbiddenIds: state.forbiddenIds,
      }}
    />
  </div>
);

const descriptionInput = (
  <div className="col-lg-12 mb-2">
    <Widget
      src={`${ownerId}/widget/DescriptionInput`}
      props={{
        description: state.description,
        text: "Details:",
        update: (description) => State.update({ description }),
      }}
    />
  </div>
);

const contributionTypeInput = (
  <div className="col-lg-12 mb-2">
    <Widget
      src={`${ownerId}/widget/ContributionTypeInput`}
      props={{
        contributionType: state.contributionType,
        update: (contributionType) => State.update({ contributionType }),
        allContributionTypes,
      }}
    />
  </div>
);

const startDateInput = (
  <div className="col-lg-6 mb-2">
    <Widget
      src={`${ownerId}/widget/DateInput`}
      props={{
        id: "start-date",
        text: " Start date of contribution:",
        date: state.startDate,
        update: (startDate) => State.update({ startDate }),
      }}
    />
  </div>
);

const permissionsInput = (
  <div className="col-lg-6 mb-2">
    <label htmlFor="permissions-input">Permissions for contributor:</label>
    <Typeahead
      id="permissions-input"
      labelKey="name"
      onChange={(permissions) =>
        State.update({
          permissions,
        })
      }
      options={[{ name: "Admin" }]}
      placeholder="Admin or leave blank"
      selected={state.permissions}
      positionFixed
    />
  </div>
);

const entityIdInput = (
  <Widget
    src={`${ownerId}/widget/AdminEntityAccountIdInput`}
    props={{
      update: (entityId) => {
        State.update({ entityId });
        Near.asyncView(
          ownerId,
          "get_entity_invites",
          { account_id: entityId[0].name },
          "final"
        ).then((invites) =>
          State.update({
            forbiddenIds: new Set(Object.keys(invites)),
          })
        );
      },
      accountId: context.accountId,
      selected: state.entityId,
    }}
  />
);

const onSubmit = () => {
  if (!state.accountIdValid) {
    return;
  }

  const args = {
    entity_id: state.entityId[0].name,
    contributor_id: state.accountId,
    description: state.description,
    start_date: `${new Date(state.startDate).getTime()}`,
    contribution_type: convertType(state.contributionType[0]),
    permissions: state.permissions.map(({ name }) => name),
  };

  Near.call(ownerId, "invite_contributor", args);
};

const header = <div className="card-header">Invite contributor</div>;

const body = (
  <div className="row">
    {accountIdInput}
    {entityIdInput}
    {contributionTypeInput}
    {startDateInput}
    {permissionsInput}
    {descriptionInput}
  </div>
);

return (
  <Widget
    src={`${ownerId}/widget/Modal`}
    props={{
      title: "Invite to contribute",
      confirmText: (
        <>
          <i className="bi-send" />
          <span>Send invitation</span>
        </>
      ),
      onConfirm: onSubmit,
      hidden: props.hidden,
      onClose: props.onClose,
      body,
      id,
    }}
  />
);
