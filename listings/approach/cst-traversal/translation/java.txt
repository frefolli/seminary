if tsnode.type == "method_declaration" {
  let function = Function {
    name: process_name(tsnode.get_field("name")),
    type: process_type(tsnode.get_field("type")),
  }
  process_parameters(tsnode.get_field("parameters"));
  process_body(tsnode.get_field("body"));
}
