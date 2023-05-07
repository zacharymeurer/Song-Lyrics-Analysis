
pub mod file_reading {
    use std::error::Error;
    use serde::Deserialize;
    use crate::graph::graph::Edge;

    #[derive(Debug, Deserialize)]
    #[derive(Clone)]
    #[allow(non_snake_case)]
    pub struct Song {
        pub Artist: String,
        pub Title: String,
        pub Lyrics: String,
    }

    #[derive(Debug, Deserialize)]
    struct BossaSong {
        song_name: String,
        artist: String,
        song_lyrics: String,
        song_composition: String,
        song_lang: String,
    }

    mod shared_words_helpers {
        use std::collections::HashSet;
        use super::Song;
        use crate::graph::graph::Edge;
        
        pub fn clean_lyrics(lyrics: &String) -> HashSet<String> { //gets rid of trash in csv file and returns unique lyrics
            let temp = lyrics.replace(|c: char| !c.is_alphanumeric() & !(c==' '), "");
            let temp = temp.to_lowercase();
            let mut temp = temp.split(" ").collect::<Vec<&str>>();
            temp.retain(|&x| x != "");
            temp.retain(|&x| x != "urlcopyembedcopy");
            let temp = temp.into_iter().map(|x| x.to_string()).collect();
            return temp
        }
    
        pub fn shared_to_edge(song1: &Song, song2: &Song, shared: Vec<String>) -> Edge { //turns values into edge after cleaning
            let artist_title1 = format!("{} - {}", song1.Artist.replace("\u{200b}", ""), song1.Title.replace("\u{200b}", ""));
            let artist_title2 = format!("{} - {}", song2.Artist.replace("\u{200b}", ""), song2.Title.replace("\u{200b}", ""));
            let len = shared.len();
            return ((artist_title1,0), (artist_title2,0), shared, len)
        }
    }

    pub fn shared_words(song1: &Song, song2: &Song) -> Edge { //helper function for read_csv; gives edge containing shared words
        use shared_words_helpers::clean_lyrics;
        use shared_words_helpers::shared_to_edge;
        let unique1 = clean_lyrics(&song1.Lyrics);
        let unique2 = clean_lyrics(&song2.Lyrics);
        let shared = unique1.intersection(&unique2).into_iter().map(|f| f.to_string()).collect::<Vec<String>>();
        return shared_to_edge(song1, song2, shared)
    }

    pub fn read_songs_csv_to_edges(path: &str) -> Result<Vec<Edge>, Box<dyn Error>> { //reads csv, then turns it to edges
        let mut reader = csv::Reader::from_path(path)?;
        let mut list_of_edges: Vec<Edge> = Vec::new();
        let mut records: Vec<Song> = Vec::new();
        for result in reader.deserialize() {
            let current_record: Song = result?;
            for record in records.iter(){
                //check whether record (current Song) shares any lyrics with songs already in the songs list
                let edge: Edge = shared_words(&current_record, record);
                if edge.3 > 0 { //if there are shared_lyrics, then add the edge
                    list_of_edges.push(edge);
                }
            }
            
            records.push(current_record);
        }
        return Ok(distance_fix::num_shared_words_to_distance(list_of_edges))
    }

    pub fn read_bossa_csv_to_edges(path: &str) -> Result<Vec<Edge>, Box<dyn Error>> { //reads csv, then turns it to edges
        let mut reader = csv::Reader::from_path(path)?;
        let mut list_of_edges: Vec<Edge> = Vec::new();
        let mut records: Vec<Song> = Vec::new();
        for result in reader.deserialize() {
            //collects from csv file to BossaSong, then converts BossaSong to Song
            let bossa_record: BossaSong = result?;
            let current_record = Song {
                Artist: bossa_record.artist,
                Title: bossa_record.song_name,
                Lyrics: bossa_record.song_lyrics,
            };
            
            for record in records.iter(){
                //check whether record (current Song) shares any lyrics with songs already in the songs list
                let edge: Edge = shared_words(&current_record, record);
                if edge.3 > 0 { //if there are shared_lyrics, then add the edge
                    list_of_edges.push(edge);
                }
            }
            
            records.push(current_record);
        }
        return Ok(distance_fix::num_shared_words_to_distance(list_of_edges))
    }

    /*fn bossa_to_song(bossa_record: Bossa_Song) -> Song {
        let current_record = Song {
            Artist: bossa_record.artist,
            Title: bossa_record.song_name,
            Lyrics: bossa_record.song_lyrics,
        };
        return current_record
    }*/

    mod distance_fix { //Distance = Max edge's # of shared lyrics - This edge's # of shared lyrics + 1
        use crate::graph::graph::Edge;
        pub fn num_shared_words_to_distance(list_of_edges: Vec<Edge>) -> Vec<Edge> {
            //Songs with the most shared lyrics have the smallest distance (1)
            //Songs with the least shared lyrics have the largest distance (n: max # of shared words)
            let mut fixed_edges: Vec<Edge> = Vec::new();
            let max = max_shared_words(&list_of_edges).3;
            for (s1, s2, shared, distance) in list_of_edges.iter() {
                fixed_edges.push((s1.clone(),s2.clone(),shared.clone(),max-distance+1))
            }
            return fixed_edges
        }
    
        fn max_shared_words(list_of_edges: &Vec<Edge>) -> Edge { //Finds Edge with greatest number of shared_lyrics before change in distance
            let mut best_edge: Edge = list_of_edges[0].clone();
            for edge in list_of_edges.iter() {
                if edge.3 > best_edge.3 {
                    best_edge = edge.clone();
                }
            }
            return best_edge
        }
    }


}
