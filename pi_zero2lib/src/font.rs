pub const ALPHABET: [[bool; 64]; 26] = [
    // A (already provided)
    [
        false, false, true, true, true, false, false, false,
        false, true, true, false, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, true, true, true, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // B (already provided)
    [
        true, true, true, true, true, false, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, true, true, true, false, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, true, true, false, false,
        true, true, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // C (already provided)
    [
        false, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // D (already provided)
    [
        true, true, true, true, true, false, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, true, true, false, false,
        true, true, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // E (already provided)
    [
        true, true, true, true, true, true, true, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, true, true, true, true, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // F (already provided)
    [
        true, true, true, true, true, true, true, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, true, true, true, true, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // G (already provided)
    [
        false, true, true, true, true, true, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, true, true,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // H
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, true, true, true, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // I
    [
        true, true, true, true, true, true, true, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        true, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // J
    [
        false, false, false, false, true, true, true, false,
        false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false,
        false, false, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // K
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, true, true, false, false,
        true, true, false, true, true, false, false, false,
        true, true, true, true, false, false, false, false,
        true, true, false, true, true, false, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // L
    [
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // M
    [
        true, true, false, false, false, true, true, false,
        true, true, true, false, true, true, true, false,
        true, true, true, true, true, true, true, false,
        true, true, false, true, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // N
    [
        true, true, false, false, false, true, true, false,
        true, true, true, false, false, true, true, false,
        true, true, true, true, false, true, true, false,
        true, true, false, true, true, true, true, false,
        true, true, false, false, true, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // O
    [
        false, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // P
    [
        true, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, true, true, true, true, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // Q
    [
        false, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, true, false, true, true, false,
        true, true, false, false, true, true, false, false,
        false, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // R
    [
        true, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, true, true, true, true, false, false,
        true, true, false, true, true, false, false, false,
        true, true, false, false, true, true, false, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // S
    [
        false, true, true, true, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, false, false, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // T
    [
        true, true, true, true, true, true, true, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // U
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // V
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, false, true, true, false, false,
        false, false, true, true, true, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // W
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        true, true, false, true, false, true, true, false,
        true, true, true, true, true, true, true, false,
        true, true, true, false, true, true, true, false,
        true, true, false, false, false, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
    // X
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, false, true, true, false, false,
        false, false, true, true, true, false, false, false,
        false, false, true, true, true, false, false, false,
        false, true, true, false, true, true, false, false,
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
    ],
    // Y
    [
        true, true, false, false, false, true, true, false,
        true, true, false, false, false, true, true, false,
        false, true, true, false, true, true, false, false,
        false, false, true, true, true, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // Z
    [
        true, true, true, true, true, true, true, false,
        false, false, false, false, true, true, false, false,
        false, false, false, true, true, false, false, false,
        false, false, true, true, false, false, false, false,
        false, true, true, false, false, false, false, false,
        true, true, false, false, false, false, false, false,
        true, true, true, true, true, true, true, false,
        false, false, false, false, false, false, false, false,
    ],
];

pub const SPECIAL_CHARS: [[bool; 64]; 1] = [
    [
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,
        false, false, false, false, false, false, false , false,

    ]
];

pub const NUMBERS: [[bool; 64]; 10] = [
    // 0
    [
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 1
    [
        false, false, false, true, false, false, false, false,
        false, false, true, true, false, false, false, false,
        false, true, false, true, false, false, false, false,
        false, false, false, true, false, false, false, false,
        false, false, false, true, false, false, false, false,
        false, false, false, true, false, false, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 2
    [
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 3
    [
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 4
    [
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 5
    [
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 6
    [
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, false, false, false,
        true, false, false, false, false, false, false, false,
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 7
    [
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 8
    [
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
    // 9
    [
        true, true, true, true, true, true, false, false,
        true, false, false, false, false, true, false, false,
        true, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, true, false, false,
        false, false, false, false, false, true, false, false,
        true, true, true, true, true, true, false, false,
        false, false, false, false, false, false, false, false,
    ],
];