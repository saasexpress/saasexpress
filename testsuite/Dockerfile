FROM cypress/included:13.17.0

WORKDIR /app

COPY package.json package-lock.json cypress.config.js tsconfig.json ./
RUN npm install

ENTRYPOINT ["npx", "cypress", "run"]