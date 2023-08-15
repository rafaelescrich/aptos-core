// Copyright © Aptos Foundation

use namable_closures::{Closure, closure};
use crate::graph::{NodeIndex, WeightedGraph};
use rand::seq::SliceRandom;
use aptos_types::closuretools::{ClosureTools, MapClosure};

/// A trait for batched streams for undirected graphs with weighted nodes and edges.
pub trait GraphStream: Sized {
    /// The weight of a node.
    type NodeWeight;

    /// The weight of an edge.
    type EdgeWeight;

    /// An iterator over the neighbours of a node in the graph.
    type NeighboursIter<'a>: Iterator<Item = (NodeIndex, Self::EdgeWeight)>
    where
        Self: 'a;

    /// An iterator over the nodes in a batch.
    type Batch<'a>: IntoIterator<Item = (NodeIndex, Self::NodeWeight, Self::NeighboursIter<'a>)>
    where
        Self: 'a;

    /// Advances the stream and returns the next value.
    ///
    /// Returns [`None`] when stream is finished.
    fn next_batch(&mut self) -> Option<Self::Batch<'_>>;

    /// Returns the total number of batches remaining in the stream, if available.
    fn opt_remaining_batch_count(&self) -> Option<usize> {
        None
    }

    /// Returns the total number of nodes in all remaining batches of the stream combined,
    /// if available.
    fn opt_remaining_node_count(&self) -> Option<usize> {
        None
    }

    /// Returns the total number of nodes in the whole graph, including already processed batches,
    /// if available.
    fn opt_total_node_count(&self) -> Option<usize> {
        None
    }

    /// Returns the total number of edges in the whole graph, including already processed batches,
    /// if available.
    fn opt_total_edge_count(&self) -> Option<usize> {
        None
    }

    /// Returns the total weight of all nodes in the whole graph,
    /// including already processed batches, if available.
    fn opt_total_node_weight(&self) -> Option<Self::NodeWeight> {
        None
    }

    /// Returns the total weight of all edges in the whole graph,
    /// including already processed batches, if available.
    fn opt_total_edge_weight(&self) -> Option<Self::EdgeWeight> {
        None
    }
}

/// A trait for graph streams with known exact node count.
pub trait ExactNodeCountGraphStream: GraphStream {
    fn remaining_node_count(&self) -> usize {
        self.opt_remaining_node_count().unwrap()
    }

    fn total_node_count(&self) -> usize {
        self.opt_total_node_count().unwrap()
    }
}

// A mutable reference to a `GraphStream` is a `GraphStream` itself.
impl<'a, S> GraphStream for &'a mut S
where
    S: GraphStream,
{
    type NodeWeight = S::NodeWeight;
    type EdgeWeight = S::EdgeWeight;

    type NeighboursIter<'b> = S::NeighboursIter<'b>
    where
        Self: 'b;

    type Batch<'b> = S::Batch<'b>
    where
        Self: 'b;

    fn next_batch(&mut self) -> Option<Self::Batch<'_>> {
        (**self).next_batch()
    }

    fn opt_remaining_batch_count(&self) -> Option<usize> {
        (**self).opt_remaining_batch_count()
    }

    fn opt_remaining_node_count(&self) -> Option<usize> {
        (**self).opt_remaining_node_count()
    }

    fn opt_total_node_count(&self) -> Option<usize> {
        (**self).opt_total_node_count()
    }

    fn opt_total_edge_count(&self) -> Option<usize> {
        (**self).opt_total_edge_count()
    }

    fn opt_total_node_weight(&self) -> Option<Self::NodeWeight> {
        (**self).opt_total_node_weight()
    }

    fn opt_total_edge_weight(&self) -> Option<Self::EdgeWeight> {
        (**self).opt_total_edge_weight()
    }
}

/// A trait for a generic graph streamer.
pub trait GraphStreamer<G: WeightedGraph> {
    type Stream<'graph>: GraphStream<
        NodeWeight = G::NodeWeight,
        EdgeWeight = G::EdgeWeight,
    >
    where
        Self: 'graph,
        G: 'graph;

    fn stream<'graph>(&self, graph: &'graph G) -> Self::Stream<'graph>;
}

/// Streams graphs in batches of fixed size, in order from `0` to `node_count() - 1`.
pub struct InputOrderGraphStreamer {
    batch_size: usize,
}

impl InputOrderGraphStreamer {
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }
}

impl<G: WeightedGraph> GraphStreamer<G> for InputOrderGraphStreamer {
    type Stream<'graph> = InputOrderGraphStream<'graph, G>
    where
        G: 'graph;

    fn stream<'graph>(&self, graph: &'graph G) -> Self::Stream<'graph> {
        InputOrderGraphStream::new(graph, self.batch_size)
    }
}

/// Streams graphs in batches of fixed size, in random order.
pub struct RandomOrderGraphStreamer {
    // TODO: add support for custom RNG / seed.
    batch_size: usize,
}

impl RandomOrderGraphStreamer {
    /// Creates a new `RandomOrderGraphStreamer` with the given batch size.
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }
}

impl<G: WeightedGraph> GraphStreamer<G> for RandomOrderGraphStreamer {
    type Stream<'graph> = RandomOrderGraphStream<'graph, G>
    where
        Self: 'graph,
        G: 'graph;

