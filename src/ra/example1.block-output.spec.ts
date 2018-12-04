import { RaTypes } from './types.enum';
import { BlockType } from './block-stream/block';


export const example1BlockOutput = [
    {
        blockType: BlockType.DECLARE,
        content: [
            {
                end: [
                    2,
                    0
                ],
                indent: 0,
                start: [
                    1,
                    0
                ],
                value: "spammer: Sequence  // this is single line comment and start of Declaration Block\n"
            },
            {
                end: [
                    3,
                    0
                ],
                indent: 1,
                start: [
                    2,
                    0
                ],
                value: "\tquestion respondentName // Block type Question with name respondentName for it's output\n"
            },
            {
                end: [
                    4,
                    0
                ],
                indent: 2,
                start: [
                    3,
                    0
                ],
                value: "\t\t>text // input type text\n"
            },
            {
                end: [
                    5,
                    0
                ],
                indent: 2,
                start: [
                    4,
                    0
                ],
                value: "\t\t!required // Validator type required\n"
            },
            {
                end: [
                    6,
                    0
                ],
                indent: 2,
                start: [
                    5,
                    0
                ],
                value: "\t\t!minLength=3 // Validator type minLength with parameter 3\n"
            },
            {
                end: [
                    7,
                    0
                ],
                indent: 2,
                start: [
                    6,
                    0
                ],
                value: "\t\t// between pair of ` there's content code by default MD\n"
            },
            {
                end: [
                    8,
                    0
                ],
                indent: 2,
                start: [
                    7,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    9,
                    0
                ],
                indent: 3,
                start: [
                    8,
                    0
                ],
                value: "\t\t\t#What is your name?\n"
            },
            {
                end: [
                    10,
                    0
                ],
                indent: 2,
                start: [
                    9,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    11,
                    0
                ],
                indent: 1,
                start: [
                    10,
                    0
                ],
                value: "\tquestion respondentAge\n"
            },
            {
                end: [
                    12,
                    0
                ],
                indent: 2,
                start: [
                    11,
                    0
                ],
                value: "\t\t>number\n"
            },
            {
                end: [
                    13,
                    0
                ],
                indent: 2,
                start: [
                    12,
                    0
                ],
                value: "\t\t!required\n"
            },
            {
                end: [
                    14,
                    0
                ],
                indent: 2,
                start: [
                    13,
                    0
                ],
                value: "\t\t!minValue=18\n"
            },
            {
                end: [
                    15,
                    0
                ],
                indent: 2,
                start: [
                    14,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    16,
                    0
                ],
                indent: 3,
                start: [
                    15,
                    0
                ],
                value: "\t\t\t#What is your age?\n"
            },
            {
                end: [
                    17,
                    0
                ],
                indent: 2,
                start: [
                    16,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    18,
                    0
                ],
                indent: 1,
                start: [
                    17,
                    0
                ],
                value: "\tquestion respondentFamilyMembers\n"
            },
            {
                end: [
                    19,
                    0
                ],
                indent: 2,
                start: [
                    18,
                    0
                ],
                value: "\t\t>list\n"
            },
            {
                end: [
                    20,
                    0
                ],
                indent: 2,
                start: [
                    19,
                    0
                ],
                value: "\t\t!minCount=2\n"
            },
            {
                end: [
                    21,
                    0
                ],
                indent: 2,
                start: [
                    20,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    22,
                    0
                ],
                indent: 3,
                start: [
                    21,
                    0
                ],
                value: "\t\t\t#Please add names of your family members.\n"
            },
            {
                end: [
                    23,
                    0
                ],
                indent: 2,
                start: [
                    22,
                    0
                ],
                value: "\t\t`\n"
            },
            {
                end: [
                    25,
                    0
                ],
                indent: 1,
                start: [
                    24,
                    0
                ],
                value: "\tif respondentFamilyMembers // this If Block checks that respondentFamilyMembers is valid and has value\n"
            },
            {
                end: [
                    26,
                    0
                ],
                indent: 2,
                start: [
                    25,
                    0
                ],
                value: "\t\tquestion familyMembersEmails\n"
            },
            {
                end: [
                    27,
                    0
                ],
                indent: 3,
                start: [
                    26,
                    0
                ],
                value: "\t\t\t>multiple\n"
            },
            {
                end: [
                    28,
                    0
                ],
                indent: 3,
                start: [
                    27,
                    0
                ],
                value: "\t\t\t>assignment\n"
            },
            {
                end: [
                    29,
                    0
                ],
                indent: 3,
                start: [
                    28,
                    0
                ],
                value: "\t\t\t!format=email\n"
            },
            {
                end: [
                    30,
                    0
                ],
                indent: 3,
                start: [
                    29,
                    0
                ],
                value: "\t\t\t+respondentFamilyMembers // + sign is for options\n"
            },
            {
                end: [
                    31,
                    0
                ],
                indent: 3,
                start: [
                    30,
                    0
                ],
                value: "\t\t\t`\n"
            },
            {
                end: [
                    32,
                    0
                ],
                indent: 4,
                start: [
                    31,
                    0
                ],
                value: "\t\t\t\t#Please add emails of your family members.\n"
            },
            {
                end: [
                    33,
                    0
                ],
                indent: 3,
                start: [
                    32,
                    0
                ],
                value: "\t\t\t`\n"
            },
            {
                end: [
                    35,
                    0
                ],
                indent: 2,
                start: [
                    34,
                    0
                ],
                value: "\t\t// this Content Block doesn't have a name it is then simply there in Sequence, not anywhere else\n"
            },
            {
                end: [
                    36,
                    0
                ],
                indent: 2,
                start: [
                    35,
                    0
                ],
                value: "\t\tcontent /* try comment */\n"
            },
            {
                end: [
                    37,
                    0
                ],
                indent: 3,
                start: [
                    36,
                    0
                ],
                value: "\t\t\t`\n"
            },
            {
                end: [
                    38,
                    0
                ],
                indent: 4,
                start: [
                    37,
                    0
                ],
                value: "\t\t\t\t###Thank you so much!\n"
            },
            {
                end: [
                    39,
                    0
                ],
                indent: 4,
                start: [
                    38,
                    0
                ],
                value: "\t\t\t\t##Please [enjoy this video](http://youtube.com/hdjkfhskfhwir)\n"
            },
            {
                end: [
                    40,
                    0
                ],
                indent: 3,
                start: [
                    39,
                    0
                ],
                value: "\t\t\t`\n"
            },
            {
                end: [
                    41,
                    0
                ],
                indent: 2,
                start: [
                    40,
                    0
                ],
                value: "\t\ttask /* try comment\n"
            },
            {
                end: [
                    42,
                    0
                ],
                indent: 2,
                start: [
                    41,
                    0
                ],
                value: "\t\t        one more time */\n"
            },
            {
                end: [
                    43,
                    0
                ],
                indent: 3,
                start: [
                    42,
                    0
                ],
                value: "\t\t\t// first thing a Task does is selecting environment as input type\n"
            },
            {
                end: [
                    44,
                    0
                ],
                indent: 3,
                start: [
                    43,
                    0
                ],
                value: "\t\t\t>server\n"
            },
            {
                end: [
                    45,
                    0
                ],
                indent: 3,
                start: [
                    44,
                    0
                ],
                value: "\t\t\t// then it selects contextually available Task mailer and provides it with argument\n"
            },
            {
                end: [
                    46,
                    0
                ],
                indent: 3,
                start: [
                    45,
                    0
                ],
                value: "\t\t\t>mailer = `\n"
            },
            {
                end: [
                    47,
                    0
                ],
                indent: 4,
                start: [
                    46,
                    0
                ],
                value: "\t\t\t\tPlease [participate in our survey](https://absqra.io/vx2zh32g5c)! Your dear {{respondentName}} already did!\n"
            },
            {
                end: [
                    48,
                    0
                ],
                indent: 3,
                start: [
                    47,
                    0
                ],
                value: "\t\t\t`\n"
            },
            {
                end: [
                    49,
                    0
                ],
                indent: 3,
                start: [
                    48,
                    0
                ],
                value: "\t\t\t+familyMembersEmails\n"
            },
            {
                end: [
                    50,
                    0
                ],
                indent: 1,
                start: [
                    49,
                    0
                ],
                value: "\telse // make them struggle in this else Block\n"
            },
            {
                end: [
                    51,
                    0
                ],
                indent: 2,
                start: [
                    50,
                    0
                ],
                value: "\t\tsequence\n"
            },
            {
                end: [
                    52,
                    0
                ],
                indent: 3,
                start: [
                    51,
                    0
                ],
                value: "\t\t\t>spammer\n"
            }
        ],
        end: [
            53,
            0
        ],
        level: 0,
        start: [
            1,
            0
        ],
        type: RaTypes.BLOCK
    },
    {
        blockType: BlockType.DECLARE,
        content: [
            {
                end: [
                    54,
                    0
                ],
                indent: 0,
                start: [
                    53,
                    0
                ],
                value: "mailer: Task\n"
            },
            {
                end: [
                    55,
                    0
                ],
                indent: 1,
                start: [
                    54,
                    0
                ],
                value: "\t// code here is in typescript\n"
            },
            {
                end: [
                    56,
                    0
                ],
                indent: 1,
                start: [
                    55,
                    0
                ],
                value: "\t`lang=typescript\n"
            },
            {
                end: [
                    57,
                    0
                ],
                indent: 2,
                start: [
                    56,
                    0
                ],
                value: "\t\t// some code sending mail here...\n"
            },
            {
                end: [
                    58,
                    0
                ],
                indent: 2,
                start: [
                    57,
                    0
                ],
                value: "\t\tfunction(content: string) {\n"
            },
            {
                end: [
                    59,
                    0
                ],
                indent: 3,
                start: [
                    58,
                    0
                ],
                value: "\t\t\tconsole.log(content)\n"
            },
            {
                end: [
                    60,
                    0
                ],
                indent: 3,
                start: [
                    59,
                    0
                ],
                value: "\t\t\t// yeah that's how we send mail\n"
            },
            {
                end: [
                    61,
                    0
                ],
                indent: 2,
                start: [
                    60,
                    0
                ],
                value: "\t\t}\n"
            },
            {
                end: [
                    61,
                    3
                ],
                indent: 1,
                start: [
                    61,
                    0
                ],
                value: "\t`"
            }
        ],
        end: [
            61,
            3
        ],
        level: 0,
        start: [
            53,
            0
        ],
        type: RaTypes.BLOCK
    }
];
