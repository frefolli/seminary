if tsnode.type == Lang.tags.defun {
  let declarator = tsnode.get_field(Lang.tags.defun.declarator.declarator);
  let function = Function {
    name: process_name(declarator.get_field(Lang.tags.defun.declarator.name)),
    type: process_type(tsnode.get_field(Lang.tags.defun.declarator.type)),
  }
  process_parameters(declarator.get_field(Lang.tags.defun.declarator.parameters));
  process_body(tsnode.get_field(Lang.tags.defun.declarator.body));
}
