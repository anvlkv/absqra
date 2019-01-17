import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token, TokenType } from '../token-stream/token';


export class ContentExpr extends Expr {
    readonly exprType = ExpressionTypes.CONTENT;

    public value: string;

    public readonly ln: string;
    public readonly lang: string;

    constructor(...tokens: Token[]){
        super(...tokens);
        // const ln = tokens.findIndex(t => t.tokenType === TokenType.KW && t.value === 'ln');
        // const lang = tokens.findIndex(t => t.tokenType === TokenType.KW && t.value === 'lang');
        // if (ln) {
            // this.ln = ;
        // }
    }

    accept(visitor: Visitor) {
        visitor.visitContentExpr(this);
    }

}