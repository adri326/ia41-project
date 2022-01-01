def bfs(initial):
    open = [(initial, [])]
    closed = set()
    closed.add(hash(initial))
    n_scanned = 0

    while len(open) > 0:
        n_scanned += 1
        (current, path) = open.pop(0)
        for (child, move) in current.next_states():
            hashed = hash(child)
            if not (hashed in closed):
                if child.solved():
                    print(n_scanned + " nodes scanned")
                    return path + [move]
                closed.add(hashed)
                open.append((child, path + [move]))

    return None
