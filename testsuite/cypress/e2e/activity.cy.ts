describe("activity", () => {
  beforeEach(() => {
    cy.callAPI("/api/activity", "GET").then(
      ({ body: activity }: Cypress.Response<any>) => {
        activity.forEach((t: any) => {
          cy.callAPI(`/api/activity/${t.id}`, "DELETE").then(
            ({ status }: Cypress.Response<any>) => {
              expect(status).to.be.equal(204);
            }
          );
        });
      }
    );
  });

  it("create a new activity", () => {
    // create a new activity and check if the activity is created
    cy.setRequestBody({
      message: "{actor} {action} {resource}",
      result: "success",
    });
    cy.callAPI("/api/activity", "POST").then(
      ({ body, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);
        expect(body.result === "success");
        expect(body.id).to.be.a("number");
        cy.log(JSON.stringify(body));
      }
    );
  });

  it("list activity", () => {
    cy.setRequestBody({
      message: "{actor} {action} {resource}",
      result: "success",
      params: {},
    });
    cy.callAPI("/api/activity", "POST").then(
      ({ body: newActivity, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);

        cy.callAPI("/api/activity", "GET").then(
          ({ body, status }: Cypress.Response<any>) => {
            expect(status).to.be.equal(200);
            expect(body.length).to.equal(1);
            expect(body[0].id).to.equal(newActivity.id);
            cy.log(JSON.stringify(body));
          }
        );
      }
    );
  });
});
