use mirrord_protocol::Port;
use nix::unistd::{getgid, Gid};
use rand::distributions::{Alphanumeric, DistString};
use tracing::{debug, info};

use crate::error::{AgentError, Result};

#[cfg_attr(test, mockall::automock)]
pub(super) trait IPTables {
    fn create_chain(&self, name: &str) -> Result<()>;
    fn remove_chain(&self, name: &str) -> Result<()>;

    fn add_rule(&self, chain: &str, rule: &str) -> Result<()>;
    fn insert_rule(&self, chain: &str, rule: &str, index: i32) -> Result<()>;
    fn list_rules(&self, chain: &str) -> Result<Vec<String>>;
    fn remove_rule(&self, chain: &str, rule: &str) -> Result<()>;
    fn replace_rule(&self, chain_name: &str, rule: &str, position: i32) -> Result<()>;
}

impl IPTables for iptables::IPTables {
    #[tracing::instrument(level = "debug", skip(self))]
    fn create_chain(&self, name: &str) -> Result<()> {
        self.new_chain(IPTABLES_TABLE_NAME, name)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))?;
        self.append(IPTABLES_TABLE_NAME, name, "-j RETURN")
            .map_err(|e| AgentError::IPTablesError(e.to_string()))?;

        info!("Chains {:#?}", self.list_chains("nat"));
        info!("Rules {:#?}", self.list_rules(name));

        Ok(())
    }

    fn replace_rule(&self, chain_name: &str, rule: &str, position: i32) -> Result<()> {
        self.replace(IPTABLES_TABLE_NAME, chain_name, rule, position)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))
    }

    fn remove_chain(&self, name: &str) -> Result<()> {
        self.flush_chain(IPTABLES_TABLE_NAME, name)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))?;
        self.delete_chain(IPTABLES_TABLE_NAME, name)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))?;

        Ok(())
    }

    fn add_rule(&self, chain: &str, rule: &str) -> Result<()> {
        self.append(IPTABLES_TABLE_NAME, chain, rule)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))
    }

    fn insert_rule(&self, chain: &str, rule: &str, index: i32) -> Result<()> {
        self.insert(IPTABLES_TABLE_NAME, chain, rule, index)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))
    }

    fn list_rules(&self, chain: &str) -> Result<Vec<String>> {
        self.list(IPTABLES_TABLE_NAME, chain)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))
    }

    fn remove_rule(&self, chain: &str, rule: &str) -> Result<()> {
        self.delete(IPTABLES_TABLE_NAME, chain, rule)
            .map_err(|e| AgentError::IPTablesError(e.to_string()))
    }
}

/// Wrapper struct for IPTables so it flushes on drop.
pub(super) struct SafeIpTables<IPT: IPTables> {
    inner: IPT,
    chain_name: String,
    formatter: IPTableFormatter,
}

