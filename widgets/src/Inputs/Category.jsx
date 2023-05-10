const ownerId = "nearhorizon.near";

const options = [
  { text: "Wallets", value: "wallets" },
  { text: "Gaming/Metaverse", value: "gaming/metaverse" },
  { text: "DeSci", value: "desci" },
  { text: "Infrastructure", value: "infrastructure" },
  { text: "NFT", value: "nft" },
  { text: "DAO", value: "dao" },
  { text: "Social impact", value: "social-impact" },
  { text: "Web2 expansion", value: "web2" },
  { text: "Web3 Product Partner", value: "web3-product-partner" },
  { text: "Other", value: "other" },
];

return (
  <Widget
    src={`${ownerId}/widget/Inputs.Select`}
    props={{
      label: "Category *",
      noLabel: props.noLabel,
      placeholder: "Wallets",
      options,
      value: props.category,
      onChange: (category) => props.update(category),
      validate: () => {
        if (!props.category) {
          props.setError("Please select a category");
        }

        if (!options.find(({ value }) => props.category.value === value)) {
          props.setError("Please select a valid category");
        }
      },
      error: props.error,
    }}
  />
);
