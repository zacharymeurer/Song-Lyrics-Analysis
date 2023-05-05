
pub mod file_reading {

    /*
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::prelude::*;

    
    pub fn read_file(path: &str) -> Vec<Vec<usize>> {
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        let mut n = 0;
        for line in buf_reader {
            let line_str = line.expect("Error reading");
            n = line_str.parse::<usize>().unwrap();
            break;
        }
        let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        for (i, line) in (buf_reader).enumerate() {
            let line_str = line.expect("Error reading");
            if i == 0 {
                continue;
            } else {
                let v: Vec<&str> = line_str.trim().split(' ').collect();
                graph_list[v[0].parse::<usize>().unwrap()].push(v[1].parse::<usize>().unwrap());
            }
        }
        return graph_list;
    }*/

    //use std::io;
    //use std::process;
    use std::collections::HashSet;
    use std::error::Error;
    use serde::Deserialize;
    
    #[derive(Debug, Deserialize)]
    #[derive(Clone)]
    #[allow(non_snake_case)]
    struct Song {
        Artist: String,
        Title: String,
        Lyrics: String,
    }

    pub fn read_csv(path: &str) -> Result<Vec<Edge>, Box<dyn Error>> {//Result<(), Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path)?;
        //let headers = reader.headers()?;
        //println!("{:?}",headers);
        let mut edges: Vec<Edge> = Vec::new();
        let mut records: Vec<Song> = Vec::new();
        for result in reader.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let current_record: Song = result?;
            for record in records.iter(){
                //check whether record (current Song) shares any lyrics with songs already in lyrics list
                edges.push(shared_words(current_record.clone(), record));
            }
            
            records.push(current_record);
        }
        //println!("{:?}", edges);
        //println!("=======Done=========");
        return Ok(edges)
    }


    type Vertex = String;
    type SharedLyrics = Vec<String>;
    type Distance = usize;
    type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    fn clean_lyrics(mut lyrics: String) -> HashSet<String> {
        lyrics = lyrics.replace(|c: char| !c.is_alphanumeric() & !(c==' '), "");
        lyrics = lyrics.to_lowercase();
        let mut lyrics = lyrics.split(" ").collect::<Vec<&str>>();
        lyrics.retain(|&x| x != "");
        lyrics.retain(|&x| x != "urlcopyembedcopy");
        let lyrics = lyrics.into_iter().map(|x| x.to_string()).collect();
        return lyrics
        /*let mut words = lyrics.split(" ").collect::<Vec<&str>>();
        words.retain(|&x| x != "");
        words.retain(|&x| x != "urlcopyembedcopy");
        let unique: HashSet<_> = words.into_iter().collect();
        return unique*/
    }

    fn shared_to_edge(song1: Song, song2: &Song, shared: Vec<String>) -> Edge {
        //let shared = &shared.iter().map(|f| f.to_string()).collect::<Vec<_>>();
        let artist_title1 = format!("{}, {}", song1.Artist, song1.Title);
        let artist_title2 = format!("{}, {}", song2.Artist, song2.Title);
        return (artist_title1, artist_title2, shared.clone(), shared.len())
    }


    //helper function for read_csv
    fn shared_words(song1: Song, song2: &Song) -> Edge {
    /*pub fn shared_words(mut lyrics1: String, mut lyrics2: String) -> Vec<Edge> {
        lyrics1 = lyrics1.replace(|c: char| !c.is_alphanumeric() & !(c==' '), "");
        lyrics1 = lyrics1.to_lowercase();
        let mut words1 = lyrics1.split(" ").collect::<Vec<&str>>();
        words1.retain(|&x| x != "");
        words1.retain(|&x| x != "urlcopyembedcopy");
        let unique_1: HashSet<_> = words1.into_iter().collect();

        lyrics2 = lyrics2.replace(|c: char| !c.is_alphanumeric() & !(c==' '), "");
        lyrics2 = lyrics2.to_lowercase();
        let mut words2 = lyrics2.split(" ").collect::<Vec<&str>>();
        words2.retain(|&x| x != "");
        words2.retain(|&x| x != "urlcopyembedcopy");
        let unique_2: HashSet<_> = words2.into_iter().collect();
        */
        let unique1 = clean_lyrics(song1.Lyrics.clone());
        let unique2 = clean_lyrics(song2.Lyrics.clone());
        let shared = unique1.intersection(&unique2).into_iter().map(|f| f.to_string()).collect::<Vec<String>>();
        //let shared = shared.iter().map(|f| f.to_string()).collect::<Vec<_>>();
        
        //println!("\n===========\n{:?}", shared);

        return shared_to_edge(song1, song2, shared)

        
    }


}
