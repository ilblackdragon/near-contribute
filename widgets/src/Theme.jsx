const fkGroteskFamily = fetch(
    "https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;700&display=swap"
  ).body;
  
  const interFamily = fetch(
    "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&family=Space+Grotesk:wght@400;700&display=swap"
  ).body;
  
  if (!fkGroteskFamily && !interFamily) {
    return;
  }
  
  const Theme = styled.div`
  * {
      font-family: 'Space Grotesk, Inter';
  }
    ${fkGroteskFamily}
    ${interFamily}
  `;
  
  return (
    <Theme>
      {props.children}
    </Theme>
  );
  