// array.push() -- add element to back of queue
// array.shift() -- remove element from front of queue

const bfs1 = (start_node, nodes) => {
    let queue = [start_node]

    while (queue.length > 0) {
        let node = queue.shift()

        console.log(node);

        // Enque children
        for (const child of nodes.get(node)) {
            queue.push(child)
        }
    }
}

const bfs2 = (start_node, nodes) => {
    console.log(start_node);

    // Enque children
    for (const child of nodes.get(start_node)) {
        setTimeout(bfs2, 0, child, nodes);
    }
}

const dfs = (start_node, nodes) => {
    console.log(start_node);

    // Process children
    for (const child of nodes.get(start_node)) {
        dfs(child, nodes);
    }
}

const main = () => {
    let nodes = new Map();
    nodes.set('A', ['D', 'C', 'B'])
    nodes.set('D', ['F'])
    nodes.set('C', ['E'])
    nodes.set('B', [])
    nodes.set('F', [])
    nodes.set('E', [])

    console.log("bfs1:")
    bfs1('A', nodes)

    console.log("bfs2:")
    bfs2('A', nodes)
}

main()