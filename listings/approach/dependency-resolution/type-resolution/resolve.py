def resolve(scope: Node, reference: list[str]):
  assert len(reference) > 0
  scope = find_parent(reference.pop())
  while len(reference) > 0 and scope is not None:
    if kind(scope) == FIELD:
      scope = type(scope)
    scope = find_child(scope, reference.pop())
  return scope
