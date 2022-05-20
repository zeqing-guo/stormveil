use std::{env, path::Path, io::{self, BufRead}, fs::File, collections::BTreeMap};
use ethers_etherscan::{Client, account::{GenesisOption, TxListParams, Sort}};
use ethers_core::{utils::hex, types::Chain};

#[tokio::main]
async fn main() {
    let api_key = env::var("ETHERSCAN_API_KEY").unwrap();
    let addrs_file = env::var("ADDRESS_FILE").unwrap();
    let addrs = read_lines(addrs_file).unwrap();
    let client = Client::new(Chain::Arbitrum, api_key).unwrap();

    for addr in addrs {
        if let Ok(a) = addr {
            let mut sig_to_func_name: BTreeMap<[u8; 4], String> = BTreeMap::new();
            let mut result: BTreeMap<[u8; 4], String> = BTreeMap::new();
            let contract_abi = client.contract_abi(a.parse().unwrap()).await.unwrap();
            let functions = contract_abi.functions();
            for function in functions {
                sig_to_func_name.insert(
                    function.short_signature(), 
                    function.name.clone()
                );
            }

            let txs = client.get_transactions(
                &a.parse().unwrap(), 
                Some(TxListParams::new(10950429+1, 99999999, 0, 10000, Sort::Asc)),
            ).await.unwrap();
            for tx in txs {
                match tx.input {
                    GenesisOption::Some(input) => {
                        let input_vec = &input.to_vec();
                        let sig:[u8; 4] = [input_vec[0], input_vec[1], input_vec[2], input_vec[3]];
                        match sig_to_func_name.get(&sig) {
                            Some(name) => {
                                result.insert(sig, name.clone());
                            },
                            None => {},
                        }
                    },
                    _ => {},
                }
            }

            for (k, v) in result {
                println!("{}, 0x{}, {}", a, hex::encode(&k), v);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
