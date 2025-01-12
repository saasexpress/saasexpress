import { parse, stringify } from "yaml";

describe("dag", () => {
  it("simple append dag", () => {
    const body = `
      nodes:
      - { id: node0, action: Append }

      edges: []

      input: []
    `;
    const expected = `
      [
        "[node0] Append line",
      ]
    `;
    doit(parse(body), parse(expected));
  });

  it("simple dag", () => {
    const body = `
      operators:
      - { id: Translate, uses: Translate }

      nodes:
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node6, action: Append }
      - { id: node3, action: Append }

      edges:
      - { from: node0, to: node1 }
      - { from: node1, to: node6 }
      - { from: node6, to: node3 }

      input: [ ]
    `;
    const expected = `
      [
        "[node0] Append line",
        "[node1] Append line",
        "[node6] Append line",
        "[node3] Append line"
      ]
    `;
    doit(parse(body), parse(expected));
  });

  it("split dag", () => {
    const body = `
      operators:
      - { id: Translate, uses: Translate }

      nodes:
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node2, action: Append }
      - { id: node3a, action: Append }
      - { id: node3b, action: Append }

      edges:
      - { from: node0, to: node1 }
      - { from: node1, to: node2 }
      - { from: node2, to: node3a }
      - { from: node2, to: node3b }

      input: [ ]
    `;
    const expected = `
      [
        [
          "[node0] Append line",
          "[node1] Append line",
          "[node2] Append line",
          "[node3b] Append line"
        ],
        [
          "[node0] Append line",
          "[node1] Append line",
          "[node2] Append line",
          "[node3a] Append line"
        ]
      ]
    `;
    doit(parse(body), parse(expected));
  });

  it("split and join dag", () => {
    const body = `
      operators:
      - { id: Translate, uses: Translate }

      nodes:
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node2, action: Append }
      - { id: node3a, action: Append }
      - { id: node3b, action: Append }
      - { id: node4, action: JoinStrings }

      edges:
      - { from: node0, to: node1 }
      - { from: node1, to: node2 }
      - { from: node2, to: node3a }
      - { from: node2, to: node3b }
      - { from: node3a, to: node4 }
      - { from: node3b, to: node4 }

      input: [ ]
    `;
    const expected = `
      [
        "[node0] Append line",
        "[node0] Append line",
        "[node1] Append line",
        "[node1] Append line",
        "[node2] Append line",
        "[node2] Append line",
        "[node3a] Append line",
        "[node3b] Append line",
        "[node4] JoinStrings"
      ]
    `;
    doit(parse(body), parse(expected));
  });
});

function doit(body: any, expected: any) {
  cy.setRequestBody(body);
  cy.callAPI("/gw/dag", "POST").then(
    ({ body, status }: Cypress.Response<any>) => {
      expect(status).to.be.equal(200);
      expect(JSON.stringify(JSON.parse(body))).to.be.equal(
        JSON.stringify(expected)
      );
    }
  );
}
