use crate::widget::{context::WidgetContext, node::WidgetNode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VariantBoxProps {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_name: Option<String>,
}
implement_props_data!(VariantBoxProps);

pub fn variant_box(context: WidgetContext) -> WidgetNode {
    let WidgetContext {
        props,
        mut named_slots,
        ..
    } = context;

    let VariantBoxProps { variant_name } = props.read_cloned_or_default();

    if let Some(variant_name) = variant_name {
        named_slots.remove(&variant_name).unwrap_or_default()
    } else {
        Default::default()
    }
}
