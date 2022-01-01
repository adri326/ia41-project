def bfs(initial):
    open = [(initial, [])] # Liste des positions à explorer ainsi que du chemin parcouru jusqu'à celles-cis
    closed = set() # Liste des positions déjà explorées
    # Limitation de Python: on ne peut pas stocker d'objets dans un set(), donc on stocke uniquement le hash.
    # Ceci peut mener à des faux positifs, mais aucun faux positif faussant la solution n'a été trouvé
    closed.add(hash(initial))
    n_scanned = 0 # Compteur des noeuds explorés

    while len(open) > 0: # Tant qu'il y a des positions à explorer
        n_scanned += 1
        (current, path) = open.pop(0)
        for (child, move) in current.next_states(): # Pour toutes les positions résultante d'un mouvement...
            hashed = hash(child)
            if not (hashed in closed): # Si la position n'a pas déjà étée explorée
                if child.solved(): # Si la position est la solution, retourner la suite de coups
                    print(n_scanned, "nodes scanned")
                    return path + [move]
                closed.add(hashed) # Sinon, l'ajouter à closed
                open.append((child, path + [move])) # et l'ajouter à open

    return None
