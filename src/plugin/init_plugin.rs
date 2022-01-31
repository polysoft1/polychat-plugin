use super::{PluginInfo, APIVersion, Message};
use crate::types::Account;

#[derive(Debug)]
pub struct InitializedPlugin {
    pub supported_api: APIVersion,

    pub create_account: extern fn() -> Account,
    pub destroy_account: extern fn(acc: Account),
    pub post_message: extern fn(msg: * const Message),
    pub print: extern fn(acc: Account)
}

impl InitializedPlugin {
    pub fn new(plugin: &PluginInfo) -> Result<InitializedPlugin, String> {
        if plugin.create_account.is_none() {
            return Err("create_account is not defined".to_string());
        } else if plugin.destroy_account.is_none() {
            return Err("destroy_account is not defined".to_string());
        } else if plugin.post_message.is_none() {
            return Err("post_message is not defined".to_string());
        } else if plugin.print.is_none() {
            return Err("print is not defined".to_string());
        }

        Ok(InitializedPlugin {
            supported_api: plugin.supported_api,
            create_account: plugin.create_account.unwrap(),
            destroy_account: plugin.destroy_account.unwrap(),
            post_message: plugin.post_message.unwrap(),
            print: plugin.print.unwrap()
        })
    }
}