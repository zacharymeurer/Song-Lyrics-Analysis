pub mod graph {
    use std::collections::HashMap;

    type Vertex = (String,usize);
    type Distance = usize;
    type SharedLyrics = Vec<String>;
    pub type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    #[derive(Debug)]
    pub struct Graph {
        pub n: usize,
        pub outedges: Vec<Edge>,
        parent: Vec<Vertex>,
        rank: Vec<usize>,
    }


    impl Graph {

        fn get_parents(outedges: &Vec<Edge>) -> Vec<Vertex> { //Gets parents and gives each a unique index for mst function
            let mut parents: Vec<Vertex> = Vec::new();
            let mut index: usize = 0;
            for edge in outedges.iter() {
                let title1 = &edge.0.0;
                let title2 = &edge.1.0;
                //find unique song_names and append them to parents
                if !(parents.iter().any(|i| &i.0==title1)) {
                    parents.push((edge.0.0.clone(),index));
                    index += 1;
                } 
                if !(parents.iter().any(|i| &i.0==title2)) {
                    parents.push((edge.1.0.clone(),index));
                    index += 1;
                }
            }
            return parents
        }


        pub fn create_undirected(mut outedges:Vec<Edge>) -> Graph { //turns vector of edges to undirected graph
            let rank: Vec<usize> = vec![];
            let parent: Vec<Vertex> = Graph::get_parents(&outedges);
            for edge in outedges.iter_mut(){
                for p in parent.iter(){
                    if edge.0.0 == p.0 {
                        edge.0.1 = p.1;
                    }
                    if edge.1.0 == p.0 {
                        edge.1.1 = p.1;
                    }
                }
            }
            let n = parent.len();
            let mut g = Graph{n,outedges,parent,rank};
            g.outedges.sort_by(|a, b| a.2.cmp(&b.2));
            for _ in 0..n {
                g.rank.push(0);
            }
            return g
        }

        pub fn highest_degree_node(&self) -> (String, usize) { //Finds node with the most edges
            let mut song_appearances: HashMap<&Vertex, usize> = HashMap::new();
            for (song1,song2,_,_) in self.outedges.iter() {
                //Initializes song with 0 degrees if it doesn't already exist, then adds 1 degree to song for each edge it has
                song_appearances.entry(song1).and_modify(|degrees| *degrees += 1).or_insert(0);
                song_appearances.entry(song2).and_modify(|degrees| *degrees += 1).or_insert(0);
            }
            let mut max_degree: (String, usize) = ("".to_string(),0);
            for song in song_appearances.iter() { //finds vertex with most edges
                if &song.1 > &&max_degree.1 {
                    max_degree = (song.0.0.clone(), *song.1);
                }
            }
            return max_degree
        }
        
        fn union(&mut self, i:usize, j:usize) { //auxiliary function for kruskal_mst
            if self.rank[i] < self.rank[j] {
                self.parent[i].1 = j;
            } else if self.rank[i] > self.rank[j] {
                self.parent[j].1 = i;
            } else {
                self.parent[j].1 = i;
                self.rank[i] += 1;
            }
        }

        fn find(&mut self, i:usize) -> usize { //auxiliary function for kruskal_mst
            if self.parent[i].1 != i {
                self.parent[i].1 = self.find(self.parent[i].1);
            }
            return self.parent[i].1;
        }

        fn kruskal_mst(&mut self) -> Vec<Edge> { //finds minimum spanning tree
            let mut result: Vec<Edge> = vec![];
            let mut num_mst_e = 0;
            let mut next_edge = 0;
            while num_mst_e < self.n - 1 {
                let edge = self.outedges[next_edge].clone();
                next_edge = next_edge + 1;
                let x = self.find(edge.0.1);
                let y = self.find(edge.1.1);
                if x != y {
                    num_mst_e += 1;
                    result.push(edge);
                    self.union(x,y);
                }

            }
            return result
        }

        pub fn closeness_centrality(&mut self) -> f64 { //Normalized Closeness Centrality
            let mut min_dist = 0.0;
            for edge in self.kruskal_mst(){
                min_dist += edge.3 as f64;
            }
            return ((self.n as f64)-1.0)/min_dist
        }

        pub fn min_distance(&self) -> Edge { //Finds Edge with greatest number of shared_lyrics after change in distance
            let list_of_edges = &self.outedges;
            let mut best_edge: Edge = list_of_edges[0].clone();
            for edge in list_of_edges.iter() {
                if edge.3 < best_edge.3 {
                    best_edge = edge.clone();
                }
            }
            return best_edge
        }
    }
    
}