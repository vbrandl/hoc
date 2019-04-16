# Hits-of-Code

Small webservice, that returns a badge of the Hits-of-Code of a git repository,
as described by [Yegor
Bugayenko](https://www.yegor256.com/2014/11/14/hits-of-code.html). Currently
only GitHub repositories are supported, but it can be trivially extended to
support other platforms such as GitLab or Bitbucket.

## API

The API is as simple as

```
https://<host>/github/<user>/<repo>
```


## TODO

* Customization of badges (e.g. colors)
* Support other platforms beside GitHub
* Allow exclusion of certain files/globs from the HoC count
