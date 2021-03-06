# safe_client

[![](https://img.shields.io/badge/Project%20SAFE-Approved-green.svg)](http://maidsafe.net/applications) [![](https://img.shields.io/badge/License-GPL3-green.svg)](https://github.com/maidsafe/crust/blob/master/COPYING)


**Primary Maintainer:**     Spandan Sharma (spandan.sharma@maidsafe.net)

**Secondary Maintainer:**   Krishna Kumar (krishna.kumar@maidsafe.net)

|Crate|Linux/OS X|Windows|Coverage|Issues|
|:---:|:--------:|:-----:|:------:|:----:|
|[![](http://meritbadge.herokuapp.com/safe_client)](https://crates.io/crates/safe_client)|[![Build Status](https://travis-ci.org/maidsafe/safe_client.svg?branch=master)](https://travis-ci.org/maidsafe/safe_client)|[![Build status](https://ci.appveyor.com/api/projects/status/pqjc5v2ww7k0qx88/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/safe-client/branch/master)|[![Coverage Status](https://coveralls.io/repos/maidsafe/safe_client/badge.svg?branch=master)](https://coveralls.io/r/maidsafe/safe_client?branch=master)|[![Stories in Ready](https://badge.waffle.io/maidsafe/safe_client.png?label=ready&title=Ready)](https://waffle.io/maidsafe/safe_client)|

| [API Documentation - master branch](http://maidsafe.net/safe_client/master) | [SAFE Network System Documentation](http://systemdocs.maidsafe.net) | [MaidSafe website](http://maidsafe.net) | [Safe Community site](https://forum.safenetwork.io) |
|:------:|:-------:|:-------:|:-------:|

###Pre-requisite:
libsodium is a native dependency for [sodiumxoide](https://github.com/dnaq/sodiumoxide). Thus, install sodium by following the instructions [here](http://doc.libsodium.org/installation/index.html).

For windows:

- Download [prebuilt libsodium library](https://download.libsodium.org/libsodium/releases/libsodium-1.0.2-mingw.tar.gz)
- Extract `libsodium.a` for x86/x64 from the corresponding folder in the archive to your local filesystem
- Add this local path to `%PATH%`. (`PATH=%PATH%;<path to extracted libsodium.a dir>`)

###Build Instructions:
`safe_client` can interface conditionally against either the routing crate or a mock used for local testing.

To use it with the Mock:
```
cargo build --features "use-mock-routing"
cargo test --features "use-mock-routing"
```

To interface it with actual routing (default):
```
cargo build
cargo test
```

## TODO
### [0.1.6]
- [ ] [MAID-1295](https://maidsafe.atlassian.net/browse/MAID-1295) Remove all unwraps() AND Check for Ok(try!( and see if really required (ie., for error conversion etc)
- [ ] [MAID-1296](https://maidsafe.atlassian.net/browse/MAID-1296) Remove unwanted errors and Unexpected should take an &str instead of String
- [ ] [MAID-1297](https://maidsafe.atlassian.net/browse/MAID-1297) Evaluate test_utils in client
- [ ] [MAID-1298](https://maidsafe.atlassian.net/browse/MAID-1298) Put debug statements
- [ ] [MAID-1299](https://maidsafe.atlassian.net/browse/MAID-1299) check for all muts (eg., response_getter etc) and validate if really required
- [ ] [MAID-1300](https://maidsafe.atlassian.net/browse/MAID-1300) Error conditions in Mock Routing
- [ ] [MAID-1301](https://maidsafe.atlassian.net/browse/MAID-1301) Test cases for Error conditions in Mock
- [ ] [MAID-1302](https://maidsafe.atlassian.net/browse/MAID-1302) Ask routing for every event it is going to pass to client and handle each of them
- [ ] [MAID-1303](https://maidsafe.atlassian.net/browse/MAID-1303) Address the TODO’s and make temporary fixes as permanent (eg., listening to bootstrapped signal)
- [ ] [MAID-1304](https://maidsafe.atlassian.net/browse/MAID-1304) Test cases for TODO's and temp fixes as permanent
