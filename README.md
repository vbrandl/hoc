# Hits-of-Code

[![Hits-of-Code](https://hitsofcode.com/github/vbrandl/hoc)](https://hitsofcode.com/view/github/vbrandl/hoc)
[![Docker build](https://img.shields.io/docker/cloud/build/vbrandl/hits-of-code.svg)](https://hub.docker.com/r/vbrandl/hits-of-code)
[![Gitlab build](https://gitlab.com/vbrandl/hoc/badges/master/pipeline.svg)](https://gitlab.com/vbrandl/hoc/pipelines)

Small webservice, that returns a badge of the Hits-of-Code of a git repository, as described by [Yegor
Bugayenko](https://www.yegor256.com/2014/11/14/hits-of-code.html). It is implemented in
[Rust](https://www.rust-lang.org/), using the [actix-web](https://actix.rs/) web framework.

A live version of this API can be found on [hitsofcode.com](https://hitsofcode.com/).

## API

The API is as simple as

```
https://<host>/<service>/<user>/<repo>
```

where `<service>` is one of `gitub`, `gitlab` or `bitbucket`.


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


## Running

Run either the binary produced by cargo, the Docker container you just built (using docker-compose) or pull the image
from [Docker Hub](https://hub.docker.com/r/vbrandl/hits-of-code)

```
$ docker run -it --rm vbrandl/hits-of-code --help
```


## License

`hoc` is licensed under the MIT License ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)
