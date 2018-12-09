import { Expr, ExpressionTypes, Visitor } from './expr';
import { bool, Token, TokenType } from '..';


export class GroupingExpr extends Expr {
    public readonly exprType = ExpressionTypes.GROUPING;
    public readonly members: string[];

    public static allowedKwList = bool;

    public static isGroupingExpr(tokens: Token[]) {
        let hasStart = false,
            hasEnd = false;
        return tokens.length &&
            tokens.every((t, i) => {
                let result;
                if (i === 0) {
                    result = t.tokenType === TokenType.PUNCT && t.value === '(';
                    hasStart = result;
                }
                else if (i === tokens.length - 1) {
                    result = t.tokenType === TokenType.PUNCT && t.value === ')';
                    hasEnd = result;
                }
                else {
                    result = t.tokenType === TokenType.VAR ||
                        t.tokenType === TokenType.INLINE_CONTENT ||
                        t.tokenType === TokenType.REG_EXP ||
                        t.tokenType === TokenType.NUM ||
                        (t.tokenType === TokenType.KW && GroupingExpr.allowedKwList.indexOf(t.value) >= 0) ||
                        (t.tokenType === TokenType.PUNCT && ')('.indexOf(t.value) < 0 ) ||
                        (t.tokenType === TokenType.OP && t.value === ':') ||
                        GroupingExpr.isGroupingExpr(tokens.slice(i, tokens.length - 1));
                }

                // console.log(result, `${i+1}/${tokens.length}`, t.value, t.tokenType);

                return result;
            }) && hasStart && hasEnd;
    }

    public static groupingType(tokens: Token[]): ExpressionTypes {

    }

    public static groupingTree(tokens: Token[]) {
        const openingIndex = [],
            closingIndex = [];
        tokens.forEach((t, i) => {
            if (t.tokenType === TokenType.PUNCT && t.value === '(') {
                openingIndex.push(i);
            }
            else if (t.tokenType === TokenType.PUNCT && t.value === ')') {
                closingIndex.push(i);
            }
        });

        if (openingIndex.length !== closingIndex.length) {
            throw new SyntaxError(`Invalid grouping expression: [${tokens.map(t => t.value).join(' ')}] [${tokens[0].start[0]}:${tokens[0].start[1]}]`)
        }


        let lvl = 0;
        openingIndex.reduce((tree, o, i, all) => {
            lvl++;
            const aSide = [];
            while (o < openingIndex[i + 1] && closingIndex[i] > o && tokens[o]) {
                aSide.push(tokens[o++]);
            }
            if (closingIndex[i] === o) {
                tree.push(aSide);
                lvl--;
            }
            else if (o === openingIndex[i + 1]) {

            }
            return tree;
        },[])
    }



    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
        // this.members =
        // if (!this.members.length) {
        //     throw new EvalError(`Empty group is not allowed [${tokens[0].start[0]}:${tokens[0].start[1]}]`);
        // }
    }

    accept(visitor: Visitor) {
        visitor.visitGroupingExpr(this);
    }

}