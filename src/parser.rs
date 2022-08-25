use std::time::Duration;

use scraper::{ElementRef, Html, Selector};
use surf::{Client, Config, Url};

use crate::{
    constants::{ANIME, APPS, BASE_URL, DOCS, GAMES, LATEST, MOVIES, MUSIC, OTHER, TV, XXX},
    types::{PopularOptions, SearchOptions, Torrent},
};

pub struct Parser {
    client: Client,
}

impl Parser {
    pub fn new() -> Self {
        let client: Client = Config::new()
            .set_base_url(Url::parse(BASE_URL).unwrap())
            .set_timeout(Some(Duration::from_secs(5)))
            .try_into()
            .unwrap();
        Parser { client }
    }

    pub async fn search(
        &self,
        query: &str,
        options: &SearchOptions,
    ) -> Result<Vec<Torrent>, surf::Error> {
        let page = self
            .client
            .get(format!("/search/{}", query))
            .recv_string()
            .await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_latest(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(LATEST).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_tv(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(TV).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_games(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(GAMES).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_apps(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(APPS).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_other(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(OTHER).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_movies(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(MOVIES).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_music(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(MUSIC).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_documentaries(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(DOCS).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_anime(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(ANIME).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_xxx(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get(XXX).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    pub async fn get_all(&self) -> Result<Vec<Torrent>, surf::Error> {
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);

        let categories: Vec<String> = self.parse_categories(&document);

        let torrents: Vec<Torrent> = self.parse_latest_torrents(&document, categories);
        Ok(torrents)
    }

    pub async fn get_latest_searches(&self) -> Result<Vec<String>, surf::Error> {
        let page = self.client.get("/").recv_string().await?;
        let document = Html::parse_document(&page);
        let selector = Selector::parse("#latestSearches").unwrap();
        let element = document.select(&selector).next().unwrap();
        let searches: Vec<_> = element
            .select(&Selector::parse("p").unwrap())
            .map(|e| e.text().collect::<Vec<_>>().join(" "))
            .collect();
        Ok(searches)
    }

    pub async fn get_popular(&self, options: &PopularOptions) -> Result<Vec<Torrent>, surf::Error> {
        let mut popular: String = "/popular".to_string();
        if options.popular_in_anime {
            popular = format!("{}-anime", popular);
        }
        if options.popular_in_tv {
            popular = format!("{}-tv", popular);
        }
        if options.popular_in_movies {
            popular = format!("{}-movies", popular);
        }
        if options.popular_in_music {
            popular = format!("{}-music", popular);
        }
        if options.popular_in_games {
            popular = format!("{}-games", popular);
        }
        if options.popular_in_apps {
            popular = format!("{}-apps", popular);
        }
        if options.popular_in_other {
            popular = format!("{}-other", popular);
        }
        if options.popular_in_xxx {
            popular = format!("{}-xxx", popular);
        }
        if options.popular_in_documentaries {
            popular = format!("{}-documentaries", popular);
        }
        let page = self.client.get(popular).recv_string().await?;
        let document = Html::parse_document(&page);
        let results = self.parse_torrents(&document);
        Ok(results)
    }

    fn parse_categories(&self, document: &Html) -> Vec<String> {
        let mut categories: Vec<String> = Vec::new();

        for h2 in document.select(&Selector::parse("h2").unwrap()) {
            categories.push(
                h2.text().collect::<Vec<_>>()[1]
                    .replace("\n", "")
                    .to_string(),
            );
        }

        return categories;
    }

    fn parse_torrents(&self, document: &Html) -> Vec<Torrent> {
        let selector = Selector::parse("tbody").unwrap();
        let mut documents = document.select(&selector);
        documents.next();
        let element = documents.next().unwrap();
        let mut torrents: Vec<Torrent> = Vec::new();

        for (i, tr) in element.select(&Selector::parse("tr").unwrap()).enumerate() {
            if i < 2 {
                continue;
            }

            let links = tr
                .select(&Selector::parse("a").unwrap())
                .collect::<Vec<_>>();
            let name = links[3]
                .text()
                .collect::<Vec<_>>()
                .join("")
                .replace("\n", "");

            let a = tr.select(&Selector::parse("a").unwrap()).next();
            let link = format!(
                "{}{}",
                BASE_URL,
                a.unwrap()
                    .value()
                    .attr("href")
                    .unwrap()
                    .replace("/download/", "/")
                    .to_string()
            );

            let mut torrent = self.parse_torrent_infos(tr);
            torrent.name = name;
            torrent.link = link;
            torrents.push(torrent);
        }
        return torrents;
    }

    fn parse_latest_torrents(&self, document: &Html, categories: Vec<String>) -> Vec<Torrent> {
        let selector = Selector::parse("tbody").unwrap();
        let mut documents = document.select(&selector);
        documents.next();

        let mut torrents: Vec<Torrent> = Vec::new();
        for (i, element) in documents.enumerate() {
            for tr in element.select(&Selector::parse("tr").unwrap()) {
                let a = tr.select(&Selector::parse("a").unwrap()).next();
                let mut name = String::new();
                let mut link = String::new();

                if a.is_some() && a.unwrap().value().attr("href").is_some() {
                    name = a.unwrap().text().next().unwrap().to_string();
                    link = format!(
                        "{}{}",
                        BASE_URL,
                        a.unwrap().value().attr("href").unwrap().to_string()
                    );
                }

                let mut torrent = self.parse_torrent_infos(tr);
                torrent.name = name;
                torrent.link = link;
                torrent.category = format!("{}", categories[i]);

                if torrent.name == "" && torrent.link == "" {
                    continue;
                }

                torrents.push(torrent);
            }
        }
        return torrents;
    }

    fn parse_torrent_infos(&self, tr: ElementRef) -> Torrent {
        let mut torrent = Torrent {
            name: String::new(),
            size: String::new(),
            uploader: String::new(),
            age: String::new(),
            seed: String::new(),
            leech: String::new(),
            category: String::new(),
            link: String::new(),
        };
        for (k, td) in tr.select(&Selector::parse("td").unwrap()).enumerate() {
            match k {
                1 => {
                    torrent.size = td.text().collect::<Vec<_>>()[0]
                        .replace("\n", "")
                        .to_string()
                }
                2 => torrent.uploader = td.text().collect::<Vec<_>>()[0].to_string(),
                3 => torrent.age = td.text().collect::<Vec<_>>()[0].to_string(),
                4 => torrent.seed = td.text().collect::<Vec<_>>()[0].to_string(),
                5 => torrent.leech = td.text().collect::<Vec<_>>()[0].to_string(),
                _ => (),
            }
            match k {
                1 => torrent.size = torrent.size.replace("\n", ""),
                2 => torrent.uploader = torrent.uploader.replace("\n", ""),
                3 => torrent.age = torrent.age.replace("\n", ""),
                4 => torrent.seed = torrent.seed.replace("\n", ""),
                5 => torrent.leech = torrent.leech.replace("\n", ""),
                _ => (),
            }
        }
        return torrent;
    }
}
