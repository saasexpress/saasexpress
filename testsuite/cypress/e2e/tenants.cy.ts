describe("tenants", () => {
  it("the ui renders a basic page", () => {
    cy.visit("/ui");
  });

  beforeEach(() => {
    // delete all tenants before each test
    cy.callAPI("/api/tenants", "GET").then(
      ({ body: tenants }: Cypress.Response<any>) => {
        tenants.forEach((t: any) => {
          cy.callAPI(`/api/tenants/${t.id}`, "DELETE").then(
            ({ status }: Cypress.Response<any>) => {
              expect(status).to.be.equal(204);
            }
          );
        });
      }
    );
  });

  it("list tenants", () => {
    // create a new tenant and check if the tenant is listed
    cy.callAPI("/api/tenants", "POST").then(
      ({ body: newTenant, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);

        cy.callAPI("/api/tenants", "GET").then(
          ({ body, status }: Cypress.Response<any>) => {
            expect(status).to.be.equal(200);
            expect(body.length).to.equal(1);
            expect(body[0].id).to.equal(newTenant.id);
            cy.log(JSON.stringify(body));
          }
        );
      }
    );
  });

  it("create a new tenant", () => {
    // create a new tenant and check if the tenant is created
    cy.setRequestBody({ displayName: "test-tenant" });
    cy.callAPI("/api/tenants", "POST").then(
      ({ body, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);
        expect(body.displayName === "test-tenant");
        expect(body.id).to.be.a("string");
        cy.log(JSON.stringify(body));
      }
    );
  });

  it("update a tenant", () => {
    // create a new tenant, update the tenant and check if the tenant is updated
    cy.setRequestBody({ displayName: "test-tenant" });
    cy.callAPI("/api/tenants", "POST").then(
      ({ body }: Cypress.Response<any>) => {
        cy.setRequestBody({ displayName: "test-tenant-updated" });
        cy.callAPI(`/api/tenants/${body.id}`, "PUT").then(
          ({ body, status }: Cypress.Response<any>) => {
            expect(status).to.be.equal(200);
            expect(body.displayName === "test-tenant-updated");
            cy.log(JSON.stringify(body));
          }
        );
      }
    );
  });

  it("delete a tenant", () => {
    // create a new tenant, delete the tenent and check if the tenant is deleted
    cy.setRequestBody({ displayName: "test-tenant" });
    cy.callAPI("/api/tenants", "POST").then(
      ({ body }: Cypress.Response<any>) => {
        cy.setRequestBody({ displayName: "test-tenant-updated" });
        cy.callAPI(`/api/tenants/${body.id}`, "DELETE").then(
          ({ body, status }: Cypress.Response<any>) => {
            expect(status).to.be.equal(204);
            expect(body).is.equal("");
            cy.log(JSON.stringify(body));
            cy.callAPI("/api/tenants", "GET").then(
              ({ body: tenants, status }: Cypress.Response<any>) => {
                expect(status).to.be.equal(200);
                expect(tenants.length).to.equal(0);
                cy.log(JSON.stringify(tenants));
              }
            );
          }
        );
      }
    );
  });
});
