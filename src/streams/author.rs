use anyhow::{Result,anyhow};
use iota_streams::app::transport::{
    tangle::{
        client::{Client},
        PAYLOAD_BYTES,
        MsgId,
    }
};
use iota_streams::app_channels::api::tangle::{Address, Author, ChannelAddress};
use iota_streams::core_edsig::signature::ed25519::PublicKey;

use std::str::FromStr;

pub struct ChannelAuthor {
    author: Author<Client>,
    announcement_id: Address,
    channel_address: ChannelAddress,
}

impl ChannelAuthor {
    pub fn new(seed: &str, mwm: u8, local_pow: bool, node: &str) -> Result<ChannelAuthor> {

        // Create Client instance
        let mut client = Client::new_from_url(node);

        // Generate a multi branch Author instance and start the channel
        let mut author = Author::new(seed, "utf-8", PAYLOAD_BYTES, true, client);
        let announcement_id = author.send_announce()?;

        Ok(ChannelAuthor {
            author: author,
            announcement_id: announcement_id.clone(),
            channel_address: announcement_id.appinst.clone()
        })
    }

    pub fn get_channel_address(&self) -> Result<String> {
        let channel_address = &self.channel_address.to_string();
        Ok(String::from_str(channel_address).unwrap())
    }

    pub fn get_announcement_id(&self) -> Result<(String, String)> {
        let appinst = &self.announcement_id.appinst.to_string();
        let msgid = &self.announcement_id.msgid.to_string();
        Ok((String::from_str(appinst).unwrap(), String::from_str(msgid).unwrap()))
    }

    pub fn subscribe(&mut self, link: &str, pk: &Vec<u8>) -> Result<Address> {
        match MsgId::from_str(link) {
            Ok(msgid) => {
                self.
                    author.
                    receive_subscribe(
                        &Address {
                            appinst: self.channel_address.clone(),
                            msgid,
                        })?;

                let keyload = self.author.send_keyload(
                    &self.announcement_id,
                    &vec![],
                    &vec![PublicKey::from_bytes(pk).unwrap()]
                )?;

                // Return the sequence message link
                Ok(keyload.1.unwrap())
            },
            Err(_) => {
                Err(anyhow!("Error getting msgid from provided link: {}", link))
            }
        }
    }
}
