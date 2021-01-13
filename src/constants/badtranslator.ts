// Defines how often messages are translated by the same user
export const TRANSLATION_RATELIMIT_RESET = 5000;
// Defines the maximum number of users to have their rate limit cached
export const MAX_CACHE_SIZE = 10;
// Defines the maximum number of characters a message can have
export const MAX_MESSAGE_LENGTH = 500;
// Defines the message content that is sent when someone is sending messages too quickly
export const RATELIMIT_MESSAGE = ' you\'re sending messages too quickly!';