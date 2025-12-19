//! Workspace database model

use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    /// Workspace name
    pub name: String,
    /// Workspace path
    pub path: String,
    // active status
    pub is_active: bool,
    /// Description of the setting
    pub description: Option<String>,
    /// Created timestamp
    pub created_at: ChronoDateTimeUtc,
    /// Updated timestamp
    pub updated_at: ChronoDateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    /// Set timestamps before saving
    #[doc = " Will be called before `ActiveModel::insert` and `ActiveModel::update`"]
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::needless_arbitrary_self_type,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn before_save<'life0,'async_trait,C, >(mut self,db: &'life0 C,insert:bool,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self,DbErr> > + ::core::marker::Send+'async_trait> >where C:ConnectionTrait,C:'async_trait+ ,'life0:'async_trait,Self: ::core::marker::Send+'async_trait{
        Box::pin(async move {
            if let::core::option::Option::Some(__ret) =  ::core::option::Option::None:: <Result<Self,DbErr> >{
                #[allow(unreachable_code)]
                return __ret;
            }let insert = insert;
            let __ret:Result<Self,DbErr>  = {
                let now = chrono::Utc::now();
                if insert {
                    self.created_at = Set(now);
                }
                self.updated_at = Set(now);
                Ok(self)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    
    #[doc = " Create a new ActiveModel with default values. Also used by `Default::default()`."]
    fn new() -> Self {
        <Self as ActiveModelTrait> ::default()
    }
    
    #[doc = " Will be called after `ActiveModel::insert`, `ActiveModel::update`, and `ActiveModel::save`"]
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::needless_arbitrary_self_type,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn after_save<'life0,'async_trait,C, >(model: <Self::Entity as EntityTrait> ::Model,db: &'life0 C,insert:bool,) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result< <Self::Entity as EntityTrait> ::Model,DbErr> > + ::core::marker::Send+'async_trait> >where C:ConnectionTrait,C:'async_trait+ ,'life0:'async_trait,Self: ::core::marker::Send+'async_trait{
        Box::pin(async move {
            if let::core::option::Option::Some(__ret) =  ::core::option::Option::None:: <Result< <Self::Entity as EntityTrait> ::Model,DbErr> >{
                #[allow(unreachable_code)]
                return __ret;
            }let model = model;
            let insert = insert;
            let __ret:Result< <Self::Entity as EntityTrait> ::Model,DbErr>  = {
                Ok(model)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    
    #[doc = " Will be called before `ActiveModel::delete`"]
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::needless_arbitrary_self_type,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn before_delete<'life0,'async_trait,C, >(self,db: &'life0 C) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self,DbErr> > + ::core::marker::Send+'async_trait> >where C:ConnectionTrait,C:'async_trait+ ,'life0:'async_trait,Self: ::core::marker::Send+'async_trait{
        Box::pin(async move {
            if let::core::option::Option::Some(__ret) =  ::core::option::Option::None:: <Result<Self,DbErr> >{
                #[allow(unreachable_code)]
                return __ret;
            }let __self = self;
            let __ret:Result<Self,DbErr>  = {
                Ok(__self)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
    
    #[doc = " Will be called after `ActiveModel::delete`"]
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::async_yields_async,clippy::diverging_sub_expression,clippy::let_unit_value,clippy::needless_arbitrary_self_type,clippy::no_effect_underscore_binding,clippy::shadow_same,clippy::type_complexity,clippy::type_repetition_in_bounds,clippy::used_underscore_binding)]
    fn after_delete<'life0,'async_trait,C, >(self,db: &'life0 C) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Self,DbErr> > + ::core::marker::Send+'async_trait> >where C:ConnectionTrait,C:'async_trait+ ,'life0:'async_trait,Self: ::core::marker::Send+'async_trait{
        Box::pin(async move {
            if let::core::option::Option::Some(__ret) =  ::core::option::Option::None:: <Result<Self,DbErr> >{
                #[allow(unreachable_code)]
                return __ret;
            }let __self = self;
            let __ret:Result<Self,DbErr>  = {
                Ok(__self)
            };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
