# SaaS Express

## Getting Started

Download latest binary for your operating system from https://github.com/saasexpress/saasexpress/releases/latest

```console
saasexpress -c test.yaml
```

Save to `test.yaml` a simple graph that creates an API on port 2500, returning static JSON data.

```yaml
name: sample
nodes:
  - id: start
    operator: HTTPIn
    method: ^(GET)$
    routes:
      - /hello

  - id: translate
    operator: MessageTranslator
    template: |
      {
        "message": "Hello " + temp.http_in.query.name,
        "data": data,
        "action":  temp.http_in.method,
      }
edges:
  - { from: start, to: translate }
```

Go to: http://localhost:2243/hello

## Development

```sh
cargo watch -w src -x 'run -- -w'
```

With tokio console:

```sh
RUSTFLAGS="--cfg tokio_unstable" cargo watch -w src -x 'run -- -w'
```

## Tests

```sh
cargo test -- --test-threads=1
```

Or if you want to do development related to the test:

```sh
cargo watch -x 'test shell_works -- --test-threads=1 --nocapture'
```