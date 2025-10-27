def find_parent(scope: Node, identifier: str):
  while scope is not None:
    for child in children(scope):
      if name(child) == identifier:
        return child
    scope = parent(scope)
  return None
