# financrr Backend


![](../resources/Logo/banner_light_bg.png "Financrr Banner")

The backend and cli for financrr - The most modern finance manager you've ever seen!

---

> [!NOTE]
> This documented is intended for developers and contributors.
> For user documentation, please visit the [official documentation](https://financrr.github.io/financrr-app/docs/).

## Requirements

- [Docker](https://www.docker.com/)

**NOTE:** When deploying, it is highly recommended to use this in combination with
a [reverse proxy](https://www.cloudflare.com/learning/cdn/glossary/reverse-proxy/#:~:text=A%20reverse%20proxy%20is%20a,security%2C%20performance%2C%20and%20reliability.).
See: [Reverse proxy quick-start - Caddy Documentation](https://caddyserver.com/docs/quick-starts/reverse-proxy)

## Getting Started (Docker Compose)

1. run `bin/install.bash`
2. run docker compose using `docker compose up -d`
3. run `app/cargo loco start --server-and-worker`
4. visit [SwaggerUi](http://localhost:8080/api/openapi/swagger-ui) or [Scalar](http://localhost:8080/api/openapi/scalar)

## Development infrastructure

Everything is docker-based!  
This means that you don't run or install anything besides docker locally.

**Remember to regularly pull containers!** `docker compose pull`

<details>
<summary>Why we do this</summary>

- **Consistency**: Every developer has the same environment, no matter what OS they are using
- **Isolation**: You don't have to worry about dependencies on your local machine
- **Control**: We can better control what Versions, CLIs etc. are used

</details>

To access and interact with the containers we provide scripts like `bin/cargo` that executes `cargo` with your arguments
inside the container.  
Also some IDEs (RustRover for example) can be configured to execute their run configurations inside the container which
is useful when debugging.  
Be aware that you have to do some kind of path mapping to make this work when you IDEs does not make this automatically.

### Why we don't use dev containers

We would love to use dev containers but unfortunately, the support for them on JetBrains IDEs is not great.  
Maybe this is a user error so if you can make it work we will be open to suggestions.

## Api Documentation

We provide a full OpenAPI 3.1 specification for our API.  
It can be found at `http://localhost:8080/api`

### Swagger UI

We have a `swagger-ui` instance running at `http://localhost:8080/api/openapi/swagger-ui` for testing and research
purposes
regarding the API.  
**NOTE: Keep in mind that you have to change the URL based on your preferences (`.env` config file and/or reverse
proxies)**

### Scalar

We also have a `scalar` instance running at `http://localhost:8080/api/openapi/scalar` for testing and research purposes
regarding the API.  
**NOTE: Keep in mind that you have to change the URL based on your preferences (`.env` config file and/or reverse
proxies)**

## Default Login

We provide a default user for every fresh installation.

You can log in with the following credentials:

| E-Mail         | Password     |
|----------------|--------------|
| admin@financrr | Financrr123! |

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
