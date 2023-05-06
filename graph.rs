pub mod graph {

    type Vertex = String;
    type Distance = usize;
    type SharedLyrics = Vec<String>;
    pub type Edge = (Vertex, Vertex, SharedLyrics, Distance);

    #[derive(Debug)]
    struct Graph {
        n: usize,
        outedges: Vec<Edge>,
    }
}