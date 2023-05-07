pub mod graph {

    //use crate::file_reading::file_reading::{self, Song};


    type Vertex = String;
    type Distance = usize;
    type SharedLyrics = Vec<String>;
    pub type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    #[derive(Debug)]
    pub struct Graph {
        n: usize,
        outedges: Vec<Edge>,
        parent: Vec<Vertex>,
        rank: Vec<usize>,
    }


    impl Graph {

        fn get_parents(outedges: &Vec<Edge>) -> Vec<String> {
            let mut parents: Vec<String> = Vec::new();
            for edge in outedges.iter() {
                let title = &edge.0;
                //find unique song_names and append them to parents
                if !(parents.contains(&title)) {
                    parents.push((&edge.0).to_string())
                }
            }
            return parents
        }

        pub fn create_undirected(outedges:Vec<Edge>) -> Graph {
            let parent: Vec<Vertex> = vec![];
            let rank: Vec<usize> = vec![];
            let parents = Graph::get_parents(&outedges);
            let n = parents.len();
            let mut g = Graph{n,outedges,parent,rank};
            g.outedges.sort_by(|a, b| a.2.cmp(&b.2));
            for node in parents {
                g.parent.push(node);
                g.rank.push(0);
            }
            return g
        }

        pub fn highest_degree_node(&self) -> (Vertex, usize) {
            use std::collections::HashMap;
            let mut song_appearances: HashMap<&String, usize> = HashMap::new();
            for (song1,song2,_,_) in self.outedges.iter() {
                song_appearances.entry(song1).and_modify(|degrees| *degrees += 1).or_insert(0);
                song_appearances.entry(song2).and_modify(|degrees| *degrees += 1).or_insert(0);
            }
            let mut max_degree: (String, usize) = ("".to_string(),0);
            for song in song_appearances.iter() {
                if &song.1 > &&max_degree.1 {
                    max_degree = (song.0.to_string(), *song.1);
                }
            }
            return max_degree
        }
    }
    
}