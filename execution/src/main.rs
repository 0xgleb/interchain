use clap_builder::Parser;
use reth_node_optimism::rpc::SequencerClient;
use std::sync::Arc;

use interchain_execution_client::*;

fn main() {
    reth_cli_util::sigsegv_handler::install();

    if let Err(err) = reth::cli::Cli::<reth_node_optimism::args::RollupArgs>::parse().run(
        |builder, rollup_args| async move {
            let handle = builder
                .node(InterchainNode::new(rollup_args.clone()))
                .extend_rpc_modules(move |ctx| {
                    // register sequencer tx forwarder
                    if let Some(sequencer_http) = rollup_args.sequencer_http {
                        ctx.registry.set_eth_raw_transaction_forwarder(Arc::new(
                            SequencerClient::new(sequencer_http),
                        ));
                    }

                    Ok(())
                })
                .launch()
                .await?;

            handle.node_exit_future.await
        },
    ) {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
