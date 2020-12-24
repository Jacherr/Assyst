export interface ImageScriptTag {
    name: string
    content: string
    owner: string
    uses: number
}

export interface ImageScriptPackage {
    name: string
    content: string
    owner: string
}

export interface Reminder {
    timestamp: string,
    message: string,
    guild_id: string,
    user_id: string,
    channel_id: string,
    message_id: string
  }