    fn stream<'graph>(&self, graph: &'graph G) -> Self::Stream<'graph> {
        RandomOrderGraphStream::new(graph, self.batch_size)
    }
}

/// Streams a graph in batches of fixed size, in order from `0` to `node_count() - 1`.
pub struct InputOrderGraphStream<'graph, G> {
    graph: &'graph G,
    batch_size: usize,
    current_node: NodeIndex,
}

impl<'graph, G: WeightedGraph> InputOrderGraphStream<'graph, G> {
    pub fn new(graph: &'graph G, batch_size: usize) -> Self {
        Self {
            graph,
            batch_size,
            current_node: 0,
        }
    }
}

impl<'graph, G> GraphStream for InputOrderGraphStream<'graph, G>
where
    G: WeightedGraph,
{
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;

    type NeighboursIter<'a> = G::WeightedNeighboursIter<'a>
        where
            Self: 'a;

    type Batch<'a> = MapClosure<
        std::ops::Range<NodeIndex>,
        Closure<'a, Self, (NodeIndex,), (NodeIndex, Self::NodeWeight, Self::NeighboursIter<'a>)>,
    >
    where
        Self: 'a;

    fn next_batch(&mut self) -> Option<Self::Batch<'_>> {
        if self.current_node == self.graph.node_count() as NodeIndex {
            return None;
        }

        let batch_start = self.current_node;
        self.current_node = (self.current_node + self.batch_size as NodeIndex)
            .min(self.graph.node_count() as NodeIndex);

        Some(
            (batch_start..self.current_node).map_closure(closure!(self_ = self => |node| {
                let node_weight = self_.graph.node_weight(node);
                let neighbours = self_.graph.weighted_edges(node);
                (node, node_weight, neighbours)
            }))
        )
    }

    fn opt_remaining_batch_count(&self) -> Option<usize> {
        self.opt_remaining_node_count()
            .map(|count| (count + self.batch_size - 1) / self.batch_size)
    }

    fn opt_remaining_node_count(&self) -> Option<usize> {
        Some(self.graph.node_count() - self.current_node as usize)
    }

    fn opt_total_node_count(&self) -> Option<usize> {
        Some(self.graph.node_count())
    }

    fn opt_total_edge_count(&self) -> Option<usize> {
        Some(self.graph.edge_count())
    }

    fn opt_total_node_weight(&self) -> Option<Self::NodeWeight> {
        Some(self.graph.total_node_weight())
    }

    fn opt_total_edge_weight(&self) -> Option<Self::EdgeWeight> {
        Some(self.graph.total_edge_weight())
    }
}

/// Streams a graph in batches of fixed size, in random order.
pub struct RandomOrderGraphStream<'graph, G> {
    graph: &'graph G,
    batch_size: usize,
    order: Vec<NodeIndex>,
    current_node: NodeIndex,
}

impl<'graph, G: WeightedGraph> RandomOrderGraphStream<'graph, G> {
    pub fn new(graph: &'graph G, batch_size: usize) -> Self {
        let mut order: Vec<_> = graph.nodes().collect();
        let mut rng = rand::thread_rng();
        order.shuffle(&mut rng);

        Self {
            graph,
            batch_size,
            order,
            current_node: 0,
        }
    }
}

impl<'graph, G> GraphStream for RandomOrderGraphStream<'graph, G>
where
    G: WeightedGraph,
{
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;

    type NeighboursIter<'a> = G::WeightedNeighboursIter<'a>
    where
        Self: 'a;

    type Batch<'a> = MapClosure<
        std::iter::Copied<std::slice::Iter<'a, NodeIndex>>,
        Closure<'a, Self, (NodeIndex,), (NodeIndex, Self::NodeWeight, Self::NeighboursIter<'a>)>
    >
    where
        Self: 'a;

    fn next_batch(&mut self) -> Option<Self::Batch<'_>> {
        if self.current_node == self.order.len() as NodeIndex {
            return None;
        }

        let batch_start = self.current_node;
        self.current_node =
            (self.current_node + self.batch_size as NodeIndex).min(self.order.len() as NodeIndex);

        Some(
            (&self.order[batch_start as usize..self.current_node as usize])
                .into_iter()
                .copied()
                .map_closure(closure!(self_ = self => |node| {
                    let node_weight = self_.graph.node_weight(node);
                    let neighbours = self_.graph.weighted_edges(node);
                    (node, node_weight, neighbours)
                })),
        )
    }

    fn opt_remaining_batch_count(&self) -> Option<usize> {
        self.opt_remaining_node_count()
            .map(|count| (count + self.batch_size - 1) / self.batch_size)
    }

    fn opt_remaining_node_count(&self) -> Option<usize> {
        Some(self.graph.node_count() - self.current_node as usize)
    }

    fn opt_total_node_count(&self) -> Option<usize> {
        Some(self.graph.node_count())
    }

    fn opt_total_edge_count(&self) -> Option<usize> {
        Some(self.graph.edge_count())
    }

    fn opt_total_node_weight(&self) -> Option<Self::NodeWeight> {
        Some(self.graph.total_node_weight())
    }

    fn opt_total_edge_weight(&self) -> Option<Self::EdgeWeight> {
        Some(self.graph.total_edge_weight())
    }
}
