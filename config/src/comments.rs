// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Comments for configuration + injection into output .toml
use std::collections::HashMap;

/// maps entries to Comments that should precede them
fn comments() -> HashMap<String, String> {
	let mut retval = HashMap::new();
	retval.insert(
		"[server]".to_string(),
		"
# Generated Server Configuration File for Vcash
#
# When running the grin executable without specifying any command line
# arguments, it will look for this file in two places, in the following
# order:
#
# -The working directory
# -[user home]/.vcash
#

#########################################
### SERVER CONFIGURATION              ###
#########################################

#Server connection details
"
		.to_string(),
	);

	retval.insert(
		"api_http_addr".to_string(),
		"
#path of TLS certificate file, self-signed certificates are not supported
#tls_certificate_file = \"\"
#private key for the TLS certificate
#tls_certificate_key = \"\"

#the address on which services will listen, e.g. Transaction Pool
"
		.to_string(),
	);

	retval.insert(
		"api_secret_path".to_string(),
		"
#path of the secret token used by the API to authenticate the calls
#comment the it to disable basic auth
"
		.to_string(),
	);

	retval.insert(
		"db_root".to_string(),
		"
#the directory, relative to current, in which the grin blockchain
#is stored
"
		.to_string(),
	);

	retval.insert(
		"chain_type".to_string(),
		"
#The chain type, which defines the genesis block and the set of cuckoo
#parameters used for mining as well as wallet output coinbase maturity. Can be:
#AutomatedTesting - For CI builds and instant blockchain creation
#UserTesting - For regular user testing (cuckoo 16)
#Floonet - For the long term Floonet test network
#Mainnet
"
		.to_string(),
	);

	retval.insert(
		"chain_validation_mode".to_string(),
		"
#the chain validation mode, defines how often (if at all) we
#want to run a full chain validation. Can be:
#\"EveryBlock\" - run full chain validation when processing each block (except during sync)
#\"Disabled\" - disable full chain validation (just run regular block validation)
"
		.to_string(),
	);

	retval.insert(
		"archive_mode".to_string(),
		"
#run the node in \"full archive\" mode (default is fast-sync, pruned node)
"
		.to_string(),
	);

	retval.insert(
		"skip_sync_wait".to_string(),
		"
#skip waiting for sync on startup, (optional param, mostly for testing)
"
		.to_string(),
	);

	retval.insert(
		"run_tui".to_string(),
		"
#whether to run the ncurses TUI. Ncurses must be installed and this
#will also disable logging to stdout
"
		.to_string(),
	);

	retval.insert(
		"run_test_miner".to_string(),
		"
#Whether to run a test miner. This is only for developer testing (chaintype
#usertesting) at cuckoo 16, and will only mine into the default wallet port.
#real mining should use the standalone grin-miner
"
		.to_string(),
	);

	retval.insert(
		"[server.dandelion_config]".to_string(),
		"
#########################################
### DANDELION CONFIGURATION           ###
#########################################
"
		.to_string(),
	);

	retval.insert(
		"relay_secs".to_string(),
		"
#dandelion relay time (choose new relay peer every n secs)
"
		.to_string(),
	);

	retval.insert(
		"embargo_secs".to_string(),
		"
#fluff and broadcast after embargo expires if tx not seen on network
"
		.to_string(),
	);

	retval.insert(
		"patience_secs".to_string(),
		"
#run dandelion stem/fluff processing every n secs (stem tx aggregation in this window)
"
		.to_string(),
	);
	retval.insert(
		"stem_probability".to_string(),
		"
#dandelion stem probability (stem 90% of the time, fluff 10% of the time)
"
		.to_string(),
	);

	retval.insert(
		"[server.pool_server_config]".to_string(),
		"#test miner wallet URL (burns if this doesn't exist)
#test_miner_wallet_url = \"http://127.0.0.1:3515\"

################################################
### POOL SERVER CONFIGURATION      ###
################################################
"
		.to_string(),
	);

	retval.insert(
		"enable_pool_server".to_string(),
		"
#whether pool server is enabled
"
		.to_string(),
	);

	retval.insert(
		"pool_server_addr".to_string(),
		"
#the address on which pool server will listen
"
		.to_string(),
	);

	retval.insert(
		"chain_notify_url".to_string(),
		"
#the addresses which pool server will post new mine block param when chain height grows
"
		.to_string(),
	);

	retval.insert(
		"[server.p2p_config]".to_string(),
		"
#########################################
### SERVER P2P CONFIGURATION          ###
#########################################
#The P2P server details (i.e. the server that communicates with other
"
		.to_string(),
	);

	retval.insert(
		"host".to_string(),
		"
#The interface on which to listen.
#0.0.0.0 will listen on all interfaces, allowing others to interact
#127.0.0.1 will listen on the local machine only
"
		.to_string(),
	);

	retval.insert(
		"port".to_string(),
		"
#The port on which to listen.
"
		.to_string(),
	);

	retval.insert(
		"seeding_type".to_string(),
		"
#how to seed this server, can be None, List or DNSSeed
"
		.to_string(),
	);

	retval.insert(
		"[server.p2p_config.capabilities]".to_string(),
		"#If the seeding type is List, the list of peers to connect to can
#be specified as follows:
#seeds = [\"192.168.0.1:3514\",\"192.168.0.2:3514\"]

#hardcoded peer lists for allow/deny
#will *only* connect to peers in allow list
#peers_allow = [\"192.168.0.1:3514\", \"192.168.0.2:3514\"]
#will *never* connect to peers in deny list
#peers_deny = [\"192.168.0.3:3514\", \"192.168.0.4:3514\"]
#a list of preferred peers to connect to
#peers_preferred = [\"192.168.0.1:3514\",\"192.168.0.2:3514\"]

#how long a banned peer should stay banned
#ban_window = 10800

#maximum number of peers
#peer_max_count = 25

#preferred minimum number of peers (we'll actively keep trying to add peers
#until we get to at least this number
#peer_min_preferred_count = 8

# 15 = Bit flags for FULL_NODE
#This structure needs to be changed internally, to make it more configurable

# A preferred dandelion_peer, mainly used for testing dandelion
# dandelion_peer = \"10.0.0.1:13144\"

"
		.to_string(),
	);

	retval.insert(
		"[server.pool_config]".to_string(),
		"
#########################################
### MEMPOOL CONFIGURATION             ###
#########################################
"
		.to_string(),
	);

	retval.insert(
		"accept_fee_base".to_string(),
		"
#base fee that's accepted into the pool
"
		.to_string(),
	);

	retval.insert(
		"max_pool_size".to_string(),
		"
#maximum number of transactions allowed in the pool
"
		.to_string(),
	);

	retval.insert(
		"max_stempool_size".to_string(),
		"
#maximum number of transactions allowed in the stempool
"
		.to_string(),
	);

	retval.insert(
		"mineable_max_weight".to_string(),
		"
#maximum total weight of transactions that can get selected to build a block
"
		.to_string(),
	);

	retval.insert(
		"[server.stratum_mining_config]".to_string(),
		"
################################################
### STRATUM MINING SERVER CONFIGURATION      ###
################################################
"
		.to_string(),
	);

	retval.insert(
		"enable_stratum_server".to_string(),
		"
#whether stratum server is enabled
"
		.to_string(),
	);

	retval.insert(
		"stratum_server_addr".to_string(),
		"
#what port and address for the stratum server to listen on
"
		.to_string(),
	);

	retval.insert(
		"attempt_time_per_block".to_string(),
		"
#the amount of time, in seconds, to attempt to mine on a particular
#header before stopping and re-collecting transactions from the pool
"
		.to_string(),
	);

	retval.insert(
		"minimum_share_difficulty".to_string(),
		"
#the minimum acceptable share difficulty to request from miners
"
		.to_string(),
	);

	retval.insert(
		"wallet_listener_url".to_string(),
		"
#the wallet receiver to which coinbase rewards will be sent
"
		.to_string(),
	);

	retval.insert(
		"burn_reward".to_string(),
		"
#whether to ignore the reward (mostly for testing)
"
		.to_string(),
	);

	retval.insert(
		"[wallet]".to_string(),
		"
#########################################
### WALLET CONFIGURATION              ###
#########################################
"
		.to_string(),
	);

	retval.insert(
		"api_listen_interface".to_string(),
		"
#host IP for wallet listener, change to \"0.0.0.0\" to receive grins
"
		.to_string(),
	);

	retval.insert(
		"api_listen_port".to_string(),
		"
#path of TLS certificate file, self-signed certificates are not supported
#tls_certificate_file = \"\"
#private key for the TLS certificate
#tls_certificate_key = \"\"

#port for wallet listener
"
		.to_string(),
	);

	retval.insert(
		"owner_api_listen_port".to_string(),
		"
#port for wallet owner api
"
		.to_string(),
	);

	retval.insert(
		"api_secret_path".to_string(),
		"
#path of the secret token used by the API to authenticate the calls
#comment it to disable basic auth
"
		.to_string(),
	);
	retval.insert(
		"check_node_api_http_addr".to_string(),
		"
#where the wallet should find a running node
"
		.to_string(),
	);
	retval.insert(
		"node_api_secret_path".to_string(),
		"
#location of the node api secret for basic auth on the Grin API
"
		.to_string(),
	);
	retval.insert(
		"owner_api_include_foreign".to_string(),
		"
#include the foreign API endpoints on the same port as the owner
#API. Useful for networking environments like AWS ECS that make
#it difficult to access multiple ports on a single service.
"
		.to_string(),
	);
	retval.insert(
		"data_file_dir".to_string(),
		"
#where to find wallet files (seed, data, etc)
"
		.to_string(),
	);
	retval.insert(
		"no_commit_cache".to_string(),
		"
#If true, don't store calculated commits in the database
#better privacy, but at a performance cost of having to
#re-calculate commits every time they're used
"
		.to_string(),
	);
	retval.insert(
		"dark_background_color_scheme".to_string(),
		"
#Whether to use the black background color scheme for command line
"
		.to_string(),
	);
	retval.insert(
		"keybase_notify_ttl".to_string(),
		"
#The exploding lifetime for keybase notification on coins received.
#Unit: Minute. Default value 1440 minutes for one day.
#Refer to https://keybase.io/blog/keybase-exploding-messages for detail.
#To disable this notification, set it as 0.
"
		.to_string(),
	);

	retval.insert(
		"[logging]".to_string(),
		"
#########################################
### LOGGING CONFIGURATION             ###
#########################################
"
		.to_string(),
	);

	retval.insert(
		"log_to_stdout".to_string(),
		"
#whether to log to stdout
"
		.to_string(),
	);

	retval.insert(
		"stdout_log_level".to_string(),
		"
#log level for stdout: Error, Warning, Info, Debug, Trace
"
		.to_string(),
	);

	retval.insert(
		"log_to_file".to_string(),
		"
#whether to log to a file
"
		.to_string(),
	);

	retval.insert(
		"file_log_level".to_string(),
		"
#log level for file: Error, Warning, Info, Debug, Trace
"
		.to_string(),
	);

	retval.insert(
		"log_file_path".to_string(),
		"
#log file path
"
		.to_string(),
	);

	retval.insert(
		"log_file_append".to_string(),
		"
#whether to append to the log file (true), or replace it on every run (false)
"
		.to_string(),
	);

	retval.insert(
		"log_max_size".to_string(),
		"
#maximum log file size in bytes before performing log rotation
#comment it to disable log rotation
"
		.to_string(),
	);

	retval.insert(
		"log_max_files".to_string(),
		"
#maximum count of the log files to rotate over
"
		.to_string(),
	);

	retval
}

fn get_key(line: &str) -> String {
	if line.contains("[") && line.contains("]") && !line.contains("=") {
		return line.to_owned();
	} else if line.contains("=") {
		return line.split("=").collect::<Vec<&str>>()[0].trim().to_owned();
	} else {
		return "NOT_FOUND".to_owned();
	}
}

pub fn insert_comments(orig: String) -> String {
	let comments = comments();
	let lines: Vec<&str> = orig.split("\n").collect();
	let mut out_lines = vec![];
	for l in lines {
		let key = get_key(l);
		if let Some(v) = comments.get(&key) {
			out_lines.push(v.to_owned());
		}
		out_lines.push(l.to_owned());
		out_lines.push("\n".to_owned());
	}
	let mut ret_val = String::from("");
	for l in out_lines {
		ret_val.push_str(&l);
	}
	ret_val.to_owned()
}
