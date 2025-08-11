# Hits-of-Code

[![Hits-of-Code](https://hitsofcode.com/github/vbrandl/hoc)](https://hitsofcode.com/github/vbrandl/hoc/view)
![Build](https://github.com/vbrandl/hoc/actions/workflows/rust.yml/badge.svg)
[![dependency status](https://deps.rs/repo/github/vbrandl/hoc/status.svg)](https://deps.rs/repo/github/vbrandl/hoc)

Small webservice, that returns a badge of the Hits-of-Code of a git repository, as described by [Yegor
Bugayenko](https://www.yegor256.com/2014/11/14/hits-of-code.html). It is implemented in
[Rust](https://www.rust-lang.org/), using the [actix-web](https://actix.rs/) web framework.

A live version of this webservice can be found on [hitsofcode.com](https://hitsofcode.com/).

## Calculation

The original implementation was provided in the [hoc](https://github.com/yegor256/hoc) Ruby gem.

To calculate the Hits-of-Code metric for a local/private Git repository, you can use this one-line command (which you may want to add as a git alias):

```
git log --pretty=tformat: --numstat --ignore-space-change --ignore-all-space --ignore-submodules --no-color --find-copies-harder -M --diff-filter=ACDM -- . | awk '$1 ~ /^[0-9]+$/ && $2 ~ /^[0-9]+$/ { add += $1; del += $2 } END { print add + del }'
```

## API

The API is as simple as

```
https://<host>/<service>/<user>/<repo>
```

where `<service>` is one of `gitub`, `gitlab`, `bitbucket` or `sourcehut`. The HoC data can also be received as JSON by
appending `/json` to the reuqest path:

```
https://<host>/<service>/<user>/<repo>/json
```

There is also an overview page available via `https://<host>/<service>/<user>/<repo>/view`

To delete a repository and the cache from the server, send a `POST` request to
`https://<host>/<service>/<user>/<repo>/delete`. On the overview page, there is a button to perform this operation. It
will respond with a redirect to the overview page so the cache is rebuilt directly.

## Files excluding

If you want to ignore some files on a HoC score calculating you can add a `.hocignore` file to a root of your repository. It works just like a .gitignore file but for the HoC calculation.

## Building

The code can be built as a standalone binary, using `cargo` or as a Docker container. Run either

```
$ cargo build --release
```

or

```
$ docker build .
```

inside the repository.

I'm currently working on migrating to [nix](https://nixos.org/nix). To get a development shell, run `nix-shell`, to
build the package run `nix-build --attr package` and to build the Docker image, run `nix-build --attr dockerImage`.


## Running

Rename [`hoc.toml.example`](./hoc.toml.example) to `hoc.toml` or [`.env.example`](./.env.example) to `.env` and set the
correct value for `base_url`/`HOC_BASE_URL`. If you don't want to use a configuration or dotenv file, you can pass all
parameters directly via environment variables. For variable names see [`.env.example`](./.env.example).

To start a local instance of the service just run:

```
$ HOC_BASE_URL='http://0.0.0.0:8080' ./hoc
```

You can also use the Docker image:

```
$ docker run -p 8080:8080 --env HOC_BASE_URL='http://0.0.0.0:8080' -it --rm ghcr.io/vbrandl/hoc:latest
```

When running the binary directly, you need a `git` binary in your `PATH`.


## License

`hoc` is licensed under the MIT License ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
