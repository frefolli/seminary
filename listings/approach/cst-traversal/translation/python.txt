if tsnode.type == "function_definition" {
  let declarator = tsnode.get_field("declarator");
  let function = Function {
    name: process_name(declarator.get_field("declarator")),
    type: process_type(tsnode.get_field("type")),
  }
  process_parameters(declarator.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
