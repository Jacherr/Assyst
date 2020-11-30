import { Message } from '../structures';
import { ParsedArgs, ParsedErrors } from './argumentparser';
import { Command, FailedPermissions } from './command';
import { Context } from './context';
import { CommandRatelimit as CommandRatelimitCache, CommandRatelimitItem } from './ratelimit';
export declare namespace CommandEvents {
    interface CommandDelete {
        command: Command;
        context: Context;
        reply: Message;
    }
    interface CommandError {
        command: Command;
        context: Context;
        error: Error;
        extra?: Error | ParsedErrors;
        reply?: Message;
    }
    interface CommandFail {
        args: ParsedArgs;
        command: Command;
        context: Context;
        error: Error;
        prefix: string;
    }
    interface CommandNone {
        context: Context;
        error: Error;
    }
    interface CommandPermissionsFailClient {
        command: Command;
        context: Context;
        permissions: FailedPermissions;
    }
    interface CommandPermissionsFail {
        command: Command;
        context: Context;
        permissions: FailedPermissions;
    }
    interface CommandRatelimit {
        command: Command;
        context: Context;
        global: boolean;
        now: number;
        ratelimits: Array<{
            item: CommandRatelimitItem;
            ratelimit: CommandRatelimitCache;
            remaining: number;
        }>;
    }
    interface CommandRan {
        args: ParsedArgs;
        command: Command;
        context: Context;
        prefix: string;
    }
    interface CommandResponseDelete {
        command: Command;
        context: Context;
        reply: Message;
    }
    interface CommandRunError {
        args: ParsedArgs;
        command: Command;
        context: Context;
        error: Error;
        prefix: string;
    }
}
