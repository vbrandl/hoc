# Hits-of-Code

Small webservice, that returns a badge of the Hits-of-Code of a git repository, as described by [Yegor
Bugayenko](https://www.yegor256.com/2014/11/14/hits-of-code.html). Currently only GitHub repositories are supported, but
it can be trivially extended to support other platforms such as GitLab or Bitbucket.

A live version of this API can be found on [hoc.oldsql.cc](https://hoc.oldsql.cc/).

## API

The API is as simple as

```
https://<host>/<service>/<user>/<repo>
```

where `<service` is one of `gitub`, `gitlab` or `bitbucket`.


## TODO

* [ ] Customization of badges (e.g. colors)
* [x] Support other platforms beside GitHub (GitLab and Bitbucket)
* [ ] Allow exclusion of certain files/globs from the HoC count
