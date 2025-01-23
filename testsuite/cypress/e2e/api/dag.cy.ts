import { parse, stringify } from "yaml";

describe("dag", () => {
  it("mirror", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: out, action: JSONToBuffer }

      edges:
      - { from: in, to: out }

      input: []
    `;
    const expected = `
      [
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });
  it("simple append dag", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: node0, action: Append }
      - { id: out, action: JSONToBuffer }

      edges:
      - { from: in, to: node0 }
      - { from: node0, to: out }

      input: []
    `;
    const expected = `
      [
        "[node0] Append ''",
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });

  it("simple dag", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node6, action: Append }
      - { id: node3, action: Append }
      - { id: out, action: JSONToBuffer }

      edges:
      - { from: in, to: node0 }
      - { from: node0, to: node1 }
      - { from: node1, to: node6 }
      - { from: node6, to: node3 }
      - { from: node3, to: out }

      input: [ ]
    `;
    const expected = `
      [
        "[node0] Append ''",
        "[node1] Append ''",
        "[node6] Append ''",
        "[node3] Append ''"
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });

  it("split dag", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node2, action: Append }
      - { id: node3a, action: Append }
      - { id: node3b, action: Append }
      - { id: out, action: JSONToBuffer }

      edges:
      - { from: in, to: node0 }
      - { from: node0, to: node1 }
      - { from: node1, to: node2 }
      - { from: node2, to: node3a }
      - { from: node2, to: node3b }
      - { from: node3a, to: out }
      - { from: node3b, to: out }

      input: [ ]
    `;
    const expected = `
      [
        [
          "[node0] Append ''",
          "[node1] Append ''",
          "[node2] Append ''",
          "[node3a] Append ''"
        ],
        [
          "[node0] Append ''",
          "[node1] Append ''",
          "[node2] Append ''",
          "[node3b] Append ''"
        ]
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });

  it("split and join dag", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - { id: node0, action: Append }
      - { id: node1, action: Append }
      - { id: node2, action: Append }
      - { id: node3a, action: Append }
      - { id: node3b, action: Append }
      - { id: node4, action: JoinStrings }
      - { id: out, action: JSONToBuffer }

      edges:
      - { from: in, to: node0 }
      - { from: node0, to: node1 }
      - { from: node1, to: node2 }
      - { from: node2, to: node3a }
      - { from: node2, to: node3b }
      - { from: node3a, to: node4 }
      - { from: node3b, to: node4 }
      - { from: node4, to: out }

      input: [ ]
    `;
    const expected = `
      [
        "[node0] Append ''",
        "[node0] Append ''",
        "[node1] Append ''",
        "[node1] Append ''",
        "[node2] Append ''",
        "[node2] Append ''",
        "[node3a] Append ''",
        "[node3b] Append ''",
        "[node4] JoinStrings"
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });
});
