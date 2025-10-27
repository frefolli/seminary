def find_child(scope: Node, identifier: str):
  for child in children(scope):
    if name(child) == identifier:
      return child
  return None
