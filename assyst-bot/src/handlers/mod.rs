use std::{convert::TryInto, num::NonZeroU16};

use serenity::{
    all::{
        Attachment as SerAttachment, Embed as SerEmbed, EmbedImage as SerEmbedImage,
        EmbedThumbnail as SerEmbedThumbnail, ImageHash as SerImageHash, MessageUpdateEvent,
        User as SerUser,
    },
    model::{channel::Message as SerMessage, Timestamp as SerTimestamp},
};
use twilight_model::{
    channel::{
        message::{
            embed::{EmbedImage, EmbedThumbnail},
            Embed, MessageType,
        },
        Attachment, Message,
    },
    id::{
        marker::{
            ApplicationMarker, AttachmentMarker, ChannelMarker, GuildMarker, MessageMarker,
            UserMarker, WebhookMarker,
        },
        Id,
    },
    user::User,
    util::{ImageHash, Timestamp},
};

pub mod message_create;
pub mod message_delete;
pub mod message_update;

pub fn ser_message_update_to_twl_message(ser: MessageUpdateEvent) -> Option<Message> {
    Some(Message {
        application_id: None,
        interaction: None,
        activity: None,
        application: None,
        attachments: vec![], //ser.attachments,
        author: ser_author_to_twl_author(ser.author?),
        channel_id: Id::<ChannelMarker>::new(ser.channel_id.get()),
        content: ser.content.unwrap_or("".to_owned()),
        edited_timestamp: ser
            .edited_timestamp
            .map(|z| ser_timestamp_to_twl_timestamp(z)),
        embeds: vec![], //ser.embeds,
        flags: None,
        guild_id: ser.guild_id.map(|x| Id::<GuildMarker>::new(x.get())),
        id: Id::<MessageMarker>::new(ser.id.get()),
        kind: MessageType::Regular,
        member: None,
        mention_channels: vec![],
        mention_everyone: ser.mention_everyone?,
        mention_roles: vec![],
        mentions: vec![],
        pinned: ser.pinned?,
        reactions: vec![],
        reference: None,
        referenced_message: None,
        sticker_items: vec![],
        timestamp: ser_timestamp_to_twl_timestamp(ser.timestamp?),
        tts: false,
        webhook_id: None,
        components: vec![],
        thread: None,
        role_subscription_data: None,
    })
}

pub fn ser_message_to_twl_message(ser: SerMessage) -> Message {
    Message {
        application_id: ser
            .application_id
            .map(|x| Id::<ApplicationMarker>::new(x.get())),
        interaction: None,
        activity: None,
        application: None,
        attachments: ser_attachments_to_twl_attachments(ser.attachments),
        author: ser_author_to_twl_author(ser.author),
        channel_id: Id::<ChannelMarker>::new(ser.channel_id.get()),
        content: ser.content,
        edited_timestamp: ser
            .edited_timestamp
            .map(|z| ser_timestamp_to_twl_timestamp(z)),
        embeds: ser_embeds_to_twl_embeds(ser.embeds),
        flags: None,
        guild_id: ser.guild_id.map(|x| Id::<GuildMarker>::new(x.get())),
        id: Id::<MessageMarker>::new(ser.id.get()),
        kind: MessageType::Regular,
        member: None,
        mention_channels: vec![],
        mention_everyone: ser.mention_everyone,
        mention_roles: vec![],
        mentions: vec![],
        pinned: ser.pinned,
        reactions: vec![],
        reference: None,
        referenced_message: ser
            .referenced_message
            .map(|x| Box::new(ser_message_to_twl_message(*x))),
        sticker_items: vec![],
        timestamp: ser_timestamp_to_twl_timestamp(ser.timestamp),
        tts: false,
        webhook_id: ser.webhook_id.map(|x| Id::<WebhookMarker>::new(x.get())),
        components: vec![],
        thread: None,
        role_subscription_data: None,
    }
}

pub fn ser_attachments_to_twl_attachments(ser: Vec<SerAttachment>) -> Vec<Attachment> {
    let mut attachs: Vec<Attachment> = vec![];
    for a in ser {
        attachs.push(Attachment {
            content_type: a.content_type,
            ephemeral: a.ephemeral,
            duration_secs: a.duration_secs,
            filename: a.filename,
            flags: None,
            description: a.description,
            height: a.height.map(|x| x.into()),
            id: Id::<AttachmentMarker>::new(a.id.get()),
            proxy_url: a.proxy_url,
            size: a.size as u64,
            url: a.url,
            waveform: None,
            width: a.width.map(|x| x.into()),
        })
    }

    attachs
}

pub fn ser_embeds_to_twl_embeds(ser: Vec<SerEmbed>) -> Vec<Embed> {
    let mut embeds: Vec<Embed> = vec![];
    for a in ser {
        embeds.push(Embed {
            author: None,
            color: None,
            description: a.description,
            fields: vec![],
            footer: None,
            image: a.image.map(|x| ser_embeds_img_to_twl_image(x)),
            kind: a.kind.unwrap_or("".to_owned()),
            provider: None,
            thumbnail: a.thumbnail.map(|x| ser_embeds_thumb_to_twl_thumb(x)),
            timestamp: a.timestamp.map(|x| ser_timestamp_to_twl_timestamp(x)),
            title: a.title,
            url: a.url,
            video: None,
        })
    }
    embeds
}

pub fn ser_embeds_thumb_to_twl_thumb(ser: SerEmbedThumbnail) -> EmbedThumbnail {
    EmbedThumbnail {
        height: ser.height.map(|x| x.into()),
        proxy_url: ser.proxy_url,
        url: ser.url,
        width: ser.width.map(|x| x.into()),
    }
}

pub fn ser_embeds_img_to_twl_image(ser: SerEmbedImage) -> EmbedImage {
    EmbedImage {
        height: ser.height.map(|x| x.into()),
        proxy_url: ser.proxy_url,
        url: ser.url,
        width: ser.width.map(|x| x.into()),
    }
}

pub fn ser_timestamp_to_twl_timestamp(ser: SerTimestamp) -> Timestamp {
    Timestamp::from_secs(ser.timestamp()).unwrap()
}

pub fn ser_imagehash_to_twl_imagehash(ser: Option<SerImageHash>) -> Option<ImageHash> {
    ser.map(|x| ImageHash::parse(x.to_string().as_bytes()).ok())
        .flatten()
}

pub fn ser_author_to_twl_author(ser: SerUser) -> User {
    User {
        accent_color: None,
        avatar: ser_imagehash_to_twl_imagehash(ser.avatar),
        avatar_decoration: None,
        banner: None,
        bot: ser.bot,
        discriminator: ser.discriminator.map(|x| u16::from(x)).unwrap_or(0),
        email: None,
        flags: None,
        global_name: ser.global_name,
        id: Id::<UserMarker>::new(ser.id.get()),
        locale: None,
        mfa_enabled: None,
        name: ser.name,
        premium_type: None,
        public_flags: None,
        system: Some(ser.system),
        verified: ser.verified,
    }
}
