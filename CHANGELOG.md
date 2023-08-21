# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Dependencies
- Bump `serde` from 1.0.160 to 1.0.184 (#613, #617, #627, #636, #639, #640, #645, [#648](https://github.com/vbrandl/hoc/pull/648), [#650](https://github.com/vbrandl/hoc/pull/650), [#652](https://github.com/vbrandl/hoc/pull/652), [#653](https://github.com/vbrandl/hoc/pull/653), [#654](https://github.com/vbrandl/hoc/pull/654), [#655](https://github.com/vbrandl/hoc/pull/655), [#657](https://github.com/vbrandl/hoc/pull/657), [#666](https://github.com/vbrandl/hoc/pull/666))
- Bump `actions/cache` from 2 to 3 (#616)
- Bump `tokio` from 1.28.0 to 1.32.0 (#618, #625, #633, #634, [#659](https://github.com/vbrandl/hoc/pull/659), [#660](https://github.com/vbrandl/hoc/pull/660), [#665](https://github.com/vbrandl/hoc/pull/665))
- Bump `reqwest` from 0.11.17 to 0.11.18 (#619)
- Bump `vergen` from 8.1.3 to 8.2.4 (#621, #622, #635, #638)
- Bump `tracing-actix-web` from 0.7.4 to 0.7.6 (#623, [#656](https://github.com/vbrandl/hoc/pull/656))
- Bump `git2` from 0.17.1 to 0.17.2 (#624)
- Bump `tempfile` from 3.5.0 to 3.7.1 (#626, [#646](https://github.com/vbrandl/hoc/pull/646), [#658](https://github.com/vbrandl/hoc/pull/658))
- Bump `serde_json` from 1.0.96 to 1.0.105 (#630, #632, #637, #641, #644, [#649](https://github.com/vbrandl/hoc/pull/649), [#663](https://github.com/vbrandl/hoc/pull/663))
- Bump `openssl` from 0.10.50 to 0.10.55 (#631)
- Bump `tracing-bunyan-formatter` from 0.3.7 to 0.3.9 (#642, [#661](https://github.com/vbrandl/hoc/pull/661))
- Bump `anyhow` from 1.0.71 to 1.0.75 (#643, [#662](https://github.com/vbrandl/hoc/pull/662), [#664](https://github.com/vbrandl/hoc/pull/664))
- Bump `ructe` from 0.16.1 to 0.17.0 ([#647](https://github.com/vbrandl/hoc/pull/647))


## [0.35.0] 2023-05-04

* Updated [`h2`](https://github.com/hyperium/h2) from 0.3.16 to 0.3.17, fixes [SEC#11] ([#599])
* Updated [`git2`](https://github.com/rust-lang/git2-rs) from 0.16.1 to 0.17.0 ([#602])
* Updated [`git2`](https://github.com/rust-lang/git2-rs) from 0.17.0 to 0.17.1 ([#603])
* Updated [`tracing-subscriber`](https://github.com/tokio-rs/tracing) from 0.3.16 to 0.3.17 ([#604])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.27.0 to 1.28.0 ([#605])
* Updated [`tracing`](https://github.com/tokio-rs/tracing) from 0.1.37 to 0.1.38 ([#607])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 8.1.1 to 8.1.3 ([#608])
* Downgrade yanked [`tracing`](https://github.com/tokio-rs/tracing) 0.1.38 to 0.1.37 ([#611])
* Updated [`reqwest`](https://github.com/seanmonstar/reqwest) from 0.11.16 to 0.11.17 ([#609])
* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.70 to 1.0.71 ([#610])

[#599]: https://github.com/vbrandl/hoc/pull/599
[#602]: https://github.com/vbrandl/hoc/pull/602
[#603]: https://github.com/vbrandl/hoc/pull/603
[#604]: https://github.com/vbrandl/hoc/pull/604
[#605]: https://github.com/vbrandl/hoc/pull/605
[#607]: https://github.com/vbrandl/hoc/pull/607
[#608]: https://github.com/vbrandl/hoc/pull/608
[#611]: https://github.com/vbrandl/hoc/pull/611
[#609]: https://github.com/vbrandl/hoc/pull/609
[#610]: https://github.com/vbrandl/hoc/pull/610

[SEC#11]: https://github.com/vbrandl/hoc/security/dependabot/11


## [0.34.0] 2023-04-13

* Updated [`tracing-actix-web`](https://github.com/LukeMathWalker/tracing-actix-web) from 0.7.2 to 0.7.3 ([#578])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.156 to 1.0.158 ([#580])
* Updated [`mime`](https://github.com/hyperium/mime) from 0.3.16 to 0.3.17 ([#582])
* Updated [`dotenvy`](https://github.com/allan2/dotenvy) from 0.15.6 to 0.15.7 ([#583])
* Updated [`reqwest`](https://github.com/seanmonstar/reqwest) from 0.11.14 to 0.11.16 ([#586])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.26.0 to 1.27.0 ([#588])
* Updated [`tempfile`](https://github.com/Stebalien/tempfile) from 3.4.0 to 3.5.0 ([#590])
* Updated [`tracing-bunyan-formatter`](https://github.com/LukeMathWalker/tracing-bunyan-formatter) from 0.3.6 to 0.3.7 ([#593])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.158 to 1.0.160 ([#594])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.94 to 1.0.96 ([#595])
* Updated [`openssl`](https://github.com/sfackler/rust-openssl) from 0.10.40 to 0.10.50, fixes [SEC#6], [SEC#7], and [SEC#8] ([#596])

[#578]: https://github.com/vbrandl/hoc/pull/578
[#580]: https://github.com/vbrandl/hoc/pull/580
[#582]: https://github.com/vbrandl/hoc/pull/582
[#583]: https://github.com/vbrandl/hoc/pull/583
[#586]: https://github.com/vbrandl/hoc/pull/586
[#588]: https://github.com/vbrandl/hoc/pull/588
[#590]: https://github.com/vbrandl/hoc/pull/590
[#593]: https://github.com/vbrandl/hoc/pull/593
[#594]: https://github.com/vbrandl/hoc/pull/594
[#595]: https://github.com/vbrandl/hoc/pull/595
[#596]: https://github.com/vbrandl/hoc/pull/596

[SEC#6]: https://github.com/vbrandl/hoc/security/dependabot/6
[SEC#7]: https://github.com/vbrandl/hoc/security/dependabot/7
[SEC#8]: https://github.com/vbrandl/hoc/security/dependabot/8


## [0.33.0] 2023-03-16

* Updated [`futures`](https://github.com/rust-lang/futures-rs) from 0.3.26 to 0.3.27 ([#575])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.152 to 1.0.156 ([#576])
* Use edition 2021 ([#577])


[#575]: https://github.com/vbrandl/hoc/pull/575
[#576]: https://github.com/vbrandl/hoc/pull/576
[#577]: https://github.com/vbrandl/hoc/pull/577


## [0.32.0] 2023-03-06

* Updated [`actix-web`](https://github.com/actix/actix-web) from 4.3.0 to 4.3.1 ([#566])
* Updated [`tempfile`](https://github.com/Stebalien/tempfile) from 3.3.0 to 3.4.0 ([#567])
* Updated [`awc`](https://github.com/actix/actix-web) from 3.1.0 to 3.1.1 ([#568])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.25.0 to 1.26.0 ([#569])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.93 to 1.0.94 ([#570])

[#566]: https://github.com/vbrandl/hoc/pull/566
[#567]: https://github.com/vbrandl/hoc/pull/567
[#568]: https://github.com/vbrandl/hoc/pull/568
[#569]: https://github.com/vbrandl/hoc/pull/569
[#570]: https://github.com/vbrandl/hoc/pull/570


## [0.31.0] 2023-02-13

### Dependency Updates

* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.24.1 to 1.24.2 ([#547])
* Updated [`badgers`](https://github.com/vbrandl/badgers) from 1.1.0 to 1.2.0 ([#548])
* Updated [`reqwest`](https://github.com/seanmonstar/reqwest) from 0.11.13 to 0.11.14 ([#549])
* Updated [`bumpalo`](https://github.com/fitzgen/bumpalo) from 3.10.0 to 3.12.0 ([#550])
* Updated [`libgit2-sys`](https://github.com/rust-lang/git2-rs) from 0.14.1+1.5.0 to 0.14.2+1.5.1 ([#551])
* Updated [`git2`](https://github.com/rust-lang/git2-rs) from 0.16.0 to 0.16.1 ([#552])
* Updated [`actix-rt`](https://github.com/actix/actix-net) from 2.7.0 to 2.8.0 ([#553])
* Updated [`actix-web`](https://github.com/actix/actix-web) from 4.2.1 to 4.3.0 ([#554])
* Updated [`awc`](https://github.com/actix/actix-web) from 3.0.1 to 3.1.0 ([#555])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.24.2 to 1.25.0 ([#558])
* Updated [`ructe`](https://github.com/kaj/ructe) from 0.15.0 to 0.16.1 ([#559])
* Updated [`futures`](https://github.com/rust-lang/futures-rs) from 0.3.25 to 0.3.26 ([#560])
* Updated [`bytes`](https://github.com/tokio-rs/bytes) from 1.3.0 to 1.4.0 ([#561])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.91 to 1.0.92 ([#562])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.5.0 to 7.5.1 ([#563])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.92 to 1.0.93 ([#564])

[#547]: https://github.com/vbrandl/hoc/pull/547
[#548]: https://github.com/vbrandl/hoc/pull/548
[#550]: https://github.com/vbrandl/hoc/pull/550
[#551]: https://github.com/vbrandl/hoc/pull/551
[#552]: https://github.com/vbrandl/hoc/pull/552
[#553]: https://github.com/vbrandl/hoc/pull/553
[#554]: https://github.com/vbrandl/hoc/pull/554
[#555]: https://github.com/vbrandl/hoc/pull/555
[#558]: https://github.com/vbrandl/hoc/pull/558
[#559]: https://github.com/vbrandl/hoc/pull/559
[#560]: https://github.com/vbrandl/hoc/pull/560
[#561]: https://github.com/vbrandl/hoc/pull/561
[#562]: https://github.com/vbrandl/hoc/pull/562
[#563]: https://github.com/vbrandl/hoc/pull/563
[#564]: https://github.com/vbrandl/hoc/pull/564


## [0.30.0] 2023-01-16

### New Features

* Allow customization of the badge label ([#546])

### Dependency Updates

* Updated [`tracing-actix-web`](https://github.com/LukeMathWalker/tracing-actix-web) from 0.7.1 to 0.7.2 ([#542])
* Updated [`git2`](https://github.com/rust-lang/git2-rs) from 0.15.0 to 0.16.0 ([#544])


[#542]: https://github.com/vbrandl/hoc/pull/542
[#544]: https://github.com/vbrandl/hoc/pull/544
[#546]: https://github.com/vbrandl/hoc/pull/546

## [0.29.0] 2023-01-07

### Dependency Updates

* Updated [`badgers`](https://github.com/vbrandl/badgers) from 1.0.0 to 1.1.0 ([#530])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.90 to 1.0.91 ([#531])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.3.1 to 7.4.4 ([#533])
* Updated [`tracing-actix-web`](https://github.com/LukeMathWalker/tracing-actix-web) from 0.6.2 to 0.7.1 ([#534])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.151 to 1.0.152 ([#535])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.23.0 to 1.23.1 ([#536])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.4.4 to 7.5.0 ([#537])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.23.1 to 1.24.1 ([#539])

[#530]: https://github.com/vbrandl/hoc/pull/530
[#531]: https://github.com/vbrandl/hoc/pull/531
[#533]: https://github.com/vbrandl/hoc/pull/533
[#534]: https://github.com/vbrandl/hoc/pull/534
[#535]: https://github.com/vbrandl/hoc/pull/535
[#536]: https://github.com/vbrandl/hoc/pull/536
[#537]: https://github.com/vbrandl/hoc/pull/537
[#539]: https://github.com/vbrandl/hoc/pull/539

## [0.28.0] 2022-12-18

### Fixes

* Fixed clippy lint `needless_borrow` ([#526])

### Dependency Updates

* Updated [`bytes`](https://github.com/tokio-rs/bytes) from 1.2.1 to 1.3.0 ([#519])
* Updated [`config`](https://github.com/mehcode/config-rs) from 0.13.2 to 0.13.3 ([#522])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.88 to 1.0.90 ([#527])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.147 to 1.0.151 ([#528])

[#519]: https://github.com/vbrandl/hoc/pull/519
[#522]: https://github.com/vbrandl/hoc/pull/522
[#526]: https://github.com/vbrandl/hoc/pull/526
[#527]: https://github.com/vbrandl/hoc/pull/527
[#528]: https://github.com/vbrandl/hoc/pull/528

## [0.27.0] 2022-10-21

### Dependency Updates

* Updated [`futures`](https://github.com/rust-lang/futures-rs) from 0.3.24 to 0.3.25 ([#511])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.86 to 1.0.87 ([#510])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.145 to 1.0.147 ([#512])
* Updated [`tracing-actix-web`](https://github.com/LukeMathWalker/tracing-actix-web) from 0.6.1 to 0.6.2 ([#513])
* Updated [`reqwest`](https://github.com/seanmonstar/reqwest) from 0.11.12 to 0.11.13 ([#515])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.21.2 to 1.22.0 ([#516])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.87 to 1.0.88 ([#517])


### Fixes

* Fix clippy lint `needless-borrow` ([#514])

[#511]: https://github.com/vbrandl/hoc/pull/511
[#510]: https://github.com/vbrandl/hoc/pull/510
[#512]: https://github.com/vbrandl/hoc/pull/512
[#513]: https://github.com/vbrandl/hoc/pull/513
[#514]: https://github.com/vbrandl/hoc/pull/514
[#515]: https://github.com/vbrandl/hoc/pull/515
[#516]: https://github.com/vbrandl/hoc/pull/516
[#517]: https://github.com/vbrandl/hoc/pull/517


## [0.26.0] 2022-10-03

### Changed

* Updated [`tracing-subscriber`](https://github.com/tokio-rs/tracing) from 0.3.15 to 0.3.16 ([#501])
* Updated [`tracing`](https://github.com/tokio-rs/tracing) from 0.1.36 to 0.1.37 ([#502])
* Updated [`serde_json`](https://github.com/serde-rs/json) from 1.0.85 to 1.0.86 ([#503])
* Updated [`tracing-bunyan-formatter`](https://github.com/LukeMathWalker/tracing-bunyan-formatter) from 0.3.3 to 0.3.4 ([#504])
* Updated [`dotenvy`](https://github.com/allan2/dotenvy) from 0.15.5 to 0.15.6 ([#508])

[#501]: https://github.com/vbrandl/hoc/pull/501
[#502]: https://github.com/vbrandl/hoc/pull/502
[#503]: https://github.com/vbrandl/hoc/pull/503
[#504]: https://github.com/vbrandl/hoc/pull/504
[#508]: https://github.com/vbrandl/hoc/pull/508