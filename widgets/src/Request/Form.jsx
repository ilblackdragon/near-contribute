const ownerId = "contribut3.near";

const LineContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-start;
  gap: 0.25em;

  img {
    vertical-align: top;
  }
`;

const Name = styled.div`
  font-style: normal;
  font-weight: 600;
  font-size: 0.95em;
  line-height: 1em;
  color: #101828;
`;

const AccountId = styled.div`
  font-style: normal;
  font-weight: 400;
  font-size: 0.75em;
  line-height: 1em;
  color: #7e868c;
`;

const ImageCircle = styled.img`
  background: #fafafa;
  border-radius: 8px;
  object-fit: cover;
  width: 100%;
  height: 100%;
`;

const ImageContainer = styled.div`
  display: inline-block;
  width: 1em;
  height: 1em;
`;

const createProjectLine = (accountId, name, image) => {
  const fullName = name ?? accountId;
  const url =
    (image.ipfs_cid
      ? `https://ipfs.near.social/ipfs/${image.ipfs_cid}`
      : image.url) || "https://thewiki.io/static/media/sasha_anon.6ba19561.png";
  const imageSrc = `https://i.near.social/thumbnail/${url}`;

  return (
    <LineContainer>
      <ImageContainer title={`${fullName} @${accountId}`}>
        <ImageCircle src={imageSrc} alt="profile image" />
      </ImageContainer>
      <Name>{name}</Name>
      <AccountId>@{accountId}</AccountId>
    </LineContainer>
  );
};

const Form = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: flex-start;
  width: 60%;
  gap: 1em;
`;

const FormHeader = styled.h3`
  box-sizing: border-box;
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  padding: 0px 0px 0.5em;
  border-bottom: 1px solid #eceef0;
  font-style: normal;
  font-weight: 700;
  font-size: 1.125em;
  line-height: 1.25em;
  color: #000000;
  width: 100%;
`;

const FormFooter = styled.div`
  display: flex;
  flex-direction: row-reverse;
  align-items: center;
  justify-content: space-between;
  width: 100%;
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  gap: 3em;
  padding-bottom: 3em;
`;

const Header = styled.h1`
  font-style: normal;
  font-weight: 700;
  font-size: 2em;
  line-height: 1.4em;
  text-align: center;
  color: #000000;
`;

const SubHeader = styled.h2`
  font-style: normal;
  font-weight: 400;
  font-size: 0.95em;
  line-height: 1.25em;
  text-align: center;
  color: #101828;
`;

const ProgressBar = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  gap: 0.375em;
  width: 100%;
  height: 0.5em;
  padding: 0;
  margin: 0;

  div {
    flex-grow: 1;
    height: 100%;
    width: 50%;
    background: #00ec97;
  }

  &.half {
    div:last-child {
      background: #eceef0;
    }
  }
`;

const CancelButton = styled.a`
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
  padding: 0.75em 1em;
  gap: 0.5em;
  background: #ffffff;
  border: 1px solid #eceef0;
  border-radius: 50px;
  font-style: normal;
  font-weight: 600;
  font-size: 0.95em;
  line-height: 1em;
  text-align: center;
  color: #101828;
`;

State.init({
  projectId: null,
  projects: [],
  projectsIsFetched: false,
  tags: [],
  title: "",
  description: "",
  requestType: null,
  requestTypes: [],
  paymentType: null,
  paymentTypes: [],
  paymentSource: null,
  paymentSources: [],
  budget: null,
  deadline: null,
});

if (!state.projectsIsFetched) {
  Near.asyncView(ownerId, "get_payment_types", {}, "final", false).then(
    (paymentTypes) =>
      State.update({
        paymentTypes: paymentTypes.map((value) => ({ value, text: value })),
      })
  );
  Near.asyncView(ownerId, "get_payment_sources", {}, "final", false).then(
    (paymentSources) =>
      State.update({
        paymentSources: paymentSources.map((value) => ({ value, text: value })),
      })
  );
  Near.asyncView(ownerId, "get_request_types", {}, "final", false).then(
    (requestTypes) =>
      State.update({
        requestTypes: requestTypes.map((value) => ({ value, text: value })),
      })
  );
  Near.asyncView(
    ownerId,
    "get_admin_projects",
    { account_id: context.accountId },
    "final",
    false
  ).then((projects) => {
    Near.asyncView(
      "social.near",
      "get",
      { keys: projects.map((accountId) => `${accountId}/profile/**`) },
      "final",
      false
    ).then((data) =>
      State.update({
        projects: projects.map((accountId) => ({
          // text: <Widget
          //   src={`${ownerId}/widget/Project.Line`}
          //   props={{ accountId, size: "1em" }}
          // />,
          text: createProjectLine(
            accountId,
            data[accountId].profile.name,
            data[accountId].profile.image
          ),
          value: accountId,
        })),
        projectsIsFetched: true,
      })
    );
  });
}

