FROM node:latest AS build

WORKDIR /work

# Install dependencies based on the preferred package manager
COPY package.json yarn.lock* package-lock.json* pnpm-lock.yaml* ./
RUN \
  if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
  elif [ -f package-lock.json ]; then npm ci; \
  elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i; \
  else echo "Lockfile not found." && exit 1; \
  fi

COPY . .

RUN yarn build

FROM denoland/deno:alpine-2.0.6

# The port that your application listens to.
EXPOSE 8000

WORKDIR /app

USER deno

# Cache the dependencies as a layer (this is re-run only when deps.ts is modified).
# Ideally this will download and compile _all_ external files used in main.ts.
COPY deps.ts main.ts /app
RUN deno cache deps.ts

COPY --from=build /work/build site

CMD ["deno", "run", "--allow-net", "--allow-read", "main.ts"]