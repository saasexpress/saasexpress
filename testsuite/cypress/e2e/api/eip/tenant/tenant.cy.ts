import { parse } from "yaml";
import { v4 as uuidv4 } from "uuid";

describe("eip tenant", () => {
  it("request method based", () => {
    const uid = uuidv4().replace(/-/g, "").toUpperCase().substring(0, 5);
    const body = `
        displayName: "${uid}"
    `;
    cy.setRequestBody(parse(body));
    cy.callAPI("/api/tenants", "POST").then(
      ({ body, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);
        console.log(body);
        expect(body["displayName"]).to.be.equal(uid);
      }
    );
  });
});
