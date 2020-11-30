export interface User {
    id: string
    username: string
    discriminator: number
    avatar: string
    bot: boolean
    flags: number
    ownerids: any[]
    timestamp: string
    premiumsince: string
    role: number
    settings: number
    connections: Connection[]
    guilds: UserGuild[]
    bans: any[]
    typing: UserGuild[]
    tag: string
    totalGuilds: number
}

export interface Connection {
    url: string
    connectionid: string
    type: string
}

export interface UserGuild {
    userid: string
    guildid: string
    timestamp: Date
    joined?: Date
    guild_name: string
    channelid?: string
}

export interface Guild {
    id: string
    name: string
    icon: string
    timestamp: Date
    member_count: number
    channel_count: number
    emoji_count: number
    role_count: number
    ownerid: string
    preferred_locale: string
    region: string
    splash: string | null
    discovery_splash: string | null
    banner: string | null
    description: string | null
    flags: number
    features: any[]
    invites: Invite[]
    bans: any[]
    owner: Owner
}

export interface Invite {
    id: string
    invite: string
    timestamp: string
}

export interface Owner {
    id: string
    username: string
    discriminator: number
    avatar: string
    bot: boolean
    flags: number
    ownerids: null
    timestamp: string
    email: null
    phone: null
    premiumsince: string
    role: number
    settings: number
}
