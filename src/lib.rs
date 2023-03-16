use serde::{Serialize, Deserialize};

pub mod consts { 
    pub const HELP_MESSAGE: &str = "
Hello there, Human!

You have summoned me. Let's see about getting you what you need.

? Need technical help?
=> Post in the <#CHANNEL_ID> channel and other humans will assist you.

? Looking for the Code of Conduct?
=> Here it is: <https://opensource.facebook.com/code-of-conduct> 

? Something wrong?
=> You can flag an admin with @admin

I hope that resolves your issue!
-- Helpbot

";

    pub const HELP_COMMAND: &str = "!help";
    pub const CAT_COMMAND: &str = "!cat";
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatJson {
    pub id: String,
    pub url: String,
    pub width: u32,
    pub height: u32
}