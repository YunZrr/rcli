use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    // 0 for not supported, 1 for supported
    #[arg(long, default_value_t = 1)]
    pub uppercase: u8,
    #[arg(long, default_value_t = 1)]
    pub lowercase: u8,
    #[arg(long, default_value_t = 1)]
    pub number: u8,
    #[arg(long, default_value_t = 1)]
    pub symbol: u8,
}
