FROM denoland/deno:latest

RUN deno install --global \
  --allow-net --allow-read \
  jsr:@std/http@1/file-server

WORKDIR /app

COPY . .

RUN deno run build

ENTRYPOINT ["file-server", "dist"]
