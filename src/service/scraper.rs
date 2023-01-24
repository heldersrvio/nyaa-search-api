use crate::model::result::Result;

fn scrape(url: &str) -> Result<'static> {
    const result: Result = Result {
       category: "",
       name: "",
       download_link: "",
       magnet_link: "",
       size: 20.0,
       date: chrono::MAX_DATE,
       seeders: 0,
       leechers: 10,
       completed: 1,
    };

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrape() {
        const url: &str = "https://nyaa.si/?f=0&c=0_0&q=sword+art+online";
        const expected_result: Result = Result {
            category: "Literature - English-translated",
            name: "Sword Art Online - Girls' Ops v01-08 (Digital) (2016-2022) (Uasaha, Rurika, -KS-, Ushi)",
            download_link: "https://nyaa.si/download/1625825.torrent",
            magnet_link: "magnet:?xt=urn:btih:5d28f7f3762dd4a8747b86217c04209fa475a636&dn=Sword%20Art%20Online%20-%20Girls%27%20Ops%20v01-08%20%28Digital%29%20%282016-2022%29%20%28Uasaha%2C%20Rurika%2C%20-KS-%2C%20Ushi%29&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce",
            size: 697.6,
            date: chrono::MAX_DATE,
            seeders: 26,
            leechers: 2,
            completed: 286,
        };
        assert_eq!(scrape(url), expected_result); 
    }
}

