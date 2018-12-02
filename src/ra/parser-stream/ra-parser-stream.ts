// import { RaTokenStream } from '../token-stream/ra-token-stream';
//
// const FALSE = { type: "bool", value: false };
//
// export class RaParserStream {
//     private PRECEDENCE = {
//         "=": 1,
//         "||": 2,
//         "&&": 3,
//         "<": 7, ">": 7, "<=": 7, ">=": 7, "==": 7, "!=": 7,
//         "+": 10, "-": 10,
//         "*": 20, "/": 20, "%": 20,
//     };
//
//     constructor(
//         private input: RaTokenStream
//     ) {}
//
//     parse() {
//         return this.parseTopLevel()
//     }
//
//     private isPunc(ch) {
//         const tok = this.input.peek();
//         return tok && tok.type === "punc" && (!ch || tok.value === ch) && tok;
//     }
//     private isKw(kw) {
//         const tok = this.input.peek();
//         return tok && tok.type === "kw" && (!kw || tok.value === kw) && tok;
//     }
//     private isOp(op?) {
//         const tok = this.input.peek();
//         return tok && tok.type === "op" && (!op || tok.value === op) && tok;
//     }
//     private skipPunc(ch) {
//         if (this.isPunc(ch)) this.input.next();
//         else this.input.croak("Expecting punctuation: \"" + ch + "\"");
//     }
//     private skipKw(kw) {
//         if (this.isKw(kw)) this.input.next();
//         else this.input.croak("Expecting keyword: \"" + kw + "\"");
//     }
//     private skipOp(op) {
//         if (this.isOp(op)) this.input.next();
//         else this.input.croak("Expecting operator: \"" + op + "\"");
//     }
//     private unexpected() {
//         this.input.croak("Unexpected token: " + JSON.stringify(this.input.peek()));
//     }
//     private maybeBinary(left, myPrec) {
//         const tok = this.isOp();
//         if (tok) {
//             const hisPrec = this.PRECEDENCE[tok.value];
//             if (hisPrec > myPrec) {
//                 this.input.next();
//                 return this.maybeBinary({
//                     type     : tok.value === "=" ? "assign" : "binary",
//                     operator : tok.value,
//                     left     : left,
//                     right    : this.maybeBinary(this.parseAtom(), hisPrec)
//                 }, myPrec);
//             }
//         }
//         return left;
//     }
//     private delimited(start, stop, separator, parser) {
//         const a = [];
//         let first = true;
//         this.skipPunc(start);
//         while (!this.input.eof()) {
//             if (this.isPunc(stop)) break;
//             if (first) first = false; else this.skipPunc(separator);
//             if (this.isPunc(stop)) break;
//             a.push(parser());
//         }
//         this.skipPunc(stop);
//         return a;
//     }
//     private parseCall(func) {
//         return {
//             type: "call",
//             func: func,
//             args: this.delimited("(", ")", ",", this.parseExpression.bind(this)),
//         };
//     }
//     private parseVarName() {
//         const name = this.input.next();
//         if (name.type != "var") this.input.croak("Expecting variable name");
//         return name.value;
//     }
//     private parseIf() {
//         this.skipKw("if");
//         const cond = this.parseExpression();
//         if (!this.isPunc("{")) this.skipKw("then");
//         const then = this.parseExpression();
//         const ret: any = {
//             type: "if",
//             cond: cond,
//             then: then,
//         };
//         if (this.isKw("else")) {
//             this.input.next();
//             ret.else = this.parseExpression();
//         }
//         return ret;
//     }
//     private parseLambda() {
//         return {
//             type: "lambda",
//             vars: this.delimited("(", ")", ",", this.parseVarName.bind(this)),
//             body: this.parseExpression()
//         };
//     }
//     private parseBool() {
//         return {
//             type  : "bool",
//             value : this.input.next().value === "true"
//         };
//     }
//     private maybeCall(expr) {
//         expr = expr();
//         return this.isPunc("(") ? this.parseCall(expr) : expr;
//     }
//     private parseAtom() {
//         return this.maybeCall(() => {
//             if (this.isPunc("(")) {
//                 this.input.next();
//                 const exp = this.parseExpression();
//                 this.skipPunc(")");
//                 return exp;
//             }
//             if (this.isPunc("{")) return this.parseProg();
//             if (this.isKw("if")) return this.parseIf();
//             if (this.isKw("true") || this.isKw("false")) return this.parseBool();
//             if (this.isKw("lambda") || this.isKw("λ")) {
//                 this.input.next();
//                 return this.parseLambda();
//             }
//             const tok = this.input.next();
//             if (tok.type === "var" || tok.type === "num" || tok.type === "str")
//                 return tok;
//             this.unexpected();
//         });
//     }
//     private parseTopLevel() {
//         const prog = [];
//         while (!this.input.eof()) {
//             prog.push(this.parseExpression());
//             if (!this.input.eof()) this.skipPunc(";");
//         }
//         return { type: "prog", prog: prog };
//     }
//     private parseProg() {
//         const prog = this.delimited("{", "}", ";", this.parseExpression.bind(this));
//         if (prog.length === 0) return FALSE;
//         if (prog.length === 1) return prog[0];
//         return { type: "prog", prog: prog };
//     }
//     private parseExpression() {
//         return this.maybeCall(() => {
//             return this.maybeBinary(this.parseAtom(), 0);
//         });
//     }
// }