return (
  <Container>
    {/*<ProgressBar className={state.step === "step1" ? "half" : ""}><div /><div /></ProgressBar>*/}
    <div>
      <Header>Create new contribution request</Header>
      <SubHeader>
        Use this form to post your business needs and match with reputable
        contributors and service providers with ease
      </SubHeader>
    </div>
    <Form>
      <FormHeader>Request details</FormHeader>
      <Widget
        src={`${ownerId}/widget/Inputs.Select`}
        props={{
          label: "Request as *",
          value: state.projectId,
          options: state.projects,
          onChange: (projectId) => State.update({ projectId }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Text`}
        props={{
          label: "Title",
          placeholder: "Looking for Rust developer to create smart contracts",
          value: state.title,
          onChange: (title) => State.update({ title }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.TextArea`}
        props={{
          label: "Description",
          placeholder:
            "Crypto ipsum bitcoin ethereum dogecoin litecoin. Holo stacks fantom kava flow algorand. Gala dogecoin gala XRP binance flow. Algorand polygon bancor arweave avalanche. Holo kadena telcoin kusama BitTorrent flow holo velas horizen. TerraUSD helium filecoin terra shiba-inu. Serum algorand horizen kava flow maker telcoin algorand enjin. Dai bitcoin.",
          value: state.description,
          onChange: (description) => State.update({ description }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.MultiSelect`}
        props={{
          label: "Tags",
          placeholder: "DeFi, Gaming...",
          options: [{ name: "Wallets" }, { name: "Games" }],
          value: state.tags,
          onChange: (tags) => State.update({ tags }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Select`}
        props={{
          label: "Request type *",
          options: state.requestTypes,
          value: state.requestType,
          onChange: (requestType) => State.update({ requestType }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Select`}
        props={{
          label: "Payment type *",
          options: state.paymentTypes,
          value: state.paymentType,
          onChange: (paymentType) => State.update({ paymentType }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Select`}
        props={{
          label: "Payment source *",
          options: state.paymentSources,
          value: state.paymentSource,
          onChange: (paymentSource) => State.update({ paymentSource }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Number`}
        props={{
          label: "Budget *",
          placeholder: 1500,
          value: state.budget,
          onChange: (budget) => State.update({ budget }),
        }}
      />
      <Widget
        src={`${ownerId}/widget/Inputs.Date`}
        props={{
          label: "Deadline *",
          value: state.deadline,
          onChange: (deadline) => State.update({ deadline }),
        }}
      />
      <FormFooter>
        <Widget
          src={`${ownerId}/widget/Buttons.Green`}
          props={{
            onClick: () => {
              Near.call(ownerId, "add_request", {
                request: {
                  project_id: state.projectId.value,
                  title: state.title,
                  description: state.description,
                  open: true,
                  request_type: state.requestType.value,
                  payment_type: state.paymentType.value,
                  tags: state.tags.map(({ name }) => name),
                  source: state.paymentSource.value,
                  deadline: `${new Date(state.deadline).getTime()}`,
                  budget: Number(state.budget),
                },
              });
            },
            text: (
              <>
                <svg
                  width="18"
                  height="18"
                  viewBox="0 0 18 18"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M7.87464 10.1251L15.7496 2.25013M7.97033 10.3712L9.94141 15.4397C10.1151 15.8862 10.2019 16.1094 10.327 16.1746C10.4354 16.2311 10.5646 16.2312 10.6731 16.1748C10.7983 16.1098 10.8854 15.8866 11.0596 15.4403L16.0023 2.77453C16.1595 2.37164 16.2381 2.1702 16.1951 2.04148C16.1578 1.92969 16.0701 1.84197 15.9583 1.80462C15.8296 1.76162 15.6281 1.84023 15.2252 1.99746L2.55943 6.94021C2.11313 7.11438 1.88997 7.20146 1.82494 7.32664C1.76857 7.43516 1.76864 7.56434 1.82515 7.67279C1.89033 7.7979 2.11358 7.88472 2.56009 8.05836L7.62859 10.0294C7.71923 10.0647 7.76455 10.0823 7.80271 10.1095C7.83653 10.1337 7.86611 10.1632 7.89024 10.1971C7.91746 10.2352 7.93508 10.2805 7.97033 10.3712Z"
                    stroke="#11181C"
                    stroke-width="1.66667"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
                Publish request
              </>
            ),
          }}
        />
        <CancelButton href={`/${ownerId}/widget/Index`}>Cancel</CancelButton>
      </FormFooter>
    </Form>
  </Container>
);