const IPTABLES_TABLE_NAME: &str = "nat";
/// Wrapper for using iptables. This creates a a new chain on creation and deletes it on drop.
/// The way it works is that it adds a chain, then adds a rule to the chain that returns to the
/// original chain (fallback) and adds a rule in the "PREROUTING" table that jumps to the new chain.
/// Connections will go then PREROUTING -> OUR_CHAIN -> IF MATCH REDIRECT -> IF NOT MATCH FALLBACK
/// -> ORIGINAL_CHAIN
impl<IPT> SafeIpTables<IPT>
where
    IPT: IPTables,
{
    #[tracing::instrument(level = "debug", skip(ipt))]
    pub(super) fn new(ipt: IPT) -> Result<Self> {
        let formatter = IPTableFormatter::detect(&ipt)?;

        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);
        let chain_name = format!("MIRRORD_REDIRECT_{}", random_string);

        ipt.create_chain(&chain_name)?;

        ipt.add_rule(formatter.entrypoint(), &format!("-j {}", chain_name))?;

        Ok(Self {
            inner: ipt,
            chain_name,
            formatter,
        })
    }

    /*
        "-N MIRRORD_REDIRECT_HsKZe",
        "-A MIRRORD_REDIRECT_HsKZe -j RETURN",
        "-A MIRRORD_REDIRECT_HsKZe -p tcp -m owner --gid-owner 25188 -j RETURN",
        "-A MIRRORD_REDIRECT_HsKZe -p tcp -m tcp --dport 80 -j REDIRECT --to-ports 42773",
    */
    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) fn add_stealer_rule(&self, redirected_port: Port, target_port: Port) -> Result<()> {
        /*
        We just ignore everything:

            "-N MIRRORD_REDIRECT_J4Juk",
            "-A MIRRORD_REDIRECT_J4Juk -j RETURN",
            "-A MIRRORD_REDIRECT_J4Juk -p tcp -m tcp --dport 80 -j REDIRECT --to-ports 35673",
            "-A MIRRORD_REDIRECT_J4Juk -p tcp -m owner --gid-owner 13049 -j RETURN",

            self.add_redirect(redirected_port, target_port)
                .inspect(|_| info!("Added redirect {:#?}", self.list_rules()))
                .and_then(|_| self.add_bypass_mirrord())
                .inspect(|_| info!("Added bypass {:#?}", self.list_rules()))
        */

        self.add_bypass_mirrord()
            .inspect(|_| info!("Added bypass {:#?}", self.list_rules()))
            .and_then(|_| self.add_redirect(redirected_port, target_port))
            .inspect(|_| info!("Added redirect {:#?}", self.list_rules()))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) fn remove_stealer_rule(
        &self,
        redirected_port: Port,
        target_port: Port,
    ) -> Result<()> {
        self.remove_bypass_mirrord()
            .and_then(|_| self.remove_redirect(redirected_port, target_port))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn add_return(&self) -> Result<()> {
        self.inner.add_rule(&self.chain_name, &format!("-j RETURN"))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn remove_return(&self) -> Result<()> {
        self.inner
            .remove_rule(&self.chain_name, &format!("-j RETURN"))
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn add_bypass_mirrord(&self) -> Result<()> {
        let gid = getgid();

        self.inner.add_rule(
            &self.chain_name,
            &format!("-m owner --gid-owner {gid} -p tcp -j RETURN"),
        )
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn remove_bypass_mirrord(&self) -> Result<()> {
        let gid = getgid();

        self.inner.remove_rule(
            &self.chain_name,
            &format!("-m owner --gid-owner {gid} -p tcp -j RETURN"),
        )
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn add_redirect(&self, redirected_port: Port, target_port: Port) -> Result<()> {
        self.inner.add_rule(
            &self.chain_name,
            &self.formatter.redirect_rule(redirected_port, target_port),
        )
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn remove_redirect(&self, redirected_port: Port, target_port: Port) -> Result<()> {
        self.inner.remove_rule(
            &self.chain_name,
            &self.formatter.redirect_rule(redirected_port, target_port),
        )
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) fn list_rules(&self) -> Result<Vec<String>> {
        self.inner.list_rules(&self.chain_name)
    }
}

impl<IPT> Drop for SafeIpTables<IPT>
where
    IPT: IPTables,
{
    fn drop(&mut self) {
        self.inner
            .remove_rule(
                self.formatter.entrypoint(),
                &format!("-j {}", self.chain_name),
            )
            .unwrap();

        self.inner.remove_chain(&self.chain_name).unwrap();
    }
}

enum IPTableFormatter {
    Normal,
    Mesh,
}

impl IPTableFormatter {
    const MESH_OUTPUTS: [&'static str; 2] = ["-j PROXY_INIT_OUTPUT", "-j ISTIO_OUTPUT"];

    fn detect<IPT: IPTables>(ipt: &IPT) -> Result<Self> {
        let output = ipt.list_rules("OUTPUT")?;

        if output.iter().any(|rule| {
            IPTableFormatter::MESH_OUTPUTS
                .iter()
                .any(|mesh_output| rule.contains(mesh_output))
        }) {
            Ok(IPTableFormatter::Mesh)
        } else {
            Ok(IPTableFormatter::Normal)
        }
    }

    fn entrypoint(&self) -> &str {
        match self {
            IPTableFormatter::Normal => "PREROUTING",
            IPTableFormatter::Mesh => "OUTPUT",
        }
    }

    fn redirect_rule(&self, redirected_port: Port, target_port: Port) -> String {
        let redirect_rule = format!(
            "-m tcp -p tcp --dport {} -j REDIRECT --to-ports {}",
            redirected_port, target_port
        );

        match self {
            IPTableFormatter::Normal => redirect_rule,
            IPTableFormatter::Mesh => {
                format!("-o lo {}", redirect_rule)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;

    use super::*;

    #[test]
    fn default() {
        let mut mock = MockIPTables::new();

        mock.expect_list_rules()
            .with(eq("OUTPUT"))
            .returning(|_| Ok(vec![]));

        mock.expect_create_chain()
            .with(str::starts_with("MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_| Ok(()));

        mock.expect_remove_chain()
            .with(str::starts_with("MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_| Ok(()));

        mock.expect_add_rule()
            .with(eq("PREROUTING"), str::starts_with("-j MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_insert_rule()
            .with(
                str::starts_with("MIRRORD_REDIRECT_"),
                eq("-m tcp -p tcp --dport 69 -j REDIRECT --to-ports 420"),
                eq(1),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        mock.expect_remove_rule()
            .with(eq("PREROUTING"), str::starts_with("-j MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_remove_rule()
            .with(
                str::starts_with("MIRRORD_REDIRECT_"),
                eq("-m tcp -p tcp --dport 69 -j REDIRECT --to-ports 420"),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let ipt = SafeIpTables::new(mock).expect("Create Failed");

        assert!(ipt.add_redirect(69, 420).is_ok());

        assert!(ipt.remove_redirect(69, 420).is_ok());
    }

    #[test]
    fn linkerd() {
        let mut mock = MockIPTables::new();

        mock.expect_list_rules()
            .with(eq("OUTPUT"))
            .returning(|_| Ok(vec!["-j PROXY_INIT_OUTPUT".to_owned()]));

        mock.expect_create_chain()
            .with(str::starts_with("MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_| Ok(()));

        mock.expect_remove_chain()
            .with(str::starts_with("MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_| Ok(()));

        mock.expect_add_rule()
            .with(eq("OUTPUT"), str::starts_with("-j MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_insert_rule()
            .with(
                str::starts_with("MIRRORD_REDIRECT_"),
                eq("-o lo -m tcp -p tcp --dport 69 -j REDIRECT --to-ports 420"),
                eq(1),
            )
            .times(1)
            .returning(|_, _, _| Ok(()));

        mock.expect_remove_rule()
            .with(eq("OUTPUT"), str::starts_with("-j MIRRORD_REDIRECT_"))
            .times(1)
            .returning(|_, _| Ok(()));

        mock.expect_remove_rule()
            .with(
                str::starts_with("MIRRORD_REDIRECT_"),
                eq("-o lo -m tcp -p tcp --dport 69 -j REDIRECT --to-ports 420"),
            )
            .times(1)
            .returning(|_, _| Ok(()));

        let ipt = SafeIpTables::new(mock).expect("Create Failed");

        assert!(ipt.add_redirect(69, 420).is_ok());

        assert!(ipt.remove_redirect(69, 420).is_ok());
    }
}
