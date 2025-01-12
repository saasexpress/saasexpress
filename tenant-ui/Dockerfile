FROM denoland/deno:latest

RUN deno install --global \
  --allow-net --allow-read \
  jsr:@std/http@1/file-server

WORKDIR /app

COPY . .

# Needs "node" if we run this
# RUN deno install --allow-scripts=npm:msw@2.7.0

RUN deno run build

ENTRYPOINT ["file-server", "dist"]
