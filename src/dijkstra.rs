#[derive(Debug)]
#[derive(Clone)]
struct GraphNode {
    index: usize,
    minLengthFromStart: i32,
    minLengthFromStartNodeIndex: Option<usize>,
    processed: bool,
    edges: Vec<GraphEdge>
}

#[derive(Debug)]
#[derive(Clone)]
struct GraphEdge {
    weight: i32,
    remoteNodeIndex: usize
}


fn main () {

    let mut nodes: [GraphNode; 6] = [
        GraphNode {
            index: 0,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        },
        GraphNode {
            index: 1,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        },
        GraphNode {
            index: 2,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        },
        GraphNode {
            index: 3,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        },
        GraphNode {
            index: 4,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        },
        GraphNode {
            index: 5,
            minLengthFromStart: -1,
            minLengthFromStartNodeIndex: None,
            processed: false,
            edges: Vec::new()
        }
    ];

    nodes[0].edges.extend_from_slice(&[
        GraphEdge {
            weight: 14,
            remoteNodeIndex: 5
        },
        GraphEdge {
            weight: 9,
            remoteNodeIndex: 2
        },
        GraphEdge {
            weight: 7,
            remoteNodeIndex: 1
        }
    ]);

    let mut nodeStack = Vec::new();
    nodes[0].minLengthFromStart = 0;
    nodeStack.push(0);

    // FIXME: This part doesn't get compiled because of two mutable references
    while !nodeStack.is_empty() {
        let mut processedNode = &mut (nodes[nodeStack.pop().unwrap()]);
        processedNode.processed = true;
        for edge in &(processedNode.edges) {
            let mut remoteNode = &mut (nodes[edge.remoteNodeIndex]);
            if remoteNode.minLengthFromStart < 0 || remoteNode.minLengthFromStart > processedNode.minLengthFromStart + edge.weight {
                remoteNode.minLengthFromStart = processedNode.minLengthFromStart + edge.weight;
                remoteNode.minLengthFromStartNodeIndex = Some(processedNode.index);
            }
            if !remoteNode.processed {
                nodeStack.push(edge.remoteNodeIndex);
            }
        }
    }

    println!("Nodes {:#?}", nodes);
}
