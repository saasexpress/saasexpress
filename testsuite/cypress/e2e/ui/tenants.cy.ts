describe("tenants ui", () => {
  it("the ui renders the list of tenants", () => {
    cy.visit("/ui/tenants");
    cy.get('[data-id="list-tenants"]').should("be.visible");
  });
});
