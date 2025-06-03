# SaaS Express

## Getting Started

Download latest binary for your operating system from https://github.com/saasexpress/saasexpress/releases/latest

```console
saasexpress -i
```

Paste a simple graph that creates an API on port 2500, returning static JSON data.

```yaml
name: sample
nodes:
  - id: start
    action: HTTPIn
    config:
      method: ^(GET)$
      routes:
        - /hello

  - id: translate
    action: MessageTranslator
    config:
      template: |
        {
          "action":  input.http_method,
          "message": "Hello",
          "data": data
        }
edges:
  - { from: start, to: translate }
```

Go to: http://localhost:2500/hello

## Development

```sh
cargo watch -w src -x run
```

## Tests

```sh
cargo test -- --test-threads=1
```