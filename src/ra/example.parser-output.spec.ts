export const exampleParserOutput = {
    type: "prog",
    prog: [
        // first line:
        { type: "num", value: 123.5 },
        // second line:
        { type: "str", value: "Hello World!" },
        { type: "bool", value: true },
        { type: "bool", value: false },
        { type: "var", value: "foo" },
        {
            type: "lambda",
            vars: [ "x" ],
            body: { type: "num", value: 10 }
        }
    ]
};