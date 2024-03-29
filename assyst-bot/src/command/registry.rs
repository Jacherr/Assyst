use super::command::{Command, ParsedArgument, ParsedCommand, ParsedFlags};
use super::{
    categories::{fun::*, image_makesweet::*, image_wsi::*, misc::*, tag::*},
    command::CommandAvailability,
};
use crate::command::context::Context;
use std::future::Future;
use std::sync::Arc;
use std::{collections::HashMap, pin::Pin};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

pub type CommandResult = anyhow::Result<()>;
pub type CommandResultOuter = Pin<Box<dyn Future<Output = CommandResult> + Send>>;
pub type CommandRun =
    Box<dyn Fn(Arc<Context>, Vec<ParsedArgument>, ParsedFlags) -> CommandResultOuter + Send + Sync>;

macro_rules! register_command {
    ($self:expr, $command:expr, $run_fn:expr) => {{
        // Registering the same command for each alias is fine because it will point to the same object
        for alias in &$command.aliases {
            $self.commands.insert(alias, &*$command);
        }

        $self.commands.insert(&$command.name, &*$command);
        $self.command_runs.insert(&$command.name, Box::new(move |context, args, flags| Box::pin($run_fn(context, args, flags))));
    }}
}

pub struct CommandRegistry {
    pub command_runs: HashMap<&'static str, CommandRun>,
    pub commands: HashMap<&'static str, &'static Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        CommandRegistry {
            command_runs: HashMap::new(),
            commands: HashMap::new(),
        }
    }

    pub async fn execute_command(
        &self,
        parsed_command: ParsedCommand,
        context: Arc<Context>,
    ) -> CommandResult {
        let command_processed = Arc::new(Mutex::new(false));

        let command_processed_c = command_processed.clone();
        let context_c = context.clone();
        let assyst = context.assyst.clone();

        // thread to check if the command takes >500ms and start typing if so
        tokio::spawn(async move {
            sleep(Duration::from_millis(500)).await;
            let lock = *command_processed_c.lock().await;
            if lock == false {
                let _ = context_c
                    .assyst
                    .http
                    .create_typing_trigger(context_c.message.channel_id)
                    .await;
            }
        });

        // get the appropriate handler function
        // we already validated the command exists, so unwrapping here is safe
        let command_run = self.command_runs.get(parsed_command.calling_name).unwrap();
        let result = command_run(context, parsed_command.args, parsed_command.flags).await;
        let mut lock = command_processed.lock().await;
        *lock = true;

        let command_name = &self.commands.get(parsed_command.calling_name).unwrap().name;
        assyst
            .database
            .increment_command_uses(&command_name)
            .await?;

        result
    }

    pub fn get_command_count(&self) -> usize {
        let mut command_count = 0;
        let mut unique_command_names = Vec::new();
        for i in self
            .commands
            .values()
            .filter(|a| a.availability != CommandAvailability::Private)
        {
            if unique_command_names.contains(&&i.name) {
                continue;
            };
            unique_command_names.push(&i.name);
            command_count += 1;
        }
        command_count
    }

    pub fn get_command_from_name_or_alias(&self, name: &str) -> Option<&'static Command> {
        self.commands.get(name).and_then(|command| Some(*command))
    }

    pub fn register_commands(&mut self) {
        // register_command!(self, _3D_ROTATE_COMMAND, run_3d_rotate_command);
        // register_command!(self, FLASH_COMMAND, run_flash_command);
        register_command!(self, AHSHIT_COMMAND, run_ahshit_command);
        register_command!(self, APRIL_FOOLS_COMMAND, run_aprilfools_command);
        register_command!(self, BACK_TATTOO_COMMAND, run_back_tattoo_command);
        register_command!(self, BILLBOARD_COMMAND, run_billboard_command);
        register_command!(self, BLACKLIST_COMMAND, run_blacklist_command);
        register_command!(self, BLUR_COMMAND, run_blur_command);
        register_command!(self, BOOK_COMMAND, run_book_command);
        register_command!(self, BT_CHANNEL_COMMAND, run_btchannel_command);
        register_command!(self, BT_COMMAND, run_bt_command);
        register_command!(self, BURNTEXT_COMMAND, run_burntext_command);
        register_command!(self, CACHE_STATUS_COMMAND, run_cache_status_command);
        register_command!(self, CAPTION_COMMAND, run_caption_command);
        register_command!(self, CHARS_COMMAND, run_chars_command);
        register_command!(self, CIRCUITBOARD_COMMAND, run_circuitboard_command);
        register_command!(self, COLOR_COMMAND, run_color_command);
        register_command!(self, COMMAND_COMMAND, run_command_command);
        register_command!(self, DREAM_COMMAND, run_dream_command);
        register_command!(self, DRIP_COMMAND, run_drip_command);
        register_command!(self, ENLARGE_COMMAND, run_enlarge_command);
        register_command!(self, EXEC_COMMAND, run_exec_command);
        register_command!(self, F_SHIFT_COMMAND, run_f_shift_command);
        register_command!(self, FAKE_EVAL_COMMAND, run_fake_eval_command);
        register_command!(self, FEMURBREAKER_COMMAND, run_femurbreaker_command);
        register_command!(self, FISHEYE_COMMAND, run_fisheye_command);
        register_command!(self, FLAG_COMMAND, run_flag_command);
        register_command!(self, FLAG2_COMMAND, run_flag2_command);
        register_command!(self, FLIP_COMMAND, run_flip_command);
        register_command!(self, FLOP_COMMAND, run_flop_command);
        register_command!(self, FORTUNE_COOKIE_COMMAND, run_fortune_cookie_command);
        register_command!(self, FRAMES_COMMAND, run_frames_command);
        register_command!(self, GHOST_COMMAND, run_ghost_command);
        register_command!(self, GIF_LOOP_COMMAND, run_gif_loop_command);
        register_command!(self, GIF_MAGIK_COMMAND, run_gif_magik_command);
        register_command!(self, GIF_SCRAMBLE_COMMAND, run_gif_scramble_command);
        register_command!(self, GIF_SPEED_COMMAND, run_gif_speed_command);
        register_command!(self, GLOBE_COMMAND, run_globe_command);
        register_command!(self, GRAYSCALE_COMMAND, run_grayscale_command);
        register_command!(self, HEALTHCHECK_COMMAND, run_healthcheck_command);
        register_command!(self, HEART_LOCKET_COMMAND, run_heart_locket_command);
        register_command!(self, HELP_COMMAND, run_help_command);
        register_command!(self, IDENTIFY_COMMAND, run_identify_command);
        register_command!(self, IMAGE_INFO_COMMAND, run_image_info_command);
        register_command!(self, IMAGEMAGICK_EVAL_COMMAND, run_imagemagick_eval_command);
        register_command!(self, INVERT_COMMAND, run_invert_command);
        register_command!(self, INVITE_COMMAND, run_invite_command);
        register_command!(self, JPEG_COMMAND, run_jpeg_command);
        register_command!(self, MAGIK_COMMAND, run_magik_command);
        register_command!(self, MEME_COMMAND, run_meme_command);
        register_command!(self, MOTIVATE_COMMAND, run_motivate_command);
        register_command!(self, NEON_COMMAND, run_neon_command);
        register_command!(self, OCR_COMMAND, run_ocr_command);
        register_command!(self, OCRBT_COMMAND, run_ocrbt_command);
        register_command!(self, OCRTR_COMMAND, run_ocrtr_command);
        register_command!(self, OVERLAY_COMMAND, run_overlay_command);
        register_command!(self, PAINT_COMMAND, run_paint_command);
        register_command!(self, PATRON_STATUS_COMMAND, run_patron_status_command);
        register_command!(self, PING_COMMAND, run_ping_command);
        register_command!(self, PIXELATE_COMMAND, run_pixelate_command);
        register_command!(self, PREFIX_COMMAND, run_prefix_command);
        register_command!(self, PRINTER_COMMAND, run_printer_command);
        register_command!(self, RAINBOW_COMMAND, run_rainbow_command);
        register_command!(self, RANDOMIZE_COMMAND, run_randomize_command);
        register_command!(self, REMINDER_COMMAND, run_remind_command);
        register_command!(self, RESIZE_COMMAND, run_resize_command);
        register_command!(self, REVERSE_COMMAND, run_reverse_command);
        register_command!(self, ROTATE_COMMAND, run_rotate_command);
        register_command!(self, RUBIKS_COMMAND, run_rubiks_command);
        register_command!(self, RULE34_COMMAND, run_rule34_command);
        register_command!(self, RUST_COMMAND, run_rust_command);
        register_command!(self, SET_LOOP_COMMAND, run_set_loop_command);
        register_command!(self, SIREN_COMMAND, run_siren_command);
        register_command!(self, SOFTGLOW_COMMAND, run_bloom_command);
        register_command!(self, SPEECHBUBBLE_COMMAND, run_speechbubble_command);
        register_command!(self, SPIN_COMMAND, run_spin_command);
        register_command!(self, SPREAD_COMMAND, run_spread_command);
        register_command!(self, STATS_COMMAND, run_stats_command);
        register_command!(self, SWEDEN_COMMAND, run_sweden_command);
        register_command!(self, SWIRL_COMMAND, run_swirl_command);
        register_command!(self, TAG_COMMAND, run_tag_command);
        register_command!(self, TERRARIA_COMMAND, run_terraria_command);
        register_command!(self, TOASTER_COMMAND, run_toaster_command);
        register_command!(self, TOP_BT_COMMAND, run_top_bt_command);
        register_command!(self, TOP_COMMANDS_COMMAND, run_top_commands_command);
        register_command!(self, TOP_GUILDS_COMMAND, run_top_guilds_command);
        register_command!(self, TOWAV_COMMAND, run_towav_command);
        register_command!(self, TRANSLATE_COMMAND, run_translate_command);
        register_command!(self, WHITELIST_COMMAND, run_whitelist_command);
        register_command!(self, UNCAPTION_COMMAND, run_uncaption_command);
        register_command!(self, VALENTINE_COMMAND, run_valentine_command);
        register_command!(self, VIDEOTOGIF_COMMAND, run_videotogif_command);
        register_command!(self, WALL_COMMAND, run_wall_command);
        register_command!(self, WAVE_COMMAND, run_wave_command);
        register_command!(self, WORMHOLE_COMMAND, run_wormhole_command);
        register_command!(self, WSI_STATS_COMMAND, run_wsi_stats_command);
        register_command!(self, ZOOM_BLUR_COMMAND, run_zoom_blur_command);
        register_command!(self, ZOOM_COMMAND, run_zoom_command);
        register_command!(self, AUDIO_IDENTIFY_COMMAND, run_audio_identify_command);
        register_command!(self, DEEPFRY_COMMAND, run_deepfry_command);
        register_command!(self, URL_COMMAND, run_url_command);
        register_command!(self, DOWNLOAD_COMMAND, run_download_command);
    }
}
