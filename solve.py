def bfs(initial):
    open = [(initial, [])]
    closed = set()
    closed.add(hash(initial))

    while len(open) > 0:
        (current, path) = open.pop(0)
        for (child, move) in current.next_states():
            hashed = hash(child)
            if not (hashed in closed):
                if child.solved():
                    return path + [move]
                closed.add(hashed)
                open.append((child, path + [move]))

    return None
