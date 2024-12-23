// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************
//
//
// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })
//
//
// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })
//
//
// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })
//
//
// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })

let headers: any;
let requestBody: any = {};

Cypress.Commands.add("setHeaders", (headerValues: any) => {
  headers = headerValues;
});

Cypress.Commands.add("setRequestBody", (body: any) => {
  requestBody = JSON.stringify(body);
});

Cypress.Commands.add("callAPI", (endPoint: string, methodType: string) => {
  let body = "{}";
  if (
    methodType.toUpperCase() === "PUT" ||
    methodType.toUpperCase() === "POST"
  ) {
    body = requestBody;
  }

  return cy.request({
    url: endPoint,
    method: methodType,
    body,
    headers,
    failOnStatusCode: false,
  });
});
