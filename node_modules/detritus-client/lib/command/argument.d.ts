import { CommandArgumentTypes } from '../constants';
import { ArgumentParser } from './argumentparser';
import { Context } from './context';
export declare type ArgumentConverter = (value: string, context: Context) => Promise<any> | any;
export declare type ArgumentDefault = ((context: Context) => Promise<any> | any) | any;
export declare type ArgumentType = ArgumentConverter | Boolean | Number | String | CommandArgumentTypes | Array<ArgumentOptions>;
/**
 * Command Argument Options
 * @category Command Options
 */
export interface ArgumentOptions {
    aliases?: Array<string>;
    choices?: Array<any>;
    consume?: boolean;
    default?: ArgumentDefault;
    help?: string;
    label?: string;
    metadata?: {
        [key: string]: any;
    };
    name: string;
    prefix?: string;
    prefixes?: Array<string>;
    prefixSpace?: boolean;
    required?: boolean;
    type?: ArgumentType;
}
/**
 * Command Argument
 * @category Command
 */
export declare class Argument {
    private _aliases;
    private _label;
    private _name;
    private _names?;
    private _type;
    positionalArgs?: ArgumentParser;
    choices?: Array<any>;
    consume?: boolean;
    default: ArgumentDefault;
    help: string;
    metadata?: {
        [key: string]: any;
    };
    prefixes: Set<string>;
    required: boolean;
    constructor(options: ArgumentOptions);
    get aliases(): Array<string>;
    set aliases(value: Array<string>);
    get label(): string;
    set label(value: string);
    get name(): string;
    set name(value: string);
    get names(): Array<string>;
    get type(): ArgumentType;
    set type(value: ArgumentType);
    check(name: string): boolean;
    getInfo(content: string): {
        index: number;
        name: string;
    };
    getName(content: string): null | string;
    setPrefixes(prefixes: Array<string>, prefixSpace?: boolean): void;
    parse(value: string, context: Context): Promise<any>;
}
