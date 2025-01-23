const { defineConfig } = require("cypress");

module.exports = defineConfig({
  e2e: {
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
    baseUrl: "http://localhost:5174",
    reporter: "mochawesome",
    reporterOptions: {
      reportDir: "reporter/results",
      html: true,
      json: true,
    },
    viewportHeight: 800,
    viewportWidth: 1200,
  },
});
