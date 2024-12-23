const { defineConfig } = require("cypress");

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
    baseUrl: "http://localhost:8081",
    reporter: "mochawesome",
    reporterOptions: {
      reportDir: "reporter/results",
      html: true,
      json: true,
    },
  },
});
