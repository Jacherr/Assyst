pub mod argument_type {
    use std::sync::Arc;

    use crate::command::{
        command::{
            Argument, Command, CommandParseError, CommandParseErrorType, ParsedArgument,
            ParsedArgumentResult,
        },
        context::Context,
    };

    pub fn numerical<'a>(
        args: &Vec<&str>,
        arg: &Argument,
        command: &'a Command,
        index: usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        if args.len() <= index {
            return Err(CommandParseError::with_reply(
                "This command expects a numerical argument, but no argument was provided."
                    .to_owned(),
                Some(command),
                CommandParseErrorType::MissingArgument,
            ));
        }

        let float = args[index].parse::<f64>().map_err(|_| {
            CommandParseError::with_reply(
                format!("Invalid number provided: {}", args[index]),
                Some(command),
                CommandParseErrorType::MissingArgument,
            )
        })?;

        return match arg {
            Argument::Decimal => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                float.to_string(),
            ))),

            Argument::Integer => Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
                format!("{:.0}", float),
            ))),

            _ => unreachable!(),
        };
    }

    pub fn choice<'a>(
        choices: &'static [&'static str],
        args: &Vec<&str>,
        command: &'a Command,
        index: usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        if args.len() <= index {
            return Err(CommandParseError::with_reply(
                format!("This command expects a choice argument (one of {:?}), but no argument was provided.", choices),
                    Some(command),
                    CommandParseErrorType::MissingArgument
            ));
        }

        let choice = match choices.iter().find(|&&choice| choice == args[index]) {
            Some(k) => k,
            None => {
                return Err(CommandParseError::with_reply(
                    format!("Cannot find given argument in {:?}", choices),
                    Some(command),
                    CommandParseErrorType::InvalidArgument,
                ))
            }
        };

        Ok(ParsedArgumentResult::increment(ParsedArgument::Choice(
            choice,
        )))
    }

    pub fn string_remaining<'a>(
        context: &Arc<Context>,
        args: &Vec<&str>,
        command: &'a Command,
        index: usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        // check if no extra args or if no referenced message
        if args.len() <= index && context.message.referenced_message.is_none() {
            Err(CommandParseError::with_reply(
                "This command expects a text argument that was not provided.".to_owned(),
                Some(command),
                CommandParseErrorType::MissingArgument,
            ))
        // check if referenced message and if it has any content to use
        } else if let Some(r) = &context.message.referenced_message {
            if r.content.is_empty() {
                Err(CommandParseError::with_reply(
                    "This command expects a text argument that was not provided.".to_owned(),
                    Some(command),
                    CommandParseErrorType::MissingArgument,
                ))
            } else if !(args.len() <= index) {
                Ok(ParsedArgumentResult::r#break(ParsedArgument::Text(
                    args[index..].join(" "),
                )))
            } else {
                Ok(ParsedArgumentResult::r#break(ParsedArgument::Text(
                    r.content.clone(),
                )))
            }
        } else {
            Ok(ParsedArgumentResult::r#break(ParsedArgument::Text(
                args[index..].join(" "),
            )))
        }
    }

    pub fn string<'a>(
        args: &Vec<&str>,
        command: &'a Command,
        index: usize,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        if args.len() <= index {
            return Err(CommandParseError::with_reply(
                "This command expects a text argument that was not provided.".to_owned(),
                Some(command),
                CommandParseErrorType::MissingArgument,
            ));
        }
        Ok(ParsedArgumentResult::increment(ParsedArgument::Text(
            args[index].to_owned(),
        )))
    }
}

pub mod image_lookups {
    use std::borrow::Cow;

    use crate::util::regexes;
    use twilight_model::{
        channel::{message::sticker::StickerFormatType, Message},
        id::UserId,
    };

    pub fn emoji(argument: &str) -> Option<String> {
        let unicode_emoji = emoji::lookup_by_glyph::lookup(argument);
        if let Some(e) = unicode_emoji {
            let codepoint = e
                .codepoint
                .to_lowercase()
                .replace(" ", "-")
                .replace("-fe0f", "");

            let emoji_url = format!("https://derpystuff.gitlab.io/webstorage3/container/twemoji-JedKxRr7RNYrgV9Sauy8EGAu/{}.png", codepoint);
            return Some(emoji_url);
        }

        let emoji_id = regexes::CUSTOM_EMOJI
            .captures(argument)
            .and_then(|emoji_id_capture| emoji_id_capture.get(2))
            .and_then(|id| Some(id.as_str()))
            .and_then(|id| id.parse::<u64>().ok())?;

        let format = if argument.starts_with("<a") {
            "gif"
        } else {
            "png"
        };
        let emoji_url = format!("https://cdn.discordapp.com/emojis/{}.{}", emoji_id, format);

        return Some(emoji_url);
    }

