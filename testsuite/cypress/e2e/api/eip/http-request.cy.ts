import { parse, stringify } from "yaml";

describe("eip.http-request", () => {
  it("request method based", () => {
    const body = `
      nodes:
      - id: start
        action: NoOp
      - { id: bufin, action: BufferToJSON }
      - id: in
        action: ContentEnricher
        config:
          template: |
            {
              "message": "{actor} {action} {resource} ({id})",
              "result":  "{{ .result }}",
              "params": {
                "actor": "{{ .name }}",
                "action":   "Created",
                "resource": "Tenant",
                "id": "{{ .id }}"
              }
            }
      - id: create
        action: HTTPRequest
        config:
          method: POST
          url: http://localhost:8081/api/activity
          contentType: application/json
      - { id: j2b, action: JSONToBuffer }
      - id: done
        action: Terminate
      - { id: bufout, action: JSONToBuffer }

      edges:
      - { from: start, to: bufin }
      - { from: bufin, to: in }
      - { from: in, to: j2b }
      - { from: j2b, to: create }
      - { from: create, to: done }

      - { from: bufin, to: bufout }

      input:
        result: success
        name: Billy
        id: 12
    `;
    cy.setRequestBody(parse(body));
    cy.callAPI("/gw/dag", "POST").then(
      ({ body, status }: Cypress.Response<any>) => {
        expect(status).to.be.equal(200);
        expect(JSON.parse(body)["name"]).to.be.equal("Billy");
        expect(JSON.parse(body)["result"]).to.be.equal("success");
      }
    );
  });
});
