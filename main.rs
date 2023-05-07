mod file_reading;
mod graph;
fn main() {
    use std::time;
    use file_reading::file_reading::read_bossa_csv_to_edges;
    use file_reading::file_reading::read_songs_csv_to_edges;
    use graph::graph::Graph;
    let now = time::Instant::now();
    let songs = read_songs_csv_to_edges("Songs.csv").unwrap();
    let bossa = read_bossa_csv_to_edges("bossa_nova_songs_dataset_sample.csv").unwrap();
    let datasets = vec![songs,bossa];
    for initial_edges in datasets{
        let mut g = Graph::create_undirected(initial_edges);
        println!("The number of shared lyric connections between songs is {}, with the max possible number being {}\n",
                g.outedges.len(),((g.n*(g.n-1)) as f64)/2.0); // max edges = n(n-1)/2

        let most_shared_edge = g.min_distance();
        println!("The songs {:?} and {:?} have the most ({:?}) lyrics in common: {:?}\n",
                most_shared_edge.0.0,most_shared_edge.1.0,most_shared_edge.2.len(),most_shared_edge.2);

        let most_connected_song = g.highest_degree_node();
        println!("The song that shares lyrics with the most other songs is {} with connections to {} other songs\n",
                most_connected_song.0,most_connected_song.1);

        let closeness = g.closeness_centrality();
        println!("The normalized closeness centrality value of the graph is {}\n",
                closeness);

        let elapsed_time = now.elapsed();
        println!("Running slow_function() took {} seconds.\n\n\n\n", elapsed_time.as_secs());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shared_words() {
        use crate::file_reading::file_reading::Song;
        use crate::file_reading::file_reading::shared_words;
        let s1 = Song{Artist: "Nicolas Minajj".to_string(), Title: "Superbutt".to_string(), Lyrics: "Tree elephant 1 2 56".to_string()};
        let s2 = Song{Artist: "Nicolas Minajj".to_string(), Title: "Superbs".to_string(), Lyrics: "Tree 5, shock through every bone".to_string()};
    
        let truth = (("Nicolas Minajj - Superbutt".to_string(), 0 as usize),("Nicolas Minajj - Superbs".to_string(), 0 as usize), vec!["tree".to_string()], 1 as usize);
        assert_eq!(shared_words(&s1,&s2), truth, "test_shared_words is broken");
    }
    #[test]
    fn test_min_distance() {
        use crate::file_reading::file_reading::read_songs_csv_to_edges;
        use crate::graph::graph::Graph;
        let initial_edges = read_songs_csv_to_edges("Songs_test.csv").unwrap();
        let g = Graph::create_undirected(initial_edges);
        let truth = (("Nicolas Minajj - Superball".to_string(), 2 as usize), ("Nicolas Minajj - Superbeat".to_string(), 0 as usize), ["the", "what", "hes", "if"], 1);
        assert_eq!(g.min_distance().0, truth.0, "test_min_distance is broken");
        assert_eq!(g.min_distance().1, truth.1, "test_min_distance is broken");
        assert_eq!(g.min_distance().3, truth.3, "test_min_distance is broken");
    }

    #[test]
    fn test_highest_degree_node() {
        use crate::file_reading::file_reading::read_songs_csv_to_edges;
        use crate::graph::graph::Graph;
        let initial_edges = read_songs_csv_to_edges("Songs_test.csv").unwrap();
        let g = Graph::create_undirected(initial_edges);
        let truth = ("Nicolas Minajj - Superbass".to_string(), 4 as usize);
        assert_eq!(g.highest_degree_node(),truth);
    }

    #[test]
    fn test_closeness_centrality() {
        use crate::file_reading::file_reading::read_songs_csv_to_edges;
        use crate::graph::graph::Graph;
        let initial_edges = read_songs_csv_to_edges("Songs_test.csv").unwrap();
        let mut g = Graph::create_undirected(initial_edges);
        let truth = [6.0/23.0,6.0/22.0,6.0/21.0,6.0/20.0,6.0/19.0];
        let predict = g.closeness_centrality();
        assert!(truth.iter().any(|&x| x == predict));
    }
}
