// This Source Code Form is subject to the terms of the GNU General Public
// License, version 3. If a copy of the GPL was not distributed with this file,
// You can obtain one at https://www.gnu.org/licenses/gpl.txt.
extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate approx;
#[macro_use]
extern crate bitflags;
extern crate bytes;
#[macro_use]
extern crate downcast_rs;
#[macro_use]
extern crate failure;
extern crate failure_derive;
extern crate futures;
#[macro_use]
extern crate json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate simplelog;
#[macro_use]
extern crate structopt;
extern crate yggdrasil;

mod oh;
mod web;

use actix::prelude::*;
use failure::Fallible;
use oh::{DBServer, Hue, LegacyMCU};
use simplelog::{Config, LevelFilter, TermLogger};
use std::path::PathBuf;
use structopt::StructOpt;
use web::server::build_server;

#[derive(StructOpt, Debug)]
#[structopt(name = "open_house")]
struct Opt {
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    #[structopt(short = "h", long = "host")]
    host: Option<String>,

    #[structopt(short = "p", long = "port")]
    port: Option<u16>,

    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    run(opt).unwrap();
}

fn run(opt: Opt) -> Fallible<()> {
    let level = match opt.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    TermLogger::init(level, Config::default())?;

    let sys = System::new("open_house");

    let db = DBServer::new_from_file(&opt.config)?;
    let button_path_map = db
        .legacy_mcu
        .inspect_as(&|mcu: &LegacyMCU| &mcu.path_map)?
        .clone();
    let db_addr = db.start();

    let _server_server = build_server(
        db_addr,
        button_path_map,
        "openhouse.eyrie",
        &opt.host.unwrap_or("localhost".to_string()),
        opt.port.unwrap_or(5000),
    )?;
    //let _server_addr = server.start();

    //tree_addr.send(AddHandler())

    let _ = sys.run();
    return Ok(());
}
