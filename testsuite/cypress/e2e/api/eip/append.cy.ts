import { parse, stringify } from "yaml";

describe("eip.content-based-router", () => {
  it("request method based", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: out, action: JSONToBuffer }
      - id: append
        action: Append
        config:
          note: my note

      edges:
      - { from: in, to: append }
      - { from: append, to: out }

      input: []
    `;
    const expected = `
      [
        "[append] Append 'my note'"
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });
});
