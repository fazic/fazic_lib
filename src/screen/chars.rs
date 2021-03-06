pub const CHARS: [u64; 127] = [
    0x003c46067676663c,
    0x007c667c603c0000,
    0x003e66663e060600,
    0x003c0606063c0000,
    0x007c66667c606000,
    0x003c067e663c0000,
    0x001818187c187000,
    0x3e607c66667c0000,
    0x006666663e060600,
    0x003c18181c001800,
    0x3c60606060006000,
    0x0066361e36060600,
    0x003c181818181c00,
    0x00c6d6fefe660000,
    0x00666666663e0000,
    0x003c6666663c0000,
    0x06063e66663e0000,
    0x60607c66667c0000,
    0x00060606663e0000,
    0x003e603c067c0000,
    0x00701818187e1800,
    0x007c666666660000,
    0x00183c6666660000,
    0x006c7cfed6c60000,
    0x00663c183c660000,
    0x1e307c6666660000,
    0x007e0c18307e0000,
    0x003c0c0c0c0c0c3c,
    0x003f460c3e0c4830,
    0x003c30303030303c,
    0x181818187e3c1800,
    0x00080cfefe0c0800,
    000000000000000000,
    0x0018000018181818,
    0x0000000000666666,
    0x006666ff66ff6666,
    0x00183e603c067c18,
    0x0062660c18306646,
    0x00fc66e61c3c663c,
    0x0000000000183060,
    0x0030180c0c0c1830,
    0x000c18303030180c,
    0x0000663cff3c6600,
    0x000018187e181800,
    0x0c18180000000000,
    0x000000007e000000,
    0x0018180000000000,
    0x00060c183060c000,
    0x003c66666e76663c,
    0x007e1818181c1818,
    0x007e060c3060663c,
    0x003c66603860663c,
    0x006060fe66787060,
    0x003c6660603e067e,
    0x003c66663e06663c,
    0x001818181830667e,
    0x003c66663c66663c,
    0x003c66607c66663c,
    0x0000180000180000,
    0x0c18180000180000,
    0x0070180c060c1870,
    0x0000007e007e0000,
    0x000e18306030180e,
    0x001800183060663c,
    0x000000ffff000000,
    0x006666667e663c18,
    0x003e66663e66663e,
    0x003c66060606663c,
    0x001e36666666361e,
    0x007e06061e06067e,
    0x000606061e06067e,
    0x003c66667606663c,
    0x006666667e666666,
    0x003c18181818183c,
    0x001c363030303078,
    0x0066361e0e1e3666,
    0x007e060606060606,
    0x00c6c6c6d6feeec6,
    0x006666767e7e6e66,
    0x003c66666666663c,
    0x000606063e66663e,
    0x00703c666666663c,
    0x0066361e3e66663e,
    0x003c66603c06663c,
    0x001818181818187e,
    0x003c666666666666,
    0x00183c6666666666,
    0x00c6eefed6c6c6c6,
    0x0066663c183c6666,
    0x001818183c666666,
    0x007e060c1830607e,
    0x181818ffff181818,
    0x0c0c03030c0c0303,
    0x1818181818181818,
    0x3333cccc3333cccc,
    0x663399cc663399cc,
    000000000000000000,
    0x0f0f0f0f0f0f0f0f,
    0xffffffff00000000,
    0x00000000000000ff,
    0xff00000000000000,
    0x0303030303030303,
    0xcccc3333cccc3333,
    0xc0c0c0c0c0c0c0c0,
    0xcccc333300000000,
    0x66cc993366cc9933,
    0xc0c0c0c0c0c0c0c0,
    0x181818f8f8181818,
    0xf0f0f0f000000000,
    0x000000f8f8181818,
    0x1818181f1f000000,
    0xffff000000000000,
    0x181818f8f8000000,
    0x000000ffff181818,
    0x181818ffff000000,
    0x1818181f1f181818,
    0x0303030303030303,
    0x0707070707070707,
    0xe0e0e0e0e0e0e0e0,
    0x000000000000ffff,
    0x0000000000ffffff,
    0xffffff0000000000,
    0x00060e1e3660c080,
    0xffffffff0f0f0f0f,
    0xffffffe0e0e7e7e7,
    0xfffffffff0f0f0f0,
    0x0f0f0f0ff0f0f0f0,
];

pub fn get_char_index(char: char) -> u8 {
    match char {
        '@' => 0,
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        '[' => 27,
        '£' => 28,
        ']' => 29,
        '^' => 30,
        '{' => 31,
        '!' => 33,
        '"' => 34,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '\'' => 39,
        '(' => 40,
        ')' => 41,
        '*' => 42,
        '+' => 43,
        ',' => 44,
        '-' => 45,
        '.' => 46,
        '/' => 47,
        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ':' => 58,
        ';' => 59,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,
        '~' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,
        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        _ => 32,
    }
}

pub fn get_char(char: char) -> u64 {
    CHARS[get_char_index(char) as usize]
}
