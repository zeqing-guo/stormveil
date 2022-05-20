use std::{env, path::Path, io::{self, BufRead}, fs::File, collections::BTreeMap};
use ethers_etherscan::{Client, account::GenesisOption};
use ethers_core::{utils::hex, types::{Chain, Address}};

#[tokio::main]
async fn main() {
    let api_key = env::var("ETHERSCAN_API_KEY").unwrap();
    let addrs_file = env::var("ADDRESS_FILE").unwrap();
    let addrs = read_lines(addrs_file).unwrap();
    let client = Client::new(Chain::Arbitrum, api_key).unwrap();

    for addr in addrs {
        if let Ok(a) = addr {
            println!("address: {}", a);
            let mut sig_to_func_name: BTreeMap<[u8; 4], String> = BTreeMap::new();
            let mut result: BTreeMap<[u8; 4], String> = BTreeMap::new();
            let contract_abi_verified = client.contract_source_code(a.parse().unwrap()).await.unwrap();
            let mut contract_addr: Address = a.clone().parse().unwrap();
            for item in contract_abi_verified.items {
                if !item.implementation.is_empty() {
                    contract_addr = item.implementation.parse().unwrap();
                }
            }
            let contract_abi = client.contract_abi(contract_addr).await.unwrap();
            let functions = contract_abi.functions();
            for function in functions {
                sig_to_func_name.insert(
                    function.short_signature(), 
                    function.name.clone()
                );
            }

            let txs = client.get_transactions(
                &a.parse().unwrap(), 
                None,
            ).await.unwrap();
            println!("num of txs: {}", txs.len());
            for tx in txs {
                match tx.input {
                    GenesisOption::Some(input) => {
                        let input_vec = &input.to_vec();
                        let sig:[u8; 4] = [input_vec[0], input_vec[1], input_vec[2], input_vec[3]];
                        match sig_to_func_name.get(&sig) {
                            Some(name) => {
                                result.insert(
                                    sig, 
                                    format!("{}, {:#x}", name.clone(), tx.hash.value().unwrap())
                                );
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
            println!();
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
