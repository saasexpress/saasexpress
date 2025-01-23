import { parse } from "yaml";
import { v4 as uuidv4 } from "uuid";

describe("eip.reverse-proxy", () => {
  it("request method based", () => {
    const uid = uuidv4().replace(/-/g, "").toUpperCase().substring(0, 5);
    const body = `
      nodes:
      - id: rp
        action: ReverseProxy
        config:
          UpstreamServerURL: http://localhost:8081

      edges: []

      method: POST
      route: /api/tenants
      input:
        displayName: "${uid}"
    `;
    cy.setRequestBody(parse(body));
    cy.callAPI("/gw/dag", "POST").then(
      ({ body, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);
        expect(body["displayName"]).to.be.equal(uid);
      }
    );
  });
});
