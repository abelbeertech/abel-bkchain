use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use node_template_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, SessionConfig, StakingConfig, DemocracyConfig, 
	ElectionsConfig, CouncilConfig, TechnicalCommitteeConfig, opaque::SessionKeys,
	StakerStatus, Balance, currency::DOLLARS, WASM_BINARY, Signature, ImOnlineConfig,
};
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use sc_service::ChainType;
use hex_literal::hex;
use telemetry::TelemetryEndpoints;
use pallet_staking::Forcing;

// Note this is the URL for the telemetry server
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Babe
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		|| testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
				get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
				get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
				get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
				get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			],
			true,
		),
		vec![],
		None,
		None,
		None,
		None,
	)
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online }
}

fn testnet_genesis(initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {
	
	const STASH: Balance = 100 * DOLLARS;
	let num_endowed_accounts = endowed_accounts.len();
	
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		democracy: Some(DemocracyConfig::default()),
		elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		collective_Instance1: Some(CouncilConfig::default()),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
	}
}

// public staging network
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"My Staging Testnet",
		"my_staging",
		ChainType::Live,
		staging_testnet_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Westend Staging telemetry url is valid; qed")
		),
		Some("my_staging"),
		None,
		Default::default(),
	)
}

fn staging_testnet_genesis() -> GenesisConfig {
	// subkey inspect "$SECRET"
	let endowed_accounts = vec![
		//sudo key 5CZjNDdNk6jSDn4FfTtdn16YNuX1EYe5Mc2Cq2PMwDMfoFLJ
		hex!["d2006bef81c0c9e458ff16902fd239e2054d7d09dce9bb8941e8530159927066"].into(),
	];

	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ImOnlineId,
	)> = vec![(
		// 5Grpw9i5vNyF6pbbvw7vA8pC5Vo8GMUbG8zraLMmAn32kTNH
		hex!["9400300d911818337fdd63df73d9975594a9a62fbacccb3d3ff921fe1ad3bd25"].into(),
		// 5DLMZF33f61KvPDbJU5c2dPNQZ3jJyptsacpvsDhwNS1wUuU
		hex!["9400300d911818337fdd63df73d9975594a9a62fbacccb3d3ff921fe1ad3bd25"].into(),
		// 5Dhd2QbrSE4dyNn3YUg8j5TY3fG7ZAWZMoRRF9KUc7VPVGmC
		hex!["9400300d911818337fdd63df73d9975594a9a62fbacccb3d3ff921fe1ad3bd25"].unchecked_into(),
		// 5C6rkxAZB437B5Bf1yS4B4qjW4HZPeBp8Kzx2Se9FLKhfyHY
		hex!["bdf103ab104a9e65fbb09ba95dbcabbfd612c1a6d15bb4a25485129401474e81"].unchecked_into(),
		// 5DscuovXyY1o7DxYroYjYgipn87eqYLyQA3HJ21Utb7TqAai
		hex!["9400300d911818337fdd63df73d9975594a9a62fbacccb3d3ff921fe1ad3bd25"].unchecked_into(),
	),(
		// 5CFDk3yCSgQ2goiaksMfRMFRS7ZU28BZqPQDeAsgZUa6FRzt
		hex!["e65410f581bd803b5f5b4440d0f8bdfd27ce04dfc4c61555a646edc547126013"].into(),
		// 5F1ks2enazaPktQa3HURLK8GywzNZaGirovPtFvvbv91TLhJ
		hex!["e65410f581bd803b5f5b4440d0f8bdfd27ce04dfc4c61555a646edc547126013"].into(),
		// 5CQ7gVQj96m8y79qPCqrM291rSNREfZ1Tf2fiLPSJReWTNy2
		hex!["e65410f581bd803b5f5b4440d0f8bdfd27ce04dfc4c61555a646edc547126013"].unchecked_into(),
		// 5FyNaMc6GaioN7K9QzPJDEtGThJ1HmcruRdgtiRxaoAwn2VD
		hex!["aa4c10f7e9a452eca3a1b06924d2fec3c625888302e69f523acfce31a1247edd"].unchecked_into(),
		// 5EUhcM9WPJGvhCz1UptA7ye8TgktGqbhaeSohCkAfW76q5bS
		hex!["e65410f581bd803b5f5b4440d0f8bdfd27ce04dfc4c61555a646edc547126013"].unchecked_into(),
	),(
		// 5F6YideXfGcskpdFUczu3nZcJFmU9WKHgjjNVQjqgeVGRs66
		hex!["ee46c8a1601178a4f30a4a824e51e60644151db553f0a161425f8415bd6d3058"].into(),
		// 5F92x4qKNYaHtfp5Yy7kb9r6gHCHkN3YSvNuedERPHgrURTn
		hex!["ee46c8a1601178a4f30a4a824e51e60644151db553f0a161425f8415bd6d3058"].into(),
		// 5CLqVJSpfAdMYW1FHygEV8iEi8XFornEcCzrhw9WmFbbp8Qp
		hex!["ee46c8a1601178a4f30a4a824e51e60644151db553f0a161425f8415bd6d3058"].unchecked_into(),
		// 5HEQh8yEv4QU7joBCKYdjJJ57qU1gDAm4Xv5QZKfFnSbXpeo
		hex!["392e4be842eec7adb30302d516b963b72737936c64ad965c99e695285dce166b"].unchecked_into(),
		// 5GUEUCusMfW9c229gyuDG6XUH9pi3Cs4EZR9STtw8opfKuS6
		hex!["ee46c8a1601178a4f30a4a824e51e60644151db553f0a161425f8415bd6d3058"].unchecked_into(),
	)];
	
	const ENDOWMENT: u128 = 1_000_000 * DOLLARS;
	const STASH: u128 = 100 * DOLLARS;
	let num_endowed_accounts = endowed_accounts.len();
	
	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		sudo: Some(SudoConfig {
			key: endowed_accounts[0].clone(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(
					x.0.clone(),
					x.0.clone(),
					session_keys(x.2.clone(), x.3.clone(), x.4.clone())
				)
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::ForceNone,
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		democracy: Some(DemocracyConfig::default()),
		elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		collective_Instance1: Some(CouncilConfig::default()),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
	}
}

