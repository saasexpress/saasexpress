import { parse, stringify } from "yaml";

/**
 * - return 400 error if can not find a match
 * - if waiting for multiple parents, 
 */
describe("eip.content-based-router", () => {
  it("one rule", () => {
    const body = `
      nodes:
      - id: in
        action: ContentBasedRouter
        config:
          rules:
           - when: method == "GET"
             to: node1
      - id: node1
        action: NoOp

      edges:
      - { from: in, to: node1 }

      input: []
    `;
    const expected = `
      [
      ]
    `;
    //cy.callDagAllInOne(parse(body), parse(expected));
  });

  it("multi rules - match", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - id: router
        action: ContentBasedRouter
        config:
          rules:
           - when: method == "GET"
             to: nodeGet
          otherwise: doNothing
      - id: nodeGet
        action: Append
      - { id: out, action: JSONToBuffer }
      - { id: doNothing, action: Terminate }

      edges:
      - { from: in, to: router }
      - { from: router, to: nodeGet }
      - { from: router, to: doNothing }
      - { from: nodeGet, to: out }

      input: []
    `;
    const expected = "";
    cy.callDagAllInOne(parse(body), parse(expected));
  });

  it("multi rules - no match", () => {
    const body = `
      nodes:
      - { id: in, action: BufferToJSON }
      - id: router
        action: ContentBasedRouter
        config:
          rules:
           - when: method == "NOMATCH"
             to: nodeGet
          otherwise: doNothing
      - id: nodeGet
        action: Append
      - { id: out, action: JSONToBuffer }
      - { id: doNothing, action: Terminate }

      edges:
      - { from: in, to: router }
      - { from: router, to: nodeGet }
      - { from: router, to: doNothing }
      - { from: nodeGet, to: out }

      input: []
    `;
    const expected = `
      [
        "Data is null"
      ]
    `;
    cy.callDagAllInOne(parse(body), parse(expected));
  });
});
