/// <reference types="cypress" />

declare namespace Cypress {
  interface Chainable<Subject> {
    setHeaders(headerValues: any): void;

    setRequestBody(requestBody: any): void;

    callAPI(
      endPoint: string,
      methodType: string
    ): Chainable<Cypress.Response<any>>;
  }
}
