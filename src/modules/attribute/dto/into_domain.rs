use crate::db::utils::now;
use crate::modules::attribute::domain::attribute::{AttributeUpdate, NewAttribute};
use crate::modules::attribute::dto::attribute_dto::AttributeDtoReq;

impl Into<NewAttribute> for AttributeDtoReq {
    fn into(self) -> NewAttribute {
        NewAttribute{
            product_id: self.product_id,
            attribute_name: self.attribute_name,
            value_id: self.value_id,
            value_name: self.value_name,
            attribute_group_id: self.attribute_group_id,
            attribute_group_name: self.attribute_group_name,
            value_type: self.value_type,
            created_ad: now(),
        }
    }
}

impl Into<AttributeUpdate> for AttributeDtoReq {
    fn into(self) -> AttributeUpdate {
        AttributeUpdate {
            attribute_id: 0,
            product_id: self.product_id,
            attribute_name: self.attribute_name,
            value_id: self.value_id,
            value_name: self.value_name,
            attribute_group_id: self.attribute_group_id,
            attribute_group_name: self.attribute_group_name,
            value_type: self.value_type,
            updated_ad: now(),
        }
    }
}

impl AttributeDtoReq {
    pub fn map_attributes(values: Vec<Self>) -> Vec<NewAttribute> {
        values.iter().map(|d| {
            d.clone().into()
        }).collect()
    }
}