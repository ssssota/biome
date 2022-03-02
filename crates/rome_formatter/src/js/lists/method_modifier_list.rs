use crate::{join_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsMethodModifierList;
use rslint_parser::AstNodeList;

impl ToFormatElement for JsMethodModifierList {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            space_token(),
            formatter.format_nodes(self.iter())?,
        ))
    }
}