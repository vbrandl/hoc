# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies
- Bump `anyhow` from 1.0.89 to 1.0.91 ([#840](https://github.com/vbrandl/hoc/pull/840), [#843](https://github.com/vbrandl/hoc/pull/843))
- Bump `serde_json` from 1.0.128 to 1.0.132 ([#841](https://github.com/vbrandl/hoc/pull/841))
- Bump `tracing-actix-web` from 0.7.13 to 0.7.14 ([#839](https://github.com/vbrandl/hoc/pull/839))
- Bump `bytes` from 1.7.2 to 1.8.0 ([#842](https://github.com/vbrandl/hoc/pull/842))
- Bump `tokio` from 1.40.0 to 1.41.0 ([#844](https://github.com/vbrandl/hoc/pull/844))
- Bump `serde` from 1.0.210 to 1.0.213 ([#845](https://github.com/vbrandl/hoc/pull/845))

## [0.38.0] 2024-10-16

### Fixes

- Fix redirect for non-default branches when clearing the cache ([#833](https://github.com/vbrandl/hoc/issues/833), [#834](https://github.com/vbrandl/hoc/pull/834))

## [0.37.0] 2024-10-10

### Publishing

- Publish to GitHub Packages ([#830](https://github.com/vbrandl/hoc/pull/830))

### Dependencies
- Bump `serde` from 1.0.192 to 1.0.210 ([#704](https://github.com/vbrandl/hoc/pull/704), [#719](https://github.com/vbrandl/hoc/pull/719), [#726](https://github.com/vbrandl/hoc/pull/726), [#736](https://github.com/vbrandl/hoc/pull/736), [#761](https://github.com/vbrandl/hoc/pull/761), [#766](https://github.com/vbrandl/hoc/pull/766), [#767](https://github.com/vbrandl/hoc/pull/767), [#771](https://github.com/vbrandl/hoc/pull/771), [#775](https://github.com/vbrandl/hoc/pull/775), [#787](https://github.com/vbrandl/hoc/pull/787), [#806](https://github.com/vbrandl/hoc/pull/806), [#808](https://github.com/vbrandl/hoc/pull/808), [#813](https://github.com/vbrandl/hoc/pull/813), [#819](https://github.com/vbrandl/hoc/pull/819))
- Bump `config` from 0.13.3 to 0.14.0 ([#705](https://github.com/vbrandl/hoc/pull/705), [#729](https://github.com/vbrandl/hoc/pull/729))
- Bump `openssl` from 0.10.55 to 0.10.66 ([#706](https://github.com/vbrandl/hoc/pull/706), [#797](https://github.com/vbrandl/hoc/pull/797))
- Bump `tokio` from 1.34.0 to 1.40.0 ([#707](https://github.com/vbrandl/hoc/pull/707), [#709](https://github.com/vbrandl/hoc/pull/709), [#732](https://github.com/vbrandl/hoc/pull/732), [#756](https://github.com/vbrandl/hoc/pull/756), [#776](https://github.com/vbrandl/hoc/pull/776), [#795](https://github.com/vbrandl/hoc/pull/795), [#798](https://github.com/vbrandl/hoc/pull/798), [#800](https://github.com/vbrandl/hoc/pull/800), [#810](https://github.com/vbrandl/hoc/pull/810), [#814](https://github.com/vbrandl/hoc/pull/814))
- Bump `reqwest` from 0.11.22 to 0.12.8 ([#708](https://github.com/vbrandl/hoc/pull/708), [#728](https://github.com/vbrandl/hoc/pull/728), [#741](https://github.com/vbrandl/hoc/pull/741), [#745](https://github.com/vbrandl/hoc/pull/745), [#749](https://github.com/vbrandl/hoc/pull/749), [#750](https://github.com/vbrandl/hoc/pull/750), [#754](https://github.com/vbrandl/hoc/pull/754), [#758](https://github.com/vbrandl/hoc/pull/758), [#765](https://github.com/vbrandl/hoc/pull/765), [#781](https://github.com/vbrandl/hoc/pull/781), [#811](https://github.com/vbrandl/hoc/pull/811), [#825](https://github.com/vbrandl/hoc/pull/825))
- Bump `anyhow` from 1.0.75 to 1.0.89 ([#710](https://github.com/vbrandl/hoc/pull/710), [#714](https://github.com/vbrandl/hoc/pull/714), [#720](https://github.com/vbrandl/hoc/pull/720), [#737](https://github.com/vbrandl/hoc/pull/737), [#744](https://github.com/vbrandl/hoc/pull/744), [#759](https://github.com/vbrandl/hoc/pull/759), [#768](https://github.com/vbrandl/hoc/pull/768), [#772](https://github.com/vbrandl/hoc/pull/772), [#818](https://github.com/vbrandl/hoc/pull/818), [#820](https://github.com/vbrandl/hoc/pull/820), [#821](https://github.com/vbrandl/hoc/pull/821))
- Bump `futures` from 0.3.29 to 0.3.31 ([#711](https://github.com/vbrandl/hoc/pull/711), [#826](https://github.com/vbrandl/hoc/pull/826))
- Bump `awc` from 3.2.0 to 3.5.0 ([#712](https://github.com/vbrandl/hoc/pull/712), [#731](https://github.com/vbrandl/hoc/pull/731), [#774](https://github.com/vbrandl/hoc/pull/774))
- Bump `actix-web` from 4.4.0 to 4.8.0 ([#713](https://github.com/vbrandl/hoc/pull/713), [#739](https://github.com/vbrandl/hoc/pull/739), [#773](https://github.com/vbrandl/hoc/pull/773), [#782](https://github.com/vbrandl/hoc/pull/782))
- Bump `tempfile` from 3.8.1 to 3.13.0 ([#715](https://github.com/vbrandl/hoc/pull/715), [#752](https://github.com/vbrandl/hoc/pull/752), [#804](https://github.com/vbrandl/hoc/pull/804), [#805](https://github.com/vbrandl/hoc/pull/805), [#824](https://github.com/vbrandl/hoc/pull/824))
- Bump `serde_json` from 1.0.108 to 1.0.128 ([#717](https://github.com/vbrandl/hoc/pull/717), [#722](https://github.com/vbrandl/hoc/pull/722), [#727](https://github.com/vbrandl/hoc/pull/727), [#738](https://github.com/vbrandl/hoc/pull/738), [#760](https://github.com/vbrandl/hoc/pull/760), [#770](https://github.com/vbrandl/hoc/pull/770), [#784](https://github.com/vbrandl/hoc/pull/784), [#786](https://github.com/vbrandl/hoc/pull/786), [#799](https://github.com/vbrandl/hoc/pull/799), [#802](https://github.com/vbrandl/hoc/pull/802), [#812](https://github.com/vbrandl/hoc/pull/812), [#817](https://github.com/vbrandl/hoc/pull/817))
- Bump `vergen` from 8.2.6 to 8.3.2 ([#721](https://github.com/vbrandl/hoc/pull/721), [#730](https://github.com/vbrandl/hoc/pull/730), [#793](https://github.com/vbrandl/hoc/pull/793))
- Bump `actions/cache` from 3 to 4 ([#724](https://github.com/vbrandl/hoc/pull/724))
- Bump `h2` from 0.3.20 to 0.3.26 ([#725](https://github.com/vbrandl/hoc/pull/725), [#757](https://github.com/vbrandl/hoc/pull/757))
- Bump `git2` from 0.18.1 to 0.18.3 ([#734](https://github.com/vbrandl/hoc/pull/734), [#748](https://github.com/vbrandl/hoc/pull/748))
- Bump `tracing-actix-web` from 0.7.9 to 0.7.13 ([#742](https://github.com/vbrandl/hoc/pull/742), [#777](https://github.com/vbrandl/hoc/pull/777), [#815](https://github.com/vbrandl/hoc/pull/815), [#823](https://github.com/vbrandl/hoc/pull/823))
- Bump `mio` from 0.8.9 to 0.8.11 ([#740](https://github.com/vbrandl/hoc/pull/740))
- Bump `softprops/action-gh-release` from 1 to 2 ([#743](https://github.com/vbrandl/hoc/pull/743))
- Bump `bytes` from 1.5.0 to 1.7.2 ([#751](https://github.com/vbrandl/hoc/pull/751), [#794](https://github.com/vbrandl/hoc/pull/794), [#801](https://github.com/vbrandl/hoc/pull/801), [#803](https://github.com/vbrandl/hoc/pull/803), [#822](https://github.com/vbrandl/hoc/pull/822))
- Bump `actix-rt` from 2.9.0 to 2.10.0 ([#779](https://github.com/vbrandl/hoc/pull/779))
- Bump `lazy_static` from 1.4.0 to 1.5.0 ([#783](https://github.com/vbrandl/hoc/pull/783))
- Bump `gix-path` from 0.10.8 to 0.10.10 ([#796](https://github.com/vbrandl/hoc/pull/796), [#816](https://github.com/vbrandl/hoc/pull/816))
- Bump `rustsec/audit-check` from 1 to 2 ([#827](https://github.com/vbrandl/hoc/pull/827))

## [0.36.0] 2023-11-17
### Dependencies
- Bump `serde` from 1.0.160 to 1.0.192 (#613, #617, #627, #636, #639, #640, #645, [#648](https://github.com/vbrandl/hoc/pull/648), [#650](https://github.com/vbrandl/hoc/pull/650), [#652](https://github.com/vbrandl/hoc/pull/652), [#653](https://github.com/vbrandl/hoc/pull/653), [#654](https://github.com/vbrandl/hoc/pull/654), [#655](https://github.com/vbrandl/hoc/pull/655), [#657](https://github.com/vbrandl/hoc/pull/657), [#669](https://github.com/vbrandl/hoc/pull/669), [#687](https://github.com/vbrandl/hoc/pull/687), [#692](https://github.com/vbrandl/hoc/pull/692), [#698](https://github.com/vbrandl/hoc/pull/698))
- Bump `actions/cache` from 2 to 3 (#616)
- Bump `tokio` from 1.28.0 to 1.34.0 (#618, #625, #633, #634, [#659](https://github.com/vbrandl/hoc/pull/659), [#660](https://github.com/vbrandl/hoc/pull/660), [#665](https://github.com/vbrandl/hoc/pull/665), [#686](https://github.com/vbrandl/hoc/pull/686), [#699](https://github.com/vbrandl/hoc/pull/699))
- Bump `reqwest` from 0.11.17 to 0.11.22 (#619, [#672](https://github.com/vbrandl/hoc/pull/672), [#683](https://github.com/vbrandl/hoc/pull/683), [#684](https://github.com/vbrandl/hoc/pull/684))
- Bump `vergen` from 8.1.3 to 8.2.6 (#621, #622, #635, #638, [#679](https://github.com/vbrandl/hoc/pull/679), [#697](https://github.com/vbrandl/hoc/pull/697))
- Bump `tracing-actix-web` from 0.7.4 to 0.7.9 (#623, [#656](https://github.com/vbrandl/hoc/pull/656), [#682](https://github.com/vbrandl/hoc/pull/682), [#690](https://github.com/vbrandl/hoc/pull/690), [#701](https://github.com/vbrandl/hoc/pull/701))
- Bump `git2` from 0.17.1 to 0.18.1 (#624, [#674](https://github.com/vbrandl/hoc/pull/674), [#681](https://github.com/vbrandl/hoc/pull/681))
- Bump `tempfile` from 3.5.0 to 3.8.1 (#626, [#646](https://github.com/vbrandl/hoc/pull/646), [#658](https://github.com/vbrandl/hoc/pull/658), [#667](https://github.com/vbrandl/hoc/pull/667), [#695](https://github.com/vbrandl/hoc/pull/695))
- Bump `serde_json` from 1.0.96 to 1.0.108 (#630, #632, #637, #641, #644, [#649](https://github.com/vbrandl/hoc/pull/649), [#663](https://github.com/vbrandl/hoc/pull/663), [#678](https://github.com/vbrandl/hoc/pull/678), [#680](https://github.com/vbrandl/hoc/pull/680), [#696](https://github.com/vbrandl/hoc/pull/696))
- Bump `openssl` from 0.10.50 to 0.10.55 (#631)
- Bump `tracing-bunyan-formatter` from 0.3.7 to 0.3.9 (#642, [#661](https://github.com/vbrandl/hoc/pull/661))
- Bump `anyhow` from 1.0.71 to 1.0.75 (#643, [#662](https://github.com/vbrandl/hoc/pull/662), [#664](https://github.com/vbrandl/hoc/pull/664))
- Bump `ructe` from 0.16.1 to 0.17.0 ([#647](https://github.com/vbrandl/hoc/pull/647))
- Bump `actix-rt` from 2.8.0 to 2.9.0 ([#670](https://github.com/vbrandl/hoc/pull/670))
- Bump `actix-web` from 4.3.1 to 4.4.0 ([#673](https://github.com/vbrandl/hoc/pull/673))
- Bump `awc` from 3.1.1 to 3.2.0 ([#675](https://github.com/vbrandl/hoc/pull/675))
- Bump `actions/checkout` from 3 to 4 ([#676](https://github.com/vbrandl/hoc/pull/676))
- Bump `bytes` from 1.4.0 to 1.5.0 ([#677](https://github.com/vbrandl/hoc/pull/677))
- Bump `stefanzweifel/git-auto-commit-action` from 4 to 5 ([#685](https://github.com/vbrandl/hoc/pull/685))
- Bump `tracing` from 0.1.37 to 0.1.40 ([#688](https://github.com/vbrandl/hoc/pull/688), [#689](https://github.com/vbrandl/hoc/pull/689))
- Bump `tracing-log` from 0.1.3 to 0.2.0 ([#691](https://github.com/vbrandl/hoc/pull/691), [#693](https://github.com/vbrandl/hoc/pull/693))
- Bump `futures` from 0.3.28 to 0.3.29 ([#694](https://github.com/vbrandl/hoc/pull/694))
- Bump `tracing-subscriber` from 0.3.17 to 0.3.18 ([#702](https://github.com/vbrandl/hoc/pull/702))


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
