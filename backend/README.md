# financrr Backend



![](../resources/Logo/banner_light_bg.png "Financrr Banner")

The backend and cli for financrr - The most modern finance manager you've ever seen!

---

> [!NOTE]
> This documented is intended for developers and contributors.
> For user documentation, please visit the [official documentation](https://financrr.github.io/financrr-app/docs/).

## Requirements

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/)  (latest stable version)
- [RustUp](https://rustup.rs/) (optional, but recommended)

**NOTE:** When deploying, it is highly recommended to use this in combination with
a [reverse proxy](https://www.cloudflare.com/learning/cdn/glossary/reverse-proxy/#:~:text=A%20reverse%20proxy%20is%20a,security%2C%20performance%2C%20and%20reliability.).
See: [Reverse proxy quick-start - Caddy Documentation](https://caddyserver.com/docs/quick-starts/reverse-proxy)

## Getting Started (Docker Compose)

1. run `bin/install.bash` or `.\bin\install.ps1`
2. run docker compose using `docker compose up -d`
3. run `cargo loco start --server-and-worker`
4. visit [SwaggerUi](http://localhost:8080/api/openapi/swagger-ui) or [Scalar](http://localhost:8080/api/openapi/scalar)

## Testing

We use [cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/) to run our tests.

After installing, simply run:

On Linux:

```bash
bash bin/test.bash
```

On Windows:

```powershell
.\bin\test.ps1
```

These scripts run `cargo nextest` with the correct arguments (specifically `--test-threads 1`).
You can pass additional arguments to the script, which will be forwarded to `cargo nextest`.

## Swagger UI

We have a `swagger-ui` instance running at `http://localhost:8080/api/openapi/swagger-ui` for testing and research
purposes
regarding the API.  
**NOTE: Keep in mind that you have to change the URL based on your preferences (`.env` config file and/or reverse
proxies)**

## Scalar

We also have a `scalar` instance running at `http://localhost:8080/api/openapi/scalar` for testing and research purposes
regarding the API.  
**NOTE: Keep in mind that you have to change the URL based on your preferences (`.env` config file and/or reverse
proxies)**

## Default Login

We provide a default user for every fresh installation.

You can log in with the following credentials:

| Username | Password    |
|----------|-------------|
| admin    | Financrr123 |

**We strongly advise you to change the password for production deployments!**

## ⚠️ This ONLY supports PostgreSQL!

There is no exception to this rule, as we simply just include the PostgreSQL driver in our application.  
There is currently no way nor any plans to use/support other databases.

**Why is that?**

- Concentrating on only one database makes it way easier to develop, maintain and test existing systems
- Having an application designed for one specific database may yield performance improvements by fine-tuning both the
  database and application based on each other
- We can make use of PostgreSQL's advanced features, or query postgres-specific tables without having to worry about
  compatibility issues
