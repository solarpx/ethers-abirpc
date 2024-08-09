use core::time::Duration;

/// An Ethereum EIP-155 chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, strum::IntoStaticStr)] // Into<&'static str>, AsRef<str>, fmt::Display and serde::Serialize
#[derive(strum::VariantNames)] // NamedChain::VARIANTS
#[derive(strum::VariantArray)] // NamedChain::VARIANTS
#[derive(strum::EnumString)] // FromStr, TryFrom<&str>
#[derive(strum::EnumIter)] // NamedChain::iter
#[derive(strum::EnumCount)] // NamedChain::COUNT
#[derive(num_enum::TryFromPrimitive)] // TryFrom<u64>
#[repr(u64)]
pub enum NamedChain {
    Mainnet = 1,
    Morden = 2,
    Ropsten = 3,
    Rinkeby = 4,
    Goerli = 5,
    Kovan = 42,
    Holesky = 17_000,
    Sepolia = 11_155_111,
    Optimism = 10,
    OptimismKovan = 69,
    OptimismGoerli = 420,
    OptimismSepolia = 11_155_420,
    Arbitrum = 42_161,
    ArbitrumTestnet = 421_611,
    ArbitrumGoerli = 421_613,
    ArbitrumSepolia = 421_614,
    ArbitrumNova = 42_170,
    Cronos = 25,
    CronosTestnet = 338,
    Rsk = 30,
    BinanceSmartChain = 56,
    BinanceSmartChainTestnet = 97,
    Poa = 99,
    Sokol = 77,
    Scroll = 534_352,
    ScrollSepolia = 534_351,
    Metis = 1_088,
    Gnosis = 100,
    Polygon = 137,
    PolygonMumbai = 80_001,
    PolygonAmoy = 80_002,
    PolygonZkEvm = 1_101,
    PolygonZkEvmTestnet = 1_442,
    Fantom = 250,
    FantomTestnet = 4_002,
    Moonbeam = 1_284,
    MoonbeamDev = 1_281,
    Moonriver = 1_285,
    Moonbase = 1_287,
    Dev = 1_337,
    AnvilHardhat = 31_337,
    Evmos = 9_001,
    EvmosTestnet = 9_000,
    Chiado = 10_200,
    Oasis = 26_863,
    Emerald = 42_262,
    EmeraldTestnet = 42_261,
    FilecoinMainnet = 314,
    FilecoinCalibrationTestnet = 314_159,
    Avalanche = 43_114,
    AvalancheFuji = 43_113,
    Celo = 42_220,
    CeloAlfajores = 44_787,
    CeloBaklava = 62_320,
    Aurora = 1_313_161_554,
    AuroraTestnet = 1_313_161_555,
    Canto = 7_700,
    CantoTestnet = 740,
    Boba = 288,
    Base = 8_453,
    BaseGoerli = 84_531,
    BaseSepolia = 84_532,
    Syndr = 404,
    SyndrSepolia = 444_444,
    Shimmer = 148,
    Fraxtal = 252,
    FraxtalTestnet = 2_522,
    Blast = 81_457,
    BlastSepolia = 168_587_773,
    Linea = 59_144,
    LineaGoerli = 59_140,
    ZkSync = 324,
    ZkSyncTestnet = 280,
    Mantle = 5_000,
    MantleTestnet = 5_001,
    MantleSepolia = 5_003,
    Viction = 88,
    Zora = 7_777_777,
    ZoraGoerli = 999,
    ZoraSepolia = 999_999_999,
    Pgn = 424,
    PgnSepolia = 58_008,
    Mode = 34_443,
    ModeSepolia = 919,
    Elastos = 20,
    KakarotSepolia = 1_802_203_764,
    EtherlinkTestnet = 128_123,
    Degen = 666_666_666,
    OpBNBMainnet = 204,
    OpBNBTestnet = 5_611,
    Ronin = 2_020,
    Taiko = 167_000,
    TaikoHekla = 167_009,
    AutonomysNovaTestnet = 490_000,
    Flare = 14,
    FlareCoston2 = 114,
    Acala = 787,
    AcalaMandalaTestnet = 595,
    AcalaTestnet = 597,
    Karura = 686,
    KaruraTestnet = 596,
}

impl Default for NamedChain {
    fn default() -> Self {
        Self::Mainnet
    }
}

impl AsRef<str> for NamedChain {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl NamedChain {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }

    pub const fn average_blocktime_hint(self) -> Option<Duration> {
        use NamedChain as C;

        Some(Duration::from_millis(match self {
            C::Mainnet | C::Taiko | C::TaikoHekla => 12_000,

            C::Arbitrum
            | C::ArbitrumTestnet
            | C::ArbitrumGoerli
            | C::ArbitrumSepolia
            | C::Syndr
            | C::SyndrSepolia
            | C::ArbitrumNova => 260,

            C::Optimism
            | C::OptimismGoerli
            | C::OptimismSepolia
            | C::Base
            | C::BaseGoerli
            | C::BaseSepolia
            | C::Blast
            | C::BlastSepolia
            | C::Fraxtal
            | C::FraxtalTestnet
            | C::Zora
            | C::ZoraGoerli
            | C::ZoraSepolia
            | C::Mantle
            | C::MantleSepolia
            | C::Mode
            | C::ModeSepolia
            | C::Pgn
            | C::PgnSepolia => 2_000,

            C::Viction => 2_000,

            C::Polygon | C::PolygonMumbai | C::PolygonAmoy => 2_100,

            C::Acala
            | C::AcalaMandalaTestnet
            | C::AcalaTestnet
            | C::Karura
            | C::KaruraTestnet
            | C::Moonbeam
            | C::Moonriver => 12_500,

            C::BinanceSmartChain | C::BinanceSmartChainTestnet => 3_000,

            C::Avalanche | C::AvalancheFuji => 2_000,

            C::Fantom | C::FantomTestnet => 1_200,

            C::Cronos | C::CronosTestnet | C::Canto | C::CantoTestnet => 5_700,

            C::Evmos | C::EvmosTestnet => 1_900,

            C::Aurora | C::AuroraTestnet => 1_100,

            C::Oasis => 5_500,

            C::Emerald => 6_000,

            C::Dev | C::AnvilHardhat => 200,

            C::Celo | C::CeloAlfajores | C::CeloBaklava => 5_000,

            C::FilecoinCalibrationTestnet | C::FilecoinMainnet => 30_000,

            C::Scroll | C::ScrollSepolia => 3_000,

            C::Shimmer => 5_000,

            C::Gnosis | C::Chiado => 5_000,

            C::Elastos => 5_000,

            C::EtherlinkTestnet => 5_000,

            C::Degen => 600,

            C::Morden
            | C::Ropsten
            | C::Rinkeby
            | C::Goerli
            | C::Kovan
            | C::Sepolia
            | C::Holesky
            | C::MantleTestnet
            | C::Moonbase
            | C::MoonbeamDev
            | C::OptimismKovan
            | C::Poa
            | C::Sokol
            | C::Rsk
            | C::EmeraldTestnet
            | C::Boba
            | C::ZkSync
            | C::ZkSyncTestnet
            | C::PolygonZkEvm
            | C::PolygonZkEvmTestnet
            | C::Metis
            | C::Linea
            | C::LineaGoerli
            | C::KakarotSepolia => return None,

            C::OpBNBMainnet | C::OpBNBTestnet | C::AutonomysNovaTestnet => 1_000,

            C::Ronin => 3_000,

            C::Flare => 1_800,

            C::FlareCoston2 => 2_500,
        }))
    }
}
