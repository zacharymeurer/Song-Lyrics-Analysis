pub mod graph {
    use crate::file_reading::file_reading;


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

    impl Graph {
        pub fn create_undirected(outedges:Vec<Edge>) -> Graph {
            let parent: Vec<Vertex> = vec![];
            let rank: Vec<usize> = vec![];
            let parents = get_parents(&outedges);
            let n = parents.len();
            let mut g = Graph{n,outedges,parent,rank};
            g.outedges.sort_by(|a, b| a.2.cmp(&b.2));
            for node in parents {
                g.parent.push(node);
                g.rank.push(0);
            }
            return g
        }
    }
    
}