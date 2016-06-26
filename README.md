# New Relic Rust Bindings

The New Relic [SDK](https://docs.newrelic.com/docs/agents/agent-sdk) only works on linux. There is a mock New Relic API for non-linux that always works. This was done to allow people to work on the code without requiring Linux.

## Library Installation

The library is expected to be installed at `/usr/local/nr_agent_sdk`. In addition, `ld` needs to be configured via (`ldconfig`) to search for the New Relic libraries. The following will install and setup `ld` correctly:

```console
curl -sO http://download.newrelic.com/agent_sdk/nr_agent_sdk-v0.16.2.0-beta.x86_64.tar.gz && \
  tar -xzf nr_agent_sdk-v0.16.2.0-beta.x86_64.tar.gz && \
  mv nr_agent_sdk-v0.16.2.0-beta.x86_64 /usr/local/nr_agent_sdk && \
  echo "/usr/local/nr_agent_sdk/lib/" > /etc/ld.so.conf.d/newrelic.conf
```

## Debugging

If you are debugging the New Relic SDK, then you are strongly encouraged to enable logging. In the Docker container, copy the `/usr/local/nr_agent_sdk/config/log4cplus.properties` file
to `$HOME/.newlic`. Then modify the `$HOME/.newrelic/log4cplus.properties` file to change the log level from `info` to `all` (there are two places that need to be changed).

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
