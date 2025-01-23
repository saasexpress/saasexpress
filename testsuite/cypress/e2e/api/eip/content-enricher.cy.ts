import { parse, stringify } from "yaml";

describe("eip.content-enricher", () => {
  it("request method based", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: out, action: JSONToBuffer }
      - id: utest
        action: ContentEnricher
        config:
          template: |-
            {
              "message": "{actor} {action} {resource} ({id})",
              "result":  "{{ .in.result }}",
              "params": {
                "actor": "{{ .in.name }}",
                "action":   "Created",
                "resource": "Tenant",
                "id": "{{ .in.id }}"
              }
            }

      edges:
      - { from: in, to: utest }
      - { from: utest, to: out }

      input:
        result: success
        name: Billy
        id: 12
    `;
    const expected = `
      {
        "message": "{actor} {action} {resource} ({id})",
        "params": {
          "action": "Created",
          "actor": "Billy",
          "id": "12",
          "resource": "Tenant"
        },
        "result": "success"
      }
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });
});
