# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

This project uses [*towncrier*](https://towncrier.readthedocs.io/) and the changes for the upcoming release can be found in <https://github.com/metalbear-co/mirrord/tree/main/changelog.d/>.

<!-- towncrier release notes start -->

## [3.154.0](https://github.com/metalbear-co/mirrord/tree/3.154.0) - 2025-07-30


### Added

- Add machine_id to UserData so it can be used in analytics.
- Extended `MirrordKafkaClientConfig` CRD with a kind of a SASL OAUTH token
  provider to use.


### Fixed

- Added "Ready!" message when `mirrord port-forward` command finishes setup.
- Fixed a bug in `kube` crate which did not show messages printed out from
  interactive cluster auth.
- Fixed progress message printed when mirrord automatically adds probe ports to
  `feature.network.incoming.http_filter.ports`.
- Reverted changes to SIP log levels that might cause bad instructions under
  the hood.


### Internal

- Add derive for PartialEq and Eq traits to MirrordSessionSpec and nested
  structs.

## [3.153.0](https://github.com/metalbear-co/mirrord/tree/3.153.0) - 2025-07-28


### Added

- Added the `mirrord newsletter` command, which opens the sign-up page in the
  browser.


### Changed

- Passthrough mirroring is now enabled by default, unless mirrord for Teams is
  used.


### Fixed

- Fixed a bug where the SIP patching process was discarding too many open files error
  during layer injection.
- Fixed a typo in Istio Ambient warning message.

## [3.152.1](https://github.com/metalbear-co/mirrord/tree/3.152.1) - 2025-07-22


### Fixed

- Agent communication port now uses the `SO_REUSEADDR` flag, fixing cases where agent
  port is reused in a fast consecutive manner and fails.
- Fixed a bug where mirrord-agents were lingering after all client connections
  were gone.


### Internal

- Added more traffic mirroring tests.

## [3.152.0](https://github.com/metalbear-co/mirrord/tree/3.152.0) - 2025-07-18


### Added

- Added the config option `experimmental.sip_log_destination` to write basic SIP logs to a file.
  [#3407](https://github.com/metalbear-co/mirrord/issues/3407)

## [3.151.0](https://github.com/metalbear-co/mirrord/tree/3.151.0) - 2025-07-16


### Added

- Added a new traffic mirroring implementation, in which the connections are
  redirected using iptables. The new implementation can be enabled in the
  agent configuration with the `agent.passthrough_mirroring` flag.

## [3.150.0](https://github.com/metalbear-co/mirrord/tree/3.150.0) - 2025-07-14


### Changed

- Changed semantics of the `agent.nftables` configuration field.

  When the field is not set, the agent will pick between `iptables-legacy` and
  `iptables-nft` at runtime,
  based on kernel support and existing mesh rules.

  When the field is set, the agent will always use either `iptables-legacy` or
  `iptables-nft`.
- The agent now eagerly detects HTTP in incoming connections.


### Internal

- Add E2E test for using an asterisk queue-id on client config, together with
  SQS json in fallback field.
- Added an E2E test for splitting SQS queues based on env var regex.
- E2E tests for splitting SQS with queue names in env vars originating in
  ConfigMaps.
- Improved SQS E2E tests, removing unnecessary sleeps.

## [3.149.0](https://github.com/metalbear-co/mirrord/tree/3.149.0) - 2025-07-11


### Added

- mirrord-intproxy now propagates critical errors to the mirrord-layer,
  solving the issue where the user application was terminating with a very
  generic
  `Proxy error, connectivity issue or a bug` error message.
  [#3161](https://github.com/metalbear-co/mirrord/issues/3161)
- Added MirrordClusterSession CRD initial implementation, currently hidden
  behind "experimental" feature for mirrord-operator crate.
- Added browser extension configuration and initiation.


### Changed

- Change the SIP patch dir to be nested in the same folder as the extracted
  layer (`TMPDIR/mirrord`).
- Do not check for pod running or if the deployment has replicas when using
  copy-target.
- Clearer error when iptables are dirty.


### Fixed

- Fixed an issue where mirrord was failing to spawn an ephemeral agent due to
  `resourceVersion` conflict.


### Internal

- Skip targetless pod priority class test if `MIRRORD_E2E` is not set to true.

## [3.148.0](https://github.com/metalbear-co/mirrord/tree/3.148.0) - 2025-07-01


### Added

- Added a warning that notifies about the possibility of losing
  requests unmatched requests when both HTTP filter and copy target
  are used.
  [#3223](https://github.com/metalbear-co/mirrord/issues/3223)
- Added E2BIG error handling in the mirrord CLI. mirrord CLI
  now shows a more informative error.
  [#3254](https://github.com/metalbear-co/mirrord/issues/3254)


### Changed

- Made the documentation on outgoing traffic configuration clearer.


### Fixed

- Fixed an issue where the copy target feature was failing with a request
  timeout.


### Internal

- Introduced a new struct `Payload` that enables to cheaply clone mirrord-protocol
  messages and enables almost zero copy message handling.
  [#3365](https://github.com/metalbear-co/mirrord/issues/3365)
- Added a release preparation script.
- Added a startup probe to the HTTP server containers deployed in the E2E
  tests.
- Fixed the install script.

## [3.147.0](https://github.com/metalbear-co/mirrord/tree/3.147.0) - 2025-06-26


### Changed

- mirrord exec config_file with no extension, e.g. heredoc, now assumed to be
  of json format [#3370](https://github.com/metalbear-co/mirrord/issues/3370)
- mirrord now extracts layer to temp_dir()/mirrord to allow easier whitelisting
  with Carbon Black
  [#3373](https://github.com/metalbear-co/mirrord/issues/3373)
- Expand current profile config doc to address the new namespaced profile
  feature.


### Internal

- Add sns flag to queue registry CRD.
- Skip priority class e2e test.

## [3.146.0](https://github.com/metalbear-co/mirrord/tree/3.146.0) - 2025-06-24


### Added

- Added a new mirrord config `agent.priority_class` field for specifying a priority class name
  for targetless agent pods.
  [#1007](https://github.com/metalbear-co/mirrord/issues/1007)
- Added a new `mirrord dump -p <PORT> -t <TARGET>` command. The command allows for getting dump
  of target's incoming traffic.


### Changed

- Changed `container.override_host_ip` config to use Docker's internal address by
  default when running `mirrord container docker` (also changes
  `external_proxy.host_ip` to 0.0.0.0).
  [#3285](https://github.com/metalbear-co/mirrord/issues/3285)
- Removed Discord links.

## [3.145.0](https://github.com/metalbear-co/mirrord/tree/3.145.0) - 2025-06-17


### Added

- Introduced namespaced mirrord profile.
- Pass the user's git branch to the operator to allow integration with mirrord
  Jira app.


### Fixed

- Fix duplicated http_filter_ports and not having the default 80,8080.
- Optimized mirrord-agent's memory usage when mirroring incoming traffic.


### Internal

- Add workflow dispatched action to trigger operator e2e from mirrord.

## [3.144.0](https://github.com/metalbear-co/mirrord/tree/3.144.0) - 2025-06-10


### Added

- Add user-agent to version check
- Added an option to exclude agent's communication port from sidecar proxy if
  the target is in a service mesh.


### Changed

- Automatically add health probe ports to http_filter ports (if a filter is
  set). [#3244](https://github.com/metalbear-co/mirrord/issues/3244)


### Fixed

- Fixed big packets crashing mirrord agent - skip and warn, allow user to
  override max packet size


### Internal

- Add a badge for the community Slack to README
- Added an E2E test for splitting SQS queues based on env var regex.

## [3.143.0](https://github.com/metalbear-co/mirrord/tree/3.143.0) - 2025-05-29


### Added

- Added `fallbackName` to the `MirrordWorkloadQueueRegistry` CRD.
- Added support for SQS splitting without copy target
  if the feature is enabled in the operator.


### Changed

- Updated `mirrord.dev` links in docs.
- When `skip_sip` is not set in user's configuration, use a default list that
  currently only contains `git`.


### Fixed

- Fixed an issue where copy target sessions could fail to reuse an existing
  copy.


### Internal

- Added another `PolicyRule` to the operator's `ClusterRole`, allowing the
  operator to fetch `ConfigMap`s for SQS splitting.
- Extended the `MirrordWorkloadQueueRegistry` CRD to allow for specifying
  multiple queues with a single queue id, based on matching environment
  variables with a regex.
- Extended the `MirrordWorkloadQueueRegistry` to allow for selecting queue
  names from JSON maps stored in environment variables.
- In the resources generated by `mirrord operator setup`, give the operator's
  cluster role permissions to create and delete
  `MutatingWebhookConfiguration`s.

## [3.142.2](https://github.com/metalbear-co/mirrord/tree/3.142.2) - 2025-05-22


### Added

- Added a link to Slack to error messages and documentation.


### Changed

- When mirrord sends an error response to a stolen request, the request body
  now indicates whether the error comes from the agent or from the internal
  proxy.


### Fixed

- Added `experimental.ignore_system_proxy_config` flag to disable any system
  proxy from affecting the running app.
  [#3329](https://github.com/metalbear-co/mirrord/issues/3329)


### Internal

- Client's `mirrord-protocol` version is now stored in one place and shared
  across agent's tasks.

## [3.142.1](https://github.com/metalbear-co/mirrord/tree/3.142.1) - 2025-05-16


### Changed

- mirrord-agent is now less strict when parsing intercepted HTTP/1 requests.


### Fixed

- Fixed the logic for dirty iptables detection and cleanup in the
  mirrord-agent.

## [3.142.0](https://github.com/metalbear-co/mirrord/tree/3.142.0) - 2025-05-12


### Added

- Add new `container.override_host_ip` config key to override what address will
  be used as host addr inside the container.
  [#3289](https://github.com/metalbear-co/mirrord/issues/3289)
- Add the `-t` argument to the `mirrord ls` command to list targets of a
  specific type. Also allow target types to be read from env.
- Added support for skipping custom build tools via environment variable and
  `skip_build_tools` in config.


### Changed

- Renamed `MirorrdProfile` to `MirrordClusterProfile` and support both.
  [#mirorrdprofile](https://github.com/metalbear-co/mirrord/issues/mirorrdprofile)
- Increase timeout of outgoing.rs tests to (hopefully) reduce flakiness.
  [#2679](https://github.com/metalbear-co/mirrord/issues/2679)
- In iptables guard, skip iptables cleanup if the child agent process has
  already done so. [#3220](https://github.com/metalbear-co/mirrord/issues/3220)
- Improved HTTP detection logic in the mirrord agent.
  [#3296](https://github.com/metalbear-co/mirrord/issues/3296)
- Add watch permission for mirrord policies (so the operator can use a
  reflector and cache policies).


### Internal

- Fix the issue running layer integration test on macos aarch64
  when using pre-built layer lib fat binary.
  [#3267](https://github.com/metalbear-co/mirrord/issues/3267)
- Add `x86_64-apple-darwin` and `aarch64-apple-darwin` as targets for
  cargo-deny check.
- Adjusted mirrord-protocol in preparation for passthrough mirroring.
- Improved `listen_ports` integration test to reduce flakes.

## [3.141.0](https://github.com/metalbear-co/mirrord/tree/3.141.0) - 2025-04-28


### Added

- Added the option to skip sip patching.
- Extended the `MirrordKafkaTopicsConsumer` CRD with a `split_ttl` field.


### Changed

- Increased the maximum allowed size of `config.feature.fs.readonly_file_buffer`
  to 15 MB. Added a warning when using size over 1 MB.


### Fixed

- Tied SIP patch files to the version of mirrord binary, so that fixes to patching
  logic will create new files.
  [#3245](https://github.com/metalbear-co/mirrord/issues/3245)
- Fixed docs on how to specify multiple binaries.
  [#3271](https://github.com/metalbear-co/mirrord/issues/3271)
- Fixed logic for detecting whether the operator supports Kafka splitting
  without copying the target.


### Internal

- Added a verification of case-insensitive matching in the SQS E2E test.

## [3.140.0](https://github.com/metalbear-co/mirrord/tree/3.140.0) - 2025-04-22


### Added

- New config value to allow override of listen ip for external proxy
  (`external_proxy.host_ip`).
  [#3274](https://github.com/metalbear-co/mirrord/issues/3274)


### Changed

- The IP table chain names used by the agent are no longer randomized. The
  agent detects if another agent is already
  running or if a previous cleanup failed.
  [#3159](https://github.com/metalbear-co/mirrord/issues/3159)
- Corrected agent pod creation progress message.


### Fixed

- Fix issue with port-forward feature that prevented more than one open
  connection per socket addr.
  [#3158](https://github.com/metalbear-co/mirrord/issues/3158)
- Use ss for killing existing connections to allow stealing to begin


### Internal

- Add cargo-deny action to check for any advisory or license issues.

  *Small updates to dependencies is also inculded*
  [#3250](https://github.com/metalbear-co/mirrord/issues/3250)
- Refactors the e2e tests utils.rs into multiple files/modules.

## [3.139.1](https://github.com/metalbear-co/mirrord/tree/3.139.1) - 2025-04-17


### Fixed

- Fixed an issue where mirrord-agent was being OOM killed due to a traffic
  redirection loop.


### Internal

- Added verification of tmp queues deletion in the SQS E2E test.
- Added E2E tests for rollout targets.
  [#781](https://github.com/metalbear-co/mirrord/issues/781)

## [3.139.0](https://github.com/metalbear-co/mirrord/tree/3.139.0) - 2025-04-16


### Added

- Added config file option for `mirrord operator session` commands (`-f`).
  [#3178](https://github.com/metalbear-co/mirrord/issues/3178)


### Changed

- Increased number of OS threads used by the mirrord agent.

## [3.138.0](https://github.com/metalbear-co/mirrord/tree/3.138.0) - 2025-04-15


### Added

- Added `applies_to_copy_targets` to the mirrord Policy specs, so we can enable
  policies when using copied targets.
  [#797](https://github.com/metalbear-co/mirrord/issues/797)
- Added a sanity e2e test for the copy target feature.
  [#799](https://github.com/metalbear-co/mirrord/issues/799)
- Added CLI support for Kafka splitting without copying the target.
- The agent now issues warnings to clients that do not meet minimum
  protocol version requirement for stealing an HTTP request.
  [#3119](https://github.com/metalbear-co/mirrord/issues/3119)


### Changed

- Improved mermaid diagrams readability (changed colors and lines).
  [#819](https://github.com/metalbear-co/mirrord/issues/819)
- Improved mirrord progress message from spawning the agent without an
  operator.
  [#3167](https://github.com/metalbear-co/mirrord/issues/3167)
- By default, the `DOTNET_STARTUP_HOOKS` env var will not be fetched from the
  target.
- Updated contributing guidance.
- Updated dev container base image and JSON configuration file.


### Fixed

- mirrord now correctly detects whether the configuration allows for stealing
  health checks from the targeted pod.
  [#3188](https://github.com/metalbear-co/mirrord/issues/3188)
- Fixed `mirrord_agent_dns_request_count` Prometheus metric.


### Internal

- Reworked agent's threading model to avoid spawning excessive threads.
  [#agent-threading](https://github.com/metalbear-co/mirrord/issues/agent-threading)
- Added some documentation to the `mirrord-protocol` crate.
  [#3182](https://github.com/metalbear-co/mirrord/issues/3182)
- Added some documentation in the `mirrord-agent` crate.
- Added timeouts to concurrent steal E2E tests.
- Extracted incoming traffic redirection logic to a separate task running in
  the target's network namespace.
- Fixed 3 test HTTP servers used in E2E tests.
- Fixed one of the E2E tests.
- Test specifying a queue by URL instead of by name in SQS splitting.

## [3.137.0](https://github.com/metalbear-co/mirrord/tree/3.137.0) - 2025-03-26


### Added

- mirrord prolicies now allow for enforcing usage of mirrord profiles.


### Changed

- Moved the `readonly_file_buffer` configuration option from experimental to
  `config.feature.fs`.
  [#2984](https://github.com/metalbear-co/mirrord/issues/2984)
- Allow ping-pong an extra timeout period if intproxy recives other messages
  from the agent in last period.


### Fixed

- Added a limit on the size of `config.feature.fs.readonly_file_buffer` to a maximum of 1 MB
  to avoid EIO errors.
  [#3206](https://github.com/metalbear-co/mirrord/issues/3206)
- Fixed a bug related to stealing IPv6 traffic (resolving original destination
  of a stolen connection).
- Fixed an issue where mirrord was preventing the local application from making
  gRPC connections to sidecar containers.
  [#3212](https://github.com/metalbear-co/mirrord/issues/3212)


### Internal

- Added crate level docs to mirrord-macros.
- Added crate level docs to mirrord-sip.
- Added titles for schemars so the rename will not affect the title.
- Added unit tests for recent SIP regressions.
- Extracted agent's iptables logic to a separate crate.
- Added crate level docs to mirrord-cli.
  [#168](https://github.com/metalbear-co/mirrord/issues/168)
- Changed the release workflow to use GHCR cache when building and pushing the
  `mirrord-cli` image.

## [3.136.0](https://github.com/metalbear-co/mirrord/tree/3.136.0) - 2025-03-21


### Added

- mirrord CLI now prints the path to the internal proxy logfile.
  [#3137](https://github.com/metalbear-co/mirrord/issues/3137)
- Added support for new CRD - MirrordProfile. mirrord profiles allow for
  storing mirrord config templates in the cluster and applying them to the
  mirrord config at runtime.


### Fixed

- Regression in running some SIP-protected binaries with mirrord.
  [#3149](https://github.com/metalbear-co/mirrord/issues/3149)
- Regression in running SIP-protected binaries that have entitelements.
  [#3184](https://github.com/metalbear-co/mirrord/issues/3184)


### Internal

- mirrord CLI children no longer resolve the configuration themselves. Instead,
  they used an encoded complete config provided by the parent.
  [#3136](https://github.com/metalbear-co/mirrord/issues/3136)

## [3.135.0](https://github.com/metalbear-co/mirrord/tree/3.135.0) - 2025-03-18


### Added

- mirrord now issues a warning when the user's config allows for
  stealing health checks from the target.
  [#3121](https://github.com/metalbear-co/mirrord/issues/3121)


### Changed

- `502 Bad Gateway` responses returned by the mirrord-agent now contain the
  source error.


### Fixed

- Added missing `PodTemplate` permissions to the `ClusterRole` produced by
  `mirrord operator setup`.
- Fixed a bug where mirrord was producing a malformed `credentials` file.
- Fixed a bug where mirrord was unable to target Argo Rollouts with both
  `workloadRef`s and `selector`s.


### Internal

- Fixed one of the test applications used in the E2E tests.
- Pinned `ziglang` version to `0.13.0.post1` in the CI due to compilation
  failure.
- Updated every JSON schema used our CRDs to use a k8s style
  namespacing for schema names for better compliance with openapi.
- Updated some dependencies.
- Switched to our own forks of some GitHub Actions.

## [3.134.2](https://github.com/metalbear-co/mirrord/tree/3.134.2) - 2025-03-06


### Added

- If a stolen HTTP request matches filters of multiple users,
  the users who don't get the request are now notified with a log message.
  [#3120](https://github.com/metalbear-co/mirrord/issues/3120)


### Changed

- Improved the `mirrord_agent_http_request_in_progress_count` metric.
  [#3092](https://github.com/metalbear-co/mirrord/issues/3092)


### Fixed

- Fixed `unlink` and `unlinkat` logic for files vs. directories.
  [#3094](https://github.com/metalbear-co/mirrord/issues/3094)
- Fixed an bug in TCP mirroring feature.
- Fixed an error where mirrord would sometimes fail with `NotImplemented` error
  due to latency on agent/operator connection.
- Fixed an issue where mirrord was unable to perform an HTTP/1 upgrade over a
  local TLS connection.
- Improved remote DNS errors returned to the client application from the
  mirrord-agent.


### Internal

- Add E2E tests for `unlink` and `unlinkat` hooks.
  [#3094](https://github.com/metalbear-co/mirrord/issues/3094)
- E2E tests now use k8s portforwarding to make requests to deployed test
  applications.
- Improved tracing in `mirrord-intproxy` and changed log format to JSON.
- Improved two outgoing E2E tests.
- Remove the encoded config from env vars when printing debug log on startup

## [3.134.1](https://github.com/metalbear-co/mirrord/tree/3.134.1) - 2025-02-28


### Fixed

- Fixed mirrord failing to load when running emulated in an x86 shell by using
  code shim in builds for arm64 *and* x86.
  [#3052](https://github.com/metalbear-co/mirrord/issues/3052)
- Fix go -> execve sip use cases, i.e air or go reloaders
  [#3123](https://github.com/metalbear-co/mirrord/issues/3123)
- Fixed mirrord Operator's `ClusterRole` generated by the `mirrord operator
  setup` command.


### Internal

- Added a unit test to confirm that our local HTTPS client does not attemp an
  HTTP/1 upgrade to HTTP/2, if the upgrade is already handled by ALPN.
- Added an E2E test for filtered TLS stealing.
- Added more logging in the `mirrord ls` command.
- Fixed `mirrord ls` E2E tests. Now there's only one test, and it uses a fresh
  namespace with a random name.
- Fixed a small bug in tracing in the `mirrord-kube` crate.
- Made some minor improvements to `mirrord-kube`'s typesystem.

## [3.134.0](https://github.com/metalbear-co/mirrord/tree/3.134.0) - 2025-02-24


### Added

- Added support for stealing HTTPS requests with a filter (requires mirrord
  Operator). [#2771](https://github.com/metalbear-co/mirrord/issues/2771)
- Added Nix installation instructions to the README.
  [#3034](https://github.com/metalbear-co/mirrord/issues/3034)


### Fixed

- Fixed an issue where stealing a remote port was preventing the application
  from making TCP connections with the same destination port number.
  [#3006](https://github.com/metalbear-co/mirrord/issues/3006)
- Fixed the order of path checks/operations in file ops handlers in the mirrord
  layer. [#3095](https://github.com/metalbear-co/mirrord/issues/3095)
- Fixed an issue where mirrord was sometimes unable to steal traffic from more
  than one port with an HTTP filter.


### Internal

- Add a section to `CONTRIBUTING.md` on how to test the release workflow.
- Changed `Debug` implementation for `InternalHttpRequest` and
  `InternalHttpResponse` not to print full headers.
  Printing full headers creates a lot of spam in the logs.
- Changed the `conntrack` subcommand used to flush connections when traffic
  stealing starts.

## [3.133.1](https://github.com/metalbear-co/mirrord/tree/3.133.1) - 2025-02-19


### Fixed

- Added a reconnection mechanism when using mirrord operator.
  [#2901](https://github.com/metalbear-co/mirrord/issues/2901)
- Fixed issues with rollout targets without a `selector` field present.
  [#3063](https://github.com/metalbear-co/mirrord/issues/3063)
- Look for the correct pid that matches the targets container_id (by searching
  /proc/pid/cgroup).
  [#3076](https://github.com/metalbear-co/mirrord/issues/3076)
- Prevent reading a remote directory from producing an 'unexpected response'
  error and crashing.
  [#3081](https://github.com/metalbear-co/mirrord/issues/3081)
- Fixed a remote DNS regression introduced when `hickory-resolver` and
  `hickory-proto` versions were bumped.
  [#3098](https://github.com/metalbear-co/mirrord/issues/3098)
- mirrord CLI now correctly emits logs when enabled with `RUST_LOG` environment
  variable. [#3099](https://github.com/metalbear-co/mirrord/issues/3099)


### Internal

- Removed dependency on the umaintained `dotenv` crate. Replaced with a
  dependency on the `dotenvy` crate.
- Removed some unnecessary dependencies from the `mirrord-layer` crate.
- Added a naive update to our port forward wrapper to force first check error
  channel instead of ranomly picking branch on `tokio::select!` impl.

## [3.133.0](https://github.com/metalbear-co/mirrord/tree/3.133.0) - 2025-02-17


### Added

- Added an option to configure timeout for idle local HTTP connections
  (`experimental.idle_local_http_connection_timeout`).


### Changed

- Improved the warning produced when the user specifies agent namespace for a
  targetless run.


### Fixed

- Correct statfs data for Go.
  [#3044](https://github.com/metalbear-co/mirrord/issues/3044)
- Updated `hickory-resolver` and `hickory-proto` to `0.25.0-alpha.5` and `rand`
  from `0.8` to `0.9`.
  [#3079](https://github.com/metalbear-co/mirrord/issues/3079)
- Respect ignored paths and path mapping in statfs hook.
  [#3082](https://github.com/metalbear-co/mirrord/issues/3082)
- Some FS libc calls could be carried out remotely instead of locally in some
  cases. [#3083](https://github.com/metalbear-co/mirrord/issues/3083)
- `mirrord ls` command now does not list unnecessary target types when called
  from a plugin/extension.
  [#3086](https://github.com/metalbear-co/mirrord/issues/3086)
- Fixed wrong link for ipv6 config docs.


### Internal

- Cleanup ci.yaml - we don't need the "docker" flag in E2E tests anymore.
- Elaborate in CONTRIBUTING.md about things to be tested when adding a new
  hook.
- Extracted agent configuration to a separate crate.

## [3.132.1](https://github.com/metalbear-co/mirrord/tree/3.132.1) - 2025-02-06


### Fixed

- Fixed operator connect URL produced by the CLI when a target container is
  specified.

## [3.132.0](https://github.com/metalbear-co/mirrord/tree/3.132.0) - 2025-02-06


### Removed

- Removed faulty `statfs` hook for Go applications.


### Added

- Added Kubernetes ReplicaSet as a new target type (requires mirrord Operator).


### Changed

- Namespace for `targetless` runs is now specified with the
  `target.namespace` config field (or the `MIRRORD_TARGET_NAMESPACE`
  environment variable).
  `agent.namespace` field is ignored in targetless runs.


### Internal

- Some code snippets in the configuration documentation were missing the
  closing backticks, which resulted in bad formatting when converted to
  markdown and displayed on the website.
- Added instructions to CONTRIBUTING.md for changing the agent log level in
  mirrord config.
- Added mirrord policy support for specifying pattern requirment for header
  filter when performing steal-with-filter.
- Removed `envfile` dependency.
- Update policy doc for http_filter header.
- Updated `tests::operator::sanity::mirrord_ls` test after adding ReplicaSet
  support.
- `mirrord-kube` now allows for setting agent listen port.

## [3.131.2](https://github.com/metalbear-co/mirrord/tree/3.131.2) - 2025-01-29


### Fixed

- Removed an error log on a `notimplemented` response from the `mirrord-intproxy`,
  might fix go crash.
  [#3044](https://github.com/metalbear-co/mirrord/issues/3044)


### Internal

- Updated Rust toolchain to nightly-2025-01-22.

## [3.131.1](https://github.com/metalbear-co/mirrord/tree/3.131.1) - 2025-01-28


### Changed

- mirrord commands now accept the `-f`/`--config-file` argument without the value as well.
  When no value is provided, `./.mirrord/mirrord.json` is assumed.
  [#1706](https://github.com/metalbear-co/mirrord/issues/1706)


### Fixed

- Added ping pong subtask to mirrord-extproxy to keep agent connection alive while it is
  up. [#3030](https://github.com/metalbear-co/mirrord/issues/3030)
- `agent.privileged` no longer affects targetless agents' pods.

## [3.131.0](https://github.com/metalbear-co/mirrord/tree/3.131.0) - 2025-01-27


### Added

- `statfs` support
  [#statfs](https://github.com/metalbear-co/mirrord/issues/statfs)
- Support for in-cluster DNS resolution of IPv6 addresses.
  [#2958](https://github.com/metalbear-co/mirrord/issues/2958)
- Prometheus metrics to the mirrord-agent.
  [#2975](https://github.com/metalbear-co/mirrord/issues/2975)
- Kubernetes Service as a new type of mirrord target (requires mirrord
  operator).


### Fixed

- Misleading doc for `.target.namespace` config.
  [#3009](https://github.com/metalbear-co/mirrord/issues/3009)
- Agent now correctly clears incoming port subscriptions of disconnected
  clients.
- mirrord no longer uses the default `{"operator": "Exists"}` tolerations when
  spawning targetless agent pods.

## [3.130.0](https://github.com/metalbear-co/mirrord/tree/3.130.0) - 2025-01-21


### Added

- Added support for `rmdir`, `unlink` and `unlinkat`.
  [#2221](https://github.com/metalbear-co/mirrord/issues/2221)


### Changed

- Updated `configuration.md` and improved `.feature.env.mapping` doc.


### Fixed

- Stopped mirrord entering a crash loop when trying to load into some processes
  like VSCode's `watchdog.js` when the user config contained a call to
  `get_env()`, which occurred due to missing env - the config is now only
  rendered once and set into an env var.
  [#2936](https://github.com/metalbear-co/mirrord/issues/2936)
- Fixed an issue where HTTP requests stolen with a filter would hang with a
  single-threaded local HTTP server.
  Improved handling of incoming connections on the local machine (e.g
  introduces reuse of local HTTP connections).
  [#3013](https://github.com/metalbear-co/mirrord/issues/3013)


### Internal

- Extended `mirrord-protocol` with info logs from the agent.

## [3.129.0](https://github.com/metalbear-co/mirrord/tree/3.129.0) - 2025-01-14


### Added

- Support for stealing incoming connections that are over IPv6.
  [#2956](https://github.com/metalbear-co/mirrord/issues/2956)
- mirrord policy to control file ops from the operator.
- mirrord policy to restrict fetching remote environment variables.


### Changed

- Updated how intproxy is outputing logfile when using container mode, now logs
  will be written on host machine.
  [#2868](https://github.com/metalbear-co/mirrord/issues/2868)
- Changed log level for debugger ports detection.
  [#2986](https://github.com/metalbear-co/mirrord/issues/2986)
- Readonly file buffering is not enabled by default to improve performance
  [#3004](https://github.com/metalbear-co/mirrord/issues/3004)
- Extended docs for HTTP filter in the mirrord config.


### Fixed

- Fixed panic when Go >=1.23.3 verifies pidfd support on Linux.
  [#2988](https://github.com/metalbear-co/mirrord/issues/2988)
- Fix misleading agent IO operation error that always mentioned getaddrinfo.
  [#2992](https://github.com/metalbear-co/mirrord/issues/2992)
- Fixed a bug where port mirroring block (due to active mirrord policies) would
  terminate the mirrord session.


### Internal

- Added lint for unused crate dependencies.
  [#2843](https://github.com/metalbear-co/mirrord/issues/2843)
- Fixed fs policy E2E test.
- Pinned `cargo-chef` version to `0.1.68` in the dockerfiles.
- Added available namespaces to `mirrord ls` output. New output format is
  enabled with a flag in an environment variable.
  [#2999](https://github.com/metalbear-co/mirrord/issues/2999)

## [3.128.0](https://github.com/metalbear-co/mirrord/tree/3.128.0) - 2024-12-19


### Added

- Added to mirrord config a new experimental field
  `.experimental.readonly_file_buffer`. If set to a value greater than 0,
  mirrord will fetch remote readonly files in chunks of at least this size (in bytes).
  This is to improve performance with applications that make many small reads
  from remote files.
  [#2069](https://github.com/metalbear-co/mirrord/issues/2069)
- Added `mirrord container-ext` command that should be used by extension and
  works similarly to `mirrord ext` but for containers.
- Added runAsNonRoot and RO file system to operator deployment
- Added custom resource definition for cluster-wide mirrord policy -
  `MirrordClusterPolicy`.
- Added mapping option for env vars config, allowing the user to map multiple env
  vars to another value based on regexes.
  [#2920](https://github.com/metalbear-co/mirrord/issues/2920)
- Added mkdir support
  [#2221](https://github.com/metalbear-co/mirrord/issues/2221)


### Fixed

- Added debugger port detection type for the node `--inspect`, `--inspect-wait`
  and `--inspect-brk` flags.
  [#2936](https://github.com/metalbear-co/mirrord/issues/2936)
- Fixed `mirrord operator setup` - added missing `/tmp` volume to the operator
  deployment.


### Internal

- Added E2E test for `MirrordClusterPolicy` that blocks incoming traffic
  mirroring.

## [3.127.0](https://github.com/metalbear-co/mirrord/tree/3.127.0) - 2024-12-10


### Added

- `MirrordPolicy` can now block traffic mirroring (requires operator support).


### Changed

- Updated dependencies.
  [#2952](https://github.com/metalbear-co/mirrord/issues/2952)


### Fixed

- Fixed link to operator docs.


### Internal

- Added `mirrord-protocol` message for rejecting mirror port subscription due
  to `MirrordPolicy`.
- Updated hickory dependency version.

## [3.126.0](https://github.com/metalbear-co/mirrord/tree/3.125.3) - 2024-12-06


### Added

- Added SQS splitting state to mirrord operator status reporting (requires operator support).


### Changed

- Hidden files and directories in `$HOME` directory are now read locally by
  default.


### Fixed

- Can now run cs-installed sbt. We now only need to be able to parse the first
  line of a script, so we now support scripts like that sbt, which starts with
  a normal shebang but then has text in a weird encoding, or maybe non-textual
  data. [#2947](https://github.com/metalbear-co/mirrord/issues/2947)
- Prevent reverse port forwarding from ending unexpectedly due to
  unexpected connection end.
  [#2962](https://github.com/metalbear-co/mirrord/issues/2962)
- Added a sleep and await on it after websocket connection to drive IO runtime
  and prevent websocket closing without handshake.


### Internal

- Added optional `loadFromSecret` field to Kafka client config spec to allow
  setting properties from a Secret.
- Allow the operator to fetch Secrets in the operator namespace.
- use `Stdio::null()` for "sidecar start" patch added in
  [#2933](https://github.com/metalbear-co/mirrord/pull/2933).

## [3.125.2](https://github.com/metalbear-co/mirrord/tree/3.125.2) - 2024-11-29


### Fixed

- Manually call `docker start <sidecar_id>` if after our sidecar `run` command
  the container hasn't started yet and is in "created" status.
  [#2927](https://github.com/metalbear-co/mirrord/issues/2927)


### Internal

- Fixed return type of a function in mirrord-operator client code.

## [3.125.1](https://github.com/metalbear-co/mirrord/tree/3.125.1) - 2024-11-27


### Fixed

- Added retry of HTTP requests (intproxy) on hyper's `IncompleteMessage` error.


### Internal

- Updated `RolloutSpec` and operator setup.
- Added `expect_file_open_for_reading` for `/etc/resolv.conf` path in
  `test_issue2283` test.
  [#2935](https://github.com/metalbear-co/mirrord/issues/2935)

## [3.125.0](https://github.com/metalbear-co/mirrord/tree/3.125.0) - 2024-11-21


### Added

- Added a configuration option that allows for specifying an env file for
  mirrord execution.
  [#1913](https://github.com/metalbear-co/mirrord/issues/1913)
- Added notice that fs mapping does not apply to relative paths.
  [#2894](https://github.com/metalbear-co/mirrord/issues/2894)

### Changed

- Ignore paths that start with the current dir path, instead of any path that
  contains the current dir path. Also, ignore only paths that end with the
  current exe's path, not all that contain it.
- Print a warning to the user when `-p` is provided as part of `mirrord container`
  run command, as it may cause issues because of our usage of
  container type network mode.


### Fixed

- Change `getifaddrs` hook to allocate memory for a new list instead of modifying
  list returned from libc call.
  [#2903](https://github.com/metalbear-co/mirrord/issues/2903)
- Read current dir, current exe, and temp dir locally, also when they contain
  characters with a meaning for regexes, like e.g. paretheses.


### Internal

- Add argocd application permissions to operator setup.
- Add explanation about boolean configurations in env/fs
- Changes the Result alias to CliResult, and config to layer_config (in some
  places).
- build script forwards args to cargo build.

## [3.124.2](https://github.com/metalbear-co/mirrord/tree/3.124.2) - 2024-11-08


### Fixed

- Fix agent crash on sniffer failure
  [#2909](https://github.com/metalbear-co/mirrord/issues/2909)
- Fix file mapping doesn't affect xstat

## [3.124.1](https://github.com/metalbear-co/mirrord/tree/3.124.1) - 2024-11-07


### Changed

- Bump dependencies


### Fixed

- Fix crash when listing interfaces caused by enabling the new hook by default

## [3.124.0](https://github.com/metalbear-co/mirrord/tree/3.124.0) - 2024-11-06


### Changed

- hide ipv6 interfaces by default
  [#2849](https://github.com/metalbear-co/mirrord/issues/2849)


### Fixed

- Make sure agent doesn't send `Close` message when Sniffer fails to load.
  [#2896](https://github.com/metalbear-co/mirrord/issues/2896)


## [3.123.0](https://github.com/metalbear-co/mirrord/tree/3.123.0) - 2024-11-05


### Changed

- log better errors of local file creation and add option to use alternative
  way [#2889](https://github.com/metalbear-co/mirrord/issues/2889)
- add .class to be always local


### Fixed

- use /dev/null by default
  [#2889](https://github.com/metalbear-co/mirrord/issues/2889)


## [3.122.1](https://github.com/metalbear-co/mirrord/tree/3.122.1) - 2024-10-30


### Changed

- Bump rust version to 2024-10-11 on macOS [match Linux]


### Fixed

- Add arm64 layer to macOS fat binary.
  [#2885](https://github.com/metalbear-co/mirrord/issues/2885)

## [3.122.0](https://github.com/metalbear-co/mirrord/tree/3.122.0) - 2024-10-30


### Added

- Added serviceAccount option to agent config
  [#2876](https://github.com/metalbear-co/mirrord/issues/2876)


### Changed

- Allow targetless mode to run with local fs-mode.
  [#2368](https://github.com/metalbear-co/mirrord/issues/2368)
- Remove unstable tags on feature.network.outgoing.filter config
  [#2862](https://github.com/metalbear-co/mirrord/issues/2862)
- Add option to have logs when running ext commands


### Fixed

- Added experimental disable_reuseaddr to bypass the issue
  [#2819](https://github.com/metalbear-co/mirrord/issues/2819)
- `mirrord-intproxy` no longer lingers forever when the user tries to execute a
  non-existent binary.
  [#2869](https://github.com/metalbear-co/mirrord/issues/2869)


### Internal

- Extended GitHub bug report template.
  [#2664](https://github.com/metalbear-co/mirrord/issues/2664)
- Switch building layer on macos arm64 to use code shim.
  [#2872](https://github.com/metalbear-co/mirrord/issues/2872)
- Added CRD for `MirrordOperatorUser`, adjusted operator setup.
- Changed debug display of operator session ID. Now it's printed in upper HEX
  to match operator status display.

## [3.121.1](https://github.com/metalbear-co/mirrord/tree/3.121.1) - 2024-10-22


### Changed

- Improve error logging and reporting when a getaddrinfo-adjacent failure
  happens due to IO in the agent.
  [#2287](https://github.com/metalbear-co/mirrord/issues/2287)
- Improve error checking for InvalidCertificate errors in mirrord-cli.
  [#2824](https://github.com/metalbear-co/mirrord/issues/2824)
- Ignore CATALINA_HOME env by default
- Skip mirrord injections into `bazel-real`, considering it a build tool.


### Fixed

- Fix a bug where file mode was ignored when Go applications were creating
  local files. [#2614](https://github.com/metalbear-co/mirrord/issues/2614)
- Update mirrord-container sidecar logs command to improve printing of errors.
  [#2726](https://github.com/metalbear-co/mirrord/issues/2726)
- Fix SIP detection on scripts with no shebang, SIP of default interpreter is
  now used [#2797](https://github.com/metalbear-co/mirrord/issues/2797)
- Bump dependencies, fix empty user in a kube context

## [3.121.0](https://github.com/metalbear-co/mirrord/tree/3.121.0) - 2024-10-17


### Added

- Added support for Istio CNI
  [#2851](https://github.com/metalbear-co/mirrord/issues/2851)
- Added `nodeSelector` option to agent config.


### Changed

- Allowed filtered steal requests to be retried when we get a Reset from
  hyper(h2).


### Fixed

- Fixed an issue where `mirrord exec ... -- npm run serve` in a Vue project was
  failing with `EAFNOSUPPORT: address family not supported ::1:80`. Added new
  `.experimental.hide_ipv6_interfaces` configuration entry that allows for
  hiding local IPv6 interface addresses from the user application.
  [#2807](https://github.com/metalbear-co/mirrord/issues/2807)
- Fixed wrong warning being displayed when binding UDP port 0 and filtering HTTP.
  [#2812](https://github.com/metalbear-co/mirrord/issues/2812)
- mirrord now respects `insecure-skip-tls-verify` option set in the kubeconfig
  when `accept_invalid_certificates` is not provided in the mirrord config.
  [#2825](https://github.com/metalbear-co/mirrord/issues/2825)


### Internal

- Downgraded Rust toochain to nightly-2024-09-12.
  [#downgrade-rust](https://github.com/metalbear-co/mirrord/issues/downgrade-rust)
- Added integration (regression) test for binding port 0 twice.
  [#2812](https://github.com/metalbear-co/mirrord/issues/2812)

## [3.120.1](https://github.com/metalbear-co/mirrord/tree/3.120.1) - 2024-10-14


### Removed

- Remove support for IPv6 sockets with mirrord.
  [#2836](https://github.com/metalbear-co/mirrord/issues/2836)


### Internal

- Update github actions dependencies

## [3.120.0](https://github.com/metalbear-co/mirrord/tree/3.120.0) - 2024-10-13


### Added

- Added Kafka splitting feature.
  [#2601](https://github.com/metalbear-co/mirrord/issues/2601)


### Changed

- Add analytics about usage of experimental features
- Add option to have logs when running ext commands
- update dependencies


### Fixed

- Fixed a bug where `all_of` and `any_of` HTTP filters were stealing all HTTP
  traffic. [#2817](https://github.com/metalbear-co/mirrord/issues/2817)
- Handle IPv4 in IPv6, should help with regressions related to allowing
  AF_INET6 [#2827](https://github.com/metalbear-co/mirrord/issues/2827)


## [3.119.1](https://github.com/metalbear-co/mirrord/tree/3.119.1) - 2024-10-09


### Changed

- Allow setting port for int/extproxy from the command line.


### Fixed

- Use new kube rs to support empty user.
  [#2803](https://github.com/metalbear-co/mirrord/issues/2803)
- Allow using IPv6 sockets with mirrord.
  [#2807](https://github.com/metalbear-co/mirrord/issues/2807)
- Fix mirrord making double bind of port 0 fail.

## [3.119.0](https://github.com/metalbear-co/mirrord/tree/3.119.0) - 2024-10-07


### Added

- Add reverse port forwarding which can be used to proxy data from a remote
  port on the target pod to a local one -
  if only one port is specified, it will be used for both.
  ```
  mirrord port-forward [options] -R [remote_port:]local_port
  ```

  To use the incoming network mode and filters from a config file, use -f as
  normal:
  ```
  mirrord port-forward [options] -R [remote_port:]local_port -f
  config_file.toml
  ```

  [#2609](https://github.com/metalbear-co/mirrord/issues/2609)


### Changed

- Dependency tree does not contain tonic 0.11.
- Use forked version of apple-codesign to remove RSA dependency


### Fixed

- Collect and pass environment variables to the process to be executed locally
  instead of setting them for the entire local environment, which was causing
  interference with analytics instrumentation.
  [#2783](https://github.com/metalbear-co/mirrord/issues/2783)
- Don't drop RSTs, makes long-lived connections drop on steal start
  [#2794](https://github.com/metalbear-co/mirrord/issues/2794)

## [3.118.1](https://github.com/metalbear-co/mirrord/tree/3.118.1) - 2024-10-02


### Added

- Internal proxy now explicitly logs exit error.


### Changed

- Enabled readlink hook by default.
  [#2518](https://github.com/metalbear-co/mirrord/issues/2518)
- Prompt user for intproxy logs (when intproxy crashes).
  Adds `.log` as a file type for intproxy default log file.
  [#2750](https://github.com/metalbear-co/mirrord/issues/2750)
- Refactor how mirrord gets a target when the operator is enabled, and warn
  when randomly selecting a container in multi-container situations (if the
  user did not specify a container).


### Fixed

- Handle cases where target pod has IPv6
  [#2788](https://github.com/metalbear-co/mirrord/issues/2788)


### Internal

- Fix CI failures due to "externally-managed-environment" error
- Run go mod tidy on all go stuff

## [3.118.0](https://github.com/metalbear-co/mirrord/tree/3.118.0) - 2024-09-22


### Added

- Add `cli_extra_args` field to `container` config to allow specifing custom
  arguments for `mirrord container` sidecar container.

  ```json
  {
    "container": {
      "cli_extra_args": ["--network", "host"]
    }
  }
  ```
  this config will spawn mirrord cli container with `<runtime> run --network
  host --rm -d ...`.
  [#2756](https://github.com/metalbear-co/mirrord/issues/2756)


### Changed

- Increase timeout of layer-intproxy socket connection to a ludicrous amount.
  [#2652](https://github.com/metalbear-co/mirrord/issues/2652)
- Have intproxy log to a file in /tmp by default.
  [#2750](https://github.com/metalbear-co/mirrord/issues/2750)
- Bump dependencies


### Fixed

- Add a retry for port-forward agent connection if error was recived via error
  channel after websocket was established.
  [#2759](https://github.com/metalbear-co/mirrord/issues/2759)


### Internal

- E2E tests for SQS splitting.


## [3.117.0](https://github.com/metalbear-co/mirrord/tree/3.117.0) - 2024-09-12


### Added

- Detect Telepresence's traffic-agent and warn user about incompatibility
  [#2738](https://github.com/metalbear-co/mirrord/issues/2738)


### Internal

- Suggest mfT when user uses HTTP filter, only show user one warning for
  multipod/ HTTP filter.
  [#2701](https://github.com/metalbear-co/mirrord/issues/2701)
- Add attribution to docs links using query parameters.
  [#2703](https://github.com/metalbear-co/mirrord/issues/2703)
- Add patch to allow a user to reuse copy-target and fix issue where prelauch
  commands in intellij prevented execution.
- Sort the targets from mirrord ls in a more user friendly way, starting from
  pods.


## [3.116.3](https://github.com/metalbear-co/mirrord/tree/3.116.3) - 2024-09-05


### Fixed

- Fixed `mirrord ls` hanging when there is a lot of possible targets in the
  cluster.
- Update detour for `dns_configuration_copy` to return remote value from
  "/etc/resolv.conf" to fix nodejs dns resolution not working on macos.
  [#2713](https://github.com/metalbear-co/mirrord/issues/2713)


### Internal

- Restored concurrency in `mirrord ls` list requests.

## [3.116.2](https://github.com/metalbear-co/mirrord/tree/3.116.2) - 2024-09-05


### Changed

- Add option to have logs when running ext commands


## [3.116.1](https://github.com/metalbear-co/mirrord/tree/3.116.1) - 2024-09-04


### Fixed

- Fixed upload of mirrord binaries' shasums to homebrew repository in the
  release action.
- Fix mirrord ls hanging by making so `KubeResourceSeeker` will list different
  kinds of resources sequentially instead of in parallel.
  [#2724](https://github.com/metalbear-co/mirrord/issues/2724)

## [3.116.0](https://github.com/metalbear-co/mirrord/tree/3.116.0) - 2024-09-03


### Added

- Add initial and very basic implementation of vpn
  [#2387](https://github.com/metalbear-co/mirrord/issues/2387)
- Add warning when user tries to mirrord exec [container], pointing them to use
  mirrord container instead.
  [#2599](https://github.com/metalbear-co/mirrord/issues/2599)
- Add support for hostname resolution in port-forward.
  [#2696](https://github.com/metalbear-co/mirrord/issues/2696)
- Add support for all_of, any_of composite http filters in config.
  [#2699](https://github.com/metalbear-co/mirrord/issues/2699)


### Changed

- mirrord now produces a more descriptive error message when it fails to call
  authentication command specified in the kubeconfig.
  [#2575](https://github.com/metalbear-co/mirrord/issues/2575)
- SQS CRD field names changed to camelCase.


### Fixed

- Start on deprecating operator target list.
  [#2706](https://github.com/metalbear-co/mirrord/issues/2706)


### Internal

- Adds new (operator) targets for mirrord ls test.
- Change permissions to use new `SubjectAccessReview` api instead of
  `impersonate`.

  Added:
  ```yaml
  - apiGroups:
    - authorization.k8s.io
    resources:
    - subjectaccessreviews
    verbs:
    - create
  ```

  Removed:
  ```yaml
  - apiGroups:
    - ""
    - authentication.k8s.io
    resources:
    - groups
    - users
    - userextras/accesskeyid
    - userextras/arn
    - userextras/canonicalarn
    - userextras/sessionname
    - userextras/iam.gke.io/user-assertion
    - userextras/user-assertion.cloud.google.com
    - userextras/principalid
    - userextras/oid
    - userextras/username
    - userextras/licensekey
    verbs:
    - impersonate
  ```
- Fix some typos
- Fixed target type formatting and E2E test.
- Fixed urlfied form of target types.
- Rejecting empty composite HTTP filters during config validation.

## [3.115.1](https://github.com/metalbear-co/mirrord/tree/3.115.1) - 2024-08-21


### Fixed

- Add retry for checking intproxy logs to get its listening port, Prevents any
  issues when it takes a bit of time for intproxy to start when running in
  container mode. [#2687](https://github.com/metalbear-co/mirrord/issues/2687)
- Fixed `mirrord-agent` not picking up graceful shutdown signal.
  [#2690](https://github.com/metalbear-co/mirrord/issues/2690)

## [3.115.0](https://github.com/metalbear-co/mirrord/tree/3.115.0) - 2024-08-21


### Added

- Adds a batching readdir requests, which should improve the performance when
  traversing large directories. Introduces a new `ReadDirBatched` message to the protocol.
  [#2611](https://github.com/metalbear-co/mirrord/issues/2611)


### Fixed

- Fix hooking on arm64 Go on Linux
  [#2680](https://github.com/metalbear-co/mirrord/issues/2680)


### Internal

- Adds intproxy logs for the integration tests in CI.

## [3.114.1](https://github.com/metalbear-co/mirrord/tree/3.114.1) - 2024-08-18


### Fixed

- Make splitqueues optional to support old version
  [#2675](https://github.com/metalbear-co/mirrord/issues/2675)


### Internal

- Update kube rs to use mainstream
  [#2636](https://github.com/metalbear-co/mirrord/issues/2636)
- Use main CI action for go e2e setup.


## [3.114.0](https://github.com/metalbear-co/mirrord/tree/3.114.0) - 2024-08-16


### Added

- Add port forwarding feature which can be used to proxy data from a local port
  to a remote one -
  if the local port is not specified, it will default to the same as the remote
  ```
  mirrord port-forward [options] -L [local_port:]remote_ip:remote_port
  ```
  [#567](https://github.com/metalbear-co/mirrord/issues/567)
- Client side support for the upcoming SQS queue splitting support in *mirrord
  for Teams*. [#2066](https://github.com/metalbear-co/mirrord/issues/2066)

## [3.113.1](https://github.com/metalbear-co/mirrord/tree/3.113.1) - 2024-08-15


### Fixed

- Fix small error in shared sockets that resulted in it adding the shared
  socket env several times.
  [#864](https://github.com/metalbear-co/mirrord/issues/864)
- Specify that `mirrord container` is an unstable feature.
  [#2641](https://github.com/metalbear-co/mirrord/issues/2641)
- Fix IncomingConfig json schema regression.
  [#2662](https://github.com/metalbear-co/mirrord/issues/2662)
- Fix `arm64` version of `mirrord-cli` container image and add github cache for
  container builds.
- Fixed symbol hooks for Go 1.23.


### Internal

- Updated Go versions used in CI to 1.21, 1.22 and 1.23.
  [#2660](https://github.com/metalbear-co/mirrord/issues/2660)
- Some agent code housekeeping, including improved tracing and errors and
  removal of index allocator.

## [3.113.0](https://github.com/metalbear-co/mirrord/tree/3.113.0) - 2024-08-14


### Added

- Add new api to run mirrord inside container

  ```
  mirrord container [options] -- <docker/podman> run ...
  ```

  Because we need to run internal proxy process on the same network as the
  process loaded with `mirrord-layer`, to keep config and kubernetes
  comparability the communication to mirrord agent is made via external proxy
  that will run on the host machine.
  ```
                     ┌────────────────┐
                 k8s │ mirrord agent  │
                     └─────┬────▲─────┘
                           │    │
                           │    │
                     ┌─────▼────┴─────┐
      container host │ external proxy │
                     └─────┬────▲─────┘
                           │    │
                           │    │
                     ┌─────▼────┴─────┐◄──────┐
   sidecar container │ internal proxy │       │
                     └──┬─────────────┴──┐    │
          run container │ mirrord-layer  ├────┘
                        └────────────────┘
  ```
  [#1658](https://github.com/metalbear-co/mirrord/issues/1658)


### Fixed

- Add custom handling for istio ambient mode where we set
  `/proc/sys/net/ipv4/conf/all/route_localnet` to `1` so it does require
  `agent.privileged = true` to work. (See
  [#2456](https://github.com/metalbear-co/mirrord/issues/2456))
  [#2456](https://github.com/metalbear-co/mirrord/issues/2456)
- Fix issue introduced in #2612 that broke configs with one-value definition
  for IncomingConfig for network feature.
  [#2647](https://github.com/metalbear-co/mirrord/issues/2647)


### Internal

- Fixed a flake in layer integration tests.
  [#2632](https://github.com/metalbear-co/mirrord/issues/2632)
- Add `execution_kind` into analytics `event_properties` where `mirrord
  container` results in 1 and `mirrord exec` results in 2.
- Cleanup unused dependencies
- Remove double analytics reporting by setting so intproxy will only report
  errors if it is running inside a container.
- Update actions
- Update the location of `--` in container command
  ```
  mirrord container [options] <docker/podman/nerdctl> run -- ...
  ```
  Now will be
  ```
  mirrord container [options] -- <docker/podman/nerdctl> run ...
  ```
  or simply
  ```
  mirrord container [options] <docker/podman/nerdctl> run ...
  ```

## [3.112.1](https://github.com/metalbear-co/mirrord/tree/3.112.1) - 2024-08-05


### Added

- Added `experimental.enable_exec_hooks_linux` switch to the mirrord config.


### Changed

- Change operator port from 3000 to 443 to work without any FW exceptions


### Fixed

- Fixed execve hook (fix data race on process initialization, might fix more stuff)
  [#2624](https://github.com/metalbear-co/mirrord/issues/2624)
- Added new VSCode debugpy args layout to debugger port detection


### Internal

- Pinned `towncrier` version to `23.11.0` due to breaking update.

## [3.112.0](https://github.com/metalbear-co/mirrord/tree/3.112.0) - 2024-07-30


### Added

- Add fs mapping, under `feature.fs.mapping` now it's possible to specify regex
  match and replace for paths while running mirrord exec.

  Example:

  ```toml
  [feature.fs.mapping]
  "/var/app/temp" = "/tmp" # Will replace all calls to read/write/scan for
  "/var/app/temp/sample.txt" to "/tmp/sample.txt"
  "/var/app/.cache" = "/workspace/mirrord$0" # Will replace
  "/var/app/.cache/sample.txt" to
  "/workspace/mirrord/var/app/.cache/sample.txt" see
  [Regex::replace](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)
  ``` [#2068](https://github.com/metalbear-co/mirrord/issues/2068)
- Warning when mirrord automatically picked one of multiple containers on the
  target.


### Changed

- Allows targeting StatefulSet without the copy_target feature (still requires
  operator though).


### Fixed

- Remove invalid schema doc mentioning podname as a valid pod target selector.
  [#721](https://github.com/metalbear-co/mirrord/issues/721)
- Pass the list of UserSocket to child processes when exec is called through an
  env var MIRRORD_SHARED_SOCKETS.
  [#864](https://github.com/metalbear-co/mirrord/issues/864)
- Fixed an issue where operator license was incorrectly recognized as expired
  when it was expiring later the same day.
- Fixed new exec hooks breaking execution of Flask apps.


### Internal

- Added `clippy` check on test code to the CI.
- Regenerated config docs.

## [3.111.0](https://github.com/metalbear-co/mirrord/tree/3.111.0) - 2024-07-17


### Added

- Extended `feature.network.dns` config with an optional local/remote filter,
  following `feature.network.outgoing` pattern.
  [#2581](https://github.com/metalbear-co/mirrord/issues/2581)


### Fixed

- Update loopback detection to include pod ip's
  [#2572](https://github.com/metalbear-co/mirrord/issues/2572)
- Fixed a bug where enabling remote DNS prevented making a local connection
  with telnet. [#2579](https://github.com/metalbear-co/mirrord/issues/2579)
- Remove automatic ignore of incoming/outgoing traffic for ports 50000-60000
  [#2597](https://github.com/metalbear-co/mirrord/issues/2597)


### Internal

- Add test to ensure empty streamed request doesn't hang if empty
  [#2593](https://github.com/metalbear-co/mirrord/issues/2593)

## [3.110.0](https://github.com/metalbear-co/mirrord/tree/3.110.0) - 2024-07-12


### Added

- Added experimental.trust_any_certificate to enable making app trust any
  certificate on macOS
  [#2576](https://github.com/metalbear-co/mirrord/issues/2576)


### Fixed

- Fix empty request streaming hanging forever
  [#2590](https://github.com/metalbear-co/mirrord/issues/2590)

## [3.109.0](https://github.com/metalbear-co/mirrord/tree/3.109.0) - 2024-07-10


### Changed

- mirrord commands now provide a nicer error message when the operator required
  but not installed.
  [#1730](https://github.com/metalbear-co/mirrord/issues/1730)
- Add Unknown target variant for forwards compatibility.
  [#2515](https://github.com/metalbear-co/mirrord/issues/2515)


### Fixed

- Improved agent performance when mirroring is under high load.
  [#2529](https://github.com/metalbear-co/mirrord/issues/2529)
- Don't include non-running pods in node capacity check
  [#2582](https://github.com/metalbear-co/mirrord/issues/2582)
- Add exclusion for DOTNET_EnableDiagnostics to make DotNet debugging work by
  default


### Internal

- CLI now sends additional headers with each request to the mirrord operator.
  [#2466](https://github.com/metalbear-co/mirrord/issues/2466)
- Add mirrord-operator-apiserver-authentication `Role` and `RoleBinding` to
  fetch `extension-apiserver-authentication` configmap from "kube-system".
- Fixed compilation errors in `mirrord-operator` crate with only `crd` feature
  enabled.
- Fixed compilation of `mirrord-operator` crate with no features.
- Updated `x509-certificate` dependency.


## [3.108.0](https://github.com/metalbear-co/mirrord/tree/3.108.0) - 2024-07-02


### Added

- Added support for streaming HTTP responses.
  [#2557](https://github.com/metalbear-co/mirrord/issues/2557)


### Changed

- Changed http path filter to include query params in match
  [#2551](https://github.com/metalbear-co/mirrord/issues/2551)
- Configuration documentation contents order.
- Errors that occur when using discovery API to detect mirrord operator are no
  longer fatal. When such error is encountered, mirrord command falls back to
  using the OSS version.


### Fixed

- When using mesh use `lo` interface for mirroring traffic.
  [#2452](https://github.com/metalbear-co/mirrord/issues/2452)


### Internal

- Correct version of HTTP response is sent based on agent protocol version.
  [#2562](https://github.com/metalbear-co/mirrord/issues/2562)
- `mirrord-intproxy` crate unit tests are now part of the CI.


## [3.107.0](https://github.com/metalbear-co/mirrord/tree/3.107.0) - 2024-06-25


### Added

- Added support for intercepting streaming HTTP requests with an HTTP filter.
  [#2478](https://github.com/metalbear-co/mirrord/issues/2478)
- mirrord now queries kube discovery API to confirm that mirrord operator is
  not installed (when an attempt to use operator API fails).
  [#2487](https://github.com/metalbear-co/mirrord/issues/2487)


### Fixed

- Fix network interface configuration not propagating to agent
  [#2539](https://github.com/metalbear-co/mirrord/issues/2539)


## [3.106.0](https://github.com/metalbear-co/mirrord/tree/3.106.0) - 2024-06-18


### Added

- Add cronjobs and statefulsets(/scale) to operator role setup.
- Allows a CronJob and StatefulSet to be used as a target when copy_target is
  enabled.


### Changed

- Put the copy_target config example in the proper place on the main complete
  config sample. [#2508](https://github.com/metalbear-co/mirrord/issues/2508)
- Dependencies update


### Fixed

- A few changes to medschool - refactored the code, changed the algorithm
  taking into consideration we don't ever drop fields.
  [#1580](https://github.com/metalbear-co/mirrord/issues/1580)
- Kill the intproxy child process when mirrord-cli execvp fails.
  [#2386](https://github.com/metalbear-co/mirrord/issues/2386)
- mirrord CLI no longer incorrectly warns the user about soon license
  expiration (renewing licenses).
  [#2526](https://github.com/metalbear-co/mirrord/issues/2526)
- Downgrade certificate dependency to avoid loss of support for older
  certificates
- Fix json snippets in configuration docs by escaping backslashes and removing
  trailing commas.
- Fixed crash on missing cwd/exe
- Fixed rustls initialization.


### Internal

- Fix race in CI e2e waiting for wrong job - test_agent instead of
  test_agent_image
- Removed VSCode settings from repo.
- Update configuration docs
- Use upstream kubers


## [3.105.0](https://github.com/metalbear-co/mirrord/tree/3.105.0) - 2024-06-12


### Added

- Add readlink hook (under experimental config).
  [#2488](https://github.com/metalbear-co/mirrord/issues/2488)
- Display filtered and unfiltered stolen ports when filter is set.
- When an http filter is set and a port is bound that is not included in the
  filtered ports, and there are no unfiltered ports specified, emit a warning.


### Changed

- Now not accepting configs with the same port in
  `feature.network.incoming.ports` and in
  `feature.network.incoming.http_filter.ports`.


### Fixed

- Fixed SIP issue with Turbo
  [#2500](https://github.com/metalbear-co/mirrord/issues/2500)
- Fixed mirrord-agent/cli protocol negotation


### Internal

- Remove baidu.com from E2E tests.
- Update CONTRIBUTING.md
- rust-analyzer will check all targets if (targets) installed


## [3.104.0](https://github.com/metalbear-co/mirrord/tree/3.104.0) - 2024-06-06


### Added

- Emit a warning when the `port_mapping` field of the configuration contains an
  unnecessary mapping of a port to itself.


### Changed

- Update syn to version 2.
  [#1235](https://github.com/metalbear-co/mirrord/issues/1235)


### Fixed

- Fix HTTP2/1.1 translated messages dropping
  [#2497](https://github.com/metalbear-co/mirrord/issues/2497)
- Clean hostname/name sent to operator to fix issue of hostname with linebreaks
- Fixed a bug where two mirrord sessions could not target the same pod while
  stealing from different ports.
- Fixed typo in auto-generated docs for mirrord config.


### Internal

- Added healthcheck examples to filter configuration docs. Also some other
  minor doc fixes.
- Fixed concurrent steal operator tests, removed obsolete error variants, fixed
  `cargo` warnings in test code.


## [3.103.0](https://github.com/metalbear-co/mirrord/tree/3.103.0) - 2024-05-29


### Added

- Allows a Job to be used as a target when copy_target is enabled.


### Changed

- Allows the user to set labels and annotations for the agent job and pod via
  agent config.


### Fixed

- mirrord now prints an informative error when the targeted pod is not in
  correct state (e.g. is not `Running` or the target container is not `ready`).
  When picking a pod from target deployment/rollout, mirrord filters out pods
  that are not in correct state.
  [#462](https://github.com/metalbear-co/mirrord/issues/462)
- Fix config printout error showing repeated messages.
- Fixed listing targets when using operator ignoring namespace - always using
  default
- Fixed missing pods/deployments with more than 1 container when using operator
  ls


### Internal

- Fixed DNS e2e flake
- Update release action to use latest macOS


## [3.102.0](https://github.com/metalbear-co/mirrord/tree/3.102.0) - 2024-05-22


### Removed

- Remove deprecated unstable pause feature
  [#2458](https://github.com/metalbear-co/mirrord/issues/2458)


### Added

- Added json_log config under agent to control whether the agent produces logs
  as normal tracing or json.
  [#2155](https://github.com/metalbear-co/mirrord/issues/2155)
- Added config info printout at session start.
  [#2367](https://github.com/metalbear-co/mirrord/issues/2367)


### Fixed

- Fixed agent crashing when mirrord target is explicitly set to `targetless`.
  [#2444](https://github.com/metalbear-co/mirrord/issues/2444)
- Fixed confusing errors produced when creating an agent.


### Internal

- Agent compiles on macOS
  [#2453](https://github.com/metalbear-co/mirrord/issues/2453)
- Added `uses_operator` flag to `mirrord ext` output (to be inspected in the
  plugin/extension).
- Skip tests from running twice


## [3.101.0](https://github.com/metalbear-co/mirrord/tree/3.101.0) - 2024-05-14


### Changed

- Use operator to list targets to avoid inconsistencies
  [#1959](https://github.com/metalbear-co/mirrord/issues/1959)
- Don't print error on permission denied


### Fixed

- Fixed a bug where outgoing connections where not intercepted from bound
  sockets. [#2438](https://github.com/metalbear-co/mirrord/issues/2438)


### Internal

- Fix all check-rust-docs warnings
  [#1399](https://github.com/metalbear-co/mirrord/issues/1399)
- Use `with_env_vars` in flaky `fs_config_default` test
  [#2163](https://github.com/metalbear-co/mirrord/issues/2163)
- Move LicenseInfoOwned to another module
- Update contributing guide.


## [3.100.1](https://github.com/metalbear-co/mirrord/tree/3.100.1) - 2024-05-06


### Fixed

- mirrord-agent now catches SIGTERM signal and cleans iptables during graceful
  deletion. [#2427](https://github.com/metalbear-co/mirrord/issues/2427)
- Fixed ping pong logic for intproxy-agent communication. Intproxy now sends
  pings on a fixed schedule, regardless of any other messages.


## [3.100.0](https://github.com/metalbear-co/mirrord/tree/3.100.0) - 2024-05-05


### Added

- Added experimental, temp feature for supporting hazelcast pings
  [#2421](https://github.com/metalbear-co/mirrord/issues/2421)
- Provide value hint to Clap for generating shell completions for config_file
  to
  only resolve to files, not just first match.


### Changed

- Changed default env exclude to have `BUNDLE_WITHOUT`
  [#2417](https://github.com/metalbear-co/mirrord/issues/2417)
- Append more permissions to operator clusterrole
- Improve tera templating config error to dig into source and give out more
  details.
- env.unset feature is now case insensitive


## [3.99.2](https://github.com/metalbear-co/mirrord/tree/3.99.2) - 2024-04-30


### Fixed

- Fixed case where resolving DNS failed when setting HTTP filter
  [#2411](https://github.com/metalbear-co/mirrord/issues/2411)


## [3.99.1](https://github.com/metalbear-co/mirrord/tree/3.99.1) - 2024-04-30


### Changed

- Change agent resolver to only resolve IPv4
- Fallback to OSS when operator license expired


### Fixed

- Fix IntelliJ Rider newest version stuck on macOS
  [#2408](https://github.com/metalbear-co/mirrord/issues/2408)
- Fix case were agent log message causes startup failure


## [3.99.0](https://github.com/metalbear-co/mirrord/tree/3.99.0) - 2024-04-28


### Added

- added configuration option to control (local) hostname resolving
  [#2395](https://github.com/metalbear-co/mirrord/issues/2395)
- Add ability to configure DNS params for agent (timeout, attempts)


### Changed

- Change ports and http_ports in incoming configuration to be checked upon
  mapped port instead of local
  [#2371](https://github.com/metalbear-co/mirrord/issues/2371)
- Read /Network locally by default on macOS


### Fixed

- Fix medschool dropping fields sometimes
  [#1580](https://github.com/metalbear-co/mirrord/issues/1580)
- Fix DNS resolving case on macOS + Java Netty
  [#2400](https://github.com/metalbear-co/mirrord/issues/2400)


## [3.98.1](https://github.com/metalbear-co/mirrord/tree/3.98.1) - 2024-04-23


### Changed

- Internal proxy now emits plain text instead of ANSI


### Fixed

- don't re-resolve when connecting to loopback on outgoing filter
  [#2389](https://github.com/metalbear-co/mirrord/issues/2389)
- Added `JetBrains.Debugger.Worker` to the list of known build tools, fixing
  compatibility with Rider 2024.1.


### Internal

- Added getters to `KubernetesAPI`.
- Improved tracing in the agent connection task.
- Sending client certificate public key instead of fingerprint in the analytics
  to match operator's behavior.


## [3.98.0](https://github.com/metalbear-co/mirrord/tree/3.98.0) - 2024-04-18


### Added

- Added `create` and `delete` verbs on `pods` resource in
  `clusterrole/mirrord-operator` for operator setup.


### Changed

- Set timeout of dns request to 1s and only attempt once
  [#2379](https://github.com/metalbear-co/mirrord/issues/2379)


### Fixed

- Fix memory issue when binding


### Internal

- Remove `AgentManagment` because only `KubernetesAPI` implements it now and
  there is no need for this abstraction and moved the used functions straight
  onto `KubernetesAPI`.
- Split off pod template to a separate `ContainerVariant` implementation used
  inside of `JobVariant` and `JobTargetedVariant`.
- Updates to rust nightly-2024-04-15. Also fixes some pointer
  copy_from_nonoverlapping issues.


## [3.97.0](https://github.com/metalbear-co/mirrord/tree/3.97.0) - 2024-04-16


### Added

- Agent now authenticates TLS connections, using a provided X509 certificate
  (mirrord for Teams only).
  [#2362](https://github.com/metalbear-co/mirrord/issues/2362)


### Changed

- Changed port stealing configuration:
  1. Added new `ports` field to the `incoming` configuration. The field lists
  ports that should be stolen/mirrored. Other ports remain local.
  2. Changed the way `incoming.http_filter.ports` field is interpreted. Ports
  not listed in this field are not stolen, unless listed in `incoming.ports`.
  [#2366](https://github.com/metalbear-co/mirrord/issues/2366)


### Fixed

- Change reqwest to use rustls with native certificates to work in more cases


## [3.96.1](https://github.com/metalbear-co/mirrord/tree/3.96.1) - 2024-04-14


### Changed

- Increase max fd in internal proxy to fix connection limit issues


### Fixed

- Fixed layer making process zombie by calling panic from hookerror, also use
  `sigkill` instead of `sigterm`


## [3.96.0](https://github.com/metalbear-co/mirrord/tree/3.96.0) - 2024-04-09


### Changed

- mirrord now listens on 0.0.0.0 when requested and changes address to
  localhost only when needed.
  [#2319](https://github.com/metalbear-co/mirrord/issues/2319)


### Internal

- Adjusted `mirrord-kube` and `mirrord-agent` crates to allow wrapping
  client-agent connections in TLS.
  [#2299](https://github.com/metalbear-co/mirrord/issues/2299)
- Removed dummy span name from logs.


## [3.95.2](https://github.com/metalbear-co/mirrord/tree/3.95.2) - 2024-04-07


### Internal

- Fix release build, don't fail release on warnings
  Some lint warnings appear only on release build, ignore it.
- Fix release compilation CI


## [3.95.1](https://github.com/metalbear-co/mirrord/tree/3.95.1) - 2024-04-07


### Fixed

- Allow `target` be a `string` in the JSON Schema
  [#2188](https://github.com/metalbear-co/mirrord/issues/2188)
- Fixed excessive stack consumption in the `mirrord-layer` by reducing tracing
  in release profile.


### Internal

- Fix e2e failing on release because image doesn't exist
- Use upstream tracing


## [3.95.0](https://github.com/metalbear-co/mirrord/tree/3.95.0) - 2024-04-02


### Changed

- mirrord now unsets the env from within the process aswell


## [3.94.0](https://github.com/metalbear-co/mirrord/tree/3.94.0) - 2024-04-01


### Added

- New config `env.unset` that allows user to unset environment variables in the
  executed process.
  This is useful for unsetting env like `HTTP_PROXY`, `AWS_PROFILE` that come
  from the local environment
  and cause undesired behavior (because those aren't needed for deployed apps).
  [#2260](https://github.com/metalbear-co/mirrord/issues/2260)


## [3.93.1](https://github.com/metalbear-co/mirrord/tree/3.93.1) - 2024-03-31


### Fixed

- Fix new IDE progress breaking older plugins.
  Three issues fixed:
  1. Show the new progress only when env var is set (to be set in newer IDE
  versions).
  2. Multi pod warning was showing everytime when no operator, not only when
  targetting a deployment + no operator.
  3. Show the message for rollouts as well.
  [#2339](https://github.com/metalbear-co/mirrord/issues/2339)


### Internal

- Update Frida version to 16.2.1


## [3.93.0](https://github.com/metalbear-co/mirrord/tree/3.93.0) - 2024-03-31


### Added

- Added handling HTTP upgrades in filtered connections (`mirrord-agent`).
  Refactored TCP stealer code.
  [#2270](https://github.com/metalbear-co/mirrord/issues/2270)
- Add a new diagnostic command to calculate mirrord session latency


### Changed

- Changed `agent.image` config to also accept an extended version where you may
  specify both _registry_ and _tag_ with `agent.image.registry` and
  `agent.image.tag`.
- Proxy errors now don't propagate back to libc but exit with a message
- `use_proxy` behavior is now setting the proxy env to empty value instead of
  unsetting. This should help with cases where
  we need it to propogate to the extensions.


### Fixed

- Internal proxy and agent now properly handle connection shutdowns.
  [#2309](https://github.com/metalbear-co/mirrord/issues/2309)
- Fix some open/fd potential issues
- Fixed the display of agent startup errors to the user.
- Fixed timeout set on new internal proxy connection in `fork` detour.


### Internal

- Adds new message type `IdeMessage`. Allows us to send messages to the IDE
  that should be shown in notification boxes, with buttons/actions.
- Change design around analyticsreporter to be more robust/clean
- Prepared an e2e test for stealing WebSockets connections with an HTTP filter
  set.


## [3.92.1](https://github.com/metalbear-co/mirrord/tree/3.92.1) - 2024-03-17


### Removed

- Removed problematic DNS cache from internal proxy.


### Fixed

- Fixed a bug with handling hints passed to `getaddrinfo` function.


### Internal

- stealer to steal in incoming mode typo
  [#docs-incoming-stealer](https://github.com/metalbear-co/mirrord/issues/docs-incoming-stealer)
- Add `kuma-sidecar` `kuma-init` to default list of skipped containers. (no
  need to target these containers)
  [#skip-kuma-containers](https://github.com/metalbear-co/mirrord/issues/skip-kuma-containers)
- Added a unit test to internal proxy DNS cache. Added more tracing to internal
  proxy.


## [3.92.0](https://github.com/metalbear-co/mirrord/tree/3.92.0) - 2024-03-13


### Added

- Added support for `statx` function.
  [#2204](https://github.com/metalbear-co/mirrord/issues/2204)


### Fixed

- Fix incoming network interception via port-forward when "stealing" traffic
  with a mesh like linkerd or istio (Using the same `OUTPUT` iptable rules for
  both meshed and not meshed networks)
  [#2255](https://github.com/metalbear-co/mirrord/issues/2255)
- Add Kuma mesh detection and support to mirrord-agent.
  [#2296](https://github.com/metalbear-co/mirrord/issues/2296)
- Added sidecar exclusion for kuma mesh, fixing issues running in that setup


## [3.91.0](https://github.com/metalbear-co/mirrord/tree/3.91.0) - 2024-03-05


### Added

- Adds operator session management commands to mirrord-cli, these are: `mirrord
  operator session kill-all`, `mirrord operator session kill --id {id}`, and
  the hidden `mirrrod operator session retain-active`.
  [#217](https://github.com/metalbear-co/mirrord/issues/217)
- Notify user on license validity.
  [#382](https://github.com/metalbear-co/mirrord/issues/382)


### Changed

- Adds a new `PolicyRule` for `delete` and `deletecollection` of `sessions` for
  `mirrord operator setup`.
  [#456](https://github.com/metalbear-co/mirrord/issues/456)
- Change pause feature from unstable to deprecated
- Increased size of buffers used by TCP steal to read incoming streams (from 4k
  to 64k in the agent, from 1k to 64k in the internal proxy).
- Increased size of buffers used by outgoing feature to read streams (from 4k
  to 64k in the agent, from 1k to 64k in the internal proxy).


### Fixed

- Fixed a bug where `gethostbyname` calls where intercepted regardless of the
  remote dns feature status.
  [#2281](https://github.com/metalbear-co/mirrord/issues/2281)
- Fixed a bug where non-existent hosts in outgoing filter would prevent the
  application from initiating outgoing connections.
  [#2283](https://github.com/metalbear-co/mirrord/issues/2283)
- Remove special handling for DNS when dealing with UDP outgoing sockets
  (manual UDP resolving).
  [#2289](https://github.com/metalbear-co/mirrord/issues/2289)


### Internal

- Fixed lints in hook macros.


## [3.90.0](https://github.com/metalbear-co/mirrord/tree/3.90.0) - 2024-02-27


### Added

- Add agent configuration to use nftables instead of iptables-legacy to make it
  work in mesh that uses nftables.
  [#2272](https://github.com/metalbear-co/mirrord/issues/2272)
- The agent now processes all DNS queries concurrently. Also, client sessions
  in the agent do not block on the DNS queries.


### Changed

- change kubeconfig path expansion to use env as well
  [#2262](https://github.com/metalbear-co/mirrord/issues/2262)
- Increase internal proxy timeout from 5 seconds to 10 seconds to fix long
  agent ops


### Internal

- Add information how to use config file in config docs


## [3.89.1](https://github.com/metalbear-co/mirrord/tree/3.89.1) - 2024-02-22


### Fixed

- Fixed issue with Golang calling fstat on Linux causing crash
  [#2254](https://github.com/metalbear-co/mirrord/issues/2254)


## [3.89.0](https://github.com/metalbear-co/mirrord/tree/3.89.0) - 2024-02-22


### Changed

- Change intproxy log to append
- use_proxy configuration now applies to mirrord operator status, and mirrord
  ls


## [3.88.0](https://github.com/metalbear-co/mirrord/tree/3.88.0) - 2024-02-18


### Added

- Add log level and log destination for int proxy
  [#2246](https://github.com/metalbear-co/mirrord/issues/2246)


### Changed

- 1. mirrord CLI now does not check target type when the `copy_target` feature
  is enabled. The check is now done only in the operator.
  2. `mirrord operator setup` not includes permissions to read and change
  rollouts scale.


### Fixed

- Incoming traffic was being mirrord when set to `false`.


## [3.87.0](https://github.com/metalbear-co/mirrord/tree/3.87.0) - 2024-02-15


### Removed

- Remove pause tests as part of deprecation


### Added

- Changed internal proxy to allow for HTTP upgrades with filtered HTTP steal.
  [#2224](https://github.com/metalbear-co/mirrord/issues/2224)
- Added support for selecting malfunctioning targets with `copy_target`
  feature. [#2239](https://github.com/metalbear-co/mirrord/issues/2239)
- Added configuration option `feature.env.load_from_process`, which allows for
  changing the way mirrord loads environment variables from the remote target.


### Fixed

- Add missing permissions needed by operator for copy and scaledown


### Internal

- Add IDE release e2e to the ci-success job
  [#2184](https://github.com/metalbear-co/mirrord/issues/2184)
- Change cleanup to delete without grace
- Removed `#![allow(incomplete_features)]` from the code - no longer needed.
- Reorganized `TcpSubscriptionStealer` code - extracted port subscriptions into
  a separate struct.


## [3.86.1](https://github.com/metalbear-co/mirrord/tree/3.86.1) - 2024-02-05


### Fixed

- Added `runAsNonRoot: false` and `runAsUser: 0` to the security context of an
  epheremal agent when running privileged (to prevent overriding these values
  with values from the pod spec).
- Disabled unix sockets being wrongfully sent to the agent when socket isn't
  connected

## [3.86.0](https://github.com/metalbear-co/mirrord/tree/3.86.0) - 2024-01-29


### Changed

- `JAVA_TOOL_OPTIONS` is excluded by default from the environment variables
  that are fetched from the target.


### Internal

- Update links to new docs


## [3.85.1](https://github.com/metalbear-co/mirrord/tree/3.85.1) - 2024-01-29


### Fixed

- Running `mirrod exec go run EXECUTABLE` on macOS with go1.21.
  [#2202](https://github.com/metalbear-co/mirrord/issues/2202)
- Fixed a compilation bug in `mirrord-operator` crate tests.


## [3.85.0](https://github.com/metalbear-co/mirrord/tree/3.85.0) - 2024-01-24


### Added

- Added license subscription id to operator status CRD. Adjusted
  `CredentialStore` to preserve signing key pair for the same operator license
  subscription id. [#2190](https://github.com/metalbear-co/mirrord/issues/2190)
- CLI now sends machine host + username to show in mirrord operator status
  (not sent to our cloud!)
- Report port locks and filters in operator status


### Changed

- Change configuration parsing to be strict unallowing unknown fields
- Cluster DNS resolving now happens by nameserver order rather by statistics


### Fixed

- Running R on macOS.
  [#2186](https://github.com/metalbear-co/mirrord/issues/2186)
- Running scripts with whitespaces in the shebang.
  [#2193](https://github.com/metalbear-co/mirrord/issues/2193)


### Internal

- Allow both `x86_64` and `arm64` when patching thin binaries.
  [#2186](https://github.com/metalbear-co/mirrord/issues/2186)


## [3.84.1](https://github.com/metalbear-co/mirrord/tree/3.84.1) - 2024-01-19


### Fixed

- Add support for shebang containing spaces like asdf's node does
  [#2181](https://github.com/metalbear-co/mirrord/issues/2181)


## [3.84.0](https://github.com/metalbear-co/mirrord/tree/3.84.0) - 2024-01-18


### Added

- Report namespace for operator sessions


### Fixed

- add preadv and readv to fix erlang file reading
  [#2178](https://github.com/metalbear-co/mirrord/issues/2178)


## [3.83.0](https://github.com/metalbear-co/mirrord/tree/3.83.0) - 2024-01-11


### Changed

- Filesystem: File not found default filter happens after checking local filter
- The `copy_target` feature is now officialy stable.
- `mirrord operator status` reports active copy targets.


### Fixed

- Remove guard from dlopen, making calls from within dlopen hookable,
  potentially fixing issues


### Internal

- Moved the docs from `CopyTargetFileConfig` to `CopyTargetConfig` so that
  medschool reads them.
  [#2164](https://github.com/metalbear-co/mirrord/issues/2164)
- Fixed layer panics in trace only mode.


## [3.82.0](https://github.com/metalbear-co/mirrord/tree/3.82.0) - 2024-01-08


### Removed

- Removed `http_header_filter`. Please use `http_filter` with key
  `header_filter` instead.


### Added

- `mirrord operator setup` defines a `MirrordPolicy` CRD so that admins can
  block certain features by creating policies. When recieving a forbidden error
  from the operator for trying to steal traffic, mirrord shows an error and
  exits.


### Fixed

- Added userextras/oid to mirrord operator role to solve issues in some AKS
  clusters [#2152](https://github.com/metalbear-co/mirrord/issues/2152)


### Internal

- Add function to load agentfileconfig from path


## [3.81.0](https://github.com/metalbear-co/mirrord/tree/3.81.0) - 2024-01-01


### Changed

- Changed setup to not create self signed, letting operator fallback to it
  automatically on runtime
- Update dependencies


### Fixed

- Fix opendir not being hooked on macOS arm64


### Internal

- Deprecate go 18, enable go 21
- Fix some test labels, separate tests by flags, update contributing guide
- Re-add outgoing traffic with many requests tests


## [3.80.0](https://github.com/metalbear-co/mirrord/tree/3.80.0) - 2023-12-27


### Changed

- Remove unstable from ignore localhost


### Fixed

- Allow license key that starts with -
  [#2140](https://github.com/metalbear-co/mirrord/issues/2140)
- Fix job lingering by exiting always successfuly on agent


## [3.79.2](https://github.com/metalbear-co/mirrord/tree/3.79.2) - 2023-12-24


### Fixed

- Added hook for realpath function, fixing issues on files not found in Java


## [3.79.1](https://github.com/metalbear-co/mirrord/tree/3.79.1) - 2023-12-23


### Fixed

- Fix dirfd crashing on macOS


## [3.79.0](https://github.com/metalbear-co/mirrord/tree/3.79.0) - 2023-12-21


### Removed

- Remove waitlist signup from CLI


### Added

- Added new `teams` command to the CLI.
- Remove support for old cri-o, use new CRI API (v1)


### Fixed

- Uses the `syscalls` crate to handle calling the syscalls for go. And adds
  `pwrite64`, `pread64`, `fsync` and `fdatasync` hooks for go.
  [#2099](https://github.com/metalbear-co/mirrord/issues/2099)


### Internal

- Debug instructions for intproxy in contributer guide.
- Flush outgoing the console loggers after each logs so that we can see more
  logs before the layer or the intproxy exit when debugging.


## [3.78.2](https://github.com/metalbear-co/mirrord/tree/3.78.2) - 2023-12-14


### Fixed

- Fixed config verification in IDE context when the config does not specify the
  target but uses the `scale_down` feature.


## [3.78.1](https://github.com/metalbear-co/mirrord/tree/3.78.1) - 2023-12-13


### Fixed

- Removed confusing error from `mirrord exec` progress.
  [#2115](https://github.com/metalbear-co/mirrord/issues/2115)
- Support binding [::] by resolving to ipv4 unspecified. Fix gRPC Python
  running in Docker
  [#2117](https://github.com/metalbear-co/mirrord/issues/2117)


## [3.78.0](https://github.com/metalbear-co/mirrord/tree/3.78.0) - 2023-12-12


### Fixed

- Fix create react apps by adding node related files to default local
  [#2074](https://github.com/metalbear-co/mirrord/issues/2074)
- Fixed an issue with internal proxy timing out when the user application
  spawns lengthy build processes.
  [#2101](https://github.com/metalbear-co/mirrord/issues/2101)


### Internal

- Adds a new trait `LicenseValidity` implemented for `DateTime` to help us when
  checking a license's validity. Relevant for
  [#346](https://github.com/metalbear-co/operator/issues/346).
- Update rust to nightly-2023-12-07.


## [3.77.1](https://github.com/metalbear-co/mirrord/tree/3.77.1) - 2023-12-11


### Fixed

- Fix asdf compatability by adjusting local files read defaults
  [#2051](https://github.com/metalbear-co/mirrord/issues/2051)


## [3.77.0](https://github.com/metalbear-co/mirrord/tree/3.77.0) - 2023-12-07


### Added

- `mirrord verify-config` now outputs a list of available target types.
  [#2096](https://github.com/metalbear-co/mirrord/issues/2096)


### Fixed

- Changed `operator` config to be optional. If the option is set to `true`,
  mirrord always uses the operator and aborts in case of failure. If the option
  is set to `false`, mirrord does not attempt to use the operator. If the
  option is not set at all, mirrord attempts to use the operator, but does not
  abort in case it could not be found.
- Fixed config verification in IDE context when `copy_target` feature is used.


### Internal

- Reduce some code duplication around protocol and agent connection.


## [3.76.0](https://github.com/metalbear-co/mirrord/tree/3.76.0) - 2023-12-04


### Added

- Added support for connecting to the cluster with an HTTP proxy.
  [#2087](https://github.com/metalbear-co/mirrord/issues/2087)


### Changed

- Improved incoming config docs


### Fixed

- Improved handling of operator-related errors.
  [#2049](https://github.com/metalbear-co/mirrord/issues/2049)


## [3.75.3](https://github.com/metalbear-co/mirrord/tree/3.75.3) - 2023-11-23


### Added

- Added new configuration 'use_proxy' that lets user disable usage of http/s
  proxy by mirrord even when env is set


### Fixed

- Changed the way `targetless` is printed in `mirrord verify-config` to allow
  the IDEs to properly show target selection dialogs.


## [3.75.2](https://github.com/metalbear-co/mirrord/tree/3.75.2) - 2023-11-22


### Fixed

- Fixed issues with mirroring incoming TCP connections when targeting multi-pod
  deployments. [#2078](https://github.com/metalbear-co/mirrord/issues/2078)


## [3.75.1](https://github.com/metalbear-co/mirrord/tree/3.75.1) - 2023-11-14


### Fixed

- Add a hook for
  [gethostbyname](https://www.man7.org/linux/man-pages/man3/gethostbyname.3.html)
  to allow erlang/elixir to resolve DNS.
  [#2055](https://github.com/metalbear-co/mirrord/issues/2055)
- Change spammy connect log's level from info to trace.


### Internal

- Documentation of `env` config pattern matching.


## [3.75.0](https://github.com/metalbear-co/mirrord/tree/3.75.0) - 2023-11-08


### Added

- Added 'copy pod' operator feature to the CLI.
  [#1974](https://github.com/metalbear-co/mirrord/issues/1974)
- Added option to scale down target deployment when using `copy target`
  feature. [#2053](https://github.com/metalbear-co/mirrord/issues/2053)


### Fixed

- Don't drop mutex in child on fork_detour, fixes bug with elixir.
  [#2047](https://github.com/metalbear-co/mirrord/issues/2047)
- Fixed `port_mapping` feature.
  [#2058](https://github.com/metalbear-co/mirrord/issues/2058)
- Local file filter now applies to directory listing [regex] and not just
  underlying files


### Internal

- Improved crates structure around internal proxy and mirrord console.
  [#2039](https://github.com/metalbear-co/mirrord/issues/2039)


## [3.74.1](https://github.com/metalbear-co/mirrord/tree/3.74.1) - 2023-10-31


### Fixed

- Support for cluster information in exec plugin (`KUBERNETES_EXEC_INFO`)
  [#2037](https://github.com/metalbear-co/mirrord/issues/2037)
- Fixed logging using `mirrord-console`.


## [3.74.0](https://github.com/metalbear-co/mirrord/tree/3.74.0) - 2023-10-27


### Added

- Added source identifier to waitlist register


### Fixed

- `tokio` runtime dropped from layer.
  [#1952](https://github.com/metalbear-co/mirrord/issues/1952)


### Internal

- Fix the markdown in one section of the FS configuration documentation.
- Fixed python e2e test not running - removed it since it never worked and
  adapting it would take time.


## [3.73.1](https://github.com/metalbear-co/mirrord/tree/3.73.1) - 2023-10-24


### Fixed

- Fixed `KUBERNETES_EXEC_INFO` environment variable passed to `kubectl`
  authentication plugins.


## [3.73.0](https://github.com/metalbear-co/mirrord/tree/3.73.0) - 2023-10-23


### Added

- Added k0s support - add k0s containerd socket path.
  [#2014](https://github.com/metalbear-co/mirrord/issues/2014)


### Fixed

- Clarify more about the pre-defined FS exceptions in docs, link to lists.
  [#2020](https://github.com/metalbear-co/mirrord/issues/2020)


### Internal

- Move path pattern default sets to separate files so that we can link to them
  from docs. [#2019](https://github.com/metalbear-co/mirrord/issues/2019)


## [3.72.1](https://github.com/metalbear-co/mirrord/tree/3.72.1) - 2023-10-18


### Fixed

- Added `--mesh` option under `cli::Mode::Ephemeral`, allowing the agent to run
  in a mesh context with an ephemeral target.
  [#2009](https://github.com/metalbear-co/mirrord/issues/2009)


## [3.72.0](https://github.com/metalbear-co/mirrord/tree/3.72.0) - 2023-10-12


### Added

- SOCKS5 proxy is now supported.
  [#1734](https://github.com/metalbear-co/mirrord/issues/1734)


### Fixed

- Implemented missing hooks for `readdir` and `readdir64`.
  [#2001](https://github.com/metalbear-co/mirrord/issues/2001)


## [3.71.2](https://github.com/metalbear-co/mirrord/tree/3.71.2) - 2023-10-10


### Fixed

- Reverted breaking change in CLI for config verify
  [#1993](https://github.com/metalbear-co/mirrord/issues/1993)
- Adding some e2e tests for  to protect against breaking changes in the cli.
  [#1997](https://github.com/metalbear-co/mirrord/issues/1997)


### Internal

- Add homepage link to README
- Example in style guide conforms with the rule it's supposed to explain.


## [3.71.1](https://github.com/metalbear-co/mirrord/tree/3.71.1) - 2023-10-04


### Fixed

- Adds the optional `--ide` flag to `mirrord verify-config [--ide] --path
  {/config/path}`, turning some errors into warnings (target related).
  [#1979](https://github.com/metalbear-co/mirrord/issues/1979)


## [3.71.0](https://github.com/metalbear-co/mirrord/tree/3.71.0) - 2023-10-03


### Added

- Add ability to override container resource requests/limits for job agents via
  `agent.resources` config.
  [#1983](https://github.com/metalbear-co/mirrord/issues/1983)


### Fixed

- Propagate to the agent that we're in a mesh context (moved `MeshVendor` to a
  common crate), and handle the special case for `istio`, where the sniffer
  should capture traffic on the `lo` interface.
  [#1963](https://github.com/metalbear-co/mirrord/issues/1963)


### Internal

- Bump dependencies
- Remove quotes around regex in release branch check job


## [3.70.0](https://github.com/metalbear-co/mirrord/tree/3.70.0) - 2023-09-27


### Added

- Added templating for mirrord config using Tera engine.
  [#1817](https://github.com/metalbear-co/mirrord/issues/1817)


### Fixed

- Running `mix` works now (bug was calling `lstat` in `stat` bypass).
  [#1967](https://github.com/metalbear-co/mirrord/issues/1967)
- Fix progress message shows wrong latest version
  [#1972](https://github.com/metalbear-co/mirrord/issues/1972)


### Internal

- Remove `run_as_user` from operator deployment's `security_context`.


## [3.69.0](https://github.com/metalbear-co/mirrord/tree/3.69.0) - 2023-09-26


### Removed

- Remove spammy messages from progress
  [#1934](https://github.com/metalbear-co/mirrord/issues/1934)


### Added

- Added the ability to specify targetless in config file, to allow
  non-interactive targetless in IDEs
  [#1962](https://github.com/metalbear-co/mirrord/issues/1962)


### Changed

- Change targetless + steal mode to warning instead of error.
- Changed file filter to exclude jar files from being read remote by default
  [#1968](https://github.com/metalbear-co/mirrord/issues/1968)


### Fixed

- Fixes selecting container to use when using operator


## [3.68.0](https://github.com/metalbear-co/mirrord/tree/3.68.0) - 2023-09-19


### Added

- New subcommand for generating shell completions for
  bash/fish/zsh/powershell/elvish
  [#1947](https://github.com/metalbear-co/mirrord/issues/1947)


### Fixed

- Fix mirrord-cli verify-config command not serializing failures correctly due
  to serde not being able to serialize newtype pattern in tagged unions.
  [#1840](https://github.com/metalbear-co/mirrord/issues/1840)


### Internal

- CI: Add quotes to branch name in check_if_release_branch
  [#add-quotes-ci](https://github.com/metalbear-co/mirrord/issues/add-quotes-ci)
- Bump and clean depenedencies in our code and in tests
- Remove process feature from Tokio in layer package


## [3.67.0](https://github.com/metalbear-co/mirrord/tree/3.67.0) - 2023-09-13


### Added

- Add new command `mirrord verify-config [path]` to the mirrord-cli. It
  verifies a mirrord config file producing a tool friendly output.
  [#1840](https://github.com/metalbear-co/mirrord/issues/1840)


### Fixed

- Support mirroring existing sessions by introducing an HTTP check when the
  sniffer receives a tcp packet.
  [#1317](https://github.com/metalbear-co/mirrord/issues/1317)


### Internal

- Run IDE e2e tests on release.
  [#1893](https://github.com/metalbear-co/mirrord/issues/1893)
- Add `document` method on `KeyPair` to access pem encoded key.


## [3.66.0](https://github.com/metalbear-co/mirrord/tree/3.66.0) - 2023-09-12


### Added

- Added support for pausing ephemeral containers. This feature requires the
  agent to have privileged access.
  [#1358](https://github.com/metalbear-co/mirrord/issues/1358)


### Changed

- Add ruby related ENV to the default exclude list


### Internal

- Add concurrent steal tests - operator


## [3.65.2](https://github.com/metalbear-co/mirrord/tree/3.65.2) - 2023-09-10


### Changed

- Add ruby related ENV to the default exclude list


### Fixed

- Fixed connecting to unspecified ip i.e 0.0.0.0
  [#1928](https://github.com/metalbear-co/mirrord/issues/1928)


### Internal

- Refactor code to match newest Rust nightly - 2023-09-07
  [#1457](https://github.com/metalbear-co/mirrord/issues/1457)
- Update tj-actions to v39 to check for fixes for similar hashes


## [3.65.1](https://github.com/metalbear-co/mirrord/tree/3.65.1) - 2023-09-07


### Changed

- Add some Ruby env to excluded: GEM_HOME, GEM_PATH
  [#1892](https://github.com/metalbear-co/mirrord/issues/1892)


### Fixed

- Disable opendir hook on aarch macOS since it crashes due to arm64e issues
  [#1920](https://github.com/metalbear-co/mirrord/issues/1920)


## [3.65.0](https://github.com/metalbear-co/mirrord/tree/3.65.0) - 2023-09-06


### Added

- Add hooks for `opendir`, `readdir64_r`, `__lxstat`, `__lxstat64`,
  `__xstat64`. Also contains a refactor of the `*stat` family of hooks (they
  now call a shared function), and `openat64` as its own function.
  [#1899](https://github.com/metalbear-co/mirrord/issues/1899)


### Changed

- CLI: Change operator setup to fetch from API version of operator to install
- Don't error on agent ready missing absent, add retry in connection to
  upstream agent


### Fixed

- Fixed agent crashing on flush connections not enabled
  [#1904](https://github.com/metalbear-co/mirrord/issues/1904)
- CI: run macOS clippy on all of the codebase
- Fixed pause flakiness by improving our cleanup process [using watch]


### Internal

- CLI: Change to use drain instead of hoping sleep would help
- Enable ephemeral containers for tests on operator


## [3.64.2](https://github.com/metalbear-co/mirrord/tree/3.64.2) - 2023-09-05


### Changed

- Changed the targeted and non targeted flows for creating agent.
  [#1844](https://github.com/metalbear-co/mirrord/issues/1844)
- Add info about errors to TELEMETRY.md


### Fixed

- Change agent to use current thread runtime, multi thread is enabled by
  mistake.
  * Added a sleep before exiting both in layer and agent to allow
  `tokio::spawn` tasks spawned from `Drop` finish.
  * Changed implementation of pause guard to use `tokio::spawn` - fixes pause
  in combination with above change.
- Fix analytics not being sent on drop


### Internal

- Refactor [if Some / else None] pattern to use [bool.then()] in medschool.
  [#1890](https://github.com/metalbear-co/mirrord/issues/1890)
- Add a "Try to reduce the use of non-optional `Option`s" rule to the mirrord
  style guide.
- Label mirroring integration tests as flaky


## [3.64.1](https://github.com/metalbear-co/mirrord/tree/3.64.1) - 2023-09-03


### Fixed

- Remove extra message of `{"type":"FinishedTask","name":"mirrord preparing to
  launch","success":true,"message":null}` that causes breakage in extensions.


## [3.64.0](https://github.com/metalbear-co/mirrord/tree/3.64.0) - 2023-09-02


### Added

- Warn when running in a mesh service with mirror mode.
  [#1768](https://github.com/metalbear-co/mirrord/issues/1768)


### Fixed

- Fixed detecting operator not found as an error
  [#1868](https://github.com/metalbear-co/mirrord/issues/1868)
- Update hyper and hyper-util to fix double select call (and properly handle
  large http traffic).
  [#1869](https://github.com/metalbear-co/mirrord/issues/1869)
- Fixed ongoing connections not being stolen by changing our flush mechanism -
  add a rule to drop marked connections and then mark existing connections when
  starting to steal.
  [#1870](https://github.com/metalbear-co/mirrord/issues/1870)
- Fixed panic on crash analytics
  [#1872](https://github.com/metalbear-co/mirrord/issues/1872)
- Fixed showing error on each file not found triggered by the file not found
  configuration
- Regex meta characters are now escaped in `$HOME` (`not-found` file filter).


### Internal

- Set fork env in integration tests using uvicorn
  [#1627](https://github.com/metalbear-co/mirrord/issues/1627)
- Fix complete_passthrough response flake
  [#1811](https://github.com/metalbear-co/mirrord/issues/1811)
- Fix typo in flush connections conntrack command


## [3.63.0](https://github.com/metalbear-co/mirrord/tree/3.63.0) - 2023-08-28


### Added

- Add the ability to send analytics on errors and not only on successful runs.
  [#1785](https://github.com/metalbear-co/mirrord/issues/1785)
- Report back internal proxy error stream to cli
  [#1855](https://github.com/metalbear-co/mirrord/issues/1855)


### Changed

- Changed config unstable/deprecations to be aggregated with other config
  warnings [#1860](https://github.com/metalbear-co/mirrord/issues/1860)


### Fixed

- `not-found` file filter fixed to only match files inside the `$HOME`
  directory. [#1863](https://github.com/metalbear-co/mirrord/issues/1863)
- Fix openshift detection taking too long by querying a subset instead of all
  APIs


### Internal

- CI Improvements:
  - Unify lint and integration for macOS to save cache and runner.
  - Remove trace logging from integration tests on macos
  - use node 18 for testing since installing 19 in CI takes hours.
  - remove `build_mirrord` job - quite useless as it's used only in other
  workflow, so have it there and re-use cache
    also save some cache,
  - specify target for all cargo invocations to re-use cache efficiently.
  - fix flake with node server closing before time
- Fix regression in kube api blocking operator from compiling
- Reorganize the CI with the following objective of unifying as much as we can
  CI that can run on the same host, this is to have less caches and have better
  compilation time (as there's overlap). Things done:

  - Remove the build layer CI, since we now have an integration tests that
  check it + clippy for aarch darwin / Linux
  - Make clippy run for all of the project for aarch64 linux instead of agent
  only
  - Revert removal of Rust cache from e2e (was by mistake)
  - Don't use "cache" for other Gos since it will try to overwrite and have bad
  results.


## [3.62.0](https://github.com/metalbear-co/mirrord/tree/3.62.0) - 2023-08-26


### Added

- Add analytics collection to operator session information.
  [#1805](https://github.com/metalbear-co/mirrord/issues/1805)
- Added an extra `not-found` file filter to improve experience when using cloud
  services under mirrord.
  [#1694](https://github.com/metalbear-co/mirrord/issues/1694)


### Changed

- Update telemetry.md with new info about mirrord for Teams
  [#1837](https://github.com/metalbear-co/mirrord/issues/1837)
- Changed keep alive to happen from internal proxy to support cases where layer
  process is stuck [breakpoint/etc]
  [#1839](https://github.com/metalbear-co/mirrord/issues/1839)
- Changed CLI progress to print warnings and not only set it as the last
  message of progress
- Changed config verify to return aggregated warnings list for user to print
  instead of warn in current progress - can fix issues with extension where we
  printed to stderr.


### Fixed

- Fix ephemeral agent creation api using agent namespace instead of target.
  Add note about agent namespace being irrelevant in ephemeral.
- Fix macOS SIP potential issues from exec having mirrord loaded into the code
  sign binary.
- Fix operator setup so `MIRRORD_OPERATOR_IMAGE` will function properly.
- Fixed issue connecting to ephemeral container when target is in different
  namespace


### Internal

- Changed e2e to use a shared setup e2e action to leverage all GitHub caches,
  reduce e2e time by half.
- Changes to the CI to make it fater:
  - Use go cache for integration test
  - use mac-13 runner for tests


## [3.61.0](https://github.com/metalbear-co/mirrord/tree/3.61.0) - 2023-08-22


### Added

- Support DNS resolution for the outgoing filter config.
  [#702](https://github.com/metalbear-co/mirrord/issues/702)


### Fixed

- Fixed wrong errno being set by mirrord, fixing various flows that rely on
  errno even when return code is ok
  [#1828](https://github.com/metalbear-co/mirrord/issues/1828)


## [3.60.0](https://github.com/metalbear-co/mirrord/tree/3.60.0) - 2023-08-21


### Added

- Detect and warn when cluster is openshift
  [#1560](https://github.com/metalbear-co/mirrord/issues/1560)
- Add missing hook for open64, fixing certificate loading on C# + Linux
  [#1815](https://github.com/metalbear-co/mirrord/issues/1815)
- Small changes relevant to operator for #1782.


### Fixed

- Fixed environment on ephemeral container
  This is done by two things:

  1. There was an issue where we used `self` instead of `1` to obtain env based
  on pid.
  2. We didn't have container runtime to use for fetching, so now we also copy
  env from the original pod spec and set it to ours.
  [#1818](https://github.com/metalbear-co/mirrord/issues/1818)


### Internal

- Added a missing comma in the documentation


## [3.59.0](https://github.com/metalbear-co/mirrord/tree/3.59.0) - 2023-08-18


### Added

- Add option to run agent container as privileged - `"agent" : {"privileged":
  true}`
  Should help with Bottlerocket or other secured k8s environments.
  Applicable for both job/ephemeral.
  [#1806](https://github.com/metalbear-co/mirrord/issues/1806)


### Fixed

- Send only ResponseError::DnsLookup for all errors during DNS lookups
  [#1809](https://github.com/metalbear-co/mirrord/issues/1809)


### Internal

- Update Frida dependency


## [3.58.0](https://github.com/metalbear-co/mirrord/tree/3.58.0) - 2023-08-17


### Added

- Introduced hooks for sendmsg and recvmsg, so mongodb+srv protocol (Csharp)
  may resolve DNS (implementation follows previous sendto and recvfrom patch).
  [#1776](https://github.com/metalbear-co/mirrord/issues/1776)


### Fixed

- Fixed more complicated scenarios using Go on Linux Arm


### Internal

- Move e2e setup to a bash script


## [3.57.2](https://github.com/metalbear-co/mirrord/tree/3.57.2) - 2023-08-16


### Fixed

- Fix crash on forks by leaking HOOK_SENDER
  [#1792](https://github.com/metalbear-co/mirrord/issues/1792)
- CLI now uses the json progress tracker as default.


## [3.57.1](https://github.com/metalbear-co/mirrord/tree/3.57.1) - 2023-08-15


### Internal

- Add `nodes` and `pods/log` resource permissions to `mirrord-operator`
  ClusterRole.


## [3.57.0](https://github.com/metalbear-co/mirrord/tree/3.57.0) - 2023-08-15


### Added

- Add hooks for Go 1.19 >= on Linux Arm
  [#563](https://github.com/metalbear-co/mirrord/issues/563)
- Add node allocatabillity check to prevent OutOfPods error on agent job.
  [#1782](https://github.com/metalbear-co/mirrord/issues/1782)


### Changed

- Incoming config now supports off mode, which is also used when `"incoming":
  "off"`.
  When incoming is off, listen requests go through.
  Changed targetless to warn on listen, since bind can happen on outgoing
  sockets as well.


### Fixed

- Replaced termspin with indicatif, fix multi line issues and refactored
  progress. [#1664](https://github.com/metalbear-co/mirrord/issues/1664)


### Internal

- Add trace only mode for layer for easier debugging
- Agent now prints its version in the initial "agent ready" message.


## [3.56.1](https://github.com/metalbear-co/mirrord/tree/3.56.1) - 2023-08-09


### Fixed

- Add missing hook for `read$NOCANCEL`, fixes reading remote files in some
  scenarios. [#1747](https://github.com/metalbear-co/mirrord/issues/1747)


### Internal

- Updated `hyper` version.
  [#1774](https://github.com/metalbear-co/mirrord/issues/1774)


## [3.56.0](https://github.com/metalbear-co/mirrord/tree/3.56.0) - 2023-08-07


### Added

- Added `internal_proxy` timeout configurations to allow users specify timeouts
  in edge cases. [#1761](https://github.com/metalbear-co/mirrord/issues/1761)


### Changed

- Change operator / cli version mismatch to show only when mirrord is older
  than operator


### Fixed

- Fix grpc errors caused by missing trailers on filtered http responses.
  [#1731](https://github.com/metalbear-co/mirrord/issues/1731)


## [3.55.2](https://github.com/metalbear-co/mirrord/tree/3.55.2) - 2023-08-06


### Fixed

- macOS - Running Go build with mirrord while Go binary is SIP protected is
  fixed by enabling file hooks on SIP load mode.
  [#1764](https://github.com/metalbear-co/mirrord/issues/1764)


## [3.55.1](https://github.com/metalbear-co/mirrord/tree/3.55.1) - 2023-08-06


### Fixed

- Try to resolve an issue where internal proxy is under heavy load since bash
  scripts does a lot of fork/exec by:
  1. Increasing internal proxy's listen backlog (might not help on macOS)
  2. Change internal proxy to create the upstream (agent) connection in a
  different task, allowing it to keep accepting.
  [#1716](https://github.com/metalbear-co/mirrord/issues/1716)
- Fixed detecting skipped processes during layer injection.
  [#1752](https://github.com/metalbear-co/mirrord/issues/1752)
- Fix fork issues by changing layer runtime to be current thread
  [#1759](https://github.com/metalbear-co/mirrord/issues/1759)


## [3.55.0](https://github.com/metalbear-co/mirrord/tree/3.55.0) - 2023-08-03


### Added

- Add support for selecting Kubeconfig context to use by either using:
  1. Configuration option `kube_context`.
  2. mirrord exec argument `--context`
  3. Environment variable `MIRRORD_KUBE_CONTEXT`
  [#1735](https://github.com/metalbear-co/mirrord/issues/1735)


### Changed

- Add userextras/prinicipalid to operators cluster role
- Document behavior of deployment in OSS vs mirrord for Teams
- Skip HashiCorp Vault supporting containers


### Fixed

- Fix fork issue on macOS
  [#1745](https://github.com/metalbear-co/mirrord/issues/1745)
- Add access for the operator's cluster role to argoproj rollouts
  [#1751](https://github.com/metalbear-co/mirrord/issues/1751)
- Fixed warning on using deployment target with no operator


### Internal

- Attempt to fix fork issue on macOS by avoiding access to CF on fork
  [#1745](https://github.com/metalbear-co/mirrord/issues/1745)
- Add note to the HttpRequest message with information to add when we break the
  protocol.
- Change deployment warning to be emit on cli only
- Refactor the organization of go hooks in preparation for support for arm


## [3.54.1](https://github.com/metalbear-co/mirrord/tree/3.54.1) - 2023-08-01


### Fixed

- Sometimes the internal proxy doesn't flush before we do redirection then
  caller can't read port
  leading to "Couldn't get port of internal proxy"


### Internal

- Remove signal dependency from layer


## [3.54.0](https://github.com/metalbear-co/mirrord/tree/3.54.0) - 2023-07-31


### Added

- Added mirrord-operator-user `ClusterRole` to operator setup with RBAC
  permissions to use operator.
  [#1428](https://github.com/metalbear-co/mirrord/issues/1428)


### Fixed

- Exclude environment variable COMPLUS_EnableDiagnostics, fixes [mirrord
  intellij #67](https://github.com/metalbear-co/mirrord-intellij/issues/67)
  [#1728](https://github.com/metalbear-co/mirrord/issues/1728)


### Internal

- Redirect stderr and stdout of internal proxy to /dev/null (stdout only after
  printing port).
  [#int-proxy-dev-null](https://github.com/metalbear-co/mirrord/issues/int-proxy-dev-null)


## [3.53.3](https://github.com/metalbear-co/mirrord/tree/3.53.3) - 2023-07-26


### Fixed

- Add `jspawnhelper` to build-tool list & fix skip_processes detection
  [#1709](https://github.com/metalbear-co/mirrord/issues/1709)
- Use `shellexpand` to resolve tilde for `kubeconfig`.
  [#1721](https://github.com/metalbear-co/mirrord/issues/1721)


## [3.53.2](https://github.com/metalbear-co/mirrord/tree/3.53.2) - 2023-07-24


### Fixed

- Add automatic skip for build-tools `"skip_build_tools": boolean` to config
  [default: True] (build-tool list: `as`, `cc`, `ld`, `go`, `air`, `asm`,
  `cc1`, `cgo`, `dlv`, `gcc`, `git`, `link`, `math`, `cargo`, `hpack`, `rustc`,
  `compile`, `collect2`, `cargo-watch` and `debugserver`)
  [#1478](https://github.com/metalbear-co/mirrord/issues/1478)
- Fix `feature.env.override` documentation "overrides" -> "override".
  [mirrord.dev#120](https://github.com/metalbear-co/mirrord.dev/issues/120)
- Specify default value for `agent.tolerations` in docs as json instead of
  yaml.


## [3.53.1](https://github.com/metalbear-co/mirrord/tree/3.53.1) - 2023-07-23


### Changed

- Changed internal proxy to drop stdout/stderr after it finishes loading


### Internal

- Fixed flakes caused by stdout/stderr not being flushed before after process
  is done


## [3.53.0](https://github.com/metalbear-co/mirrord/tree/3.53.0) - 2023-07-20


### Added

- Add support for `agent.tolerations` configuraion field for setting agent
  `Toleration`s to work around `Taint`s in the cluster.
  [#1692](https://github.com/metalbear-co/mirrord/issues/1692)


### Changed

- Added env var exclusion for `_JAVA_OPTIONS` to avoid loading remote jars or
  settings that wouldn't work locally.
  [#1695](https://github.com/metalbear-co/mirrord/issues/1695)


## [3.52.1](https://github.com/metalbear-co/mirrord/tree/3.52.1) - 2023-07-19


### Internal

- Added Java Debug port detecting to use in VSCode/IntelliJ
  [#1689](https://github.com/metalbear-co/mirrord/issues/1689)


## [3.52.0](https://github.com/metalbear-co/mirrord/tree/3.52.0) - 2023-07-18


### Added

- Support for OIDC refresh token
  [#1460](https://github.com/metalbear-co/mirrord/issues/1460)


### Fixed

- Fixed case where proxy can timeout since it holds a stale connection. Added
  heartbeat to the connection handling
- Fixed dynamic pause with operator not working - moved pause request to be
  from internal proxy
- Update the code to reimplement the fix but without moving the pinging source.


### Internal

- Add Cargo Chef to mirrord-agent docker image for better utilisation of cache
  layers.
- Reduce amount of API calls to use operator


## [3.51.1](https://github.com/metalbear-co/mirrord/tree/3.51.1) - 2023-07-16


### Changed

- 'mirrod ls' command now no longer lists crashed pods as targets
  [#1617](https://github.com/metalbear-co/mirrord/issues/1617)


### Internal

- Add `readinessProbe` and `livenessProbe` to operator deployment.
- Add support for operator feature flags & new "proxy" verb api.
- Remove operator pvc from setup.


## [3.51.0](https://github.com/metalbear-co/mirrord/tree/3.51.0) - 2023-07-16


### Added

- Add outgoing traffic filter feature.

  Adds a way of controlling from where outgoing traffic should go, either
  through the remote pod, or from the local app. Can be configured with the
  `remote` and `local` options under `feature.network.outgoing`.
  [#702](https://github.com/metalbear-co/mirrord/issues/702)
- mirrord configuration now allows disabling Linux capabilities for the agent
  container. [#1662](https://github.com/metalbear-co/mirrord/issues/1662)
- Add env to specify operator image


### Internal

- Add new error to the kube error enum needed by operator
- Update VS Code gif in README


## [3.50.5](https://github.com/metalbear-co/mirrord/tree/3.50.5) - 2023-07-13


### Fixed

- Make sure conntrack flushes the correct port.
  [#1655](https://github.com/metalbear-co/mirrord/issues/1655)
- Added `CAP_NET_RAW` Linux capability agent.


### Internal

- Use a patched (fixed) version of rasn, otherwise doing a cargo update breaks
  compilation due to rasn-derive from rasn used in apple-codesign.


## [3.50.4](https://github.com/metalbear-co/mirrord/tree/3.50.4) - 2023-07-11


### Fixed

- Layer now passes detected debugger port down to the child processes.
  [#1641](https://github.com/metalbear-co/mirrord/issues/1641)


## [3.50.3](https://github.com/metalbear-co/mirrord/tree/3.50.3) - 2023-07-11


### Fixed

- Fixed double slash in uri path on `pause/unpause` requests in agent
  [#1638](https://github.com/metalbear-co/mirrord/issues/1638)
- Fix agent steal crash by adding fallback to using only `PREROUTING` iptable
  chain. [#1640](https://github.com/metalbear-co/mirrord/issues/1640)


### Internal

- Add test for operator setup command that checks that the setup passes with
  `mirrord operator setup ... | kubectl apply --dry-run=client -f -`.
  [#create-operator-setup-test](https://github.com/metalbear-co/mirrord/issues/create-operator-setup-test)
- Add --prepend --input and --output args when running medschool.


## [3.50.2](https://github.com/metalbear-co/mirrord/tree/3.50.2) - 2023-07-10


### Changed

- Changed agent job definition to tolerate everything
  [#1634](https://github.com/metalbear-co/mirrord/issues/1634)


### Internal

- Fix a critical typo in operator setup added in 3.50.1.
  [#setup-hotfix](https://github.com/metalbear-co/mirrord/issues/setup-hotfix)


## [3.50.1](https://github.com/metalbear-co/mirrord/tree/3.50.1) - 2023-07-09


### Fixed

- Small fix to operator setup command.


### Internal

- We were overriding the fs mode only for the file filter, but not for the rest
  of the config.


## [3.50.0](https://github.com/metalbear-co/mirrord/tree/3.50.0) - 2023-07-07


### Removed

- Removed error capture error trace feature


### Added

- Add support for Argo rollout target.
  [#1593](https://github.com/metalbear-co/mirrord/issues/1593)


### Changed

- Agent container is no longer privileged. Instead, it is given a specific set
  of Linux capabilities: `CAP_NET_ADMIN`, `CAP_SYS_PTRACE`, `CAP_SYS_ADMIN`.
  [#1615](https://github.com/metalbear-co/mirrord/issues/1615)
- Changed agent job definition to include limits
  [#1621](https://github.com/metalbear-co/mirrord/issues/1621)


### Fixed

- Running java 17.0.6-tem with mirrord.
  [#1459](https://github.com/metalbear-co/mirrord/issues/1459)


### Internal

- Add support for installing operator with online license key.
- Cleaning kube resources after e2e tests no longer needs two runtime threads.


## [3.49.1](https://github.com/metalbear-co/mirrord/tree/3.49.1) - 2023-07-04


### Changed

- Small optimization in file reads to avoid sending empty data
  [#1254](https://github.com/metalbear-co/mirrord/issues/1254)
- Changed internal proxy to close after 5s of inactivity instead of 1
- use Frida's replace_fast in Linux Go hooks


### Fixed

- Child processes of python application would hang after a fork without an
  exec. [#1588](https://github.com/metalbear-co/mirrord/issues/1588)


### Internal

- Change locks in test process to be async to avoid deadlocks
- Fix listen ports flakiness by handling shutdown messages if they arrive
- Use Tokio current thread runtime in tests as it seems to be less flaky
- Use current thread Tokio runtime in fork integration test


## [3.49.0](https://github.com/metalbear-co/mirrord/tree/3.49.0) - 2023-07-03


### Added

- Added new analytics, see TELEMETRY.md for more details.


### Internal

- Fix some text in the operator documentation and progress reporting
- Remove IDE instructions from CONTRIBUTING.md


## [3.48.0](https://github.com/metalbear-co/mirrord/tree/3.48.0) - 2023-06-29


### Added

- Added Deployment to list of targets returnd from `mirrord ls`.
  [#1503](https://github.com/metalbear-co/mirrord/issues/1503)


### Changed

- Bump rust nightly to 2023-04-19 (latest nightly with support for const std
  traits). [#1457](https://github.com/metalbear-co/mirrord/issues/1457)
- Change loglevel of warnings to info of logs that were mistakenly warning
- Moved IntelliJ to its own repository and versioning


### Fixed

- Hook send_to and recv_from, leveraging our existing UDP interceptor mechanism
  to manually resolve DNS (as expected by netty, especially relevant for
  macos). [#1458](https://github.com/metalbear-co/mirrord/issues/1458)
- Add new rule to the OUTPUT chain of iptables in agent to support kubectl
  port-forward [#1479](https://github.com/metalbear-co/mirrord/issues/1479)
- If the local user application closes a socket but continues running, we now
  also stop mirroring/stealing from the target.
  [#1530](https://github.com/metalbear-co/mirrord/issues/1530)
- Add /home and /usr to the default file filter.
  [#1582](https://github.com/metalbear-co/mirrord/issues/1582)
- Fixed reporting EADDRINUSE as an error


### Internal

- (Operator only) Add `feature.network.incoming.on_concurrent_steal` option to
  allow overriding port locks.
- Improve medschool to produce more deterministic configuration.md, and
  (mostly) fixes it dropping some configuration docs during processing.
- Make mirrord ls deployment fetch parallel.
- Remove unused CRD for operator and don't error on missing operator
  credentials


## [3.47.0](https://github.com/metalbear-co/mirrord/tree/3.47.0) - 2023-06-20


### Added

- Added `listen_ports` to `incoming` config to control what port is actually
  being used locally
  so mirrored/stolen ports can still be accessed locally via those. If port
  provided by `listen_ports`
  isn't available, application will receive `EADDRINUSE`.
  Example configuration:
  ```json
  {
      "feature":
      {
          "incoming": {
              "listen_ports": [[80, 7111]]
          }
      }
  }
  ```
  will make port 80 available on 7111 locally, while stealing/mirroring port
  80. [#1554](https://github.com/metalbear-co/mirrord/issues/1554)


### Changed

- Changed the logic of choosing local port to use for intercepting mirror/steal
  sockets
  now instead of assigning a random port always, we try to use the original one
  and if we fail we assign random port.
  This only happens if `listen_ports` isn't used.
  [#1554](https://github.com/metalbear-co/mirrord/issues/1554)
- The path `/opt` itself is read locally by default (up until now paths inside
  that directory were read locally by default, but not the directory itself).
  [#1570](https://github.com/metalbear-co/mirrord/issues/1570)
- Changed back required IntelliJ version to 222+ from 223+
- Moved VSCode extension to its own repository and versioning
  https://github.com/metalbear-co/mirrord-vscode


### Fixed

- Running python with mirrord on apple CPUs.
  [#1570](https://github.com/metalbear-co/mirrord/issues/1570)


### Internal

- Use tagged version of ci-agent-build, so we can update Rust and the agent
  independently. [#1457](https://github.com/metalbear-co/mirrord/issues/1457)


## [3.46.0](https://github.com/metalbear-co/mirrord/tree/3.46.0) - 2023-06-14


### Added

- Add support for HTTP Path filtering
  [#1512](https://github.com/metalbear-co/mirrord/issues/1512)


### Changed

- Refactor vscode-ext code to be more modular


### Fixed

- Fixed bogus warnings in the VS Code extension.
  [#1504](https://github.com/metalbear-co/mirrord/issues/1504)
- Mirroring/stealing a port for a second time after the user application closed
  it once. [#1526](https://github.com/metalbear-co/mirrord/issues/1526)
- fixed using dotnet debugger on VSCode
  [#1529](https://github.com/metalbear-co/mirrord/issues/1529)
- Properly detecting and ignoring localhost port used by Rider's debugger.
- fix vscode SIP patch not working


### Internal

- Add a state Persistent Volume Claim to operator deployment setup.
- Bring the style guide into the repo.
- Fix vscode e2e job not running
- Remove OpenSSL dependency again
- Switch to new licensing and operator authenticaion flow.
- fix launch json for vscode extension
- fix macos build script to use directory's toolchain


## [3.45.2](https://github.com/metalbear-co/mirrord/tree/3.45.2) - 2023-06-12


### Internal

- Remove frida openSSL dependency


## [3.45.1](https://github.com/metalbear-co/mirrord/tree/3.45.1) - 2023-06-11


### Fixed

- Installation script now does not use `sudo` when not needed. This enbables
  installing mirrord in a `RUN` step in an ubuntu docker container, without
  installing `sudo` in an earlier step.
  [#1514](https://github.com/metalbear-co/mirrord/issues/1514)
- fix crio on openshift
  [#1534](https://github.com/metalbear-co/mirrord/issues/1534)
- Skipping `gcc` when debugging Go in VS Code extension.


### Internal

- change `mirrord-protocol` to have its own versioning. add `mirrord-macros`
  and `protocol_break` attribute to mark places we want to break on major
  updates.
  Add CI to verify that if protocol is changed, `Cargo.toml` is changed as well
  (to force bumps)
  Fix some of the structs being `OS` controlled, potentially breaking the
  protocol between different OS's.
  (`GetDEnts64(RemoteResult<GetDEnts64Response>),`)
  [#1355](https://github.com/metalbear-co/mirrord/issues/1355)
- Partial refactor towards 1512
  [#1512](https://github.com/metalbear-co/mirrord/issues/1512)
- Add integration test for DNS resolution
- Bumped versions of some VS Code extension dependencies.
- Frida bump and other dependencies
- Integration test for recv_from
- Reorganize dev docs
- Update our socket2 dependency, since the code we pushed there was released.


## [3.45.0](https://github.com/metalbear-co/mirrord/tree/3.45.0) - 2023-06-05


### Added

- Rider is now supported by the IntelliJ plugin.
  [#1012](https://github.com/metalbear-co/mirrord/issues/1012)


### Fixed

- Chagned agent to not return errors on reading from outgoing sockets, and
  layer to not crash in that case anyway


### Internal

- Use one thread for namespaced runtimes
  [#1287](https://github.com/metalbear-co/mirrord/issues/1287)
- Better timeformatting in e2e and maybe reduce flakiness?
- Fix nodejs deprecation warnings in CI
- Set MIRRORD_AGENT_IMAGE for vscode e2e


## [3.44.2](https://github.com/metalbear-co/mirrord/tree/3.44.2) - 2023-06-01


### Changed

- Change phrasing on version mismatch warning.
- Add `/Volumes` to default local on macOS
- Change Ping interval from 60s down to 30s.
- Changed local read defaults - list now includes `^/sbin(/|$)` and
  `^/var/run/com.apple`.


### Fixed

- Running postman with mirrord works.
  [#1445](https://github.com/metalbear-co/mirrord/issues/1445)
- Return valid error code when dns lookup fails, instead of -1.


### Internal

- Add E2E tests for vscode extension
  [#201](https://github.com/metalbear-co/mirrord/issues/201)
- Fixed flaky integration tests.
  [#1452](https://github.com/metalbear-co/mirrord/issues/1452)
- Fixed e2e tests' flakiness in the CI.
  [#1453](https://github.com/metalbear-co/mirrord/issues/1453)
- Change CI log level to be debug instead of trace
- Hooking `_NSGetExecutablePath` on macOS to strip the `mirrord-bin` temp dir
  off the path.
- Introduce a tool to extract config docs into a markdown file. Update docs to
  match whats in mirrord-dev.
- On macOS, if we path a binary for SIP and it is in a path that is inside a
  directory that has a name that ends with `.app`, we add the frameworks
  directory to `DYLD_FALLBACK_FRAMEWORK_PATH`.
- Provide buffer for `IndexAllocator` to avoid re-use of indices too fast


## [3.44.1](https://github.com/metalbear-co/mirrord/tree/3.44.1) - 2023-05-26


### Changed

- Never importing `RUST_LOG` environment variable from the cluster, regardless
  of configuration.


### Fixed

- Provide helpful error messages on errors in IDEs.
  [#1392](https://github.com/metalbear-co/mirrord/issues/1392)
- Log level control when running targetless.
  [#1446](https://github.com/metalbear-co/mirrord/issues/1446)
- Change to sticky balloon on warnings in intelliJ
  [#1456](https://github.com/metalbear-co/mirrord/issues/1456)
- Setting the namespace via the configuration was not possible in the IDE
  without also setting a target in the configuration file.
  [#1461](https://github.com/metalbear-co/mirrord/issues/1461)
- fixed IntelliJ failing silently when error happened on listing pods


### Internal

- Fix the test of reading from the SIP patch dir, that was not testing
  anything.
- Make the path field of `TargetConfig` an `Option`.


## [3.44.0](https://github.com/metalbear-co/mirrord/tree/3.44.0) - 2023-05-24


### Added

- Changed agent's pause feature. Now the pause is requested dynamically by CLI
  during setup and the agent keeps the target container paused until exit or
  the unpause request was received.
  [#1408](https://github.com/metalbear-co/mirrord/issues/1408)
- Added support for NPM run configuration on JetBrains products.
  [#1418](https://github.com/metalbear-co/mirrord/issues/1418)


### Changed

- Change mirrord ls to show only pods that are in running state (not
  crashing,starting,etc)
  [#1436](https://github.com/metalbear-co/mirrord/issues/1436)
- Change fs mode to be local with overrides when targetless is used
- Make progress text consitently lowercase.


### Fixed

- Fix misalignment on IntelliJ not accepting complex path in target
  [#1441](https://github.com/metalbear-co/mirrord/issues/1441)
- Add impersonate permissions for GCP specific RBAC in operator


### Internal

- Fix node spawn test flakiness on macOS
  [#1431](https://github.com/metalbear-co/mirrord/issues/1431)


## [3.43.0](https://github.com/metalbear-co/mirrord/tree/3.43.0) - 2023-05-22


### Added

- Support for targetless execution: when not specifying any target for the
  agent, mirrord now spins up an independent agent. This can be useful e.g. if
  you are just interested in getting the cluster's DNS resolution and outgoing
  connectivity but don't want any pod's incoming traffic or FS.
  [#574](https://github.com/metalbear-co/mirrord/issues/574)
- Support for targetless mode in IntelliJ based IDEs.
  [#1375](https://github.com/metalbear-co/mirrord/issues/1375)
- Support for targetless mode in vscode.
  [#1376](https://github.com/metalbear-co/mirrord/issues/1376)


### Changed

- If a user application tries to read paths inside mirrord's temp dir, we hook
  that and read the path outside instead.
  [#1403](https://github.com/metalbear-co/mirrord/issues/1403)
- Don't print error if we fail checking for operator


### Fixed

- Added better detection for protected binaries, fixes not loading into Go
  binary [#1397](https://github.com/metalbear-co/mirrord/issues/1397)
- Disallow binding on the same address:port twice. Solves part of issue 1123.
  [#1123](https://github.com/metalbear-co/mirrord/issues/1123)
- Fix the lost update bug with config dropdown for intelliJ
  Fix the lost update bug with config dropdown for intelliJ.
  [#1420](https://github.com/metalbear-co/mirrord/issues/1420)
- Fix intelliJ compatability issue by implementing missing
  createPopupActionGroup


### Internal

- Run IntelliJ Plugin Verifier on CI
  [#1417](https://github.com/metalbear-co/mirrord/issues/1417)
- Remove bors.toml since we use GH merge queue now
- Upgrade k8s dependencies and rustls, remove ugly feature ip patch


## [3.42.0](https://github.com/metalbear-co/mirrord/tree/3.42.0) - 2023-05-15


### Added

- mirrord config dropdown for intelliJ.
  [#1030](https://github.com/metalbear-co/mirrord/issues/1030)
- Log agent version when initializing the agent.


### Changed

- Remove quotes in InvalidTarget' target error message


### Fixed

- Use ProgressManager for mirrord progress on intelliJ
  [#1337](https://github.com/metalbear-co/mirrord/issues/1337)
- Fixed `go run` failing because of reading remote files by maing paths under
  /private and /var/folders read locally by default.
  [#1397](https://github.com/metalbear-co/mirrord/issues/1397)
- Fix not loading into Go because of SIP by adding into default patched
  binaries


## [3.41.1](https://github.com/metalbear-co/mirrord/tree/3.41.1) - 2023-05-07


### Fixed

- Fixed regression in GoLand and NodeJS causing a crash
  [#1389](https://github.com/metalbear-co/mirrord/issues/1389)


## [3.41.0](https://github.com/metalbear-co/mirrord/tree/3.41.0) - 2023-05-06


### Added

- Last selected target is now remembered in IntelliJ extension and shown first
  in the target selection dialog.
  [#1347](https://github.com/metalbear-co/mirrord/issues/1347)
- Warn user when their mirrord version doesn't match the operator version.


### Changed

- mirrord loading progress is displayed in the staus indicator on IntelliJ,
  replacing the singleton notifier
  [#1337](https://github.com/metalbear-co/mirrord/issues/1337)


### Fixed

- Fix crash on unexpected LogMessage
  [#1380](https://github.com/metalbear-co/mirrord/issues/1380)
- Added hook for recvfrom to support cases where caller expects the messages to
  be from address they were sent to.
  [#1386](https://github.com/metalbear-co/mirrord/issues/1386)


### Internal

- Add x-session-id to operator request, that is persistent across child
  processes in a single mirrord exec.
- Improve metadata for VSCode extension
- Remove unnecessary DNS resolve on agent addr when incluster feature is
  enabled in mirrord-kube.


## [3.40.0](https://github.com/metalbear-co/mirrord/tree/3.40.0) - 2023-05-01


### Added

- Add a message informing users of the operator when they impersonate
  deployments with mirrord.
  [#add-operator-message](https://github.com/metalbear-co/mirrord/issues/add-operator-message)
- Last selected target is now remembered in VS Code and shown first in the
  quick pick widget.
  [#1348](https://github.com/metalbear-co/mirrord/issues/1348)


### Fixed

- PyCharm plugin now detects `pydevd` debugger and properly excludes its port.
  [#1020](https://github.com/metalbear-co/mirrord/issues/1020)
- VS Code extension now detects `debugpy` debugger and properly excludes its
  port. [#1145](https://github.com/metalbear-co/mirrord/issues/1145)
- Fixed delve patch not working on GoLand macOS when running go tests
  [#1364](https://github.com/metalbear-co/mirrord/issues/1364)
- Fixed issues when importing some packages in Python caused by PYTHONPATH to
  be used from the remote pod (add it to exclude)


### Internal

- Added Clippy lint for slicing and indexing.
  [#1049](https://github.com/metalbear-co/mirrord/issues/1049)
- Eliminate unused variable warnings for E2E tests on macOS.


## [3.39.1](https://github.com/metalbear-co/mirrord/tree/3.39.1) - 2023-04-21


### Changed

- Updated IntelliJ usage gif.


### Fixed

- Add magic fix (by polling send_request) to (connection was not ready) hyper
  error. Also adds some more logs around HTTP stealer.
  [#1302](https://github.com/metalbear-co/mirrord/issues/1302)


### Internal

- Fix arduino/setup-protoc rate limiting error.


## [3.39.0](https://github.com/metalbear-co/mirrord/tree/3.39.0) - 2023-04-19


### Added

- Support for Node.js on IntelliJ - run/debug JavaScript scripts on IntelliJ
  with mirrord. [#1284](https://github.com/metalbear-co/mirrord/issues/1284)


### Fixed

- Use RemoteFile ops in gethostname to not have a local fd.
  [#1202](https://github.com/metalbear-co/mirrord/issues/1202)


### Internal

- Fix latest tag
- Project build instructions in the testing guide now include the protoc
  dependency.


## [3.38.1](https://github.com/metalbear-co/mirrord/tree/3.38.1) - 2023-04-19


### Fixed

- Release action should work now.

### Internal

- Add protobuf-compiler to rust docs action

## [3.38.0](https://github.com/metalbear-co/mirrord/tree/3.38.0) - 2023-04-18


### Added

- Add support for cri-o container runtime.
  [#1258](https://github.com/metalbear-co/mirrord/issues/1258)
- A descriptive message is now presented in the IntelliJ extension when no
  target is available. Listing targets failure is now handled and an error
  notification is presented.
  [#1267](https://github.com/metalbear-co/mirrord/issues/1267)
- Added waitlist registration via cli.
  Join the waitlist to try out first mirrord for Teams which is invite only at
  the moment. [#1303](https://github.com/metalbear-co/mirrord/issues/1303)
- Add email option to help messages.
  [#1318](https://github.com/metalbear-co/mirrord/issues/1318)


### Changed

- When patching for SIP, use arm64 if possible (running on aarch64 and an arm64
  binary is available).
  [#1155](https://github.com/metalbear-co/mirrord/issues/1155)
- Changed our Discord invite link to https://discord.gg/metalbear


### Fixed

- Change detour bypass to be more robust, not crashing in case it can't update
  the bypass [#1320](https://github.com/metalbear-co/mirrord/issues/1320)


### Internal

- Added integration tests for outgoing UDP and TCP.
  [#1051](https://github.com/metalbear-co/mirrord/issues/1051)
- All Kubernetes resources are now deleted after E2E tests. Use
  `MIRRORD_E2E_PRESERVE_FAILED` environment variable to preserve resources from
  failed tests. All resources created for E2E tests now share a constant label
  `mirrord-e2e-test-resource=true`.
  [#1256](https://github.com/metalbear-co/mirrord/issues/1256)
- Added a debugging guide for the IntelliJ extension.
  [#1278](https://github.com/metalbear-co/mirrord/issues/1278)
- Add `impersonate` permission on `userextras/accesskeyid`, `userextras/arn`,
  `userextras/canonicalarn` and `userextras/sessionname` resources to operator
  setup.
- Sometimes when using console logger mirrord crashes since tokio runtime isn't
  initialized, changed to just use a thread


## [3.37.0](https://github.com/metalbear-co/mirrord/tree/3.37.0) - 2023-04-14


### Removed

- Removed armv7 builds that were wrongly added


### Added

- Add `ignore_ports` to `incoming` configuration so you can have ports that
  only listen
  locally (mirrord will not steal/mirror those ports).
  [#1295](https://github.com/metalbear-co/mirrord/issues/1295)
- Add support for `xstatfs` to prevent unexpected behavior with SQLite.
  [#1270](https://github.com/metalbear-co/mirrord/issues/1270)


### Changed

- Improved bad target error
  [#1291](https://github.com/metalbear-co/mirrord/issues/1291)


### Internal

- Optimize agent Dockerfile for better cache use
  [#1280](https://github.com/metalbear-co/mirrord/issues/1280)
- Cover more areas of the code and targets using clippy in CI and fix its
  warnings
- Rely more on Rusts own async trait and drop async-trait crate (the agent cant
  fully switch yet though).
  [#use-rust-async-traits](https://github.com/metalbear-co/mirrord/issues/use-rust-async-traits)


## [3.36.0](https://github.com/metalbear-co/mirrord/tree/3.36.0) - 2023-04-13


### Added

- Notify clients about errors happening in agent's background tasks.
  [#1163](https://github.com/metalbear-co/mirrord/issues/1163)
- Add support for the imagePullSecrets parameter on the agent pod. This can be
  specified in the configuration file, under agent.image_pull_secrets.
  [#1276](https://github.com/metalbear-co/mirrord/issues/1276)


### Internal

- Fix pause E2E test.
  [#1261](https://github.com/metalbear-co/mirrord/issues/1261)


## [3.35.0](https://github.com/metalbear-co/mirrord/tree/3.35.0) - 2023-04-11


### Added

- Added an error prompt to the VS Code extension when there is no available
  target in the configured namespace.
  [#1266](https://github.com/metalbear-co/mirrord/issues/1266)


### Changed

- HTTP traffic stealer now supports HTTP/2 requests.
  [#922](https://github.com/metalbear-co/mirrord/issues/922)


### Fixed

- Executable field was set to null if present, but no SIP patching was done.
  [#1271](https://github.com/metalbear-co/mirrord/issues/1271)
- Fixed random crash in `close_layer_fd` caused by supposed closing of
  stdout/stderr then calling to log that writes to it


### Internal

- Use DashMap for `OPEN_DIRS`
  [#1240](https://github.com/metalbear-co/mirrord/issues/1240)
- Use DashMap for `MANAGED_ADDRINFO`
  [#1241](https://github.com/metalbear-co/mirrord/issues/1241)
- Use DashMap for `ConnectionQueue`
  [#1242](https://github.com/metalbear-co/mirrord/issues/1242)
- Implemented `Default` for `Subscriptions`. Replaced usages of
  `Subscriptions::new` with `Default::default`.
- Improve testing guide.
- Removed unnecessary trait bounds for `Default` implementation on
  `IndexAllocator`. Replaced usages of `IndexAllocator::new` with
  `Default::default`.
- Update contributing guide.
- Update testing and building docs, and add instructions for the IDE
  extensions.


## [3.34.0](https://github.com/metalbear-co/mirrord/tree/3.34.0) - 2023-03-30


### Added

- Support for running SIP binaries via the vscode extension, for common
  configuration types.
  [#1061](https://github.com/metalbear-co/mirrord/issues/1061)


### Changed

- Add the failed connection address on failure to debug easily
- New IntelliJ icons - feel free to give feedback


### Fixed

- Fix internal proxy receiving signals from terminal targeted for the mirrord
  process/parent process by using setsid
  [#1232](https://github.com/metalbear-co/mirrord/issues/1232)
- fix listing pods failing when config file exists on macOS
  [#1245](https://github.com/metalbear-co/mirrord/issues/1245)


### Internal

- Use DashMap instead of Mutex<HashMap> for `SOCKETS`
  [#1239](https://github.com/metalbear-co/mirrord/issues/1239)
- Some small changes to make building the JetBrains plugin locally simpler.
- Update IntelliJ dependencies
- Update dependencies
- Update rust and remove unneccessary feature.


## [3.33.1](https://github.com/metalbear-co/mirrord/tree/3.33.1) - 2023-03-28


### Changed

- Add default requests and limits values to mirrord-operator setup
  (100m/100Mi).


### Fixed

- Change CLI's version update message to display the correct command when
  mirrord has been installed with homebrew.
  [#1194](https://github.com/metalbear-co/mirrord/issues/1194)
- fix using config with WSL on JetBrains
  [#1210](https://github.com/metalbear-co/mirrord/issues/1210)
- Fix internal proxy exiting before IntelliJ connects to it in some situations
  (maven). Issue was parent process closing causing child to exit. Fixed by
  waiting from the extension call to the child.
  [#1211](https://github.com/metalbear-co/mirrord/issues/1211)
- mirrord-cli: update cli so failing to use operator will fallback to
  no-operator mode.
  [#1218](https://github.com/metalbear-co/mirrord/issues/1218)
- Add option to install specific version using the `install.sh` script via
  command line argument or `VERSION` environment variable
  [#1222](https://github.com/metalbear-co/mirrord/issues/1222)
- Change connection reset to be a trace message instead of error
- Error when agent exits.


### Internal

- Bring the testing documentation into the repo, link it in readme, and add
  some information.
- Introduce CheckedInto trait to convert raw pointers (checking for null) in
  Detour values.
  [#detours](https://github.com/metalbear-co/mirrord/issues/detours)
- Re-enable http mirror e2e tests..
  [#947](https://github.com/metalbear-co/mirrord/issues/947)
- Change OPEN_FILES from Mutex HashMap to just using DashMap.
  [#1206](https://github.com/metalbear-co/mirrord/issues/1206)
- Refactor file ops open/read/close to allow us to directly manipulate the
  remote file (in agent) withouht going through C (mainly used to not leak the
  remote file due to how gethostname works).

  Change dup to take an argument that signals if we should change the fd from
  SOCKETS to OPEN_FILES (or vice-versa).
  [#1202](https://github.com/metalbear-co/mirrord/issues/1202)




## [3.33.0](https://github.com/metalbear-co/mirrord/tree/3.33.0) - 2023-03-22


### Added

- Support for outgoing unix stream sockets (configurable via config file or
  environment variable).
  [#1105](https://github.com/metalbear-co/mirrord/issues/1105)
- Add  version of hooked functions.
  [#1203](https://github.com/metalbear-co/mirrord/issues/1203)


### Changed

- add `Hash` trait on `mirrord_operator::license::License` struct
- dependencies bump and cleanup
- fix mirrord loading twice (to build also) and improve error message when no
  pods found


### Fixed

- fix f-stream functions by removing its hooks and add missing underlying libc
  calls [#947](https://github.com/metalbear-co/mirrord/issues/947)
- fix deadlock in go20 test (remove trace?)
  [#1206](https://github.com/metalbear-co/mirrord/issues/1206)


### Internal

- set timeout for flaky/hanging test


## [3.32.3](https://github.com/metalbear-co/mirrord/tree/3.32.3) - 2023-03-19


### Changed

- change outgoing connection drop to be trace instead of error since it's not
  an error


### Fixed

- Support stealing on meshed services with ports specified in
  --skip-inbound-ports on linkerd and itsio equivalent.
  [#1041](https://github.com/metalbear-co/mirrord/issues/1041)


## [3.32.2](https://github.com/metalbear-co/mirrord/tree/3.32.2) - 2023-03-14


### Fixed

- fix microk8s support by adding possible containerd socket path
  [#1186](https://github.com/metalbear-co/mirrord/issues/1186)
- fix gethostname null termination missing
  [#1189](https://github.com/metalbear-co/mirrord/issues/1189)
- Update webbrowser dependency to fix security issue.


## [3.32.1](https://github.com/metalbear-co/mirrord/tree/3.32.1) - 2023-03-12


### Fixed

- fix mirroring not handling big requests - increase buffer size (in rawsocket
  dependency).
  also trace logs to not log the data.
  [#1178](https://github.com/metalbear-co/mirrord/issues/1178)
- fix environment regression by mixing the two approaches together.
  priority is proc > oci (via container api)
  [#1180](https://github.com/metalbear-co/mirrord/issues/1180)


### Internal

- compile/test speed improvements

  1. add CARGO_NET_GIT_FETCH_WITH_CLI=true to agent's Dockerfile since we found
  out it
      saves a lot of time on fetching (around takes 60s when using libgit2)
  2. change `rust-toolchain.toml` so it won't auto install unneeded targets
  always
  3. remove `toolchain: nightly` parameter from `actions-rs/toolchain@v1` since
  it's
      not needed because we have `rust-toolchain.toml`
      saves a lot of time on fetching (takes around 60s when using libgit2)
  4. switch to use `actions-rust-lang/setup-rust-toolchain@v1` instead of
  `actions-rs/toolchain@v1`
      since it's deprecated and doesn't support `rust-toolchain.toml`
  5. remove s`Swatinem/rust-cache@v2` since it's included in
  `actions-rust-lang/setup-rust-toolchain@v1`
  6. use latest version of `Apple-Actions/import-codesign-certs` to remove
  warnings
- print logs of stealer/sniffer start failure
- run docker/containerd runtime at the same time to make e2e faster
- use base images for agent to reduce build time


## [3.32.0](https://github.com/metalbear-co/mirrord/tree/3.32.0) - 2023-03-08


### Changed

- mirrord-layer: changed result of `getsockname` to return requested socket on
  `bind` instead of the detoured socket address
  [#1047](https://github.com/metalbear-co/mirrord/issues/1047)
- mirrord-layer: Added `SocketId` to `UserSocket` as a better way of
  identifying sockets, part of #1054.
  [#1054](https://github.com/metalbear-co/mirrord/issues/1054)
- CHANGELOG - changed to use towncrier
- Change socket error on reading from outgoing sockets and mirror to be info
  instead of error


### Fixed

- Possible bug when bound address is bypassed and socket stays in `SOCKETS`
  map.


### Internal

- Change release.yaml so pushing final tags will occur only on real releases
  while manual releases will push into `ghcr.io/metalbear-co/mirrord-staging:
  ${{ github.sha }}`
  so we can leverage github CI for testing images.
- Don't build builder image as part of the build, use a prebuilt image -
  improve cd time
  Use `taiki-e/install-action` instead of `cargo install` (compiles from
  source) for installing `cross`.
- Fix broken aarch build


## 3.31.0

### Added

- config: `ignore_localhost` to `outgoing` config for ignoring localhost connections, meaning it will connect to local
  instead of remote localhost.
- config: `ignore_localhost` to `incoming` config for ignoring localhost bound sockets, meaning it will not steal/mirror those.
- combination of `ignore_localhost` in `incoming` and `outgoing` can be useful when you run complex processes that does
  IPC over localhost.
- `sip_binaries` to config file to allow specifying SIP-protected binaries that needs to be patched
  when mirrord doesn't detect those. See [#1152](https://github.com/metalbear-co/mirrord/issues/1152).

### Fixed

- Unnecessary error logs when running a script that uses `env` in its shebang.
- VSCode extension: running Python script with debugger fails because it tries to connect to the debugger port remotely.
- Big file leading to timeout: we found out that `bincode` doesn't do so well with large chunked messages
  so we limited remote read size to 1 megabyte, and read operation supports getting partial data until EOF.
- mirrord-operator: fix silent fail when parsing websocket messages fails.

### Changed

- improved mirrord cli help message.
- mirrord-config: Change `flush_connections` default to `true`, related to
  [#1029](https://github.com/metalbear-co/mirrord/issues/1029).

## 3.30.0

### Added

- mirrord-layer: Added `port_mapping` under `incoming` configuration to allow mapping local ports to custom
  remote port, for example you can listen on port 9999 locally and it will steal/mirror
  the remote 80 port if `port_mapping: [[9999, 80]]`. See [#1129](https://github.com/metalbear-co/mirrord/issues/1129)

### Fixed

- Fix issue when two (or more) containerd sockets exist and we use the wrong one. Fixes [#1133](https://github.com/metalbear-co/mirrord/issues/1133).
- Invalid toml in environment variables configuration examples.

### Changed

- Use container's runtime env instead of reading it from `/proc/{container_root_pid}/environ` as some processes (such as nginx) wipe it. Fixes [#1135](https://github.com/metalbear-co/mirrord/issues/1135)
- Removed the prefix "test" from all test names - [#1065](https://github.com/metalbear-co/mirrord/issues/1065).
- Created symbolic link from the vscode directory to the `LICENSE` and `CHANGELOG.md` files so that mirrord developers
  don't need to copy them there before building the app.
- mirrord-layer: `socket` hook will now block ipv6 requests and will return EAFNOSUPPORT. See [#1121](https://github.com/metalbear-co/mirrord/issues/1121).

## 3.29.0

### Added

- mirrord debug feature (for mirrord developers to debug mirrord): Cause the agent to exit early with an error.
- mirrord E2E tests: support for custom namespaces.

### Fixed

- Unpause the target container before exiting if the agent exits early on an error and the container is paused -
   [#1111](https://github.com/metalbear-co/mirrord/issues/1111).
- intellij-plugin: fix issue where execution hangs when running using Gradle. Fixes [#1120](https://github.com/metalbear-co/mirrord/issues/1120).
- intellij-plugin: fix issue where mirrord doesn't load into gradle, was found when fixing [#1120](https://github.com/metalbear-co/mirrord/issues/1120).
- mirrord-agent: reintroduce `-o lo` back to iptable rules to prevent issue where outinging messags could be intersepted by mirrord as incoming ones.
- mirrord-layer: binding same port on different IPs leads to a crash due to `ListenAlreadyExists` error.
  This is now ignored with a `info` message since we can't know if the IP/Port was already bound
  or not. Created a follow up issue to complete implementation and error at application's bind.

## 3.28.4

### Fixed

- VSCode Extension: Fix wrong CLI path on Linux

## 3.28.3

### Fixed

- VSCode Extension: Fix wrong CLI path

## 3.28.2

### Fixed

- Fix error in VSCode extension compilation

## 3.28.1

### Fixed

- CI: fix error caused by missing dir

## 3.28.0

### Changed

- Change VSCode extension to package all binaries and select the correct one based on the platform. Fixes [#1101](https://github.com/metalbear-co/mirrord/issues/1101).
- agent: add log to error when handling a client message fails.

### Fixed

- agent: Make sniffer optional to support cases when it's not available and mirroring is not required.

## 3.27.1

### Changed

- Update operator version

## 3.27.0

### Fixed

- mirrord now handles it when the local app closes a forwarded stolen tcp connection instead of exiting with an error. Potential fix for [#1063](https://github.com/metalbear-co/mirrord/issues/1063).
- missing kubeconfig doesn't fail extensions (it failed because it first tried to resolve the default then used custom one)

### Changed

- layer: Don't print error when tcp socket faces error as it can be a normal flow.
- internal proxy - set different timeout for `mirrord exec` and running from extension
  fixing race conditions when running from IntelliJ/VSCode.
- Changed `with_span_events` from `FmtSpan::Active` to `FmtSpan::NEW | FmtSpan::CLOSE`.
  Practically this means we will have less logs on enter/exit to span and only when it's first created
  and when it's closed.
- JetBrains Plugin: Add debug logs for investigating user issues.
- JetBrains compatability: set limit from 222 (2022.2.4) since 221 isn't supported by us.
- Make `kubeconfig` setting effective always by using `-f` in `mirrord ls`.
- mirrord agent can now run without sniffer, will not be able to mirror but can still steal.
  this is to enable users who have older kernel (4.20>=) to use the steal feature.

## 3.26.1

### Fixed

- VSCode Extension: Prevent double prompting of the user to select the target if not specified in config. See [#1080](https://github.com/metalbear-co/mirrord/issues/1080).

### Changed

- JetBrains enable support from 2021.3 (like we had in 3.14.3).

## 3.26.0

### Changed

- mirrord-agent: localhost traffic (like healthprobes) won't be stolen by mirrord on meshed targets to allign behavior with non meshed targets. See [#1070](https://github.com/metalbear-co/mirrord/pull/1070)
- Filter out agent pods from `mirrord ls`, for better IDE UX. Closes [#1045](https://github.com/metalbear-co/mirrord/issues/1045).
- Not exiting on SIP-check fail. Instead, logging an error and letting the program fail as it would without mirrord.
  See [#951](https://github.com/metalbear-co/mirrord/issues/951).

### Fixed

- Fix cache does not work on test-agent workflow. See [#251](https://github.com/metalbear-co/mirrord/issues/251).
- CI: merge queue + branch protection issues

## 3.25.0

### Added

- `gethostname` detour that returns contents of `/etc/hostname` from target pod. See relevant [#1041](https://github.com/metalbear-co/mirrord/issues/1041).

### Fixed

- `getsockname` now returns the **remote** local address of the socket, instead of the
  **local fake** address of the socket.
  This should fix issues with Akka or other software that checks the local address and
  expects it to match the **local ip of the pod**.
  This breaks agent protocol (agent/layer need to match).
- GoLand debug fails because of reading `/private/var/folders` remotely (trying to access self file?). fixed with filter change (see below)

### Changed

- VSCode extension: update dialog message
- JetBrains: can now change focus from search field to targets using tab/shift+tab (for backwrad)
- Refactor - mirrord cli now spawns `internal proxy` which does the Kubernetes operations for
  the layer, so layer need not interact with k8s (solves issues with remote/local env mix)
- filter: add `/private/var/folders" to default local read override
- filter: fixed regex for `/tmp` default local read override
- disable flask e2e until we solve the glibc issue (probably fstream issue)

## 3.24.0

### Added

- Add a field to mirrord-config to specify custom path for kubeconfig , resolves [#1027](https://github.com/metalbear-co/mirrord/issues/1027).

### Changed

- Removed limit on future builds `untilBuild` in JetBrains plugin.
- IntelliJ-ext: change the dialog to provide a sorted list and make it searchable, resolves [#1031](https://github.com/metalbear-co/mirrord/issues/1031).
- mirrord-layer: Changed default to read AWS credentials + config from remote pod.
- Removed unused env var (`MIRRORD_EXTERNAL_ENV`)
- mirrord-agent: Use `conntrack` to flush stealer connections (temporary fix for
  [#1029](https://github.com/metalbear-co/mirrord/issues/1029)).

### Fixed

- Added env guard to be used in cli + extension to prevent (self) misconfigurations (our kube settings being used from remote).

## 3.23.0

### Fixed

- mirrord-config: Fix disabled feature for env in config file, `env = false` should work. See [#1015](https://github.com/metalbear-co/mirrord/issues/1015).
- VS Code extension: release universal extension as a fallback for Windows and other platforms to be used with WSL/Remote development. Fixes [#1017](https://github.com/metalbear-co/mirrord/issues/1017)
- Fix `MIRRORD_AGENT_RUST_LOG` can't be more than info due to dependency on info log.
- Fix pause feature not working in extension due to writing to stdout (changed to use trace)

### Changed

- `DNSLookup` failures changed to be info log from error since it is a common case.
- mirrord-agent: now prints "agent ready" instead of logging it so it can't be fudged with `RUST_LOG` control.
- mirrord-agent: `agent::layer_recv` changed instrumentation to be trace instead of info.
- mirrord-layer/agent: change ttl of job to be 1 second for cases where 0 means in cluster don't clean up.
- Convert go fileops e2e tests into integration tests. Part of
  [#994](https://github.com/metalbear-co/mirrord/issues/994#issuecomment-1410721960).

## 3.22.0

### Changed

- Rust: update rust toolchain (and agent rust `DOCKERFILE`) to `nightly-2023-01-31`.
- exec/spawn detour refactor [#999](https://github.com/metalbear-co/mirrord/issues/999).
- mirrord-layer: Partialy load mirrord on certian processes that spawn other processes to allow sip patch on the spawned process.
  This to prevent breaking mirrord-layer load if parent process is specified in `--skip-processes`.  (macOS only)

### Fixed

- mirrord-layer: DNS resolving doesn't work when having a non-OS resolver (using UDP sockets)
  since `/etc/resolv.conf` and `/etc/hosts` were in the local read override,
  leading to use the local nameserver for resolving. Fixes [#989](https://github.com/metalbear-co/mirrord/issues/989)
- mirrord-agent: Infinite reading a file when using `fgets`/`read_line` due to bug seeking to start of file.
- Rare deadlock on file close that caused the e2e file-ops test to sometimes fail
  ([#994](https://github.com/metalbear-co/mirrord/issues/994)).

## 3.21.0

### Added

- Support for Go's `os.ReadDir` on Linux (by hooking the `getdents64` syscall). Part of
  [#120](https://github.com/metalbear-co/mirrord/issues/120).
- Test mirrord with Go 1.20rc3.

### Changed

- mirrord-agent: Wrap agent with a parent proccess to doublecheck the clearing of iptables. See [#955](https://github.com/metalbear-co/mirrord/issues/955)
- mirrord-layer: Change `HOOK_SENDER` from `Option` to `OnceLock`.

### Fixed

- mirrord-agent: Handle HTTP upgrade requests when the stealer feature is enabled
  (with HTTP traffic) PR [#973](https://github.com/metalbear-co/mirrord/pull/973).
- E2E tests compile on MacOS.
- mirrord could not load into some newer binaries of node -
  [#987](https://github.com/metalbear-co/mirrord/issues/987). Now hooking also `posix_spawn`, since node now uses
  `libuv`'s `uv_spawn` (which in turn calls `posix_spawn`) instead of libc's `execvp` (which calls `execve`).
- Read files from the temp dir (defined by the system's `TMPDIR`) locally, closes
  [#986](https://github.com/metalbear-co/mirrord/issues/986).

## 3.20.0

### Added

- Support impersonation in operator

### Fixed

- Go crash in some scenarios [#834](https://github.com/metalbear-co/mirrord/issues/834).
- Remove already deprecated `--no-fs` and `--rw` options, that do not do anything anymore, but were still listed in the
  help message.
- Bug: SIP would fail the second time to run scripts for which the user does not have write permissions.

### Changed

- Change layer/cli logs to be to stderr instead of stdout to avoid mixing with the output of the application. Closes [#786](https://github.com/metalbear-co/mirrord/issues/786)

## 3.19.2

### Changed

- Code refactor: moved all file request and response types into own file.

## 3.19.1

### Fixed

- Changelog error failing the JetBrains release.

## 3.19.0

### Changed

- mirrord-operator: replace operator api to use KubernetesAPI extension. [#915](https://github.com/metalbear-co/mirrord/pull/915)

### Fixed

- tests: flaky passthrough fix. Avoid 2 agents running at the same time, add minimal sleep (1s)
- macOS x64/SIP(arm): fix double hooking `fstatat$INODE64`. Possible crash and undefined behavior.

### Added

- introduce `mirrord-console` - a utility to debug and investigate mirrord issues.

### Deprecated

- Remove old fs mode
  - cli: no `--rw` or `--no-fs`.
  - layer: no `MIRRORD_FILE_OPS`/`MIRRORD_FILE_RO_OPS`/`MIRRORD_FILE_FILTER_INCLUDE`/`MIRRORD_FILE_FILTER_EXCLUDE`

## 3.18.2

### Fixed

- crash when `getaddrinfo` is bypassed and libc tries to free our structure. Closes [#930](https://github.com/metalbear-co/mirrord/issues/930)
- Stealer hangs on short streams left open and fails on short closed streams to filtered HTTP ports -
 [#926](https://github.com/metalbear-co/mirrord/issues/926).

## 3.18.1

### Fixed

- Issue when connect returns `libc::EINTR` or `libc::EINPROGRESS` causing outgoing connections to fail.
- config: file config updated to fix simple pattern of IncomingConfig. [#933](https://github.com/metalbear-co/mirrord/pull/933)

## 3.18.0

### Added

- Agent now sends error encountered back to layer for better UX when bad times happen. (This only applies to error happening on connection-level).
- Partial ls flow for Go on macOS (implemented `fdopendir` and `readdir_r`). Closes [#902](https://github.com/metalbear-co/mirrord/issues/902)
- New feature: HTTP traffic filter!
  - Allows the user to steal HTTP traffic based on HTTP request headers, for example `Client: me` would steal requests that match this header,
    while letting unmatched requests (and non-HTTP packets) through to their original destinations.

### Fixed

- Update the setup-qemu-action action to remove a deprecation warning in the Release Workflow
- stat functions now support directories.
- Possible bugs with fds being closed before time (we now handle dup'ing of fds, and hold those as ref counts)

### Changed

- agent: Return better error message when failing to use `PACKET_IGNORE_OUTGOING` flag.

## 3.17.0

### Added

- Add brew command to README

### Fixed

- intellij plugin: mirrord icon should always load now.
- intellij plugin: on target selection cancel, don't show error - just disable mirrord for the run and show message.
- fixed setting a breakpoint in GoLand on simple app hanging on release build (disabled lto). - Fixes [#906](https://github.com/metalbear-co/mirrord/issues/906).

### Deprecated

- Removed `disabled` in favor of `local` in `fs` configuration.

### Changed

- update `kube` dependency + bump other
- update `dlv` packed with plugins.

## 3.16.2

### Fixed

- Add go to skipped processes in JetBrains plugin. Solving GoLand bug.

## 3.16.1

### Fixed

- Running on specific Kubernetes setups, such as Docker for Desktop should work again.

## 3.16.0

### Added

- Add golang stat hooks, closes [#856](https://github.com/metalbear-co/mirrord/issues/856)

### Fixed

- agent: mount /var from host and reconfigure docker socket to /var/run/docker.sock for better compatibility
- Error on specifying namespace in configuration without path (pod/container/deployment). Closes [#830](https://github.com/metalbear-co/mirrord/issues/830)
- IntelliJ plugin with new UI enabled now shows buttons. Closes [#881](https://github.com/metalbear-co/mirrord/issues/881)
- Fix deprecation warnings (partially), update checkout action to version 3.

### Changed

- Refactored detours to use new helper function `Result::as_hook` to simplify flow. (no change in behavior)

## 3.15.2

### Added

- Logging for IntelliJ plugin for debugging/bug reports.

### Fixed

- Crash when mirroring and state is different between local and remote (happens in Mesh).
  We now ignore messages that are not in the expected state. (as we can't do anything about it).
- agent: Fix typo in socket path for k3s environments
- intellij-plugin: fix missing telemetry/version check

## 3.15.1

### Added

- Add `__xstat` hook, fixes [#867]((https://github.com/metalbear-co/mirrord/issues/867))

### Fixed

- Fix build scripts for the refactored IntelliJ plugin

## 3.15.0

### Added

- agent: Add support for k3s envs
- IntelliJ plugin - refactor, uses cli like vs code.

### Fixed

- getaddrinfo: if node is NULL just bypass, as it's just for choosing ip/port, Fixes[#858](https://github.com/metalbear-co/mirrord/issues/858) and [#848](https://github.com/metalbear-co/mirrord/issues/848)

### Changed

- cli now loads env, removes go env stuff at load, might fix some bugs there.

## 3.14.3

### Fixed

- Create empty release to overcome temporary issue with VS Code marketplace publication

## 3.14.2

### Fixed

- vscode ext: use process env for running mirrord. Fixes [#854](https://github.com/metalbear-co/mirrord/issues/854)

## 3.14.1

### Fixed

- layer + go - connect didn't intercept sometimes (we lacked a match). Fixes [851](https://github.com/metalbear-co/mirrord/issues/851).

## 3.14.0

### Changed

- cli: Set environment variables from cli to spawned process instead of layer when using `mirrord exec`.
- cli: use miette for nicer errors
- cli: some ext exec preparations, nothing user facing yet.
- vs code ext: use cli, fixes some env bugs with go and better user experience.

## 3.13.5

### Changed

- Don't add temp prefix when using `extract` command.
- VS Code extension: mirrord enable/disable to be per workspace.
- VS Code extension: bundle the resources
- Add `/System` to default ignore list.
- Remove `test_mirrord_layer` from CI as it's covered in integration testing.

### Fixed

- fd leak on Linux when using libuv (Node). Caused undefined behavior. Fixes [#757](https://github.com/metalbear-co/mirrord/issues/757).

### Misc

- Better separation in mirrord cli.

## 3.13.4

### Changed

- Adjust filters - all directory filters also filter the directory itself (for when lstat/stating the directory).
  Added `/Applications`

## 3.13.3

### Added

- Add `mirrord ls` which allows listing target path. Hidden from user at the moment, as for now it's meant for extension use only.

### Changed

- Refactor e2e tests: split into modules based on functionality they test.
- internal refactor in mirrord-agent: Stealer feature changed from working per connection to now starting with
  the agent itself ("global"). Got rid of `steal_worker` in favor of a similar abstraction to what
  we have in `sniffer.rs` (`TcpConnectionStealer` that acts as the traffic stealing task, and
  `TcpStealerApi` which bridges the communication between the agent and the stealer task).
- Tests CI: don't wait for integration tests to start testing E2E tests.

### Fixed

- Add missing `fstat`/`lstat`/`fstatat`/`stat` hooks.

## 3.13.2

### Fixed

- Weird crash that started happening after Frida upgrade on macOS M1.

## 3.13.1

### Fixed

- Fix asdf:
  - Add `/tmp` not just `/tmp/` to exclusion.
  - Add `.tool-version` to exclusion.
  - `fclose` was calling close which doesn't flush.

## 3.13.0

### Changed

- IntelliJ Plugin: downgrade Java to version 11.
- IntelliJ Plugin: update platform version to 2022.3.
- Disable progress in mirrord-layer - can cause issues with forks and generally confusing now
  that agent is created by cli (and soon to be created by IDE plugin via cli).
- Update to Frida 16.0.7
- Add more paths to the default ignore list (`/snap` and `*/.asdf/*`) - to fix asdf issues.
- Add `/bin/` to default ignore list - asdf should be okay now!
- Update GitHub action to use latest `rust-cache`

### Added

- mirrord-operator: Add securityContext section for deployment in operator setup

### Fixed

- Fix `--fs-mode=local` didn't disable hooks as it was supposed to.
- Fix hooking wrong libc functions because of lack of module specification - add function to resolve
  module name to hook from (libc on Unix,libsystem on macOS). Partially fixes asdf issue.

## 3.12.1

### Added

- E2E test for pause feature with service that logs http requests and a service that makes requests.
- mirrord-layer: automatic operator discovery and connection if deployed on cluster. (Discovery can be disabled with `MIRRORD_OPERATOR_ENABLE=false`).

### Changed

- Added `/tmp/` to be excluded from file ops by default. Fixes [#800](https://github.com/metalbear-co/mirrord/issues/800).

### Misc

- Reformatted a bit the file stuff, to make it more readable. We now have `FILE_MODE` instead of `FILE_OPS_*` internally.
- Changed fileops test to also test write override (mirrord mode is read and override specific path)

## 3.12.0

### Added

- `--pause` feature (unstable). See [#712](https://github.com/metalbear-co/mirrord/issues/712).
- operator setup cli feature.
- mirrord-layer: operator connection that can be used instad of using kubernetes api to access agents.

### Changed

- CI: cancel previous runs of same PR.
- cli: set canonical path for config file to avoid possible issues when child processes change current working directory.
- config: Refactor config proc macro and behavior - we now error if a config value is wrong instead of defaulting.
- layer: panic on error instead of exiting without any message.
- CI: don't run CI on draft PRs.
- Update dependencies.
- Update to clap v4 (cli parser crate).
- Started deprecation of fsmode=disabled, use fsmode=local instead.

### Fixed

- Typo in `--agent-startup-timeout` flag.

## 3.11.2

### Fixed

- Agent dockerfile: fix build for cross arch

### Changed

- Added clippy on macOS and cleaned warnings.

## 3.11.1

### Fixed

- release.yaml: Linux AArch64 for real this time. (embedded so was x64)

### Changed

- Create agent in the cli and pass environment variables to exec'd process to improve agent re-use.
- IntelliJ: change default log level to warning (match cli/vscode).
- IntelliJ: don't show progress (can make some tests/scenarios fail).
- release.yaml: Build layer/cli with Centos 7 compatible glibc (AmazonLinux2 support).
- Change CPU/memory values requested by the Job agent to the lowest values possible.

## 3.11.0

### Added

- MacOS: Support for executing SIP binaries in user applications. We hook `execve`
  and create a SIP-free version of the binary on-the-go and execute that instead of
  the SIP binary.
  This means we now support running bash scripts with mirrord also on MacOS.
  Closes [#649](https://github.com/metalbear-co/mirrord/issues/649).

### Changed

- Only warn about invalid certificates once per agent.
- Reduce tokio features to needed ones only.

### Fixed

- CI: Fix regex for homebrew formula
- Potentially ignoring write calls (`fd < 2`).
- CI: Fix release for linux aarch64. Fixes [#760](https://github.com/metalbear-co/mirrord/issues/760).
- Possible cases where we don't close fds correctly.

## 3.10.4

### Fixed

- VS Code Extension: Fix crash when no env vars are defined in launch.json

## 3.10.3

### Changed

- CLI: change temp lib file to only be created for new versions
- mirrord-config: refactored macro so future implementations will be easier

### Fixed

- Release: fix homebrew release step

## 3.10.2

### Fixed

- CI: fix `release_gh` zip file step

## 3.10.1

### Changed

- CI: download shasums and add git username/email to make the homebrew release work
- Remove `unimplemented` for some IO cases, we now return `Unknown` instead. Also added warning logs for these cases to track.
- Only recommend `--accept-invalid-certificates` on connection errors if not already set.
- Terminate user application on connection error instead of only stopping mirrord.

## 3.10.0

### Added

- CI: Update homebrew formula on release, refer [#484](https://github.com/metalbear-co/mirrord/issues/484)

### Changed

- VS Code Extension: change extension to use the target specified in the mirrord config file, if specified, rather than show the pod dropdown

## 3.9.0

### Added

- `MIRRORD_AGENT_NETWORK_INTERFACE` environment variable/file config to let user control which network interface to use. Workaround for [#670](https://github.com/metalbear-co/mirrord/issues/670).
- mirrord-config: `deprecated` and `unstable` tags to MirrordConfg macro for messaging user when using said fields

### Changed

- VS Code Extension: change extension to use a mirrord-config file for configuration
- VS Code Extension: use the IDE's telemetry settings to determine if telemetry should be enabled

## 3.8.0

### Changed

- mirrord-layer: Remove `unwrap` from initialization functions.
- Log level of operation bypassing log from warn to trace (for real this time).
- Perform filesystem operations for paths in `/home` locally by default (for real this time).

### Added

- VS Code Extension: add JSON schema
- Bypass SIP on MacOS on the executed binary, (also via shebang).
  See [[#649](https://github.com/metalbear-co/mirrord/issues/649)].
  This does not yet include binaries that are executed by the first binary.

### Fixed

- fix markdown job by adding the checkout action

## 3.7.3

### Fixed

- mirrord-agent: No longer resolves to `eth0` by default, now we first try to resolve
  the appropriate network interface, if this fails then we use `eth0` as a last resort.
  Fixes [#670](https://github.com/metalbear-co/mirrord/issues/670).

### Changed

- intelliJ: use custom delve on macos

## 3.7.2

### Fixed

- Release: fix broken docker build step caused by folder restructure

## 3.7.1

### Fixed

- using gcloud auth for kubernetes. (mistakenly loaded layer into it)
- debugging Go on VSCode. We patch to use our own delivered delve.
- Changed layer not to crash when connection is closed by agent. Closed [#693](https://github.com/metalbear-co/mirrord/issues/693).

### Changed

- IntelliJ: fallback to using a textfield if listing namespaces fails

## 3.7.0

### Added

- mirrord-config: New `mirrord-schema.json` file that contains docs and types which should help the user write their mirrord
  config files. This file has to be manually generated (there is a test to help you remember).

### Fixed

- IntelliJ: Fix occurring of small namespace selection window and make mirrord dialogs resizable
- IntelliJ: Fix bug when pressing cancel in mirrord dialog and rerunning the application no mirrord window appears again
- VS Code: Fix crash occurring because it used deprecated env vars.

### Changed

- mirrord-config: Take `schema` feature out of feature flag (now it's always on).
- mirrord-config: Add docs for the user config types.

## 3.6.0

### Added

- mirrord-layer: Allow capturing tracing logs to file and print github issue creation link via MIRRORD_CAPTURE_ERROR_TRACE env variable

### Fixed

- Fix vscode artifacts where arm64 package was not released.
- IntelliJ plugin: if namespaces can't be accessed, use the default namespace

### Changed

- Add `/home` to default file exclude list.
- Changed log level of `Bypassing operation...` from warning to trace.
- IntelliJ settings default to match CLI/VSCode.

## 3.5.3

### Fixed

- Fixed broken release step for VS Code Darwin arm64 version

## 3.5.2

### Fixed

- Fixed breaking vscode release step

## 3.5.1

### Fixed

- Fixed an issue with the release CI

### Changed

- Update target file config to have `namespace` nested inside of `target` and not a separate `target_namespace`.
  See [#587](https://github.com/metalbear-co/mirrord/issues/587) and [#667](https://github.com/metalbear-co/mirrord/issues/667)

## 3.5.0

### Added

- aarch64 release binaries (no go support yet, no IntelliJ also).
- mirrord-layer: Add [`FileFilter`](mirrord-layer/src/file/filter.rs) that allows the user to include or exclude file paths (with regex support) for file operations.

### Changed

- mirrord-layer: Improve error message when user tries to run a program with args without `--`.
- Add tests for environment variables passed to KubeApi for authentication feature for cli credential fetch
- Remove openssl/libssl dependency, cross compilation is easier now. (It wasn't needed/used)
- mirrord-config: Changed the way [`fs`](mirrord-config/src/fs.rs) works: now it supports 2 modes `Simple` and `Advanced`,
  where `Simple` is similar to the old behavior (enables read-only, read-write, or disable file ops), and `Advanced`
  allows the user to specify include and exclude (regexes) filters for [`FileFilter`](mirrord-layer/src/file/filter.rs).
- Lint `README` and update it for `--target` flag.
- mirrord-layer: improve error message for invalid targets.

### Removed

- `--pod-name`, `--pod-namespace`, `--impersonated_container_name` have been removed in favor of `--target`, `--target-namespace`

### Fixed

- Env var to ignore ports used by a debugger for intelliJ/VSCode, refer [#644](https://github.com/metalbear-co/mirrord/issues/644)

## 3.4.0

### Added

- Add changelog for intelliJ extension, closes [#542](https://github.com/metalbear-co/mirrord/issues/542)
- Add filter for changelog to ci.yml
- Telemetry for intelliJ extension.

### Changed

- Update intelliJ extension: lint & bump java version to 17.
- Added `/Users` and `/Library` to path to ignore for file operations to improve UX on macOS.
- Use same default options as CLI in intelliJ extension.
- Improve UI layout of intelliJ extension.
- Separate tcp and udp outgoing option in intelliJ extension.
- Tighter control of witch environment variables would be passed to the KubeApi when fetching credentials via cli in kube-config. See [#637](https://github.com/metalbear-co/mirrord/issues/637)

### Fixed

- Lint Changelog and fix level of a "Changed" tag.
- File operations - following symlinks now works as expected. Previously, absolute symlinks lead to use our own path instead of target path.
  For example, AWS/K8S uses `/var/run/..` for service account credentials. In many machines, `/var/run` is symlink to `/run`
  so we were using `/run/..` instead of `/proc/{target_pid}/root/run`.
- Fix not reappearing window after pressing cancel-button in intelliJ extension.

## 3.3.0

### Added

- Telemetries, see [TELEMETRY.md](./TELEMETRY.md) for more information.

### Changed

- Added timeout for "waiting for pod to be ready..." in mirrord-layer to prevent unresponsive behavior. See [#579](https://github.com/metalbear-co/mirrord/issues/579)
- IntelliJ Extension: Default log level to `ERROR` from `DEBUG`

### Fixed

- Issue with [bottlerocket](https://github.com/bottlerocket-os/bottlerocket) where they use `/run/dockershim.sock`
  instead of the default containerd path. Add new path as fallback.

## 3.2.0

### Changed

- Extended support for both `-s` and `-x` wildcard matching, now supports `PREFIX_*`, `*_SUFFIX`, ect.
- Add to env default ignore `JAVA_HOME`,`HOMEPATH`,`CLASSPATH`,`JAVA_EXE` as it's usually runtime that you don't want
  from remote. Possibly fixes issue discussed on Discord (used complained that they had to use absolute path and not
  relative).
- Add `jvm.cfg` to default bypass for files.
- Clarify wrong target error message.
- mirrord-layer: Improve error message in `connection::handle_error`.

### Fixed

- Don't ignore passed `--pod-namespace` argument, closes
  [[#605](https://github.com/metalbear-co/mirrord/issues/605)]
- Replace deprecated environment variables in IntelliJ plugin
- Issues with IntelliJ extension when debugging Kotlin applications
- Scrollable list for pods and namespaces for IntelliJ extension,
  closes [[#610](https://github.com/metalbear-co/mirrord/issues/610)]

### Deprecated

- `--impersonated-container-name` and `MIRRORD_IMPERSONATED_CONTAINER_NAME` are
  deprecated in favor of `--target` or `MIRRORD_IMPERSONATED_TARGET`
- `--pod-namespace` and `MIRRORD_AGENT_IMPERSONATED_POD_NAMESPACE` are deprecated in
  favor of `--target-namespace` and `MIRRORD_TARGET_NAMESPACE`

## 3.1.3

### Changed

- release: VS Code extension release as stable and not pre-release.

### Fixed

- Dev container failing to execute `apt-get install -y clang`

## 3.1.2

### Changed

- Update some texts in documentation, READMEs, and extension package descriptions
- IntelliJ version check on enabling instead of on project start. Don't check again after less than 3 minutes.

## 3.1.1

### Fixed

- IntelliJ plugin crashing on run because both include and exclude were being set for env vars.

## 3.1.0

### Added

- `pwrite` hook (used by `dotnet`);

### Fixed

- Issue [#577](https://github.com/metalbear-co/mirrord/issues/577). Changed non-error logs from `error!` to `trace!`.

### Changed

- Agent pod definition now has `requests` specifications to avoid being defaulted to high values.
  See [#579](https://github.com/metalbear-co/mirrord/issues/579).
- Change VSCode extension configuration to have file ops, outgoing traffic, DNS, and environment variables turned on by
  default.
- update intelliJ extension: toggles + panel for include/exclude env vars

## 3.0.22-alpha

### Changed

- Exclude internal configuration fields from generated schema.

### Fixed

- Issue [#531](https://github.com/metalbear-co/mirrord/issues/531). We now detect NixOS/Devbox usage and add `sh` to
  skipped list.

## 3.0.21-alpha

### Added

- Reuse agent - first process that runs will create the agent and its children will be able to reuse the same one to
  avoid creating many agents.
- Don't print progress for child processes to avoid confusion.
- Skip istio/linkerd-proxy/init container when mirroring a pod without a specific container name.
- Add "linkerd.io/inject": "disabled" annotation to pod created by mirrord to avoid linkerd auto inject.
- mirrord-layer: support `-target deployment/deployment_name/container/container_name` flag to run on a specific
  container.
- `/nix/*` path is now ignored for file operations to support NixOS.
- Shortcut `deploy` for `deployment` in target argument.
- Added the ability to override environment variables in the config file.

### Changed

- Print exit message when terminating application due to an unhandled error in the layer.
- mirrord-layer: refactored `pod_api.rs` to be more maintainble.
- Use kube config namespace by default.
- mirrord-layer: Ignore `EAFNOSUPPORT` error reporting (valid scenario).

## 3.0.20-alpha

### Added

- `pread` hook (used by `dotnet`);
- mirrord-layer: ignore opening self-binary (temporal SDK calculates the hash of the binary, and it fails because it
  happens remotely)
- Layer integration tests with more apps (testing with Go only on MacOS because of
  known crash on Linux - [[#380](https://github.com/metalbear-co/mirrord/issues/380)]).
  Closes [[#472](https://github.com/metalbear-co/mirrord/issues/472)].
- Added progress reporting to the CLI.
- CI: use [bors](https://bors.tech/) for merging! woohoo.

### Changed

- Don't report InProgress io error as error (log as info)
- mirrord-layer: Added some `dotnet` files to `IGNORE_FILES` regex set;
- mirrord-layer: Added the `Detour` type for use in the `ops` modules instead of `HookResult`. This type supports
  returning a `Bypass` to avoid manually checking if a hook actually failed or if we should just bypass it;
- mirrord-protocol: Reduce duplicated types around `read` operation;
- Layer integration tests for more apps. Closes
  [[#472](https://github.com/metalbear-co/mirrord/issues/472)].
- Rename http mirroring tests from `integration` to `http_mirroring` since there are
  now also integration tests in other files.
- Delete useless `e2e_macos` CI job.
- Integration tests also display test process output (with mirrord logs) when they
  time out.
- CI: mirrord-layer UT and integration run in same job.
- .devcontainer: Added missing dependencies and also kind for running e2e tests.

### Fixed

- Fix IntelliJ Extension artifact - use glob pattern
- Use LabelSelector instead of app=* to select pods from deployments
- Added another
  protection [to not execute in child processes from k8s auth](https://github.com/metalbear-co/mirrord/issues/531) by
  setting an env flag to avoid loading then removing it after executing the api.

## 3.0.19-alpha

### Added

- Release image for armv7 (Cloud ARM)

### Fixed

- Release for non-amd64 arch failed because of lack of QEMU step in the github action. Re-added it

## 3.0.18-alpha

### Changed

- Replaced `pcap` dependency with our own `rawsocket` to make cross compiling faster and easier.

## 3.0.17-alpha

### Fixed

- Release CI: Remove another failing step

## 3.0.16-alpha

### Fixed

- Release CI: Temporarily comment out failing step

## 3.0.15-alpha

### Fixed

- Release CI: Fix checkout action position in intelliJ release.

## 3.0.14-alpha

### Added

- Layer integration test. Tests the layer's loading and hooking in an http mirroring simulation with a flask web app.
  Addresses but does not
  close [[#472](https://github.com/metalbear-co/mirrord/issues/472)] (more integration tests still needed).

### Fixed

- Release CI: Fix paths for release artifacts

## 3.0.13-alpha

### Added

- mirrord-cli: added a SIP protection check for macos binaries,
  closes [[#412](https://github.com/metalbear-co/mirrord/issues/412)]

### Fixed

- Fixed unused dependencies issue, closes [[#494](https://github.com/metalbear-co/mirrord/issues/494)]

### Changed

- Remove building of arm64 Docker image from the release CI

## 3.0.12-alpha

### Added

- Release CI: add extensions as artifacts, closes [[#355](https://github.com/metalbear-co/mirrord/issues/355)]

### Changed

- Remote operations that fail logged on `info` level instead of `error` because having a file not found, connection
  failed, etc can be part of a valid successful flow.
- mirrord-layer: When handling an outgoing connection to localhost, check first if it's a socket we intercept/mirror,
  then just let it connect normally.
- mirrord-layer: removed `tracing::instrument` from `*_detour` functions.

### Fixed

- `getaddrinfo` now uses [`trust-dns-resolver`](https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/) when
  resolving DNS (previously it would do a `getaddrinfo` call in mirrord-agent that could result in incompatibility
  between the mirrored pod and the user environments).
- Support clusters running Istio. Closes [[#485](https://github.com/metalbear-co/mirrord/issues/485)].

## 3.0.11-alpha

### Added

- Support impersonated deployments, closes [[#293](https://github.com/metalbear-co/mirrord/issues/293)]
- Shorter way to select which deployment/pod/container to impersonate through `--target`
  or `MIRRORD_IMPERSONATED_TARGET`, closes [[#392](https://github.com/metalbear-co/mirrord/issues/392)]
- mirrord-layer: Support config from file alongside environment variables.
- intellij-ext: Add version check, closes [[#289](https://github.com/metalbear-co/mirrord/issues/289)]
- intellij-ext: better support for Windows with WSL.

### Deprecated

- `--pod-name` or `MIRRORD_AGENT_IMPERSONATED_POD_NAME` is deprecated in favor of `--target`
  or `MIRRORD_IMPERSONATED_TARGET`

### Fixed

- tcp-steal working with linkerd meshing.
- mirrord-layer should exit when agent disconnects or unable to make initial connection

## 3.0.10-alpha

### Added

- Test that verifies that outgoing UDP traffic (only with a bind to non-0 port and a
  call to `connect`) is successfully intercepted and forwarded.

### Fixed

- macOS binaries should be okay now.

## 3.0.9-alpha

### Changed

- Ignore http tests because they are unstable, and they block the CI.
- Bundle arm64 binary into the universal binary for MacOS.

## 3.0.8-alpha

### Fixed

- release CI: Fix dylib path for `dd`.

## 3.0.7-alpha

### Fixed

- mirrord-layer: Fix `connect` returning error when called on UDP sockets and the
  outgoing traffic feature of mirrord is disabled.
- mirrord-agent: Add a `tokio::time:timeout` to `TcpStream::connect`, fixes golang issue where sometimes it would get
  stuck attempting to connect on IPv6.
- intelliJ-ext: Fix CLion crash issue, closes [[#317](https://github.com/metalbear-co/mirrord/issues/317)]
- vscode-ext: Support debugging Go, and fix issues with configuring file ops and traffic stealing.

### Changed

- mirrord-layer: Remove check for ignored IP (localhost) from `connect`.
- mirrord-layer: Refactor `connect` function to be less bloated.
- `.dockerignore` now ignores more useless files (reduces mirrord-agent image build time, and size).
- mirrord-agent: Use `tracing::instrument` for the outgoing traffic feature.
- mirrord-agent: `IndexAllocator` now uses `ConnectionId` for outgoing traffic feature.

## 3.0.6-alpha

### Changed

- mirrord-layer: Remove `tracing::instrument` from `go_env::goenvs_unix_detour`.
- mirrord-layer: Log to info instead of error when failing to write to local tunneled streams.

### Added

- mirrord-layer, mirrord-cli: new command line argument/environment variable - `MIRRORD_SKIP_PROCESSES` to provide a
  list of comma separated processes to not to load into.
  Closes [[#298](https://github.com/metalbear-co/mirrord/issues/298)]
  , [[#308](https://github.com/metalbear-co/mirrord/issues/308)]
- release CI: add arm64e to the universal dylib
- intellij-ext: Add support for Goland

## 3.0.5-alpha

### Fixed

- mirrord-layer: Return errors from agent when `connect` fails back to the hook (previously we were handling these as
  errors in layer, so `connect` had slightly wrong behavior).
- mirrord-layer: instrumenting error when `write_detur` is called to stdout/stderr
- mirrord-layer: workaround for `presented server name type wasn't supported` error when Kubernetes server has IP for CN
  in certificate. [[#388](https://github.com/metalbear-co/mirrord/issues/388)]

### Changed

- mirrord-layer: Use `tracing::instrument` to improve logs.

### Added

- Outgoing UDP test with node. Closes [[#323](https://github.com/metalbear-co/mirrord/issues/323)]

## 3.0.4-alpha

### Fixed

- Fix crash in VS Code extension happening because the MIRRORD_OVERRIDE_ENV_VARS_INCLUDE and
  MIRRORD_OVERRIDE_ENV_VARS_EXCLUDE vars being populated with empty values (rather than not being populated at all)
  .Closes [[#413](https://github.com/metalbear-co/mirrord/issues/413)].
- Add exception to gradle when dylib/so file is not found.
  Closes [[#345](https://github.com/metalbear-co/mirrord/issues/345)]
- mirrord-layer: Return errors from agent when `connect` fails back to the hook (previously we were handling these as
  errors in layer, so `connect` had slightly wrong behavior).

## 3.0.3-alpha

### Changed

- Changed agent namespace to default to the pod namespace.
  Closes [[#404](https://github.com/metalbear-co/mirrord/issues/404)].

## 3.0.2-alpha

### Added

- Code sign Apple binaries.
- CD - Update latest tag after release is published.

### Changed

- In `go-e2e` test, call `os.Exit` instead fo sending `SIGINT` to the process.
- Install script now downloads latest tag instead of main branch to avoid downtime on installs.

### Fixed

- Fix Environment parsing error when value contained '='
  Closes [[#387](https://github.com/metalbear-co/mirrord/issues/387)].
- Fix bug in outgoing traffic with multiple requests in quick succession.
  Closes [[#331](https://github.com/metalbear-co/mirrord/issues/331)].

## 3.0.1-alpha

### Fixed

- Add missing dependency breaking the VS Code release.

## 3.0.0-alpha

### Added

- New feature: UDP outgoing, mainly for Go DNS but should work for most use cases also!
- E2E: add tests for python's fastapi with uvicorn
- Socket ops - `connect`: ignore localhost and ports 50000 - 60000 (reserved for debugger)
- Add "*.plist" to `IGNORE_REGEX`, refer [[#350](https://github.com/metalbear-co/mirrord/issues/350)].

### Changed

- Change all functionality (incoming traffic mirroring, remote DNS outgoing traffic, environment variables, file reads)
  to be enabled by default. ***Note that flags now disable functionality***

### Fixed

- mirrord-layer: User-friendly error for invalid kubernetes api certificate
- mirrord-cli: Add random prefix to the generated shared lib to prevent Bus Error/EXC_BAD_ACCESS
- Support for Go 1.19>= syscall hooking
- Fix Python debugger crash in VS Code Extension. Closes [[#350](https://github.com/metalbear-co/mirrord/issues/350)].

## 2.13.0

### Added

- Release arm64 agent image.

### Fixed

- Use selected namespace in IntelliJ plugin instead of always using default namespace.

## 2.12.1

### Fixed

- Fix bug where VS Code extension would crash on startup due to new configuration values not being the correct type.
- Unset DYLD_INSERT_LIBRARIES/LD_PRELOAD when creating the agent.
  Closes [[#330](https://github.com/metalbear-co/mirrord/issues/330)].
- Fix NullPointerException in IntelliJ Extension. Closes [[#335](https://github.com/metalbear-co/mirrord/issues/335)].
- FIx dylib/so paths for the IntelliJ Extension. Closes [[#337](https://github.com/metalbear-co/mirrord/pull/352)].

## 2.12.0

### Added

- Add more configuration values to the VS Code extension.
- Warning when using remote tcp without remote DNS (can cause ipv6/v4 issues).
  Closes [#327](https://github.com/metalbear-co/mirrord/issues/327)

### Fixed

- VS Code needed restart to apply kubectl config/context change.
  Closes [316](https://github.com/metalbear-co/mirrord/issues/316).
- Fixed DNS feature causing crash on macOS on invalid DNS name due to mismatch of return
  codes. [#321](https://github.com/metalbear-co/mirrord/issues/321).
- Fixed DNS feature not using impersonated container namespace, resulting with incorrect resolved DNS names.
- mirrord-agent: Use `IndexAllocator` to properly generate `ConnectionId`s for the tcp outgoing feature.
- tests: Fix outgoing and DNS tests that were passing invalid flags to mirrord.
- Go Hooks - use global ENABLED_FILE_OPS
- Support macOS with apple chip in the IntelliJ plugin.
  Closes [#337](https://github.com/metalbear-co/mirrord/issues/337).

## 2.11.0

### Added

- New feature: mirrord now supports TCP traffic stealing instead of mirroring. You can enable it by
  passing `--tcp-steal` flag to cli.

### Fixed

- mirrord-layer: Go environment variables crash - run Go env setup in a different stack (should
  fix [#292](https://github.com/metalbear-co/mirrord/issues/292))

### Changed

- mirrord-layer: Add `#![feature(let_chains)]` to `lib.rs` to support new compiler version.

## 2.10.1

### Fixed

- CI:Release - Fix typo that broke the build

## 2.10.0

### Added

- New feature, [tcp outgoing traffic](https://github.com/metalbear-co/mirrord/issues/27). It's now possible to make
  requests to a remote host from the staging environment context. You can enable this feature setting
  the `MIRRORD_TCP_OUTGOING` variable to true, or using the `-o` option in mirrord-cli.
- mirrord-cli add login command for logging in to metalbear-cloud
- CI:Release - Provide zip and sha256 sums

### Fixed

- Environment variables feature on Golang programs. Issue #292 closed in #299

## 2.9.1

### Fixed

- CI - set typescript version at 4.7.4 to fix broken release action

## 2.9.0

### Added

- Support for Golang fileops
- IntelliJ Extension for mirrord

### Changed

- mirrord-layer: Added common `Result` type to to reduce boilerplate, removed dependency of `anyhow` crate.
- mirrord-layer: Split `LayerError` into `LayerError` and `HookError` to distinguish between errors that can be handled
  by the layer and errors that can be handled by the hook. (no more requiring libc errno for each error!).
  Closes [#247](https://github.com/metalbear-co/mirrord/issues/247)

## 2.8.1

### Fixed

- CI - remove usage of ubuntu-18.04 machines (deprecated)

## 2.8.0

### Added

- E2E - add basic env tests for bash scripts

### Fixed

- mirrord-agent - Update pcap library, hopefully will fix dropped packets (syn sometimes missed in e2e).
- mirrord-agent/layer - Sometimes layer tries to connect to agent before it finsihed loading, even though pod is
  running. Added watching the log stream for a "ready" log message before attempting to connect.

### Changed

- E2E - describe all pods on failure and add file name to print of logs.
- E2E - print timestamp of stdout/stderr of `TestProcess`.
- E2E - Don't delete pod/service on failure, instead leave them for debugging.
- mirrord-agent - Don't use `tokio::spawn` for spawning `sniffer` (or any other namespace changing task) to avoid
  namespace-clashing/undefined behavior. Possibly fixing bugs.
- Change the version check on the VS Code extension to happen when mirrord is enabled rather than when the IDE starts
  up.

## 2.7.0

### Added

- mirrord-layer: You can now pass `MIRRORD_AGENT_COMMUNICATION_TIMEOUT` as environment variable to control agent
  timeout.
- Expand file system operations with `access` and `faccessat` hooks for absolute paths

### Fixed

- Ephemeral Containers didn't wait for the right condition, leading to timeouts in many cases.
- mirrord-layer: Wait for the correct condition in job creation, resolving startup/timeout issues.
- mirrord-layer: Add a sleep on closing local socket after receiving close to let local application respond before
  closing.
- mirrord-layer: Fix DNS issue where `ai_addr` would not live long enough (breaking the remote DNS feature).

### Changed

- Removed unused dependencies from `mirrord-layer/Cargo.toml`. (Closes #220)
- reduce e2e flakiness (add message sent on tcp listen subscription, wait for that message)
- reduce e2e flakiness - increase timeout time
- mirrord-layer - increase agent creation timeout (to reduce e2e flakiness on macOS)
- E2E - Don't do file stuff on http traffic to reduce flakiness (doesn't add any coverage value..)
- mirrord-layer - Change tcp mirror tunnel `select` to be biased so it flushes all data before closing it (better
  testing, reduces e2e flakiness)
- E2E - unify resolve_node_host for linux and macOS with support for wsl provided Docker & Kubernetes
- E2E - add `trace` for tests to have paramaterized arguments printed
- mirrord-agent - add debug print of args to identify runs
- E2E - remove double `--extract-path` parameter in tests
- E2E - macOS colima start with 3 cores and 8GB of RAM.
- E2E - Increase agent communication timeout to reduce flakiness.
- mirrord-layer - add `DetourGuard` to prevent unwanted calls to detours from our code.
- mirrord-layer - extract reused detours to seperate logic functions
- E2E - macOS run only sanity http mirror traffic with Python

## 2.6.0

### Added

- Add a flag for the agent, `--ephemeral-container`, to correctly refer to the filesystem i.e. refer to root path
  as `/proc/1/root` when the flag is on, otherwise `/`.
- Add support for Golang on amd64 (x86-64).

### Changed

- Assign a random port number instead of `61337`. (Reason: A forking process creates multiple agents sending traffic on
  the same port, causing addrinuse error.)
- `mirrord-layer/socket` now uses `socket2::SockAddr` to comply with Rust's new IP format.

### Fixed

- Fix filesystem tests to only run if the default path exists.
- Fix extension not running due to the node_modules directory not being packaged.

## 2.5.0

### Added

- New feature, [remote DNS resolving](https://github.com/metalbear-co/mirrord/issues/27#issuecomment-1154072686).
  It is now possible to use the remote's `addrinfo` by setting the `MIRRORD_REMOTE_DNS` variable to
  `true`, or using the `-d` option in mirrord-cli.
- New feature, [Ephemeral Containers](https://github.com/metalbear-co/mirrord/issues/172).
  Use Kubernetes beta feature `Ephemeral Containers` to mirror traffic with the `--ephemeral-container` flag.
- E2E tests on macos for Golang using the Gin framework.

### Changed

- Refactored `mirrord-layer/socket` into a module structure similar to `mirrord-layer/file`.
- Refactored the error part of the many `Result<Response, ResponseError>`.
- Refactored `file` related functions, created `FileHandler` and improved structure.
- Refactored error handling in mirrord-layer.
- E2E: Collect minikube logs and fix collecting container logs
- E2E: macOS use colima instead of minikube.
- Refactored `mirrord-layer/lib.rs` - no more passing many arguments! :)
- Refactored `mirrord-layer/lib.rs` - remove `unwrap()` and propagate error using `Result`

### Fixed

- Handle unwraps in fileops to gracefully exit and enable python fileops tests.
- Changed `addrinfo` to `VecDeque` - fixes a potential bug (loss of order)

## 2.4.1

### Added

- mirrord-cli `exec` subcommand accepts `--extract-path` argument to set the directory to extract the library to. Used
  for tests mainly.
- mirrord-layer provides `MIRRORD_IMPERSONATED_CONTAINER_NAME` environment variable to specify container name to
  impersonate. mirrord-cli accepts argument to set variable.
- vscode-ext provides quick-select for setting `MIRRORD_IMPERSONATED_CONTAINER_NAME`

### Changed

- Refactor e2e, enable only Node HTTP mirroring test.
- E2E: add macOS to E2E, support using minikube by env var.
- E2E: Skip loading to docker before loading to minikube (load directly to minikube..)
- layer: Environment variables now load before process starts, no more race conditions.

### Fixed

- Support connections that start with tcp flags in addition to Syn (on macOS CI we saw CWR + NS)
- `fcntl` error on macOS [#184](https://github.com/metalbear-co/mirrord/issues/184) by a workaround.

## 2.3.1

### Changed

- Refactor(agent) - change `FileManager` to be per peer, thus removing the need of it being in a different task, moving
  the handling to the peer logic, change structure of peer handling to a struct.
- Don't fail environment variable request if none exists.
- E2E: Don't assert jobs and pods length, to allow better debugging and less flakiness.
- Refactor(agent) - Main loop doesn't pass messages around but instead spawned peers interact directly with tcp sniffer.
  Renamed Peer -> Client and ClientID.
- Add context to agent/job creation errors (Fixes #112)
- Add context to stream creation error (Fixes #110)
- Change E2E to use real app, closes [#149](https://github.com/metalbear-co/mirrord/issues/149)

## 2.3.0

### Added

- Add support for overriding a process' environment variables by setting `MIRRORD_OVERRIDE_ENV_VARS` to `true`. To
  filter out undesired variables, use the `MIRRORD_OVERRIDE_FILTER_ENV_VARS` configuration with arguments such
  as `FOO;BAR`.

### Changed

- Remove `unwrap` from the `Future` that was waiting for Kube pod to spin up in `pod_api.rs`. (Fixes #110)
- Speed up agent container image building by using a more specific base image.
- CI: Remove building agent before building & running tests (duplicate)
- CI: Add Docker cache to Docker build-push action to reduce build duration.
- CD release: Fix universal binary for macOS
- Refactor: Change protocol + mirrord-layer to split messages into modules, so main module only handles general
  messages, passing down to the appropriate module for handling.
- Add a CLI flag to specify `MIRRORD_AGENT_TTL`
- CI: Collect mirrord-agent logs in case of failure in e2e.
- Add "app" = "mirrord" label to the agent pod for log collection at ease.
- CI: Add sleep after local app finishes loading for agent to load filter make tests less flaky.
- Handle relative paths for open, openat
- Fix once cell renamings, PR [#98165](https://github.com/rust-lang/rust/pull/98165)
- Enable the blocking feature of the `reqwest` library

## 2.2.1

### Changed

- Compile universal binaries for MacOS. (Fixes #131)
- E2E small improvements, removing sleeps. (Fixes #99)

## 2.2.0

### Added

- File operations are now available behind the `MIRRORD_FILE_OPS` env variable, this means that mirrord now hooks into
  the following file functions: `open`, `fopen`, `fdopen`, `openat`, `read`, `fread`, `fileno`, `lseek`, and `write` to
  provide a mirrored file system.
- Support for running x64 (Intel) binary on arm (Silicon) macOS using mirrord. This will download and use the x64
  mirrord-layer binary when needed.
- Add detours for fcntl/dup system calls, closes [#51](https://github.com/metalbear-co/mirrord/issues/51)

### Changed

- Add graceful exit for library extraction logic in case of error.
- Refactor the CI by splitting the building of mirrord-agent in a separate job and caching the agent image for E2E
  tests.
- Update bug report template to apply to the latest version of mirrord.
- Change release profile to strip debuginfo and enable LTO.
- VS Code extension - update dependencies.
- CLI & macOS: Extract to `/tmp/` instead of `$TMPDIR` as the executed process is getting killed for some reason.

### Fixed

- Fix bug that caused configuration changes in the VS Code extension not to work
- Fix typos

## 2.1.0

### Added

- Prompt user to update if their version is outdated in the VS Code extension or CLI.
- Add support for docker runtime, closes [#95](https://github.com/metalbear-co/mirrord/issues/95).
- Add a keep-alive to keep the agent-pod from exiting, closes [#63](https://github.com/metalbear-co/mirrord/issues/63)

## 2.0.4

Complete refactor and re-write of everything.

- The CLI/VSCode extension now use `mirrord-layer` which loads into debugged process using `LD_PRELOAD`
  /`DYLD_INSERT_LIBRARIES`.
  It hooks some of the syscalls in order to proxy incoming traffic into the process as if it was running in the remote
  pod.
- Mono repo
- Fixed unwraps inside
  of [agent-creation](https://github.com/metalbear-co/mirrord/blob/main/mirrord-layer/src/lib.rs#L75),
  closes [#191](https://github.com/metalbear-co/mirrord/issues/191)
