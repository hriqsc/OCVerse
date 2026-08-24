<h1>OCVERSE</h1>

**Description:** 
This is a small project that I made for a couple of friends. It works as a dynamic archive of original characters, with the intention of making it easy to search for character references.

The website can be found at http://147.15.50.130:8080/hub

# Developing

## Client and Server

- `frontend`: Uses Vue 3 with TypeScript.
- `backend`: Uses Rust with actix_web.

To build, you can use the script build.sh

- `backend`  : builds the release binary into the /backend/target/release/ folder.
- `frontend` : builds the frontend into the frontend/dist/ folder.
- `docker`   : builds and composes Docker containers for the backend and frontend, for testing purposes.
- `all`      : runs all of the above options. 

```bat
bash build.sh <option>
```
## Testing

The backend unit tests can be seen in [endpoints.rs](backend/src/test/endpoints.rs)\
or run via [test.sh](backend/test.sh).\
**Warning**\
Backend unit testing may be outdated.


## Docs 

- [Backend Documentation](docs/backend_docs.md)
- [Frontend Documentation](docs/frontend_docs.md)