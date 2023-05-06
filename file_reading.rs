
pub mod file_reading {
    use std::collections::HashSet;
    use std::error::Error;
    use serde::Deserialize;
    use crate::graph::graph::Edge;

    #[derive(Debug, Deserialize)]
    #[derive(Clone)]
    #[allow(non_snake_case)]
    pub struct Song {
        Artist: String,
        Title: String,
        Lyrics: String,
    }

    //type Vertex = String;
    //type SharedLyrics = Vec<String>;
    //type Distance = usize;
    //type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    pub fn read_csv_to_edges(path: &str) -> Result<Vec<Edge>, Box<dyn Error>> {//Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut list_of_edges: Vec<Edge> = Vec::new();
        let mut records: Vec<Song> = Vec::new();
        for result in reader.deserialize() {
            let current_record: Song = result?;
            for record in records.iter(){
                //check whether record (current Song) shares any lyrics with songs already in lyrics list
                let edge: Edge = shared_words(&current_record, record);
                if edge.3 > 0 {
                    list_of_edges.push(edge);
                }
                //edges.push(shared_words(&current_record, record));
            }
            
            records.push(current_record);
        }
        return Ok(num_shared_words_to_distance(list_of_edges))
    }

    fn num_shared_words_to_distance(list_of_edges: Vec<Edge>) -> Vec<Edge> {
        //Songs with the most shared lyrics have the smallest distance
        //Songs with the least shared lyrics have the largest distance
        let mut fixed_edges: Vec<Edge> = Vec::new();
        let mut max_shared_words = 0;
        for (_, _, _, distance) in list_of_edges.iter() {
            if distance > &max_shared_words {
                max_shared_words = *distance
            }
        }
        for (s1, s2, shared, distance) in list_of_edges.iter() {
            fixed_edges.push((s1.clone(),s2.clone(),shared.clone(),max_shared_words-distance))
        }
        return fixed_edges
    }

    fn clean_lyrics(lyrics: &String) -> HashSet<String> {
        let temp = lyrics.replace(|c: char| !c.is_alphanumeric() & !(c==' '), "");
        let temp = temp.to_lowercase();
        let mut temp = temp.split(" ").collect::<Vec<&str>>();
        temp.retain(|&x| x != "");
        temp.retain(|&x| x != "urlcopyembedcopy");
        temp.retain(|&x| x != "\\u{200b}");
        let temp = temp.into_iter().map(|x| x.to_string()).collect();
        return temp
    }

    fn shared_to_edge(song1: &Song, song2: &Song, shared: Vec<String>) -> Edge {
        let artist_title1 = format!("{} - {}", song1.Artist.replace("\u{200b}", ""), song1.Title.replace("\u{200b}", ""));
        let artist_title2 = format!("{} - {}", song2.Artist.replace("\u{200b}", ""), song2.Title.replace("\u{200b}", ""));
        let len = calculate_length(&shared);
        return (artist_title1, artist_title2, shared, len)
    }

    fn calculate_length(shared: &Vec<String>) -> usize {
        return shared.len()
    }


    //helper function for read_csv
    fn shared_words(song1: &Song, song2: &Song) -> Edge {
        let unique1 = clean_lyrics(&song1.Lyrics);
        let unique2 = clean_lyrics(&song2.Lyrics);
        let shared = unique1.intersection(&unique2).into_iter().map(|f| f.to_string()).collect::<Vec<String>>();
        return shared_to_edge(song1, song2, shared)
    }


}