    pub async fn user(http: &twilight_http::Client, argument: &str) -> Option<String> {
        let user_id = regexes::USER_MENTION
            .captures(argument)
            .and_then(|user_id_capture| user_id_capture.get(1))
            .and_then(|id| Some(id.as_str()))
            .and_then(|id| id.parse::<u64>().ok())?;

        let user = http.user(UserId::from(user_id)).await.ok()??;
        let avatar_hash = user.avatar;
        match avatar_hash {
            Some(hash) => {
                let format = if hash.starts_with("a_") { "gif" } else { "png" };
                Some(format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.{}?size=1024",
                    user_id, hash, format
                ))
            }
            None => {
                let discrim = user.discriminator.parse::<u16>().ok()?;
                let avatar_number = discrim % 5;
                Some(format!(
                    "https://cdn.discordapp.com/embed/avatars/{}.png",
                    avatar_number
                ))
            }
        }
    }

    pub fn sticker(message: &Message) -> Option<String> {
        let sticker = message.sticker_items.first()?;
        let r#type = sticker.format_type;
        if r#type == StickerFormatType::Lottie {
            Some(format!(
                "https://cdn.discordapp.com/stickers/{}.json",
                sticker.id
            ))
        } else {
            Some(format!(
                "https://cdn.discordapp.com/stickers/{}.png",
                sticker.id
            ))
        }
    }

    pub fn embed<'a>(message: &Message) -> Option<&str> {
        let embed = message.embeds.first()?;

        if let Some(e) = &embed.url {
            if e.starts_with("https://tenor.com/view/") {
                return Some(e);
            };
        }

        embed
            .image
            .as_ref()
            .and_then(|img| Some(img.url.as_deref()?))
            .or_else(|| {
                embed
                    .thumbnail
                    .as_ref()
                    .and_then(|thumbnail| Some(thumbnail.url.as_deref()?))
                    .or_else(|| {
                        embed
                            .video
                            .as_ref()
                            .and_then(|video| Some(video.url.as_deref()?))
                    })
            })
    }

    pub fn attachment(message: &Message) -> Option<&str> {
        message.attachments.first().map(|a| a.url.as_str())
    }

    pub async fn message_reply(message: &Message) -> Option<Cow<'_, str>> {
        let reply = message.referenced_message.as_ref()?;
        let attachment = self::attachment(reply);
        if attachment.is_some() {
            return attachment.map(Cow::Borrowed);
        }

        let sticker = self::sticker(reply);
        if sticker.is_some() {
            return sticker.map(Cow::Owned);
        }

        let embed = self::embed(reply)?;
        Some(Cow::Borrowed(embed))
    }

    pub async fn previous_message_attachment<'m>(
        http: &twilight_http::Client,
        message: &'m Message,
    ) -> Option<Cow<'m, str>> {
        let messages = http.channel_messages(message.channel_id).await.ok()?;

        for message in messages {
            if !message.embeds.is_empty() {
                let o = embed(&message).map(|a| Cow::Owned(a.to_string()));
                if o.is_some() {
                    return o;
                }
            } else if !message.sticker_items.is_empty() {
                let o = sticker(&message).map(|a| Cow::Owned(a.to_string()));
                if o.is_some() {
                    return o;
                }
            } else if !message.attachments.is_empty() {
                let o = attachment(&message).map(|a| Cow::Owned(a.to_string()));
                if o.is_some() {
                    return o;
                }
            }
        }

        None
    }
}

pub mod subsections {
    use std::{borrow::Cow, sync::Arc};

    use assyst_common::consts;
    use twilight_model::channel::Message;

    use super::image_lookups;
    use crate::command::{
        command::{Argument, ParsedArgumentResult},
        context::Context,
    };
    use crate::{
        command::command::{CommandParseError, CommandParseErrorType, ParsedArgument},
        rest::{convert_lottie_to_gif, upload_to_filer},
        util::{download_content, regexes},
    };

