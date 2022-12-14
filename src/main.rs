use clap::{arg, Arg, ArgAction, Command};
use colored_json::ToColoredJson;
use kickasstorrent::{
    formater::format_results,
    parser::Parser,
    types::{PopularOptions, SearchOptions},
};
use owo_colors::OwoColorize;
use tabled::{builder::Builder, Style};
use urlencoding::encode;

fn cli() -> Command<'static> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    Command::new("kickasstorrent")
        .version(VERSION)
        .author("Tsiry Sandratraina <tsiry.sndr@aol.com>")
        .about(
            r#"
  _  ___      _              _______                        _   
 | |/ (_)    | |            |__   __|                      | |  
 | ' / _  ___| | ____ _ ___ ___| | ___  _ __ _ __ ___ _ __ | |_ 
 |  < | |/ __| |/ / _` / __/ __| |/ _ \| '__| '__/ _ \ '_ \| __|
 | . \| | (__|   < (_| \__ \__ \ | (_) | |  | | |  __/ | | | |_ 
 |_|\_\_|\___|_|\_\__,_|___/___/_|\___/|_|  |_|  \___|_| |_|\__|
                                                               
Search torrents from kickasstorrents"#,
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("search")
                .about("Search for torrents")
                .arg(
                    Arg::with_name("query")
                        .help("The query to search for")
                        .required(true)
                        .index(1),
                )
                .arg(
                    arg!(-t --tv ... "Search torrents in the tv shows category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-g --games ... "Search torrents in the games category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-a --apps ... "Search torrents in the apps category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-o --other ... "Search torrents in the other category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-m --movies ... "Search torrents in the movies category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-u --music ... "Search torrents in the music category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-d --documentaries ... "Search torrents in the documentaries category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-x --xxx ... "Search torrents in the xxx category")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("latest").about("Show latest torrents").arg(
                Arg::with_name("category")
                    .help("The category to search")
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(Command::new("latest-searches").about("Show latest searches"))
        .subcommand(
            Command::new("popular")
                .about("Show popular torrents")
                .arg(arg!(-t --tv ... "Show popular tv shows").action(ArgAction::SetTrue))
                .arg(arg!(-g --games ... "Show popular games").action(ArgAction::SetTrue))
                .arg(arg!(-a --apps ... "Show popular apps").action(ArgAction::SetTrue))
                .arg(arg!(-o --other ... "Show popular other torrents").action(ArgAction::SetTrue))
                .arg(arg!(-m --movies ... "Show popular movies").action(ArgAction::SetTrue))
                .arg(arg!(-u --music ... "Show popular music").action(ArgAction::SetTrue))
                .arg(
                    arg!(-d --documentaries ... "Show popular documentaries")
                        .action(ArgAction::SetTrue),
                )
                .arg(arg!(-x --xxx ... "Show popular xxx torrents").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("category")
                .about("List torrents in a category")
                .arg(
                    arg!(-n --new ... "List torrents in the new category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-t --tv ... "List torrents in the tv shows category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-g --games ... "List torrents in the games category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-a --apps ... "List torrents in the apps category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-o --other ... "List torrents in the other category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-m --movies ... "List torrents in the movies category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-u --music ... "List torrents in the music category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-d --documentaries ... "List torrents in the documentaries category")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!(-x --xxx ... "List torrents in the xxx category")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("info")
                .about("Show information about a torrent")
                .arg(
                    Arg::with_name("id")
                        .help("The id of the torrent")
                        .required(true)
                        .index(1),
                ),
        )
        .arg(arg!(-j --json ... "Output results in json format").required(false))
}

#[tokio::main]
async fn main() -> Result<(), surf::Error> {
    let matches = cli().get_matches();
    let parser = Parser::new();

    let json = matches.is_present("json");

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = encode(sub_matches.get_one::<String>("query").unwrap());

            let options = SearchOptions {
                search_in_tv: *sub_matches.get_one::<bool>("tv").unwrap_or(&false),
                search_in_games: *sub_matches.get_one::<bool>("games").unwrap_or(&false),
                search_in_apps: *sub_matches.get_one::<bool>("apps").unwrap_or(&false),
                search_in_other: *sub_matches.get_one::<bool>("other").unwrap_or(&false),
                search_in_movies: *sub_matches.get_one::<bool>("movies").unwrap_or(&false),
                search_in_music: *sub_matches.get_one::<bool>("music").unwrap_or(&false),
                search_in_documentaries: *sub_matches
                    .get_one::<bool>("documentaries")
                    .unwrap_or(&false),
                search_in_xxx: *sub_matches.get_one::<bool>("xxx").unwrap_or(&false),
                search_in_anime: *sub_matches.get_one::<bool>("anime").unwrap_or(&false),
            };

            format_results(parser.search(&query, &options).await?, json);
        }
        Some(("latest", _)) => format_results(parser.get_latest().await?, json),
        Some(("latest-searches", _)) => {
            let result = parser.get_latest_searches().await?;
            let mut builder = Builder::default();
            builder.set_columns(vec!["Latest Searches".green().to_string()]);
            result.iter().for_each(|search| {
                builder.add_record(vec![search.to_string()]);
            });
            println!("\n{}", builder.build().with(Style::psql()));
            return Ok(());
        }
        Some(("popular", sub_matches)) => {
            let options = PopularOptions {
                popular_in_anime: *sub_matches.get_one::<bool>("anime").unwrap_or(&false),
                popular_in_tv: *sub_matches.get_one::<bool>("tv").unwrap_or(&false),
                popular_in_games: *sub_matches.get_one::<bool>("games").unwrap_or(&false),
                popular_in_apps: *sub_matches.get_one::<bool>("apps").unwrap_or(&false),
                popular_in_other: *sub_matches.get_one::<bool>("other").unwrap_or(&false),
                popular_in_movies: *sub_matches.get_one::<bool>("movies").unwrap_or(&false),
                popular_in_music: *sub_matches.get_one::<bool>("music").unwrap_or(&false),
                popular_in_documentaries: *sub_matches
                    .get_one::<bool>("documentaries")
                    .unwrap_or(&false),
                popular_in_xxx: *sub_matches.get_one::<bool>("xxx").unwrap_or(&false),
            };
            format_results(parser.get_popular(&options).await?, json);
        }
        Some(("category", sub_matches)) => {
            if *sub_matches.get_one::<bool>("new").unwrap() {
                format_results(parser.get_latest().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("tv").unwrap() {
                format_results(parser.get_tv().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("games").unwrap() {
                format_results(parser.get_games().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("apps").unwrap() {
                format_results(parser.get_apps().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("other").unwrap() {
                format_results(parser.get_other().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("movies").unwrap() {
                format_results(parser.get_movies().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("music").unwrap() {
                format_results(parser.get_music().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("documentaries").unwrap() {
                format_results(parser.get_documentaries().await?, json);
                return Ok(());
            }
            if *sub_matches.get_one::<bool>("xxx").unwrap() {
                format_results(parser.get_xxx().await?, json);
                return Ok(());
            }
            format_results(parser.get_all().await?, json);
        }
        Some(("info", sub_matches)) => {
            let id = sub_matches.get_one::<String>("id").unwrap();
            let result = parser.get_torrent_details(&id).await?;
            println!(
                "{}",
                serde_json::to_string(&result)
                    .unwrap()
                    .to_colored_json_auto()
                    .unwrap()
            );
            return Ok(());
        }
        _ => unreachable!(),
    }

    Ok(())
}
