const Heading = styled.div`
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 0px;

  h1 {
    font-style: normal;
    font-weight: 700;
    font-size: 2em;
    color: #101828;
  }

  h2 {
    font-style: normal;
    font-weight: 400;
    font-size: 1em;
    line-height: 1.5em;
    color: #475467;
  }
`;

const Container = styled.div`
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  gap: 1.5em;
`;

const Section = styled.div`
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 0px;
  gap: 1.5em;

  h3 {
    font-style: normal;
    font-weight: 700;
    font-size: 1.5em;
    line-height: 36px;
    color: #000000;
  }

  span.underline {
    text-decoration: underline;
  }

  ul.numbered {
    list-style-type: number;
  }

  div.bordered {
    border: 1px solid #e6e6e6;
    padding: 1em;
  }
`;

const FirstSubSection = styled.div`
  position: relative;
  width: 90%;

  a {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
  }

  p {
    text-align: center;
    margin-top: 1em;
  }

  img.play {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  img.image {
    width: 100%;
  }
`;

return (
  <Container>
    <Widget
      src={"nearhorizon.near/widget/InfoSegment"}
      props={{
        title: "Need help?",
        description: (
          <>
            Reach out to us at{" "}
            <a href="mailto:horizon@near.foundation">horizon@near.foundation</a>
            !
          </>
        ),
      }}
    />
    <Heading>
      <h1>NEAR Horizon Getting Started Guide</h1>
      <h2>
        Welcome to the NEAR Horizon Getting Started Guide. This guide will help
        you get started with NEAR Horizon, a Web3 platform for connecting
        startups to contributors, services, resources, and financing.{" "}
      </h2>
    </Heading>
    <Section>
      <FirstSubSection>
        <a href="https://youtu.be/EN7TIUr_fkg">
          <img
            className="play"
            src="https://nearhorizonassets.s3.amazonaws.com/icon_circled-play.png"
          />
          <img
            className="image"
            src="https://nearhorizonassets.s3.amazonaws.com/petar-nh-walkthrough_thumbnail.jpg"
            alt="NH Application Walkthrough"
          />
        </a>
        <p>Video Walkthrough of the Horizon Application</p>
      </FirstSubSection>
      <br />
      <br />
      <h3>Landing Page</h3>
      <p>
        Whether you've navigated to the NEAR Horizon landing page from a shared
        link or found it through a BOS Gateway, you'll be greeted with a
        personalized dashboard.
      </p>
      <img
        alt="near horizon landing page"
        src="https://nearhorizonassets.s3.amazonaws.com/landing_basic_features_optimized.gif"
      />
      <p>
        Here you can create a new project, manage your existing projects, update
        your profile, check your inbox, and more. These options are available in
        the sidebar on the left.
      </p>
    </Section>
    <Section>
      <h3>Creating a Project</h3>
      <p>
        To create a new project, click the "Create new" button on the top right
        of the page. A drop-down will appear. Select "Project". A form will
        appear. Fill out the form with the details of your project.
      </p>
      <img
        alt="creating a project on near horizon"
        src="https://nearhorizonassets.s3.amazonaws.com/create_project_optimized.gif"
      />
      <p>
        When you are ready, click the "Create project" button at the bottom of
        the form. Or you can save the form and complete later by clicking the
        "Save for later" button (also at the bottom of the form). Once created,
        you will be navigated to the project page.{" "}
      </p>
      <p>
        Take a moment to review your project details which are displayed in the
        sidebar on the right. You can edit any of these details by clicking the
        "Edit" button next to the detail you would like to change.
      </p>
    </Section>
    <Section>
      <h3>Creating a Project</h3>
      <p>
        <strong>First, create a NEAR account for your project </strong>(if you
        do not already have one) Note If you already have a personal NEAR
        account, you can assign yourself administrative privileges to your
        project - proceed to next step. If you do not already have a personal
        NEAR account, you should create a personal account{" "}
        <span className="underline">in addition</span> to the project account.
      </p>
      <img
        src="https://s3.us-east-2.amazonaws.com/nearhorizon.assets/account-create.png"
        width=""
        alt="create NEAR account"
      />
      <p>
        To create a new project, click the "Create new" button on the top right
        of the page. A drop-down will appear. Select "Project". A form will
        appear. Fill out the form with the details of your project.
      </p>
      <img
        alt="creating a project on near horizon"
        src="https://nearhorizonassets.s3.amazonaws.com/create_project_optimized.gif"
      />
      <p>
        When you are ready, click the "Create project" button at the bottom of
        the form. Or you can save the form and complete later by clicking the
        "Save for later" button (also at the bottom of the form). Once created,
        you will be navigated to the project page.{" "}
      </p>
      <p>
        Take a moment to review your project details which are displayed in the
        sidebar on the right. You can edit any of these details by clicking the
        "Edit" button next to the detail you would like to change.
      </p>
    </Section>
    <Section>
      <h3>Managing a Project</h3>
      <p>
        <strong>
          Go to the Manage page and set up permissions for your accounts{" "}
        </strong>
      </p>
      <p>
        <ul className="numbered">
          <li>
            Your project has its own account. In order to add admins on a
            project, including yourself, you must log in with the project
            account id.{" "}
          </li>
          <li>
            Visit the <strong>Manage page </strong>of your project to grant
            appropriate permissions to yourself and your team.
          </li>
          <li>Click on the People tab of your project page</li>
          <li>Search for founders to add to your project </li>
          <li>
            Once completed, log back in with your user account to complete the
            setup process.
          </li>
        </ul>
      </p>
      <p>
        <img
          src="https://s3.us-east-2.amazonaws.com/nearhorizon.assets/project-manage-users.png"
          alt="add users to your project"
        />
      </p>
    </Section>
    <Section>
      <h3>Explore and discover Horizon </h3>
      <p>
        Horizon connects founders to each other, as well as to people and
        organizations who can help accelerate their growth. Our marketplace
        application provides a unique opportunity to tap into the power of the
        BOS and unlock the full potential of your startup. Together, we can
        create a thriving ecosystem that empowers startups to achieve their
        goals!
      </p>

      <p>
        That's it! You've created your first project on NEAR Horizon.
        Congratulations!
      </p>
      <div className="bordered">
        <strong>For further information and support, feel free to:</strong>
        <ul>
          <li>
            Contact us at{" "}
            <a href="mailto:horizon@near.foundation">horizon@near.foundation</a>{" "}
          </li>
          <li>
            Read our{" "}
            <a href="https://pages.near.org/blog/near-foundation-launches-near-horizon/">
              latest blog post about Horizon
            </a>{" "}
          </li>
        </ul>
      </div>
    </Section>
  </Container>
);