    pub async fn parse_image_argument<'a>(
        context: &Arc<Context>,
        message: &Message,
        argument: &str,
        return_as: &Argument,
    ) -> Result<ParsedArgumentResult, CommandParseError<'a>> {
        let mut should_increment = true;
        let mut try_url = image_lookups::user(&context.assyst.http, argument)
            .await
            .map(Cow::Owned);

        if try_url.is_none() {
            try_url = context
                .assyst
                .validate_url_argument(argument)
                .map(Cow::Owned);
        }

        if try_url.is_none() {
            try_url = image_lookups::attachment(message).map(Cow::Borrowed);

            if try_url.is_some() {
                should_increment = false;
            }
        }

        if try_url.is_none() {
            try_url = image_lookups::message_reply(message).await;
            if try_url.is_some() {
                should_increment = false
            };
        }

        if try_url.is_none() {
            try_url = image_lookups::emoji(argument).map(Cow::Owned);
        }

        if try_url.is_none() {
            try_url = image_lookups::sticker(message).map(Cow::Owned);
        }

        if try_url.is_none() {
            try_url =
                image_lookups::previous_message_attachment(&context.assyst.http, message).await;

            if try_url.is_some() {
                should_increment = false
            };
        };

        let mut url = try_url.ok_or_else(|| {
            CommandParseError::with_reply(
                "This command expects an image as an argument, but no image could be found."
                    .to_owned(),
                None,
                CommandParseErrorType::MissingArgument,
            )
        })?;

        // tenor urls only typically return a png, so this code visits the url
        // and extracts the appropriate GIF url from the page.
        if url.starts_with("https://tenor.com/view/") {
            let page = context
                .assyst
                .reqwest_client
                .get(url.as_ref())
                .send()
                .await
                .map_err(|e| {
                    CommandParseError::with_reply(
                        e.to_string(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?
                .text()
                .await
                .map_err(|e| {
                    CommandParseError::with_reply(
                        e.to_string(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?;

            let gif_url = regexes::TENOR_GIF
                .find(&page)
                .and_then(|url| Some(url.as_str()))
                .ok_or_else(|| {
                    CommandParseError::with_reply(
                        "Failed to extract Tenor GIF URL".to_owned(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?;

            url = Cow::Owned(gif_url.to_owned());
        };

        if url.ends_with(".json") && url.starts_with("https://cdn.discordapp.com/stickers/") {
            // we need to download it from discord and convert it to a gif first
            context
                .reply_with_text("preparing sticker...")
                .await
                .map_err(|_| {
                    CommandParseError::without_reply(
                        "failed to send message".to_owned(),
                        CommandParseErrorType::Other,
                    )
                })?;

            let content: Vec<u8> = download_content(
                &context.assyst.reqwest_client,
                &url,
                consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
            )
            .await
            .map_err(|e| {
                CommandParseError::with_reply(
                    format!("failed to download lottie sticker: {}", e),
                    None,
                    CommandParseErrorType::MediaDownloadFail,
                )
            })?;

            let string_content = String::from_utf8_lossy(&content);
            let gif = convert_lottie_to_gif(&context.assyst, &string_content.into_owned())
                .await
                .map_err(|_| {
                    CommandParseError::with_reply(
                        "failed to process lottie sticker".to_owned(),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?;

            // now we need to upload it to filer so that we have a url to work with
            // since this is how the parser works... pretty inefficient but yeah stfu
            url = upload_to_filer(context.assyst.clone(), gif, "image/gif")
                .await
                .map(Cow::Owned)
                .map_err(|e| {
                    CommandParseError::with_reply(
                        format!("failed to upload sticker: {}", e.to_string()),
                        None,
                        CommandParseErrorType::MediaDownloadFail,
                    )
                })?;
        }

        match return_as {
            Argument::ImageBuffer => {
                let result = download_content(
                    &context.assyst.reqwest_client,
                    &url,
                    consts::ABSOLUTE_INPUT_FILE_SIZE_LIMIT_BYTES,
                )
                .await
                .map_err(|e| {
                    CommandParseError::with_reply(e, None, CommandParseErrorType::MediaDownloadFail)
                })?;

                let parsed_argument_result = match should_increment {
                    true => ParsedArgumentResult::increment(ParsedArgument::Binary(result.into())),
                    false => {
                        ParsedArgumentResult::no_increment(ParsedArgument::Binary(result.into()))
                    }
                };
                Ok(parsed_argument_result)
            }
            Argument::ImageUrl => {
                let parsed_argument_result = match should_increment {
                    true => ParsedArgumentResult::increment(ParsedArgument::Text(url.to_string())),
                    false => {
                        ParsedArgumentResult::no_increment(ParsedArgument::Text(url.to_string()))
                    }
                };
                Ok(parsed_argument_result)
            }
            _ => panic!("return_as must be imageurl or imagebuffer"),
        }
    }
}
