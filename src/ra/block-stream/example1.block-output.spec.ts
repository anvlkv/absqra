import { RaTypes } from '../types.enum';
import { BlockType } from './block';


export const example1Output = {
    "blocks": [
        {
            "blockType": "declare",
            "content": [
                {
                    "indent": 0,
                    "_value": "spammer: Sequence  // this is single line comment and start of Declaration Block\n",
                    "start": [
                        1,
                        1
                    ],
                    "end": [
                        1,
                        81
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "spammer",
                            "start": [
                                1,
                                1
                            ],
                            "end": [
                                1,
                                8
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": ":",
                            "start": [
                                1,
                                8
                            ],
                            "end": [
                                1,
                                9
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "Sequence",
                            "start": [
                                1,
                                10
                            ],
                            "end": [
                                1,
                                18
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\tquestion respondentName // Block type Question with name respondentName for it's output\n",
                    "start": [
                        2,
                        1
                    ],
                    "end": [
                        2,
                        92
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "question",
                            "start": [
                                2,
                                5
                            ],
                            "end": [
                                2,
                                13
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "respondentName",
                            "start": [
                                2,
                                14
                            ],
                            "end": [
                                2,
                                28
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t>text // input type text\n",
                    "start": [
                        3,
                        1
                    ],
                    "end": [
                        3,
                        33
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                3,
                                9
                            ],
                            "end": [
                                3,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "text",
                            "start": [
                                3,
                                10
                            ],
                            "end": [
                                3,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t!required // Validator type required\n",
                    "start": [
                        4,
                        1
                    ],
                    "end": [
                        4,
                        45
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                4,
                                9
                            ],
                            "end": [
                                4,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "required",
                            "start": [
                                4,
                                10
                            ],
                            "end": [
                                4,
                                18
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t!minLength=3 // Validator type minLength with parameter 3\n",
                    "start": [
                        5,
                        1
                    ],
                    "end": [
                        5,
                        66
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                5,
                                9
                            ],
                            "end": [
                                5,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "minLength",
                            "start": [
                                5,
                                10
                            ],
                            "end": [
                                5,
                                19
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                5,
                                19
                            ],
                            "end": [
                                5,
                                20
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "num",
                            "value": 3,
                            "start": [
                                5,
                                20
                            ],
                            "end": [
                                5,
                                21
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t// between pair of ` there's content code by default MD\n",
                    "start": [
                        6,
                        1
                    ],
                    "end": [
                        6,
                        64
                    ],
                    "_tokens": []
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        7,
                        1
                    ],
                    "end": [
                        7,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                7,
                                9
                            ],
                            "end": [
                                7,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t#What is your name?\n",
                    "start": [
                        8,
                        1
                    ],
                    "end": [
                        8,
                        32
                    ],
                    "_tokens": null
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        9,
                        1
                    ],
                    "end": [
                        9,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                9,
                                9
                            ],
                            "end": [
                                9,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\tquestion respondentAge\n",
                    "start": [
                        10,
                        1
                    ],
                    "end": [
                        10,
                        27
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "question",
                            "start": [
                                10,
                                5
                            ],
                            "end": [
                                10,
                                13
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "respondentAge",
                            "start": [
                                10,
                                14
                            ],
                            "end": [
                                10,
                                27
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t>number\n",
                    "start": [
                        11,
                        1
                    ],
                    "end": [
                        11,
                        16
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                11,
                                9
                            ],
                            "end": [
                                11,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "number",
                            "start": [
                                11,
                                10
                            ],
                            "end": [
                                11,
                                16
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t!required\n",
                    "start": [
                        12,
                        1
                    ],
                    "end": [
                        12,
                        18
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                12,
                                9
                            ],
                            "end": [
                                12,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "required",
                            "start": [
                                12,
                                10
                            ],
                            "end": [
                                12,
                                18
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t!minValue=18\n",
                    "start": [
                        13,
                        1
                    ],
                    "end": [
                        13,
                        21
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                13,
                                9
                            ],
                            "end": [
                                13,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "minValue",
                            "start": [
                                13,
                                10
                            ],
                            "end": [
                                13,
                                18
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                13,
                                18
                            ],
                            "end": [
                                13,
                                19
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "num",
                            "value": 18,
                            "start": [
                                13,
                                19
                            ],
                            "end": [
                                13,
                                21
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        14,
                        1
                    ],
                    "end": [
                        14,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                14,
                                9
                            ],
                            "end": [
                                14,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t#What is your age?\n",
                    "start": [
                        15,
                        1
                    ],
                    "end": [
                        15,
                        31
                    ],
                    "_tokens": null
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        16,
                        1
                    ],
                    "end": [
                        16,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                16,
                                9
                            ],
                            "end": [
                                16,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\tquestion respondentFamilyMembers\n",
                    "start": [
                        17,
                        1
                    ],
                    "end": [
                        17,
                        37
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "question",
                            "start": [
                                17,
                                5
                            ],
                            "end": [
                                17,
                                13
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "respondentFamilyMembers",
                            "start": [
                                17,
                                14
                            ],
                            "end": [
                                17,
                                37
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t>list\n",
                    "start": [
                        18,
                        1
                    ],
                    "end": [
                        18,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                18,
                                9
                            ],
                            "end": [
                                18,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "list",
                            "start": [
                                18,
                                10
                            ],
                            "end": [
                                18,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t!minCount=2\n",
                    "start": [
                        19,
                        1
                    ],
                    "end": [
                        19,
                        20
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                19,
                                9
                            ],
                            "end": [
                                19,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "minCount",
                            "start": [
                                19,
                                10
                            ],
                            "end": [
                                19,
                                18
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                19,
                                18
                            ],
                            "end": [
                                19,
                                19
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "num",
                            "value": 2,
                            "start": [
                                19,
                                19
                            ],
                            "end": [
                                19,
                                20
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        20,
                        1
                    ],
                    "end": [
                        20,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                20,
                                9
                            ],
                            "end": [
                                20,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t#Please add names of your family members.\n",
                    "start": [
                        21,
                        1
                    ],
                    "end": [
                        21,
                        54
                    ],
                    "_tokens": null
                },
                {
                    "indent": 2,
                    "_value": "\t\t`\n",
                    "start": [
                        22,
                        1
                    ],
                    "end": [
                        22,
                        10
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                22,
                                9
                            ],
                            "end": [
                                22,
                                10
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\tif respondentFamilyMembers // this If Block checks that respondentFamilyMembers is valid and has value\n",
                    "start": [
                        24,
                        1
                    ],
                    "end": [
                        24,
                        107
                    ],
                    "_tokens": [
                        {
                            "tokenType": "kw",
                            "value": "if",
                            "start": [
                                24,
                                5
                            ],
                            "end": [
                                24,
                                7
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "respondentFamilyMembers",
                            "start": [
                                24,
                                8
                            ],
                            "end": [
                                24,
                                31
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\tquestion familyMembersEmails\n",
                    "start": [
                        25,
                        1
                    ],
                    "end": [
                        25,
                        37
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "question",
                            "start": [
                                25,
                                9
                            ],
                            "end": [
                                25,
                                17
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "familyMembersEmails",
                            "start": [
                                25,
                                18
                            ],
                            "end": [
                                25,
                                37
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t>multiple\n",
                    "start": [
                        26,
                        1
                    ],
                    "end": [
                        26,
                        22
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                26,
                                13
                            ],
                            "end": [
                                26,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "multiple",
                            "start": [
                                26,
                                14
                            ],
                            "end": [
                                26,
                                22
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t>assignment\n",
                    "start": [
                        27,
                        1
                    ],
                    "end": [
                        27,
                        24
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                27,
                                13
                            ],
                            "end": [
                                27,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "assignment",
                            "start": [
                                27,
                                14
                            ],
                            "end": [
                                27,
                                24
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t!format=email\n",
                    "start": [
                        28,
                        1
                    ],
                    "end": [
                        28,
                        26
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "!",
                            "start": [
                                28,
                                13
                            ],
                            "end": [
                                28,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "format",
                            "start": [
                                28,
                                14
                            ],
                            "end": [
                                28,
                                20
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                28,
                                20
                            ],
                            "end": [
                                28,
                                21
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "email",
                            "start": [
                                28,
                                21
                            ],
                            "end": [
                                28,
                                26
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t+respondentFamilyMembers // + sign is for options\n",
                    "start": [
                        29,
                        1
                    ],
                    "end": [
                        29,
                        62
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "+",
                            "start": [
                                29,
                                13
                            ],
                            "end": [
                                29,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "respondentFamilyMembers",
                            "start": [
                                29,
                                14
                            ],
                            "end": [
                                29,
                                37
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`ln=En\n",
                    "start": [
                        30,
                        1
                    ],
                    "end": [
                        30,
                        19
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                30,
                                13
                            ],
                            "end": [
                                30,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "kw",
                            "value": "ln",
                            "start": [
                                30,
                                14
                            ],
                            "end": [
                                30,
                                16
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                30,
                                16
                            ],
                            "end": [
                                30,
                                17
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "En",
                            "start": [
                                30,
                                17
                            ],
                            "end": [
                                30,
                                19
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 4,
                    "_value": "\t\t\t\t#Please add emails of your family members.\n",
                    "start": [
                        31,
                        1
                    ],
                    "end": [
                        31,
                        59
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        32,
                        1
                    ],
                    "end": [
                        32,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                32,
                                13
                            ],
                            "end": [
                                32,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`ln=Ru\n",
                    "start": [
                        33,
                        1
                    ],
                    "end": [
                        33,
                        19
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                33,
                                13
                            ],
                            "end": [
                                33,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "kw",
                            "value": "ln",
                            "start": [
                                33,
                                14
                            ],
                            "end": [
                                33,
                                16
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                33,
                                16
                            ],
                            "end": [
                                33,
                                17
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "Ru",
                            "start": [
                                33,
                                17
                            ],
                            "end": [
                                33,
                                19
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 4,
                    "_value": "\t\t\t\t#Пожалуйста укажите адреса электропочты членов вашей семьи.\n",
                    "start": [
                        34,
                        1
                    ],
                    "end": [
                        34,
                        76
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        35,
                        1
                    ],
                    "end": [
                        35,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                35,
                                13
                            ],
                            "end": [
                                35,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t// this Content Block doesn't have a name it is then simply there in Sequence, not anywhere else\n",
                    "start": [
                        37,
                        1
                    ],
                    "end": [
                        37,
                        105
                    ],
                    "_tokens": []
                },
                {
                    "indent": 2,
                    "_value": "\t\tcontent /* try comment */\n",
                    "start": [
                        38,
                        1
                    ],
                    "end": [
                        38,
                        34
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "content",
                            "start": [
                                38,
                                9
                            ],
                            "end": [
                                38,
                                16
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        39,
                        1
                    ],
                    "end": [
                        39,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                39,
                                13
                            ],
                            "end": [
                                39,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 4,
                    "_value": "\t\t\t\t###Thank you so much!\n",
                    "start": [
                        40,
                        1
                    ],
                    "end": [
                        40,
                        38
                    ],
                    "_tokens": null
                },
                {
                    "indent": 4,
                    "_value": "\t\t\t\t##Please [enjoy this video](http://youtube.com/hdjkfhskfhwir)\n",
                    "start": [
                        41,
                        1
                    ],
                    "end": [
                        41,
                        78
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        42,
                        1
                    ],
                    "end": [
                        42,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                42,
                                13
                            ],
                            "end": [
                                42,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "task ",
                    "start": [
                        43,
                        1
                    ],
                    "end": [
                        45,
                        78
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "task",
                            "start": [
                                43,
                                1
                            ],
                            "end": [
                                43,
                                5
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t// first thing a Task does is selecting environment as input type\n",
                    "start": [
                        45,
                        1
                    ],
                    "end": [
                        45,
                        78
                    ],
                    "_tokens": []
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t>server\n",
                    "start": [
                        46,
                        1
                    ],
                    "end": [
                        46,
                        20
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                46,
                                13
                            ],
                            "end": [
                                46,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "server",
                            "start": [
                                46,
                                14
                            ],
                            "end": [
                                46,
                                20
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t// then it selects contextually available Task mailer\n",
                    "start": [
                        47,
                        1
                    ],
                    "end": [
                        47,
                        66
                    ],
                    "_tokens": []
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t>mailer\n",
                    "start": [
                        48,
                        1
                    ],
                    "end": [
                        48,
                        20
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                48,
                                13
                            ],
                            "end": [
                                48,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "mailer",
                            "start": [
                                48,
                                14
                            ],
                            "end": [
                                48,
                                20
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t// it uses\n",
                    "start": [
                        49,
                        1
                    ],
                    "end": [
                        49,
                        23
                    ],
                    "_tokens": []
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t+familyMembersEmails\n",
                    "start": [
                        50,
                        1
                    ],
                    "end": [
                        50,
                        33
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": "+",
                            "start": [
                                50,
                                13
                            ],
                            "end": [
                                50,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "familyMembersEmails",
                            "start": [
                                50,
                                14
                            ],
                            "end": [
                                50,
                                33
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        51,
                        1
                    ],
                    "end": [
                        51,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                51,
                                13
                            ],
                            "end": [
                                51,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 4,
                    "_value": "\t\t\t\tPlease [participate in our survey](https://absqra.io/vx2zh32g5c)! Your dear {{respondentName}} already did!\n",
                    "start": [
                        52,
                        1
                    ],
                    "end": [
                        52,
                        124
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t`\n",
                    "start": [
                        53,
                        1
                    ],
                    "end": [
                        53,
                        14
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                53,
                                13
                            ],
                            "end": [
                                53,
                                14
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\telse // make them struggle in this else Block\n",
                    "start": [
                        54,
                        1
                    ],
                    "end": [
                        54,
                        50
                    ],
                    "_tokens": [
                        {
                            "tokenType": "kw",
                            "value": "else",
                            "start": [
                                54,
                                5
                            ],
                            "end": [
                                54,
                                9
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\tsequence\n",
                    "start": [
                        55,
                        1
                    ],
                    "end": [
                        55,
                        17
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "sequence",
                            "start": [
                                55,
                                9
                            ],
                            "end": [
                                55,
                                17
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t>spammer\n",
                    "start": [
                        56,
                        1
                    ],
                    "end": [
                        56,
                        21
                    ],
                    "_tokens": [
                        {
                            "tokenType": "op",
                            "value": ">",
                            "start": [
                                56,
                                13
                            ],
                            "end": [
                                56,
                                14
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "spammer",
                            "start": [
                                56,
                                14
                            ],
                            "end": [
                                56,
                                21
                            ],
                            "type": "Token"
                        }
                    ]
                }
            ],
            "type": "Block",
            "level": 0,
            "children": [
                {
                    "blockType": "invoke",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\tquestion respondentName // Block type Question with name respondentName for it's output\n",
                            "start": [
                                2,
                                1
                            ],
                            "end": [
                                2,
                                92
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "question",
                                    "start": [
                                        2,
                                        5
                                    ],
                                    "end": [
                                        2,
                                        13
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "respondentName",
                                    "start": [
                                        2,
                                        14
                                    ],
                                    "end": [
                                        2,
                                        28
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t>text // input type text\n",
                            "start": [
                                3,
                                1
                            ],
                            "end": [
                                3,
                                33
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        3,
                                        9
                                    ],
                                    "end": [
                                        3,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "text",
                                    "start": [
                                        3,
                                        10
                                    ],
                                    "end": [
                                        3,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t!required // Validator type required\n",
                            "start": [
                                4,
                                1
                            ],
                            "end": [
                                4,
                                45
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        4,
                                        9
                                    ],
                                    "end": [
                                        4,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "required",
                                    "start": [
                                        4,
                                        10
                                    ],
                                    "end": [
                                        4,
                                        18
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t!minLength=3 // Validator type minLength with parameter 3\n",
                            "start": [
                                5,
                                1
                            ],
                            "end": [
                                5,
                                66
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        5,
                                        9
                                    ],
                                    "end": [
                                        5,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "minLength",
                                    "start": [
                                        5,
                                        10
                                    ],
                                    "end": [
                                        5,
                                        19
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        5,
                                        19
                                    ],
                                    "end": [
                                        5,
                                        20
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "num",
                                    "value": 3,
                                    "start": [
                                        5,
                                        20
                                    ],
                                    "end": [
                                        5,
                                        21
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t// between pair of ` there's content code by default MD\n",
                            "start": [
                                6,
                                1
                            ],
                            "end": [
                                6,
                                64
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                7,
                                1
                            ],
                            "end": [
                                7,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        7,
                                        9
                                    ],
                                    "end": [
                                        7,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t#What is your name?\n",
                            "start": [
                                8,
                                1
                            ],
                            "end": [
                                8,
                                32
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                9,
                                1
                            ],
                            "end": [
                                9,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        9,
                                        9
                                    ],
                                    "end": [
                                        9,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1,
                    "children": [
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t>text // input type text\n",
                                    "start": [
                                        3,
                                        1
                                    ],
                                    "end": [
                                        3,
                                        33
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                3,
                                                9
                                            ],
                                            "end": [
                                                3,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "text",
                                            "start": [
                                                3,
                                                10
                                            ],
                                            "end": [
                                                3,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t!required // Validator type required\n",
                                    "start": [
                                        4,
                                        1
                                    ],
                                    "end": [
                                        4,
                                        45
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                4,
                                                9
                                            ],
                                            "end": [
                                                4,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "required",
                                            "start": [
                                                4,
                                                10
                                            ],
                                            "end": [
                                                4,
                                                18
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t!minLength=3 // Validator type minLength with parameter 3\n",
                                    "start": [
                                        5,
                                        1
                                    ],
                                    "end": [
                                        5,
                                        66
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                5,
                                                9
                                            ],
                                            "end": [
                                                5,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "minLength",
                                            "start": [
                                                5,
                                                10
                                            ],
                                            "end": [
                                                5,
                                                19
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                5,
                                                19
                                            ],
                                            "end": [
                                                5,
                                                20
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "num",
                                            "value": 3,
                                            "start": [
                                                5,
                                                20
                                            ],
                                            "end": [
                                                5,
                                                21
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "content",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        7,
                                        1
                                    ],
                                    "end": [
                                        7,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                7,
                                                9
                                            ],
                                            "end": [
                                                7,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t#What is your name?\n",
                                    "start": [
                                        8,
                                        1
                                    ],
                                    "end": [
                                        8,
                                        32
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        9,
                                        1
                                    ],
                                    "end": [
                                        9,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                9,
                                                9
                                            ],
                                            "end": [
                                                9,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2
                        }
                    ]
                },
                {
                    "blockType": "invoke",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\tquestion respondentAge\n",
                            "start": [
                                10,
                                1
                            ],
                            "end": [
                                10,
                                27
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "question",
                                    "start": [
                                        10,
                                        5
                                    ],
                                    "end": [
                                        10,
                                        13
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "respondentAge",
                                    "start": [
                                        10,
                                        14
                                    ],
                                    "end": [
                                        10,
                                        27
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t>number\n",
                            "start": [
                                11,
                                1
                            ],
                            "end": [
                                11,
                                16
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        11,
                                        9
                                    ],
                                    "end": [
                                        11,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "number",
                                    "start": [
                                        11,
                                        10
                                    ],
                                    "end": [
                                        11,
                                        16
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t!required\n",
                            "start": [
                                12,
                                1
                            ],
                            "end": [
                                12,
                                18
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        12,
                                        9
                                    ],
                                    "end": [
                                        12,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "required",
                                    "start": [
                                        12,
                                        10
                                    ],
                                    "end": [
                                        12,
                                        18
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t!minValue=18\n",
                            "start": [
                                13,
                                1
                            ],
                            "end": [
                                13,
                                21
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        13,
                                        9
                                    ],
                                    "end": [
                                        13,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "minValue",
                                    "start": [
                                        13,
                                        10
                                    ],
                                    "end": [
                                        13,
                                        18
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        13,
                                        18
                                    ],
                                    "end": [
                                        13,
                                        19
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "num",
                                    "value": 18,
                                    "start": [
                                        13,
                                        19
                                    ],
                                    "end": [
                                        13,
                                        21
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                14,
                                1
                            ],
                            "end": [
                                14,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        14,
                                        9
                                    ],
                                    "end": [
                                        14,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t#What is your age?\n",
                            "start": [
                                15,
                                1
                            ],
                            "end": [
                                15,
                                31
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                16,
                                1
                            ],
                            "end": [
                                16,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        16,
                                        9
                                    ],
                                    "end": [
                                        16,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1,
                    "children": [
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t>number\n",
                                    "start": [
                                        11,
                                        1
                                    ],
                                    "end": [
                                        11,
                                        16
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                11,
                                                9
                                            ],
                                            "end": [
                                                11,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "number",
                                            "start": [
                                                11,
                                                10
                                            ],
                                            "end": [
                                                11,
                                                16
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t!required\n",
                                    "start": [
                                        12,
                                        1
                                    ],
                                    "end": [
                                        12,
                                        18
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                12,
                                                9
                                            ],
                                            "end": [
                                                12,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "required",
                                            "start": [
                                                12,
                                                10
                                            ],
                                            "end": [
                                                12,
                                                18
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t!minValue=18\n",
                                    "start": [
                                        13,
                                        1
                                    ],
                                    "end": [
                                        13,
                                        21
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                13,
                                                9
                                            ],
                                            "end": [
                                                13,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "minValue",
                                            "start": [
                                                13,
                                                10
                                            ],
                                            "end": [
                                                13,
                                                18
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                13,
                                                18
                                            ],
                                            "end": [
                                                13,
                                                19
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "num",
                                            "value": 18,
                                            "start": [
                                                13,
                                                19
                                            ],
                                            "end": [
                                                13,
                                                21
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "content",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        14,
                                        1
                                    ],
                                    "end": [
                                        14,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                14,
                                                9
                                            ],
                                            "end": [
                                                14,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t#What is your age?\n",
                                    "start": [
                                        15,
                                        1
                                    ],
                                    "end": [
                                        15,
                                        31
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        16,
                                        1
                                    ],
                                    "end": [
                                        16,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                16,
                                                9
                                            ],
                                            "end": [
                                                16,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2
                        }
                    ]
                },
                {
                    "blockType": "invoke",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\tquestion respondentFamilyMembers\n",
                            "start": [
                                17,
                                1
                            ],
                            "end": [
                                17,
                                37
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "question",
                                    "start": [
                                        17,
                                        5
                                    ],
                                    "end": [
                                        17,
                                        13
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "respondentFamilyMembers",
                                    "start": [
                                        17,
                                        14
                                    ],
                                    "end": [
                                        17,
                                        37
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t>list\n",
                            "start": [
                                18,
                                1
                            ],
                            "end": [
                                18,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        18,
                                        9
                                    ],
                                    "end": [
                                        18,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "list",
                                    "start": [
                                        18,
                                        10
                                    ],
                                    "end": [
                                        18,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t!minCount=2\n",
                            "start": [
                                19,
                                1
                            ],
                            "end": [
                                19,
                                20
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        19,
                                        9
                                    ],
                                    "end": [
                                        19,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "minCount",
                                    "start": [
                                        19,
                                        10
                                    ],
                                    "end": [
                                        19,
                                        18
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        19,
                                        18
                                    ],
                                    "end": [
                                        19,
                                        19
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "num",
                                    "value": 2,
                                    "start": [
                                        19,
                                        19
                                    ],
                                    "end": [
                                        19,
                                        20
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                20,
                                1
                            ],
                            "end": [
                                20,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        20,
                                        9
                                    ],
                                    "end": [
                                        20,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t#Please add names of your family members.\n",
                            "start": [
                                21,
                                1
                            ],
                            "end": [
                                21,
                                54
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t`\n",
                            "start": [
                                22,
                                1
                            ],
                            "end": [
                                22,
                                10
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        22,
                                        9
                                    ],
                                    "end": [
                                        22,
                                        10
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1,
                    "children": [
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t>list\n",
                                    "start": [
                                        18,
                                        1
                                    ],
                                    "end": [
                                        18,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                18,
                                                9
                                            ],
                                            "end": [
                                                18,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "list",
                                            "start": [
                                                18,
                                                10
                                            ],
                                            "end": [
                                                18,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t!minCount=2\n",
                                    "start": [
                                        19,
                                        1
                                    ],
                                    "end": [
                                        19,
                                        20
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                19,
                                                9
                                            ],
                                            "end": [
                                                19,
                                                10
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "minCount",
                                            "start": [
                                                19,
                                                10
                                            ],
                                            "end": [
                                                19,
                                                18
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                19,
                                                18
                                            ],
                                            "end": [
                                                19,
                                                19
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "num",
                                            "value": 2,
                                            "start": [
                                                19,
                                                19
                                            ],
                                            "end": [
                                                19,
                                                20
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": null
                        },
                        {
                            "blockType": "content",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        20,
                                        1
                                    ],
                                    "end": [
                                        20,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                20,
                                                9
                                            ],
                                            "end": [
                                                20,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t#Please add names of your family members.\n",
                                    "start": [
                                        21,
                                        1
                                    ],
                                    "end": [
                                        21,
                                        54
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 2,
                                    "_value": "\t\t`\n",
                                    "start": [
                                        22,
                                        1
                                    ],
                                    "end": [
                                        22,
                                        10
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                22,
                                                9
                                            ],
                                            "end": [
                                                22,
                                                10
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2
                        }
                    ]
                },
                {
                    "blockType": "invoke",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\tif respondentFamilyMembers // this If Block checks that respondentFamilyMembers is valid and has value\n",
                            "start": [
                                24,
                                1
                            ],
                            "end": [
                                24,
                                107
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "kw",
                                    "value": "if",
                                    "start": [
                                        24,
                                        5
                                    ],
                                    "end": [
                                        24,
                                        7
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "respondentFamilyMembers",
                                    "start": [
                                        24,
                                        8
                                    ],
                                    "end": [
                                        24,
                                        31
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\tquestion familyMembersEmails\n",
                            "start": [
                                25,
                                1
                            ],
                            "end": [
                                25,
                                37
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "question",
                                    "start": [
                                        25,
                                        9
                                    ],
                                    "end": [
                                        25,
                                        17
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "familyMembersEmails",
                                    "start": [
                                        25,
                                        18
                                    ],
                                    "end": [
                                        25,
                                        37
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t>multiple\n",
                            "start": [
                                26,
                                1
                            ],
                            "end": [
                                26,
                                22
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        26,
                                        13
                                    ],
                                    "end": [
                                        26,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "multiple",
                                    "start": [
                                        26,
                                        14
                                    ],
                                    "end": [
                                        26,
                                        22
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t>assignment\n",
                            "start": [
                                27,
                                1
                            ],
                            "end": [
                                27,
                                24
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        27,
                                        13
                                    ],
                                    "end": [
                                        27,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "assignment",
                                    "start": [
                                        27,
                                        14
                                    ],
                                    "end": [
                                        27,
                                        24
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t!format=email\n",
                            "start": [
                                28,
                                1
                            ],
                            "end": [
                                28,
                                26
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "!",
                                    "start": [
                                        28,
                                        13
                                    ],
                                    "end": [
                                        28,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "format",
                                    "start": [
                                        28,
                                        14
                                    ],
                                    "end": [
                                        28,
                                        20
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        28,
                                        20
                                    ],
                                    "end": [
                                        28,
                                        21
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "email",
                                    "start": [
                                        28,
                                        21
                                    ],
                                    "end": [
                                        28,
                                        26
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t+respondentFamilyMembers // + sign is for options\n",
                            "start": [
                                29,
                                1
                            ],
                            "end": [
                                29,
                                62
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "+",
                                    "start": [
                                        29,
                                        13
                                    ],
                                    "end": [
                                        29,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "respondentFamilyMembers",
                                    "start": [
                                        29,
                                        14
                                    ],
                                    "end": [
                                        29,
                                        37
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`ln=En\n",
                            "start": [
                                30,
                                1
                            ],
                            "end": [
                                30,
                                19
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        30,
                                        13
                                    ],
                                    "end": [
                                        30,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "kw",
                                    "value": "ln",
                                    "start": [
                                        30,
                                        14
                                    ],
                                    "end": [
                                        30,
                                        16
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        30,
                                        16
                                    ],
                                    "end": [
                                        30,
                                        17
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "En",
                                    "start": [
                                        30,
                                        17
                                    ],
                                    "end": [
                                        30,
                                        19
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 4,
                            "_value": "\t\t\t\t#Please add emails of your family members.\n",
                            "start": [
                                31,
                                1
                            ],
                            "end": [
                                31,
                                59
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                32,
                                1
                            ],
                            "end": [
                                32,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        32,
                                        13
                                    ],
                                    "end": [
                                        32,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`ln=Ru\n",
                            "start": [
                                33,
                                1
                            ],
                            "end": [
                                33,
                                19
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        33,
                                        13
                                    ],
                                    "end": [
                                        33,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "kw",
                                    "value": "ln",
                                    "start": [
                                        33,
                                        14
                                    ],
                                    "end": [
                                        33,
                                        16
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        33,
                                        16
                                    ],
                                    "end": [
                                        33,
                                        17
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "Ru",
                                    "start": [
                                        33,
                                        17
                                    ],
                                    "end": [
                                        33,
                                        19
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 4,
                            "_value": "\t\t\t\t#Пожалуйста укажите адреса электропочты членов вашей семьи.\n",
                            "start": [
                                34,
                                1
                            ],
                            "end": [
                                34,
                                76
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                35,
                                1
                            ],
                            "end": [
                                35,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        35,
                                        13
                                    ],
                                    "end": [
                                        35,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t// this Content Block doesn't have a name it is then simply there in Sequence, not anywhere else\n",
                            "start": [
                                37,
                                1
                            ],
                            "end": [
                                37,
                                105
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 2,
                            "_value": "\t\tcontent /* try comment */\n",
                            "start": [
                                38,
                                1
                            ],
                            "end": [
                                38,
                                34
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "content",
                                    "start": [
                                        38,
                                        9
                                    ],
                                    "end": [
                                        38,
                                        16
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                39,
                                1
                            ],
                            "end": [
                                39,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        39,
                                        13
                                    ],
                                    "end": [
                                        39,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 4,
                            "_value": "\t\t\t\t###Thank you so much!\n",
                            "start": [
                                40,
                                1
                            ],
                            "end": [
                                40,
                                38
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 4,
                            "_value": "\t\t\t\t##Please [enjoy this video](http://youtube.com/hdjkfhskfhwir)\n",
                            "start": [
                                41,
                                1
                            ],
                            "end": [
                                41,
                                78
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                42,
                                1
                            ],
                            "end": [
                                42,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        42,
                                        13
                                    ],
                                    "end": [
                                        42,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "task ",
                            "start": [
                                43,
                                1
                            ],
                            "end": [
                                45,
                                78
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "task",
                                    "start": [
                                        43,
                                        1
                                    ],
                                    "end": [
                                        43,
                                        5
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t// first thing a Task does is selecting environment as input type\n",
                            "start": [
                                45,
                                1
                            ],
                            "end": [
                                45,
                                78
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t>server\n",
                            "start": [
                                46,
                                1
                            ],
                            "end": [
                                46,
                                20
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        46,
                                        13
                                    ],
                                    "end": [
                                        46,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "server",
                                    "start": [
                                        46,
                                        14
                                    ],
                                    "end": [
                                        46,
                                        20
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t// then it selects contextually available Task mailer\n",
                            "start": [
                                47,
                                1
                            ],
                            "end": [
                                47,
                                66
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t>mailer\n",
                            "start": [
                                48,
                                1
                            ],
                            "end": [
                                48,
                                20
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        48,
                                        13
                                    ],
                                    "end": [
                                        48,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "mailer",
                                    "start": [
                                        48,
                                        14
                                    ],
                                    "end": [
                                        48,
                                        20
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t// it uses\n",
                            "start": [
                                49,
                                1
                            ],
                            "end": [
                                49,
                                23
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t+familyMembersEmails\n",
                            "start": [
                                50,
                                1
                            ],
                            "end": [
                                50,
                                33
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": "+",
                                    "start": [
                                        50,
                                        13
                                    ],
                                    "end": [
                                        50,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "familyMembersEmails",
                                    "start": [
                                        50,
                                        14
                                    ],
                                    "end": [
                                        50,
                                        33
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                51,
                                1
                            ],
                            "end": [
                                51,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        51,
                                        13
                                    ],
                                    "end": [
                                        51,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 4,
                            "_value": "\t\t\t\tPlease [participate in our survey](https://absqra.io/vx2zh32g5c)! Your dear {{respondentName}} already did!\n",
                            "start": [
                                52,
                                1
                            ],
                            "end": [
                                52,
                                124
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t`\n",
                            "start": [
                                53,
                                1
                            ],
                            "end": [
                                53,
                                14
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        53,
                                        13
                                    ],
                                    "end": [
                                        53,
                                        14
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1,
                    "children": [
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\tquestion familyMembersEmails\n",
                                    "start": [
                                        25,
                                        1
                                    ],
                                    "end": [
                                        25,
                                        37
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "var",
                                            "value": "question",
                                            "start": [
                                                25,
                                                9
                                            ],
                                            "end": [
                                                25,
                                                17
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "familyMembersEmails",
                                            "start": [
                                                25,
                                                18
                                            ],
                                            "end": [
                                                25,
                                                37
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t>multiple\n",
                                    "start": [
                                        26,
                                        1
                                    ],
                                    "end": [
                                        26,
                                        22
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                26,
                                                13
                                            ],
                                            "end": [
                                                26,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "multiple",
                                            "start": [
                                                26,
                                                14
                                            ],
                                            "end": [
                                                26,
                                                22
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t>assignment\n",
                                    "start": [
                                        27,
                                        1
                                    ],
                                    "end": [
                                        27,
                                        24
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                27,
                                                13
                                            ],
                                            "end": [
                                                27,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "assignment",
                                            "start": [
                                                27,
                                                14
                                            ],
                                            "end": [
                                                27,
                                                24
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t!format=email\n",
                                    "start": [
                                        28,
                                        1
                                    ],
                                    "end": [
                                        28,
                                        26
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "!",
                                            "start": [
                                                28,
                                                13
                                            ],
                                            "end": [
                                                28,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "format",
                                            "start": [
                                                28,
                                                14
                                            ],
                                            "end": [
                                                28,
                                                20
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                28,
                                                20
                                            ],
                                            "end": [
                                                28,
                                                21
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "email",
                                            "start": [
                                                28,
                                                21
                                            ],
                                            "end": [
                                                28,
                                                26
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t+respondentFamilyMembers // + sign is for options\n",
                                    "start": [
                                        29,
                                        1
                                    ],
                                    "end": [
                                        29,
                                        62
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "+",
                                            "start": [
                                                29,
                                                13
                                            ],
                                            "end": [
                                                29,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "respondentFamilyMembers",
                                            "start": [
                                                29,
                                                14
                                            ],
                                            "end": [
                                                29,
                                                37
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`ln=En\n",
                                    "start": [
                                        30,
                                        1
                                    ],
                                    "end": [
                                        30,
                                        19
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                30,
                                                13
                                            ],
                                            "end": [
                                                30,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "kw",
                                            "value": "ln",
                                            "start": [
                                                30,
                                                14
                                            ],
                                            "end": [
                                                30,
                                                16
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                30,
                                                16
                                            ],
                                            "end": [
                                                30,
                                                17
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "En",
                                            "start": [
                                                30,
                                                17
                                            ],
                                            "end": [
                                                30,
                                                19
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 4,
                                    "_value": "\t\t\t\t#Please add emails of your family members.\n",
                                    "start": [
                                        31,
                                        1
                                    ],
                                    "end": [
                                        31,
                                        59
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        32,
                                        1
                                    ],
                                    "end": [
                                        32,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                32,
                                                13
                                            ],
                                            "end": [
                                                32,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`ln=Ru\n",
                                    "start": [
                                        33,
                                        1
                                    ],
                                    "end": [
                                        33,
                                        19
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                33,
                                                13
                                            ],
                                            "end": [
                                                33,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "kw",
                                            "value": "ln",
                                            "start": [
                                                33,
                                                14
                                            ],
                                            "end": [
                                                33,
                                                16
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "op",
                                            "value": "=",
                                            "start": [
                                                33,
                                                16
                                            ],
                                            "end": [
                                                33,
                                                17
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "Ru",
                                            "start": [
                                                33,
                                                17
                                            ],
                                            "end": [
                                                33,
                                                19
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 4,
                                    "_value": "\t\t\t\t#Пожалуйста укажите адреса электропочты членов вашей семьи.\n",
                                    "start": [
                                        34,
                                        1
                                    ],
                                    "end": [
                                        34,
                                        76
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        35,
                                        1
                                    ],
                                    "end": [
                                        35,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                35,
                                                13
                                            ],
                                            "end": [
                                                35,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": [
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t>multiple\n",
                                            "start": [
                                                26,
                                                1
                                            ],
                                            "end": [
                                                26,
                                                22
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": ">",
                                                    "start": [
                                                        26,
                                                        13
                                                    ],
                                                    "end": [
                                                        26,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "multiple",
                                                    "start": [
                                                        26,
                                                        14
                                                    ],
                                                    "end": [
                                                        26,
                                                        22
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t>assignment\n",
                                            "start": [
                                                27,
                                                1
                                            ],
                                            "end": [
                                                27,
                                                24
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": ">",
                                                    "start": [
                                                        27,
                                                        13
                                                    ],
                                                    "end": [
                                                        27,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "assignment",
                                                    "start": [
                                                        27,
                                                        14
                                                    ],
                                                    "end": [
                                                        27,
                                                        24
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t!format=email\n",
                                            "start": [
                                                28,
                                                1
                                            ],
                                            "end": [
                                                28,
                                                26
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": "!",
                                                    "start": [
                                                        28,
                                                        13
                                                    ],
                                                    "end": [
                                                        28,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "format",
                                                    "start": [
                                                        28,
                                                        14
                                                    ],
                                                    "end": [
                                                        28,
                                                        20
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "op",
                                                    "value": "=",
                                                    "start": [
                                                        28,
                                                        20
                                                    ],
                                                    "end": [
                                                        28,
                                                        21
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "email",
                                                    "start": [
                                                        28,
                                                        21
                                                    ],
                                                    "end": [
                                                        28,
                                                        26
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t+respondentFamilyMembers // + sign is for options\n",
                                            "start": [
                                                29,
                                                1
                                            ],
                                            "end": [
                                                29,
                                                62
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": "+",
                                                    "start": [
                                                        29,
                                                        13
                                                    ],
                                                    "end": [
                                                        29,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "respondentFamilyMembers",
                                                    "start": [
                                                        29,
                                                        14
                                                    ],
                                                    "end": [
                                                        29,
                                                        37
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "content",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`ln=En\n",
                                            "start": [
                                                30,
                                                1
                                            ],
                                            "end": [
                                                30,
                                                19
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        30,
                                                        13
                                                    ],
                                                    "end": [
                                                        30,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "kw",
                                                    "value": "ln",
                                                    "start": [
                                                        30,
                                                        14
                                                    ],
                                                    "end": [
                                                        30,
                                                        16
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "op",
                                                    "value": "=",
                                                    "start": [
                                                        30,
                                                        16
                                                    ],
                                                    "end": [
                                                        30,
                                                        17
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "En",
                                                    "start": [
                                                        30,
                                                        17
                                                    ],
                                                    "end": [
                                                        30,
                                                        19
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        },
                                        {
                                            "indent": 4,
                                            "_value": "\t\t\t\t#Please add emails of your family members.\n",
                                            "start": [
                                                31,
                                                1
                                            ],
                                            "end": [
                                                31,
                                                59
                                            ],
                                            "_tokens": null
                                        },
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                32,
                                                1
                                            ],
                                            "end": [
                                                32,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        32,
                                                        13
                                                    ],
                                                    "end": [
                                                        32,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3
                                },
                                {
                                    "blockType": "content",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`ln=Ru\n",
                                            "start": [
                                                33,
                                                1
                                            ],
                                            "end": [
                                                33,
                                                19
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        33,
                                                        13
                                                    ],
                                                    "end": [
                                                        33,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "kw",
                                                    "value": "ln",
                                                    "start": [
                                                        33,
                                                        14
                                                    ],
                                                    "end": [
                                                        33,
                                                        16
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "op",
                                                    "value": "=",
                                                    "start": [
                                                        33,
                                                        16
                                                    ],
                                                    "end": [
                                                        33,
                                                        17
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "Ru",
                                                    "start": [
                                                        33,
                                                        17
                                                    ],
                                                    "end": [
                                                        33,
                                                        19
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        },
                                        {
                                            "indent": 4,
                                            "_value": "\t\t\t\t#Пожалуйста укажите адреса электропочты членов вашей семьи.\n",
                                            "start": [
                                                34,
                                                1
                                            ],
                                            "end": [
                                                34,
                                                76
                                            ],
                                            "_tokens": null
                                        },
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                35,
                                                1
                                            ],
                                            "end": [
                                                35,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        35,
                                                        13
                                                    ],
                                                    "end": [
                                                        35,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3
                                }
                            ]
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\tcontent /* try comment */\n",
                                    "start": [
                                        38,
                                        1
                                    ],
                                    "end": [
                                        38,
                                        34
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "var",
                                            "value": "content",
                                            "start": [
                                                38,
                                                9
                                            ],
                                            "end": [
                                                38,
                                                16
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        39,
                                        1
                                    ],
                                    "end": [
                                        39,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                39,
                                                13
                                            ],
                                            "end": [
                                                39,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 4,
                                    "_value": "\t\t\t\t###Thank you so much!\n",
                                    "start": [
                                        40,
                                        1
                                    ],
                                    "end": [
                                        40,
                                        38
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 4,
                                    "_value": "\t\t\t\t##Please [enjoy this video](http://youtube.com/hdjkfhskfhwir)\n",
                                    "start": [
                                        41,
                                        1
                                    ],
                                    "end": [
                                        41,
                                        78
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        42,
                                        1
                                    ],
                                    "end": [
                                        42,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                42,
                                                13
                                            ],
                                            "end": [
                                                42,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": [
                                {
                                    "blockType": "content",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                39,
                                                1
                                            ],
                                            "end": [
                                                39,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        39,
                                                        13
                                                    ],
                                                    "end": [
                                                        39,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        },
                                        {
                                            "indent": 4,
                                            "_value": "\t\t\t\t###Thank you so much!\n",
                                            "start": [
                                                40,
                                                1
                                            ],
                                            "end": [
                                                40,
                                                38
                                            ],
                                            "_tokens": null
                                        },
                                        {
                                            "indent": 4,
                                            "_value": "\t\t\t\t##Please [enjoy this video](http://youtube.com/hdjkfhskfhwir)\n",
                                            "start": [
                                                41,
                                                1
                                            ],
                                            "end": [
                                                41,
                                                78
                                            ],
                                            "_tokens": null
                                        },
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                42,
                                                1
                                            ],
                                            "end": [
                                                42,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        42,
                                                        13
                                                    ],
                                                    "end": [
                                                        42,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3
                                }
                            ]
                        },
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "task ",
                                    "start": [
                                        43,
                                        1
                                    ],
                                    "end": [
                                        45,
                                        78
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "var",
                                            "value": "task",
                                            "start": [
                                                43,
                                                1
                                            ],
                                            "end": [
                                                43,
                                                5
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t>server\n",
                                    "start": [
                                        46,
                                        1
                                    ],
                                    "end": [
                                        46,
                                        20
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                46,
                                                13
                                            ],
                                            "end": [
                                                46,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "server",
                                            "start": [
                                                46,
                                                14
                                            ],
                                            "end": [
                                                46,
                                                20
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t// then it selects contextually available Task mailer\n",
                                    "start": [
                                        47,
                                        1
                                    ],
                                    "end": [
                                        47,
                                        66
                                    ],
                                    "_tokens": []
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t>mailer\n",
                                    "start": [
                                        48,
                                        1
                                    ],
                                    "end": [
                                        48,
                                        20
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                48,
                                                13
                                            ],
                                            "end": [
                                                48,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "mailer",
                                            "start": [
                                                48,
                                                14
                                            ],
                                            "end": [
                                                48,
                                                20
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t// it uses\n",
                                    "start": [
                                        49,
                                        1
                                    ],
                                    "end": [
                                        49,
                                        23
                                    ],
                                    "_tokens": []
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t+familyMembersEmails\n",
                                    "start": [
                                        50,
                                        1
                                    ],
                                    "end": [
                                        50,
                                        33
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": "+",
                                            "start": [
                                                50,
                                                13
                                            ],
                                            "end": [
                                                50,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "familyMembersEmails",
                                            "start": [
                                                50,
                                                14
                                            ],
                                            "end": [
                                                50,
                                                33
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        51,
                                        1
                                    ],
                                    "end": [
                                        51,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                51,
                                                13
                                            ],
                                            "end": [
                                                51,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 4,
                                    "_value": "\t\t\t\tPlease [participate in our survey](https://absqra.io/vx2zh32g5c)! Your dear {{respondentName}} already did!\n",
                                    "start": [
                                        52,
                                        1
                                    ],
                                    "end": [
                                        52,
                                        124
                                    ],
                                    "_tokens": null
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t`\n",
                                    "start": [
                                        53,
                                        1
                                    ],
                                    "end": [
                                        53,
                                        14
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "punct",
                                            "value": "`",
                                            "start": [
                                                53,
                                                13
                                            ],
                                            "end": [
                                                53,
                                                14
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": [
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t>server\n",
                                            "start": [
                                                46,
                                                1
                                            ],
                                            "end": [
                                                46,
                                                20
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": ">",
                                                    "start": [
                                                        46,
                                                        13
                                                    ],
                                                    "end": [
                                                        46,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "server",
                                                    "start": [
                                                        46,
                                                        14
                                                    ],
                                                    "end": [
                                                        46,
                                                        20
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t>mailer\n",
                                            "start": [
                                                48,
                                                1
                                            ],
                                            "end": [
                                                48,
                                                20
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": ">",
                                                    "start": [
                                                        48,
                                                        13
                                                    ],
                                                    "end": [
                                                        48,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "mailer",
                                                    "start": [
                                                        48,
                                                        14
                                                    ],
                                                    "end": [
                                                        48,
                                                        20
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t+familyMembersEmails\n",
                                            "start": [
                                                50,
                                                1
                                            ],
                                            "end": [
                                                50,
                                                33
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": "+",
                                                    "start": [
                                                        50,
                                                        13
                                                    ],
                                                    "end": [
                                                        50,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "familyMembersEmails",
                                                    "start": [
                                                        50,
                                                        14
                                                    ],
                                                    "end": [
                                                        50,
                                                        33
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                },
                                {
                                    "blockType": "content",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                51,
                                                1
                                            ],
                                            "end": [
                                                51,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        51,
                                                        13
                                                    ],
                                                    "end": [
                                                        51,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        },
                                        {
                                            "indent": 4,
                                            "_value": "\t\t\t\tPlease [participate in our survey](https://absqra.io/vx2zh32g5c)! Your dear {{respondentName}} already did!\n",
                                            "start": [
                                                52,
                                                1
                                            ],
                                            "end": [
                                                52,
                                                124
                                            ],
                                            "_tokens": null
                                        },
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t`\n",
                                            "start": [
                                                53,
                                                1
                                            ],
                                            "end": [
                                                53,
                                                14
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "punct",
                                                    "value": "`",
                                                    "start": [
                                                        53,
                                                        13
                                                    ],
                                                    "end": [
                                                        53,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3
                                }
                            ]
                        }
                    ]
                },
                {
                    "blockType": "invoke",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\telse // make them struggle in this else Block\n",
                            "start": [
                                54,
                                1
                            ],
                            "end": [
                                54,
                                50
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "kw",
                                    "value": "else",
                                    "start": [
                                        54,
                                        5
                                    ],
                                    "end": [
                                        54,
                                        9
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\tsequence\n",
                            "start": [
                                55,
                                1
                            ],
                            "end": [
                                55,
                                17
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "var",
                                    "value": "sequence",
                                    "start": [
                                        55,
                                        9
                                    ],
                                    "end": [
                                        55,
                                        17
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t>spammer\n",
                            "start": [
                                56,
                                1
                            ],
                            "end": [
                                56,
                                21
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "op",
                                    "value": ">",
                                    "start": [
                                        56,
                                        13
                                    ],
                                    "end": [
                                        56,
                                        14
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "spammer",
                                    "start": [
                                        56,
                                        14
                                    ],
                                    "end": [
                                        56,
                                        21
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1,
                    "children": [
                        {
                            "blockType": "invoke",
                            "content": [
                                {
                                    "indent": 2,
                                    "_value": "\t\tsequence\n",
                                    "start": [
                                        55,
                                        1
                                    ],
                                    "end": [
                                        55,
                                        17
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "var",
                                            "value": "sequence",
                                            "start": [
                                                55,
                                                9
                                            ],
                                            "end": [
                                                55,
                                                17
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                },
                                {
                                    "indent": 3,
                                    "_value": "\t\t\t>spammer\n",
                                    "start": [
                                        56,
                                        1
                                    ],
                                    "end": [
                                        56,
                                        21
                                    ],
                                    "_tokens": [
                                        {
                                            "tokenType": "op",
                                            "value": ">",
                                            "start": [
                                                56,
                                                13
                                            ],
                                            "end": [
                                                56,
                                                14
                                            ],
                                            "type": "Token"
                                        },
                                        {
                                            "tokenType": "var",
                                            "value": "spammer",
                                            "start": [
                                                56,
                                                14
                                            ],
                                            "end": [
                                                56,
                                                21
                                            ],
                                            "type": "Token"
                                        }
                                    ]
                                }
                            ],
                            "type": "Block",
                            "level": 2,
                            "children": [
                                {
                                    "blockType": "invoke",
                                    "content": [
                                        {
                                            "indent": 3,
                                            "_value": "\t\t\t>spammer\n",
                                            "start": [
                                                56,
                                                1
                                            ],
                                            "end": [
                                                56,
                                                21
                                            ],
                                            "_tokens": [
                                                {
                                                    "tokenType": "op",
                                                    "value": ">",
                                                    "start": [
                                                        56,
                                                        13
                                                    ],
                                                    "end": [
                                                        56,
                                                        14
                                                    ],
                                                    "type": "Token"
                                                },
                                                {
                                                    "tokenType": "var",
                                                    "value": "spammer",
                                                    "start": [
                                                        56,
                                                        14
                                                    ],
                                                    "end": [
                                                        56,
                                                        21
                                                    ],
                                                    "type": "Token"
                                                }
                                            ]
                                        }
                                    ],
                                    "type": "Block",
                                    "level": 3,
                                    "children": null
                                }
                            ]
                        }
                    ]
                }
            ]
        },
        {
            "blockType": "declare",
            "content": [
                {
                    "indent": 0,
                    "_value": "mailer: Task (body: Content, recipients: Options=1)\n",
                    "start": [
                        58,
                        1
                    ],
                    "end": [
                        58,
                        52
                    ],
                    "_tokens": [
                        {
                            "tokenType": "var",
                            "value": "mailer",
                            "start": [
                                58,
                                1
                            ],
                            "end": [
                                58,
                                7
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": ":",
                            "start": [
                                58,
                                7
                            ],
                            "end": [
                                58,
                                8
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "Task",
                            "start": [
                                58,
                                9
                            ],
                            "end": [
                                58,
                                13
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "punct",
                            "value": "(",
                            "start": [
                                58,
                                14
                            ],
                            "end": [
                                58,
                                15
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "body",
                            "start": [
                                58,
                                15
                            ],
                            "end": [
                                58,
                                19
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": ":",
                            "start": [
                                58,
                                19
                            ],
                            "end": [
                                58,
                                20
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "Content",
                            "start": [
                                58,
                                21
                            ],
                            "end": [
                                58,
                                28
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "punct",
                            "value": ",",
                            "start": [
                                58,
                                28
                            ],
                            "end": [
                                58,
                                29
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "recipients",
                            "start": [
                                58,
                                30
                            ],
                            "end": [
                                58,
                                40
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": ":",
                            "start": [
                                58,
                                40
                            ],
                            "end": [
                                58,
                                41
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "Options",
                            "start": [
                                58,
                                42
                            ],
                            "end": [
                                58,
                                49
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                58,
                                49
                            ],
                            "end": [
                                58,
                                50
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "num",
                            "value": 1,
                            "start": [
                                58,
                                50
                            ],
                            "end": [
                                58,
                                51
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "punct",
                            "value": ")",
                            "start": [
                                58,
                                51
                            ],
                            "end": [
                                58,
                                52
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 1,
                    "_value": "\t// code here is in typescript\n",
                    "start": [
                        59,
                        1
                    ],
                    "end": [
                        59,
                        34
                    ],
                    "_tokens": []
                },
                {
                    "indent": 1,
                    "_value": "\t`lang=typescript\n",
                    "start": [
                        60,
                        1
                    ],
                    "end": [
                        60,
                        21
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                60,
                                5
                            ],
                            "end": [
                                60,
                                6
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "kw",
                            "value": "lang",
                            "start": [
                                60,
                                6
                            ],
                            "end": [
                                60,
                                10
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "op",
                            "value": "=",
                            "start": [
                                60,
                                10
                            ],
                            "end": [
                                60,
                                11
                            ],
                            "type": "Token"
                        },
                        {
                            "tokenType": "var",
                            "value": "typescript",
                            "start": [
                                60,
                                11
                            ],
                            "end": [
                                60,
                                21
                            ],
                            "type": "Token"
                        }
                    ]
                },
                {
                    "indent": 2,
                    "_value": "\t\t// some code sending mail here...\n",
                    "start": [
                        61,
                        1
                    ],
                    "end": [
                        61,
                        42
                    ],
                    "_tokens": []
                },
                {
                    "indent": 2,
                    "_value": "\t\tfunction(content: string) {\n",
                    "start": [
                        62,
                        1
                    ],
                    "end": [
                        62,
                        36
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\tconsole.log(content)\n",
                    "start": [
                        63,
                        1
                    ],
                    "end": [
                        63,
                        33
                    ],
                    "_tokens": null
                },
                {
                    "indent": 3,
                    "_value": "\t\t\t// yeah that's how we send mail\n",
                    "start": [
                        64,
                        1
                    ],
                    "end": [
                        64,
                        44
                    ],
                    "_tokens": []
                },
                {
                    "indent": 2,
                    "_value": "\t\t}\n",
                    "start": [
                        65,
                        1
                    ],
                    "end": [
                        65,
                        10
                    ],
                    "_tokens": null
                },
                {
                    "indent": 1,
                    "_value": "\t`",
                    "start": [
                        66,
                        1
                    ],
                    "end": [
                        66,
                        6
                    ],
                    "_tokens": [
                        {
                            "tokenType": "punct",
                            "value": "`",
                            "start": [
                                66,
                                5
                            ],
                            "end": [
                                66,
                                6
                            ],
                            "type": "Token"
                        }
                    ]
                }
            ],
            "type": "Block",
            "level": 0,
            "children": [
                {
                    "blockType": "content",
                    "content": [
                        {
                            "indent": 1,
                            "_value": "\t`lang=typescript\n",
                            "start": [
                                60,
                                1
                            ],
                            "end": [
                                60,
                                21
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        60,
                                        5
                                    ],
                                    "end": [
                                        60,
                                        6
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "kw",
                                    "value": "lang",
                                    "start": [
                                        60,
                                        6
                                    ],
                                    "end": [
                                        60,
                                        10
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "op",
                                    "value": "=",
                                    "start": [
                                        60,
                                        10
                                    ],
                                    "end": [
                                        60,
                                        11
                                    ],
                                    "type": "Token"
                                },
                                {
                                    "tokenType": "var",
                                    "value": "typescript",
                                    "start": [
                                        60,
                                        11
                                    ],
                                    "end": [
                                        60,
                                        21
                                    ],
                                    "type": "Token"
                                }
                            ]
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t// some code sending mail here...\n",
                            "start": [
                                61,
                                1
                            ],
                            "end": [
                                61,
                                42
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 2,
                            "_value": "\t\tfunction(content: string) {\n",
                            "start": [
                                62,
                                1
                            ],
                            "end": [
                                62,
                                36
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\tconsole.log(content)\n",
                            "start": [
                                63,
                                1
                            ],
                            "end": [
                                63,
                                33
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 3,
                            "_value": "\t\t\t// yeah that's how we send mail\n",
                            "start": [
                                64,
                                1
                            ],
                            "end": [
                                64,
                                44
                            ],
                            "_tokens": []
                        },
                        {
                            "indent": 2,
                            "_value": "\t\t}\n",
                            "start": [
                                65,
                                1
                            ],
                            "end": [
                                65,
                                10
                            ],
                            "_tokens": null
                        },
                        {
                            "indent": 1,
                            "_value": "\t`",
                            "start": [
                                66,
                                1
                            ],
                            "end": [
                                66,
                                6
                            ],
                            "_tokens": [
                                {
                                    "tokenType": "punct",
                                    "value": "`",
                                    "start": [
                                        66,
                                        5
                                    ],
                                    "end": [
                                        66,
                                        6
                                    ],
                                    "type": "Token"
                                }
                            ]
                        }
                    ],
                    "type": "Block",
                    "level": 1
                }
            ]
        }
    ]
};
