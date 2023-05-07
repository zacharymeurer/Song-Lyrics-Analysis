pub mod graph {
    use std::collections::HashMap;
    //use crate::file_reading::file_reading::{self, Song};


    type Vertex = String;
    type Distance = usize;
    type SharedLyrics = Vec<String>;
    pub type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    #[derive(Debug)]
    pub struct Graph {
        n: usize,
        pub outedges: Vec<Edge>,
        parent: Vec<Vertex>,
        rank: Vec<usize>,
    }


    impl Graph {

        fn get_parents(outedges: &Vec<Edge>) -> Vec<String> {
            let mut parents: Vec<String> = Vec::new();
            for edge in outedges.iter() {
                let title1 = &edge.0;
                let title2 = &edge.1;
                //find unique song_names and append them to parents
                if !(parents.contains(&title1)) {
                    parents.push((&edge.0).to_string())
                } 
                if !(parents.contains(&title2)) {
                    parents.push((&edge.1).to_string())
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

        fn union(&mut self, i:Vertex, j:Vertex) {
            let i_index = self.parent.iter().position(|x| x == &i).unwrap();
            let j_index = self.parent.iter().position(|x| x == &j).unwrap();
            if self.rank[i_index] < self.rank[j_index] {
                self.parent[i_index] = j;
            } else if self.rank[i_index] > self.rank[j_index] {
                self.parent[j_index] = i;
            } else {
                self.parent[j_index] = i;
                self.rank[i_index] += 1;
            }
        }

        fn find(&mut self, i:Vertex) -> Vertex {
            println!("{}",i);
            let i_index = self.parent.iter().position(|x| x == &i).unwrap();
            if self.parent[i_index] != i {
                self.parent[i_index] = self.find(self.parent[i_index].clone());
            }
            println!("passed");
            return self.parent[i_index].clone();
        }

        pub fn kruskal_mst(&mut self) -> Vec<Edge> {
            let mut result: Vec<Edge> = vec![];
            let mut num_mst_e = 0;
            let mut next_edge = 0;
            while num_mst_e < self.n - 1 {
                let edge = self.outedges[next_edge].clone();
                next_edge = next_edge + 1;
                //let test1 = .clone();
                //let test2 = s2.clone();
                let x = self.find((&edge.0).to_string());
                let y = self.find((&edge.1).to_string());
                if x != y {
                    num_mst_e += 1;
                    result.push(edge);
                    self.union(x,y);
                }

            }
            return result
        }

        fn closeness_centrality(&self) -> f64 {
            //let mut closeness_values: HashMap<&String, usize> = HashMap::new();
            
            return 4.0
        }
    }
    
}