import { CommandAttributes } from '../commandclient';
import { Argument, ArgumentOptions } from './argument';
import { Context } from './context';
/**
 * @category Command
 */
export declare type ParsedArgs = {
    [key: string]: any;
} | any;
/**
 * @category Command
 */
export declare type ParsedErrors = {
    [key: string]: Error;
} | any;
/**
 * Command Argument
 * @category Command
 */
export declare class ArgumentParser {
    readonly args: Array<Argument>;
    positional: boolean;
    constructor(args?: Array<ArgumentOptions>, positional?: boolean);
    initialize(args?: Array<ArgumentOptions>): void;
    parse(attributes: CommandAttributes, context: Context): Promise<{
        errors: ParsedErrors;
        parsed: ParsedArgs;
    }>;
}
