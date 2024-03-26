# financrr Backend

![financrrBanner](https://github.com/financrr/backend/assets/48297101/9c959372-f276-4435-804a-dbd4e5acc0dc)

The backend for financrr - The most modern finance manager you've ever seen!

---

### Requirements

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/)  (latest stable version)
- [RustUp](https://rustup.rs/) (optional, but recommended)

**NOTE:** When deploying, it is highly recommended to use this in combination with
a [reverse proxy](https://www.cloudflare.com/learning/cdn/glossary/reverse-proxy/#:~:text=A%20reverse%20proxy%20is%20a,security%2C%20performance%2C%20and%20reliability.).
See: [Reverse proxy quick-start - Caddy Documentation](https://caddyserver.com/docs/quick-starts/reverse-proxy)

### Getting Started (Docker Compose)

1. run `bin/install.bash`
2. run `bin/build.bash`
3. run docker compose using `docker compose --profile dev up -d`
4. (optional) inspect logs using `docker compose --profile dev logs -f`

## Swagger UI

We have a `swagger-ui` instance running at `http://localhost:8080/swagger-ui/` (mind the trailing slash) for testing and
research purposes regarding the API.<br>
**NOTE: Keep in mind that you have to change the URL based on your preferences (`.env` config file and/or reverse
proxies)**<br>

## Default Login

We provide a default user for every fresh installation.
<br>
You can log in with the following credentials:

| Username | Password    |
|----------|-------------|
| admin    | Financrr123 |

**We strongly advise you to change the password for production deployments!**

## Docker

We use docker to run the application.<br>
It is designed to run behind a reverse proxy.<br>
To run it:

1. execute `bash bin/build.bash`
2. execute `docker compose up -d`

## ⚠️ This ONLY supports PostgreSQL!

There is no exception to this rule, as we simply just include the PostgreSQL driver in our application.<br>
There is currently no way nor any plans to use/support other databases.

**Why is that?**<br>

- Concentrating on only one database makes it way easier to develop, maintain and test existing systems
- Having an application designed for one specific database may yield performance improvements by fine-tuning both the
  database and application based on eachother
- We can make use of PostgreSQL's advanced features, or query postgres-specific tables without having to worry about
  compatibility issues

### Performance Configuration

We provide an optimized configuration for PostgreSQL.
<br>
This file based upon a general resource configuration:

1. **OS Type:** linux
2. **DB Type:** mixed
3. **Total Memory (RAM):** 6 GB
4. **CPUs num:** 4
5. **Connections num:** 100
6. **Data Storage:** ssd

If you want to optimize this configuration for your system we recommend using this
tool [here](https://pgtune.leopard.in.ua/).

## Docker

The intended way to run this, is by using Docker Compose.<br>
If you simply want to host your own instance, it is highly recommended to use this in combination with a reverse proxy.

To run it:

1. execute `bash bin/build.bash`
2. execute `docker compose --profile all up -d` (or `docker compose --profile dev up -d` for development)
