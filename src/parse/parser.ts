import { EXPRESSIONS } from "./expressions";

const EXPRESSION_OPENING_CHARACTER = '{';
const EXPRESSION_CLOSING_CHARACTER = '}';
const EXPRESSION_OP_DIVIDER = ':';
const EXPRESSION_OPERANDS_DIVIDER = '|';
const EXPRESSION_ESCAPE_CHARACTER = '\\';

export class Parser {
    public args!: string[];
    public currentDepth: number = 0;
    public errors: string[] = [];
    public ignoreSections: string[][] = [];
    public valueStore = new Map<string, string>();

    public async parse(input: string, args?: string[]) {
        this.args = args || [];
        let noChangeInOutput = false;
        let newInput = input;
        while(!noChangeInOutput) {
            let output = await this.parseInput(newInput);
            if(output === newInput) noChangeInOutput = true;
            newInput = output;
        }
        return newInput.replace(/\\{/g, '{');
    }

    private async parseInput(input: string): Promise<string> {
        for(let index = 0; index < input.length; index++) {
            if(input[index] === EXPRESSION_CLOSING_CHARACTER) {
                let expression: string | null = null;
                for(let i = index; i > -1; i--) {
                    if(input[i] === EXPRESSION_OPENING_CHARACTER && input[i - 1] !== EXPRESSION_ESCAPE_CHARACTER) {
                        expression = input.slice(i, index + 1);
                        break;
                    }
                }
                if(!expression) continue;
                let output = input;
                output = output.replace(expression, await this.parseExpression(expression));
                return output;
            }
        }
        return input;
    }

    private async parseExpression(expression: string): Promise<string> {
        expression = expression.slice(1, expression.length - 1); // remove expression opening and closing characters
        const expressionParts = expression.split(EXPRESSION_OP_DIVIDER);
        const opcode = expressionParts[0];
        const operand = expressionParts.slice(1).join(':');
        const allOperands = operand.split(EXPRESSION_OPERANDS_DIVIDER);
        const res = await this.evaluateExpression(opcode, allOperands);
        return res;
    }

    private async evaluateExpression(opcode: string, operands: string[]): Promise<string> {
        const expression = EXPRESSIONS.get(opcode);
        if(!expression) return this.formatExpressionString(opcode, operands);
        if(expression.requirements) {
            for(const requirement of expression.requirements) {
                const result = requirement.fn({ opcode, operands, parser: this });
                if(result === false) {
                    this.errors.push(`${opcode}: ${requirement.errorMessage}`);
                    return this.formatExpressionString(opcode, operands);
                }
            }
        }
        return await expression.execute({ opcode, operands, parser: this });
    }

    public formatExpressionString(opcode: string, operands: string[]) {
        return `${EXPRESSION_OPENING_CHARACTER}${opcode}${operands.length > 0 ? `${EXPRESSION_OP_DIVIDER}${operands.join('|')}` : ''}${EXPRESSION_CLOSING_CHARACTER}`
    }
}