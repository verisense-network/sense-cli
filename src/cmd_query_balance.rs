/// The `generate` command
#[derive(Debug, Clone, Parser)]
#[command(name = "qb", about = "Query the balance of an account")]
pub struct QuerybalanceCmd {
    /// The number of words in the phrase to generate. One of 12 (default), 15, 18, 21 and 24.
    #[arg(short = 'w', long, value_name = "WORDS")]
    words: Option<usize>,

    #[allow(missing_docs)]
    #[clap(flatten)]
    pub keystore_params: KeystoreParams,

    #[allow(missing_docs)]
    #[clap(flatten)]
    pub network_scheme: NetworkSchemeFlag,

    #[allow(missing_docs)]
    #[clap(flatten)]
    pub output_scheme: OutputTypeFlag,

    #[allow(missing_docs)]
    #[clap(flatten)]
    pub crypto_scheme: CryptoSchemeFlag,
}

impl GenerateCmd {
    /// Run the command
    pub fn run(&self) -> Result<(), Error> {
        let words = match self.words {
            Some(words_count) if [12, 15, 18, 21, 24].contains(&words_count) => Ok(words_count),
            Some(_) => Err(Error::Input(
                "Invalid number of words given for phrase: must be 12/15/18/21/24".into(),
            )),
            None => Ok(12),
        }?;
        let mnemonic = Mnemonic::generate(words)
            .map_err(|e| Error::Input(format!("Mnemonic generation failed: {e}").into()))?;
        let password = self.keystore_params.read_password()?;
        let output = self.output_scheme.output_type;

        let phrase = mnemonic.words().join(" ");

        with_crypto_scheme!(
            self.crypto_scheme.scheme,
            print_from_uri(&phrase, password, self.network_scheme.network, output)
        );
        Ok(())
    }
}

use std::str::FromStr;
use subxt::utils::AccountId32;
use subxt::{OnlineClient, SubstrateConfig};

#[subxt::subxt(runtime_metadata_path = "substrate_metadata.scale")]
pub mod substrate {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to connect to a Substrate node
    let api = OnlineClient::<SubstrateConfig>::new().await?;

    // The account we want to query
    let account_id = AccountId32::from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")?;

    // Query the account info
    let account_info = api
        .storage()
        .at_latest()
        .await?
        .fetch(&substrate::storage().system().account(&account_id))
        .await?;

    match account_info {
        Some(info) => {
            println!("Account: {:?}", account_id);
            println!("Free Balance: {}", info.data.free);
            println!("Reserved Balance: {}", info.data.reserved);
            println!("Total Balance: {}", info.data.free + info.data.reserved);
        }
        None => println!("Account not found"),
    }

    Ok(())
}
