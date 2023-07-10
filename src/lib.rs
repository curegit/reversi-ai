use std::cmp::max;
use std::sync::mpsc;
use std::thread;

const INTMAX: i32 = 2147483647;
const INTMIN: i32 = -2147483647;

/// ビット番号から i 座標を返す
#[no_mangle]
pub extern "C" fn index_to_position_i(n: i32) -> i32 {
    n & 0x07
}

/// ビット番号から j 座標を返す
#[no_mangle]
pub extern "C" fn index_to_position_j(n: i32) -> i32 {
    n >> 3
}

/// 座標位置からビット番号を返す
#[no_mangle]
pub extern "C" fn position_to_index(i: i32, j: i32) -> i32 {
    (j << 3) | i
}

/// n 番目にだけビットを立たせたビットボード表現を返す
#[no_mangle]
pub extern "C" fn index_to_bit(n: i32) -> u64 {
    0x01u64 << n
}

/// 特定の座標位置に対応する場所にだけビットを立たせたビットボード表現を返す
#[no_mangle]
pub extern "C" fn position_to_bit(i: i32, j: i32) -> u64 {
    index_to_bit(position_to_index(i, j))
}

// 空いているマスのビットボード表現を返す
fn empty_squares(player1: u64, player2: u64) -> u64 {
    !(player1 | player2)
}

/// myself プレイヤーが着手可能な手のビットボード表現を返す
#[no_mangle]
pub extern "C" fn possible_moves(myself: u64, opponent: u64) -> u64 {
    let blank = empty_squares(myself, opponent);
    let opp = opponent & 0x7E7E7E7E7E7E7E7E;
    let mut flip: u64;
    let mut moves: u64;
    // 北
    flip = opponent & (myself << 8);
    flip |= opponent & (flip << 8);
    flip |= opponent & (flip << 8);
    flip |= opponent & (flip << 8);
    flip |= opponent & (flip << 8);
    flip |= opponent & (flip << 8);
    moves = flip << 8;
    // 北東
    flip = opp & (myself << 7);
    flip |= opp & (flip << 7);
    flip |= opp & (flip << 7);
    flip |= opp & (flip << 7);
    flip |= opp & (flip << 7);
    flip |= opp & (flip << 7);
    moves |= flip << 7;
    // 東
    flip = opp & (myself >> 1);
    flip |= opp & (flip >> 1);
    flip |= opp & (flip >> 1);
    flip |= opp & (flip >> 1);
    flip |= opp & (flip >> 1);
    flip |= opp & (flip >> 1);
    moves |= flip >> 1;
    // 南東
    flip = opp & (myself >> 9);
    flip |= opp & (flip >> 9);
    flip |= opp & (flip >> 9);
    flip |= opp & (flip >> 9);
    flip |= opp & (flip >> 9);
    flip |= opp & (flip >> 9);
    moves |= flip >> 9;
    // 南
    flip = opponent & (myself >> 8);
    flip |= opponent & (flip >> 8);
    flip |= opponent & (flip >> 8);
    flip |= opponent & (flip >> 8);
    flip |= opponent & (flip >> 8);
    flip |= opponent & (flip >> 8);
    moves |= flip >> 8;
    // 南西
    flip = opp & (myself >> 7);
    flip |= opp & (flip >> 7);
    flip |= opp & (flip >> 7);
    flip |= opp & (flip >> 7);
    flip |= opp & (flip >> 7);
    flip |= opp & (flip >> 7);
    moves |= flip >> 7;
    // 西
    flip = opp & (myself << 1);
    flip |= opp & (flip << 1);
    flip |= opp & (flip << 1);
    flip |= opp & (flip << 1);
    flip |= opp & (flip << 1);
    flip |= opp & (flip << 1);
    moves |= flip << 1;
    // 北西
    flip = opp & (myself << 9);
    flip |= opp & (flip << 9);
    flip |= opp & (flip << 9);
    flip |= opp & (flip << 9);
    flip |= opp & (flip << 9);
    flip |= opp & (flip << 9);
    moves |= flip << 9;
    // 結果
    moves & blank
}

/// myself プレイヤーが index 地点に打ったときに返せる石のビットボード表現を返す
#[no_mangle]
pub extern "C" fn turnovers(myself: u64, opponent: u64, index: i32) -> u64 {
    let mut turns = 0x00;
    let pos = index_to_bit(index);
    let opp = opponent & 0x7E7E7E7E7E7E7E7E;
    let mut t: u64;
    // 北
    t = (pos << 8) & opponent;
    t |= (t << 8) & opponent;
    t |= (t << 8) & opponent;
    t |= (t << 8) & opponent;
    t |= (t << 8) & opponent;
    t |= (t << 8) & opponent;
    if (t << 8) & myself != 0 {
        turns |= t;
    }
    // 北東
    t = (pos << 7) & opp;
    t |= (t << 7) & opp;
    t |= (t << 7) & opp;
    t |= (t << 7) & opp;
    t |= (t << 7) & opp;
    t |= (t << 7) & opp;
    if (t << 7) & myself != 0 {
        turns |= t;
    }
    // 東
    t = (pos >> 1) & opp;
    t |= (t >> 1) & opp;
    t |= (t >> 1) & opp;
    t |= (t >> 1) & opp;
    t |= (t >> 1) & opp;
    t |= (t >> 1) & opp;
    if (t >> 1) & myself != 0 {
        turns |= t;
    }
    // 南東
    t = (pos >> 9) & opp;
    t |= (t >> 9) & opp;
    t |= (t >> 9) & opp;
    t |= (t >> 9) & opp;
    t |= (t >> 9) & opp;
    t |= (t >> 9) & opp;
    if (t >> 9) & myself != 0 {
        turns |= t;
    }
    // 南
    t = (pos >> 8) & opponent;
    t |= (t >> 8) & opponent;
    t |= (t >> 8) & opponent;
    t |= (t >> 8) & opponent;
    t |= (t >> 8) & opponent;
    t |= (t >> 8) & opponent;
    if (t >> 8) & myself != 0 {
        turns |= t;
    }
    // 南西
    t = (pos >> 7) & opp;
    t |= (t >> 7) & opp;
    t |= (t >> 7) & opp;
    t |= (t >> 7) & opp;
    t |= (t >> 7) & opp;
    t |= (t >> 7) & opp;
    if (t >> 7) & myself != 0 {
        turns |= t;
    }
    // 西
    t = (pos << 1) & opp;
    t |= (t << 1) & opp;
    t |= (t << 1) & opp;
    t |= (t << 1) & opp;
    t |= (t << 1) & opp;
    t |= (t << 1) & opp;
    if (t << 1) & myself != 0 {
        turns |= t;
    }
    // 北西
    t = (pos << 9) & opp;
    t |= (t << 9) & opp;
    t |= (t << 9) & opp;
    t |= (t << 9) & opp;
    t |= (t << 9) & opp;
    t |= (t << 9) & opp;
    if (t << 9) & myself != 0 {
        turns |= t;
    }
    turns
}

/// myself プレイヤーが index 地点に打てるかどうかを返す
#[no_mangle]
pub extern "C" fn can_place(myself: u64, opponent: u64, index: i32) -> i32 {
    (possible_moves(myself, opponent) & index_to_bit(index) != 0) as i32
}

/// myself プレイヤーが index 地点に打ったときに得られる盤を可変参照によって変更し、返した石のビットボード表現を戻り値で返す
#[no_mangle]
pub extern "C" fn place(
    myself: u64,
    opponent: u64,
    index: i32,
    nmyself: &mut u64,
    nopponent: &mut u64,
) -> u64 {
    let turns = turnovers(myself, opponent, index);
    *nmyself = myself | turns | index_to_bit(index);
    *nopponent = opponent & !turns;
    turns
}

/// 立っているビットの数を返す
#[no_mangle]
pub extern "C" fn count_bits(n: u64) -> i32 {
    let mut n = n;
    n = ((n & 0xAAAA_AAAA_AAAA_AAAA) >> 1) + (n & 0x5555_5555_5555_5555);
    n = ((n & 0xCCCC_CCCC_CCCC_CCCC) >> 2) + (n & 0x3333_3333_3333_3333);
    n = ((n & 0xF0F0_F0F0_F0F0_F0F0) >> 4) + (n & 0x0F0F_0F0F_0F0F_0F0F);
    n = ((n & 0xFF00_FF00_FF00_FF00) >> 8) + (n & 0x00FF_00FF_00FF_00FF);
    n = ((n & 0xFFFF_0000_FFFF_0000) >> 16) + (n & 0x0000_FFFF_0000_FFFF);
    n = ((n & 0xFFFF_FFFF_0000_0000) >> 32) + (n & 0x0000_0000_FFFF_FFFF);
    n as i32
}

/// myself プレイヤーの石の数から opponent プレイヤーの石の数を引いたものを返す
#[no_mangle]
pub extern "C" fn balance(myself: u64, opponent: u64) -> i32 {
    count_bits(myself) - count_bits(opponent)
}

/// 有利なほど大きいように盤上の位置ごとにつけられた重みを用いて、石のある位置の重みの和を返す
#[no_mangle]
pub extern "C" fn sum_of_weights(disks: u64) -> i32 {
    const W1: [i32; 256] = [
        0, 10000, -3000, 7000, 1000, 11000, -2000, 8000, 800, 10800, -2200, 7800, 1800, 11800,
        -1200, 8800, 800, 10800, -2200, 7800, 1800, 11800, -1200, 8800, 1600, 11600, -1400, 8600,
        2600, 12600, -400, 9600, 1000, 11000, -2000, 8000, 2000, 12000, -1000, 9000, 1800, 11800,
        -1200, 8800, 2800, 12800, -200, 9800, 1800, 11800, -1200, 8800, 2800, 12800, -200, 9800,
        2600, 12600, -400, 9600, 3600, 13600, 600, 10600, -3000, 7000, -6000, 4000, -2000, 8000,
        -5000, 5000, -2200, 7800, -5200, 4800, -1200, 8800, -4200, 5800, -2200, 7800, -5200, 4800,
        -1200, 8800, -4200, 5800, -1400, 8600, -4400, 5600, -400, 9600, -3400, 6600, -2000, 8000,
        -5000, 5000, -1000, 9000, -4000, 6000, -1200, 8800, -4200, 5800, -200, 9800, -3200, 6800,
        -1200, 8800, -4200, 5800, -200, 9800, -3200, 6800, -400, 9600, -3400, 6600, 600, 10600,
        -2400, 7600, 10000, 20000, 7000, 17000, 11000, 21000, 8000, 18000, 10800, 20800, 7800,
        17800, 11800, 21800, 8800, 18800, 10800, 20800, 7800, 17800, 11800, 21800, 8800, 18800,
        11600, 21600, 8600, 18600, 12600, 22600, 9600, 19600, 11000, 21000, 8000, 18000, 12000,
        22000, 9000, 19000, 11800, 21800, 8800, 18800, 12800, 22800, 9800, 19800, 11800, 21800,
        8800, 18800, 12800, 22800, 9800, 19800, 12600, 22600, 9600, 19600, 13600, 23600, 10600,
        20600, 7000, 17000, 4000, 14000, 8000, 18000, 5000, 15000, 7800, 17800, 4800, 14800, 8800,
        18800, 5800, 15800, 7800, 17800, 4800, 14800, 8800, 18800, 5800, 15800, 8600, 18600, 5600,
        15600, 9600, 19600, 6600, 16600, 8000, 18000, 5000, 15000, 9000, 19000, 6000, 16000, 8800,
        18800, 5800, 15800, 9800, 19800, 6800, 16800, 8800, 18800, 5800, 15800, 9800, 19800, 6800,
        16800, 9600, 19600, 6600, 16600, 10600, 20600, 7600, 17600,
    ];
    const W2: [i32; 256] = [
        0, -3000, -5000, -8000, -450, -3450, -5450, -8450, -500, -3500, -5500, -8500, -950, -3950,
        -5950, -8950, -500, -3500, -5500, -8500, -950, -3950, -5950, -8950, -1000, -4000, -6000,
        -9000, -1450, -4450, -6450, -9450, -450, -3450, -5450, -8450, -900, -3900, -5900, -8900,
        -950, -3950, -5950, -8950, -1400, -4400, -6400, -9400, -950, -3950, -5950, -8950, -1400,
        -4400, -6400, -9400, -1450, -4450, -6450, -9450, -1900, -4900, -6900, -9900, -5000, -8000,
        -10000, -13000, -5450, -8450, -10450, -13450, -5500, -8500, -10500, -13500, -5950, -8950,
        -10950, -13950, -5500, -8500, -10500, -13500, -5950, -8950, -10950, -13950, -6000, -9000,
        -11000, -14000, -6450, -9450, -11450, -14450, -5450, -8450, -10450, -13450, -5900, -8900,
        -10900, -13900, -5950, -8950, -10950, -13950, -6400, -9400, -11400, -14400, -5950, -8950,
        -10950, -13950, -6400, -9400, -11400, -14400, -6450, -9450, -11450, -14450, -6900, -9900,
        -11900, -14900, -3000, -6000, -8000, -11000, -3450, -6450, -8450, -11450, -3500, -6500,
        -8500, -11500, -3950, -6950, -8950, -11950, -3500, -6500, -8500, -11500, -3950, -6950,
        -8950, -11950, -4000, -7000, -9000, -12000, -4450, -7450, -9450, -12450, -3450, -6450,
        -8450, -11450, -3900, -6900, -8900, -11900, -3950, -6950, -8950, -11950, -4400, -7400,
        -9400, -12400, -3950, -6950, -8950, -11950, -4400, -7400, -9400, -12400, -4450, -7450,
        -9450, -12450, -4900, -7900, -9900, -12900, -8000, -11000, -13000, -16000, -8450, -11450,
        -13450, -16450, -8500, -11500, -13500, -16500, -8950, -11950, -13950, -16950, -8500,
        -11500, -13500, -16500, -8950, -11950, -13950, -16950, -9000, -12000, -14000, -17000,
        -9450, -12450, -14450, -17450, -8450, -11450, -13450, -16450, -8900, -11900, -13900,
        -16900, -8950, -11950, -13950, -16950, -9400, -12400, -14400, -17400, -8950, -11950,
        -13950, -16950, -9400, -12400, -14400, -17400, -9450, -12450, -14450, -17450, -9900,
        -12900, -14900, -17900,
    ];
    const W3: [i32; 256] = [
        0, 1000, -450, 550, 30, 1030, -420, 580, 10, 1010, -440, 560, 40, 1040, -410, 590, 10,
        1010, -440, 560, 40, 1040, -410, 590, 20, 1020, -430, 570, 50, 1050, -400, 600, 30, 1030,
        -420, 580, 60, 1060, -390, 610, 40, 1040, -410, 590, 70, 1070, -380, 620, 40, 1040, -410,
        590, 70, 1070, -380, 620, 50, 1050, -400, 600, 80, 1080, -370, 630, -450, 550, -900, 100,
        -420, 580, -870, 130, -440, 560, -890, 110, -410, 590, -860, 140, -440, 560, -890, 110,
        -410, 590, -860, 140, -430, 570, -880, 120, -400, 600, -850, 150, -420, 580, -870, 130,
        -390, 610, -840, 160, -410, 590, -860, 140, -380, 620, -830, 170, -410, 590, -860, 140,
        -380, 620, -830, 170, -400, 600, -850, 150, -370, 630, -820, 180, 1000, 2000, 550, 1550,
        1030, 2030, 580, 1580, 1010, 2010, 560, 1560, 1040, 2040, 590, 1590, 1010, 2010, 560, 1560,
        1040, 2040, 590, 1590, 1020, 2020, 570, 1570, 1050, 2050, 600, 1600, 1030, 2030, 580, 1580,
        1060, 2060, 610, 1610, 1040, 2040, 590, 1590, 1070, 2070, 620, 1620, 1040, 2040, 590, 1590,
        1070, 2070, 620, 1620, 1050, 2050, 600, 1600, 1080, 2080, 630, 1630, 550, 1550, 100, 1100,
        580, 1580, 130, 1130, 560, 1560, 110, 1110, 590, 1590, 140, 1140, 560, 1560, 110, 1110,
        590, 1590, 140, 1140, 570, 1570, 120, 1120, 600, 1600, 150, 1150, 580, 1580, 130, 1130,
        610, 1610, 160, 1160, 590, 1590, 140, 1140, 620, 1620, 170, 1170, 590, 1590, 140, 1140,
        620, 1620, 170, 1170, 600, 1600, 150, 1150, 630, 1630, 180, 1180,
    ];
    const W4: [i32; 256] = [
        0, 800, -500, 300, 10, 810, -490, 310, 50, 850, -450, 350, 60, 860, -440, 360, 50, 850,
        -450, 350, 60, 860, -440, 360, 100, 900, -400, 400, 110, 910, -390, 410, 10, 810, -490,
        310, 20, 820, -480, 320, 60, 860, -440, 360, 70, 870, -430, 370, 60, 860, -440, 360, 70,
        870, -430, 370, 110, 910, -390, 410, 120, 920, -380, 420, -500, 300, -1000, -200, -490,
        310, -990, -190, -450, 350, -950, -150, -440, 360, -940, -140, -450, 350, -950, -150, -440,
        360, -940, -140, -400, 400, -900, -100, -390, 410, -890, -90, -490, 310, -990, -190, -480,
        320, -980, -180, -440, 360, -940, -140, -430, 370, -930, -130, -440, 360, -940, -140, -430,
        370, -930, -130, -390, 410, -890, -90, -380, 420, -880, -80, 800, 1600, 300, 1100, 810,
        1610, 310, 1110, 850, 1650, 350, 1150, 860, 1660, 360, 1160, 850, 1650, 350, 1150, 860,
        1660, 360, 1160, 900, 1700, 400, 1200, 910, 1710, 410, 1210, 810, 1610, 310, 1110, 820,
        1620, 320, 1120, 860, 1660, 360, 1160, 870, 1670, 370, 1170, 860, 1660, 360, 1160, 870,
        1670, 370, 1170, 910, 1710, 410, 1210, 920, 1720, 420, 1220, 300, 1100, -200, 600, 310,
        1110, -190, 610, 350, 1150, -150, 650, 360, 1160, -140, 660, 350, 1150, -150, 650, 360,
        1160, -140, 660, 400, 1200, -100, 700, 410, 1210, -90, 710, 310, 1110, -190, 610, 320,
        1120, -180, 620, 360, 1160, -140, 660, 370, 1170, -130, 670, 360, 1160, -140, 660, 370,
        1170, -130, 670, 410, 1210, -90, 710, 420, 1220, -80, 720,
    ];
    W1[(disks & 0xFF) as usize]
        + W2[((disks >> 8) & 0xFF) as usize]
        + W3[((disks >> 16) & 0xFF) as usize]
        + W4[((disks >> 24) & 0xFF) as usize]
        + W4[((disks >> 32) & 0xFF) as usize]
        + W3[((disks >> 40) & 0xFF) as usize]
        + W2[((disks >> 48) & 0xFF) as usize]
        + W1[(disks >> 56) as usize]
}

/// myself プレイヤーに有利なほど大きな数が返る静的評価関数
#[no_mangle]
pub extern "C" fn evaluation(myself: u64, opponent: u64) -> i32 {
    let b = count_bits(myself | opponent);
    let k = 50 * b;
    (sum_of_weights(myself) - sum_of_weights(opponent))
        + k * (count_bits(possible_moves(myself, opponent))
            - count_bits(possible_moves(opponent, myself)))
}

/// turns 周りの開放度を返す
#[no_mangle]
pub extern "C" fn openness(myself: u64, opponent: u64, turns: u64) -> i32 {
    let blank = empty_squares(myself, opponent);
    let blae = blank & 0x7F7F7F7F7F7F7F7F;
    let blaw = blank & 0xFEFEFEFEFEFEFEFE;
    let mut o = 0;
    o += count_bits((turns << 8) & blank);
    o += count_bits((turns << 7) & blae);
    o += count_bits((turns >> 1) & blae);
    o += count_bits((turns >> 9) & blae);
    o += count_bits((turns >> 8) & blank);
    o += count_bits((turns >> 7) & blaw);
    o += count_bits((turns << 1) & blaw);
    o += count_bits((turns << 9) & blaw);
    o
}

/// 係数を掛けた開放度の負値を返す
#[no_mangle]
pub extern "C" fn openness_evaluation(myself: u64, opponent: u64, turns: u64) -> i32 {
    // 開放度に掛ける適当な係数
    const K: i32 = 10;
    -K * openness(myself, opponent, turns)
}

// ゲーム木の完全探索のサブルーチン
fn full_search_sub(myself: u64, opponent: u64, alpha: i32, beta: i32) -> i32 {
    let moves = possible_moves(myself, opponent);
    if moves != 0 {
        let mut alpha = alpha;
        let mut m = moves;
        let mut i = 0;
        loop {
            if m & 0x01 != 0 {
                let mut s: u64 = 0;
                let mut o: u64 = 0;
                place(myself, opponent, i, &mut s, &mut o);
                let v = -full_search_sub(o, s, -beta, -alpha);
                if v > alpha {
                    alpha = v;
                    if alpha >= beta {
                        break;
                    }
                }
            }
            m >>= 1;
            i += 1;
            if m == 0 {
                break;
            }
        }
        alpha
    } else {
        if possible_moves(opponent, myself) != 0 {
            -full_search_sub(opponent, myself, -beta, -alpha)
        } else {
            balance(myself, opponent)
        }
    }
}

/// ミニマックス戦略に基づいてゲーム木の完全探索をし、最良の手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
#[no_mangle]
pub extern "C" fn full_search(myself: u64, opponent: u64) -> i32 {
    let moves = possible_moves(myself, opponent);
    if moves == 0 {
        return -1;
    }
    let mut alpha = INTMIN;
    let beta = INTMAX;
    let mut m = moves;
    let mut i = 0;
    let mut chosen = 0;
    loop {
        if m & 0x01 != 0 {
            let mut s: u64 = 0;
            let mut o: u64 = 0;
            place(myself, opponent, i, &mut s, &mut o);
            let v = -full_search_sub(o, s, -beta, -alpha);
            if v > alpha {
                alpha = v;
                chosen = i;
            }
        }
        m >>= 1;
        i += 1;
        if m == 0 {
            break;
        }
    }
    chosen
}

/// ミニマックス戦略に基づいてゲーム木の完全探索をし、最良の手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// この関数は複数スレッドによって並列処理される
#[no_mangle]
pub extern "C" fn full_search_parallel_with(myself: u64, opponent: u64, concurrency: i32) -> i32 {
    // 打てる手がなければ終了
    let moves = possible_moves(myself, opponent);
    if moves == 0 {
        return -1;
    }
    // 探索をする
    let mut alpha = INTMIN;
    let mut m = moves;
    let mut i = 0;
    let mut chosen = 0;
    let result = thread::scope(|scope| {
        let mut handles = Vec::new();
        let (sender, receiver) = mpsc::channel();
        let mut k = 0;
        loop {
            if m & 0x01 != 0 {
                let mut s: u64 = 0;
                let mut o: u64 = 0;
                place(myself, opponent, i, &mut s, &mut o);
                // 並列性を制限
                if k >= concurrency {
                    receiver.recv().unwrap();
                } else {
                    k += 1;
                }
                let sender = sender.clone();
                // 完全探索のスレッド関数
                let handle = scope.spawn(move || {
                    let alpha: i32 = INTMIN;
                    let beta: i32 = INTMAX;
                    let value = -full_search_sub(o, s, -beta, -alpha);
                    sender.send(()).unwrap();
                    value
                });
                handles.push((i, handle));
            }
            m >>= 1;
            i += 1;
            if m == 0 {
                break;
            }
        }
        // スレッドから結果を回収する
        for (i, handle) in handles {
            let v = handle.join().unwrap();
            if v > alpha {
                alpha = v;
                chosen = i;
            }
        }
        chosen
    });
    result
}

/// ミニマックス戦略に基づいてゲーム木の完全探索をし、最良の手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
#[no_mangle]
pub extern "C" fn full_search_parallel(myself: u64, opponent: u64) -> i32 {
    let cpu_count = num_cpus::get() as i32;
    full_search_parallel_with(myself, opponent, cpu_count)
}

// ゲーム木の部分探索のサブルーチン
fn heuristic_search_sub(myself: u64, opponent: u64, depth: i32, alpha: i32, beta: i32) -> i32 {
    const CONFIDENT_VICTORY: i32 = 100000000;
    let moves = possible_moves(myself, opponent);
    if moves != 0 {
        if depth != 0 {
            let mut alpha = alpha;
            let mut m = moves;
            let mut i = 0;
            loop {
                if m & 0x01 != 0 {
                    let mut s: u64 = 0;
                    let mut o: u64 = 0;
                    place(myself, opponent, i, &mut s, &mut o);
                    let v = -heuristic_search_sub(o, s, depth - 1, -beta, -alpha);
                    if v > alpha {
                        alpha = v;
                        if alpha >= beta {
                            break;
                        }
                    }
                }
                m >>= 1;
                i += 1;
                if m == 0 {
                    break;
                }
            }
            alpha
        } else {
            evaluation(myself, opponent)
        }
    } else {
        if possible_moves(opponent, myself) != 0 {
            if depth != 0 {
                -heuristic_search_sub(opponent, myself, depth - 1, -beta, -alpha)
            } else {
                evaluation(myself, opponent)
            }
        } else {
            if balance(myself, opponent) > 0 {
                CONFIDENT_VICTORY
            } else {
                -CONFIDENT_VICTORY
            }
        }
    }
}

/// ミニマックス戦略に基づいてゲーム木の部分探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// depth は先読みの深さで、1 以上である必要があり奇数が望ましい
#[no_mangle]
pub extern "C" fn heuristic_search(myself: u64, opponent: u64, depth: i32) -> i32 {
    // 打てる手がなければ終了
    let moves = possible_moves(myself, opponent);
    if moves == 0 {
        return -1;
    }
    // 探索をする
    let mut alpha = INTMIN;
    let beta = INTMAX;
    let mut m = moves;
    let mut i = 0;
    let d = depth - 1;
    let mut chosen = 0;
    loop {
        if m & 0x01 != 0 {
            let mut s: u64 = 0;
            let mut o: u64 = 0;
            let turns = place(myself, opponent, i, &mut s, &mut o);
            let v = -heuristic_search_sub(o, s, d, -beta, -alpha)
                + openness_evaluation(myself, opponent, turns);
            if v > alpha {
                alpha = v;
                chosen = i;
            }
        }
        m >>= 1;
        i += 1;
        if m == 0 {
            break;
        }
    }
    chosen
}

/// ミニマックス戦略に基づいてゲーム木の部分探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// depth は先読みの深さで、1 以上である必要があり奇数が望ましい
///
/// この関数は複数スレッドによって並列処理される
#[no_mangle]
pub extern "C" fn heuristic_search_parallel_with(
    myself: u64,
    opponent: u64,
    depth: i32,
    concurrency: i32,
) -> i32 {
    // 打てる手がなければ終了
    let moves = possible_moves(myself, opponent);
    if moves == 0 {
        return -1;
    }
    // 探索をする
    let mut alpha = INTMIN;
    let mut m = moves;
    let mut i = 0;
    let mut chosen = 0;
    let result = thread::scope(|scope| {
        let mut handles = Vec::new();
        let (sender, receiver) = mpsc::channel();
        let mut k = 0;
        loop {
            if m & 0x01 != 0 {
                let mut s: u64 = 0;
                let mut o: u64 = 0;
                let turns = place(myself, opponent, i, &mut s, &mut o);
                let opns = openness_evaluation(myself, opponent, turns);
                // 並列性を制限
                if k >= concurrency {
                    receiver.recv().unwrap();
                } else {
                    k += 1;
                }
                let sender = sender.clone();
                // 部分探索のスレッド関数
                let handle = scope.spawn(move || {
                    let alpha: i32 = INTMIN;
                    let beta: i32 = INTMAX;
                    let value = -heuristic_search_sub(o, s, depth - 1, -beta, -alpha) + opns;
                    sender.send(()).unwrap();
                    value
                });
                handles.push((i, handle));
            }
            m >>= 1;
            i += 1;
            if m == 0 {
                break;
            }
        }
        // スレッドから結果を回収する
        for (i, handle) in handles {
            let v = handle.join().unwrap();
            if v > alpha {
                alpha = v;
                chosen = i;
            }
        }
        chosen
    });
    result
}

/// ミニマックス戦略に基づいてゲーム木の部分探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// depth は先読みの深さで、1 以上である必要があり奇数が望ましい
///
/// この関数は CPU スレッド数のスレッドによって並列処理される
#[no_mangle]
pub extern "C" fn heuristic_search_parallel(myself: u64, opponent: u64, depth: i32) -> i32 {
    let cpu_count = num_cpus::get() as i32;
    heuristic_search_parallel_with(myself, opponent, depth, cpu_count)
}

/// ミニマックス戦略に基づいてゲーム木の探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// ゲームの進行度によって部分探索と完全探索を自動で選択する
///
/// 切り替えのタイミングと、先読みの深さは数秒で結果が返るような値に調整されている
#[no_mangle]
pub extern "C" fn choose_move(myself: u64, opponent: u64) -> i32 {
    let occu = count_bits(myself | opponent);
    if occu > 50 {
        return full_search(myself, opponent);
    } else {
        let move_count = max(
            count_bits(possible_moves(myself, opponent)),
            count_bits(possible_moves(opponent, myself)),
        );
        return heuristic_search(myself, opponent, if move_count > 8 { 5 } else { 7 });
    }
}

/// ミニマックス戦略に基づいてゲーム木の探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// ゲームの進行度によって部分探索と完全探索を自動で選択する
///
/// 切り替えのタイミングと、先読みの深さは数秒で結果が返るような値に調整されている
///
/// この関数は複数スレッドによって並列処理される
///
/// 並列処理によって探索にかかる時間が短くなるので非並列版よりも深く読むようにしている
#[no_mangle]
pub extern "C" fn choose_move_parallel_with(myself: u64, opponent: u64, concurrency: i32) -> i32 {
    let occu = count_bits(myself | opponent);
    if occu > 48 {
        return full_search_parallel_with(myself, opponent, concurrency);
    } else {
        let move_count = max(
            count_bits(possible_moves(myself, opponent)),
            count_bits(possible_moves(opponent, myself)),
        );
        return heuristic_search_parallel_with(
            myself,
            opponent,
            if move_count > 8 { 7 } else { 9 },
            concurrency,
        );
    }
}

/// ミニマックス戦略に基づいてゲーム木の探索をし、最良と思われる手のビット番号を返す
///
/// 打つ手がない場合は -1 を返す
///
/// ゲームの進行度によって部分探索と完全探索を自動で選択する
///
/// 切り替えのタイミングと、先読みの深さは数秒で結果が返るような値に調整されている
///
/// この関数は CPU スレッド数のスレッドによって並列処理される
///
/// 並列処理によって探索にかかる時間が短くなるので非並列版よりも深く読むようにしている
#[no_mangle]
pub extern "C" fn choose_move_parallel(myself: u64, opponent: u64) -> i32 {
    let cpu_count = num_cpus::get() as i32;
    choose_move_parallel_with(myself, opponent, cpu_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_conversion_test() {
        // 位置座標からビット番号への変換テスト
        assert_eq!(0, position_to_index(0, 0));
        assert_eq!(1, position_to_index(1, 0));
        assert_eq!(7, position_to_index(7, 0));
        assert_eq!(8, position_to_index(0, 1));
        assert_eq!(20, position_to_index(4, 2));
        assert_eq!(29, position_to_index(5, 3));
        assert_eq!(46, position_to_index(6, 5));
        assert_eq!(63, position_to_index(7, 7));
        // ビット番号からi座標を取り出すテスト
        assert_eq!(0, index_to_position_i(0));
        assert_eq!(1, index_to_position_i(1));
        assert_eq!(7, index_to_position_i(7));
        assert_eq!(0, index_to_position_i(8));
        assert_eq!(5, index_to_position_i(21));
        assert_eq!(3, index_to_position_i(27));
        assert_eq!(4, index_to_position_i(44));
        assert_eq!(7, index_to_position_i(63));
        // ビット番号からj座標を取り出すテスト
        assert_eq!(0, index_to_position_j(0));
        assert_eq!(0, index_to_position_j(7));
        assert_eq!(1, index_to_position_j(8));
        assert_eq!(2, index_to_position_j(16));
        assert_eq!(3, index_to_position_j(30));
        assert_eq!(4, index_to_position_j(39));
        assert_eq!(6, index_to_position_j(55));
        assert_eq!(7, index_to_position_j(63));
    }

    #[test]
    fn bit_count_test() {
        assert_eq!(0, count_bits(0x0000_0000_0000_0000));
        assert_eq!(1, count_bits(0x0000_0000_0000_0001));
        assert_eq!(1, count_bits(0x1000_0000_0000_0000));
        assert_eq!(14, count_bits(0x0A00_C0D0_1E00_0430));
        assert_eq!(15, count_bits(0x3004_0500_00BD_7008));
        assert_eq!(32, count_bits(0x0123_4567_89AB_CDEF));
    }

    #[test]
    fn possible_move_test() {
        // 普通に置けるとき
        assert_eq!(
            0x0000_0804_2010_0000,
            possible_moves(0x0000_0010_0800_0000, 0x0000_0008_1000_0000)
        );
        assert_eq!(
            0x001C_0040_0026_8C00,
            possible_moves(0x0000_003C_0010_0000, 0x0000_0800_3C48_0000)
        );
        assert_eq!(
            0x000D_0442_4024_9C00,
            possible_moves(0x0000_0038_0412_0000, 0x0000_0A04_3848_0000)
        );
        assert_eq!(
            0x005F_4440_4004_FC00,
            possible_moves(0x0000_101E_1402_0000, 0x0000_2A20_2878_0000)
        );
        assert_eq!(
            0x004E_4441_4080_D800,
            possible_moves(0x0000_1018_0006_0000, 0x0000_2A26_3E78_0000)
        );
        assert_eq!(
            0x000D_8181_C181_851B,
            possible_moves(0x0042_3408_2020_2800, 0x0000_0A76_1E5E_1200)
        );
        assert_eq!(
            0x0000_0101_0101_81EF,
            possible_moves(0x0046_7C78_6000_0000, 0x0000_0206_1E7E_7E10)
        );
        assert_eq!(
            0xB021_0080_0100_000A,
            possible_moves(0x0046_CE0D_2643_F300, 0x0090_3172_D83C_0C14)
        );
        assert_eq!(
            0x0000_0020_0028_0000,
            possible_moves(0x0000_0008_0000_0000, 0x0000_0010_1810_0000)
        );
        assert_eq!(
            0x0000_3060_4004_0800,
            possible_moves(0x0000_0008_0830_0000, 0x0000_0010_3008_0400)
        );
        assert_eq!(
            0x0000_3820_0204_201E,
            possible_moves(0x0000_0008_7822_0000, 0x0000_0012_0418_1C00)
        );
        assert_eq!(
            0x0018_0501_0201_0200,
            possible_moves(0x0000_2A20_2078_0000, 0x0000_101E_1C06_0000)
        );
        assert_eq!(
            0x8038_4400_0001_0F00,
            possible_moves(0x0000_0A26_3E78_0000, 0x0040_3018_0006_0000)
        );
        assert_eq!(
            0xC7B9_8180_8000_0000,
            possible_moves(0x0000_0000_1E7E_7E10, 0x0046_7E7F_6000_0000)
        );
        assert_eq!(
            0xC7B9_8080_0181_00EF,
            possible_moves(0x0000_0102_DA3C_0010, 0x0046_7E7D_2442_FF00)
        );
        assert_eq!(
            0x4709_0000_0180_00E2,
            possible_moves(0x0010_3172_D034_0215, 0x80C6_CE0D_2E4B_FD08)
        );
        // パスが発生するとき（どこにも置けないとき）
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x0000_7E46_4242_DE02, 0x0000_01B9_3D3D_21FD)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x1236_3232_FC3E_0000, 0x0D09_0D0D_0301_0101)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x0026_3232_FE3E_0202, 0xFF19_0D0D_0101_0101)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x0046_4674_4C4C_7050, 0xFFB9_B98B_B3B3_8F8F)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x487E_4260_4854_7040, 0x8781_BD9F_B7AB_8FBF)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x767E_5B1F_4F17_3300, 0x8180_A4E0_B0E8_CCFF)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x0066_5317_4717_3300, 0xFF98_ACE8_B8E8_CCFF)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x0014_7E00_5E6E_7C00, 0x8181_81FF_A191_0004)
        );
        // 盤が埋まっているとき
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x80FE_CEA2_D2FA_C2BF, 0x7F01_315D_2D05_3D40)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x009F_0264_5010_7800, 0xFF60_FD9B_AFEF_87FF)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0x7F01_315D_2D05_3D40, 0x80FE_CEA2_D2FA_C2BF)
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            possible_moves(0xFF60_FD9B_AFEF_87FF, 0x009F_0264_5010_7800)
        );
    }

    #[test]
    fn turnover_test() {
        // 普通に置くとき
        assert_eq!(
            0x0000_0000_1000_0000,
            turnovers(
                0x0000_0010_0800_0000,
                0x0000_0008_1000_0000,
                position_to_index(5, 3)
            )
        );
        assert_eq!(
            0x0000_0000_0C08_0000,
            turnovers(
                0x0000_003C_0010_0000,
                0x0000_0800_3C48_0000,
                position_to_index(2, 2)
            )
        );
        assert_eq!(
            0x0000_0004_0000_0000,
            turnovers(
                0x0000_0038_0412_0000,
                0x0000_0A04_3848_0000,
                position_to_index(2, 5)
            )
        );
        assert_eq!(
            0x0000_0020_2000_0000,
            turnovers(
                0x0000_101E_1402_0000,
                0x0000_2A20_2878_0000,
                position_to_index(6, 3)
            )
        );
        assert_eq!(
            0x0000_2000_0000_0000,
            turnovers(
                0x0000_1018_0006_0000,
                0x0000_2A26_3E78_0000,
                position_to_index(6, 6)
            )
        );
        assert_eq!(
            0x0000_0006_0204_0000,
            turnovers(
                0x0042_3408_2020_2800,
                0x0000_0A76_1E5E_1200,
                position_to_index(0, 4)
            )
        );
        assert_eq!(
            0x0000_0000_0804_0200,
            turnovers(
                0x0046_7C78_6000_0000,
                0x0000_0206_1E7E_7E10,
                position_to_index(0, 0)
            )
        );
        assert_eq!(
            0x0000_0070_4020_0000,
            turnovers(
                0x0046_CE0D_2643_F300,
                0x0090_3172_D83C_0C14,
                position_to_index(7, 4)
            )
        );
        assert_eq!(
            0x0000_0000_1000_0000,
            turnovers(
                0x0000_0008_0000_0000,
                0x0000_0010_1810_0000,
                position_to_index(5, 2)
            )
        );
        assert_eq!(
            0x0000_0010_2000_0000,
            turnovers(
                0x0000_0008_0830_0000,
                0x0000_0010_3008_0400,
                position_to_index(5, 4)
            )
        );
        assert_eq!(
            0x0000_0000_0018_0000,
            turnovers(
                0x0000_0008_7822_0000,
                0x0000_0012_0418_1C00,
                position_to_index(2, 2)
            )
        );
        assert_eq!(
            0x0000_0008_1000_0000,
            turnovers(
                0x0000_2A20_2078_0000,
                0x0000_101E_1C06_0000,
                position_to_index(2, 5)
            )
        );
        assert_eq!(
            0x0040_2010_0000_0000,
            turnovers(
                0x0000_0A26_3E78_0000,
                0x0040_3018_0006_0000,
                position_to_index(7, 7)
            )
        );
        assert_eq!(
            0x0006_0A12_2000_0000,
            turnovers(
                0x0000_0000_1E7E_7E10,
                0x0046_7E7F_6000_0000,
                position_to_index(1, 7)
            )
        );
        assert_eq!(
            0x0000_7E40_2000_0000,
            turnovers(
                0x0000_0102_DA3C_0010,
                0x0046_7E7D_2442_FF00,
                position_to_index(7, 5)
            )
        );
        assert_eq!(
            0x0000_0001_0E01_0100,
            turnovers(
                0x0010_3172_D034_0215,
                0x80C6_CE0D_2E4B_FD08,
                position_to_index(0, 3)
            )
        );
        // 1つも返せない位置に置くとき（本来は置けない位置に置く）
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x487E_4260_4854_7040,
                0x8781_BD9F_B7AB_8FBF,
                position_to_index(5, 7)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x767E_5B1F_4F17_3300,
                0x8180_A4E0_B0E8_CCFF,
                position_to_index(3, 7)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0066_5317_4717_3300,
                0xFF98_ACE8_B8E8_CCFF,
                position_to_index(0, 6)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0014_7E00_5E6E_7C00,
                0x8181_81FF_A191_0004,
                position_to_index(3, 6)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0000_7E46_4242_DE02,
                0x0000_01B9_3D3D_21FD,
                position_to_index(0, 7)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x1236_3232_FC3E_0000,
                0x0D09_0D0D_0301_0101,
                position_to_index(6, 1)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0026_3232_FE3E_0202,
                0xFF19_0D0D_0101_0101,
                position_to_index(2, 0)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0046_4674_4C4C_7050,
                0xFFB9_B98B_B3B3_8F8F,
                position_to_index(5, 0)
            )
        );
        // すでに石で埋まっている位置に置くとき（本来は置けない位置に置く）
        assert_eq!(
            0x0000_0000_0000_023E,
            turnovers(
                0x487E_4260_4854_7040,
                0x8781_BD9F_B7AB_8FBF,
                position_to_index(0, 0)
            )
        );
        assert_eq!(
            0x0000_0428_2800_0000,
            turnovers(
                0x0066_5317_4717_3300,
                0xFF98_ACE8_B8E8_CCFF,
                position_to_index(4, 3)
            )
        );
        assert_eq!(
            0x0000_0038_2C38_0000,
            turnovers(
                0x0000_7E46_4242_DE02,
                0x0000_01B9_3D3D_21FD,
                position_to_index(4, 3)
            )
        );
        assert_eq!(
            0x0018_0808_0000_0000,
            turnovers(
                0x0026_3232_FE3E_0202,
                0xFF19_0D0D_0101_0101,
                position_to_index(3, 7)
            )
        );
        // 1つも返せない且つすでに石で埋まっている位置に置く（本来は置けない位置に置く）
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x767E_5B1F_4F17_3300,
                0x8180_A4E0_B0E8_CCFF,
                position_to_index(2, 5)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0014_7E00_5E6E_7C00,
                0x8181_81FF_A191_0004,
                position_to_index(6, 1)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x1236_3232_FC3E_0000,
                0x0D09_0D0D_0301_0101,
                position_to_index(5, 2)
            )
        );
        assert_eq!(
            0x0000_0000_0000_0000,
            turnovers(
                0x0046_4674_4C4C_7050,
                0xFFB9_B98B_B3B3_8F8F,
                position_to_index(7, 0)
            )
        );
    }

    #[test]
    fn board_update_test() {
        let self_board = 0x00F8_C687_E7AB_C0E4;
        let opponent_board = 0xFE04_3878_1854_3A18;
        let mut self_next: u64 = 0;
        let mut opponent_next: u64 = 0;
        let turns = place(
            self_board,
            opponent_board,
            position_to_index(0, 0),
            &mut self_next,
            &mut opponent_next,
        );
        assert_eq!(0x00F8_E697_EFAF_C2E5, self_next);
        assert_eq!(0xFE04_1868_1050_3818, opponent_next);
        assert_eq!(0x0000_2010_0804_0200, turns);

        let self_board = 0x4000_0810_2C44_6073;
        let opponent_board = 0xBCFD_F7EF_D3BB_9F8C;
        let mut self_next: u64 = 0;
        let mut opponent_next: u64 = 0;
        let turns = place(
            self_board,
            opponent_board,
            position_to_index(0, 7),
            &mut self_next,
            &mut opponent_next,
        );
        assert_eq!(0x4101_0911_2D45_6173, self_next);
        assert_eq!(0xBCFC_F6EE_D2BA_9E8C, opponent_next);
        assert_eq!(0x0001_0101_0101_0100, turns);
    }

    #[test]
    fn weight_test() {
        assert_eq!(0, sum_of_weights(0x0000_0000_0000_0000));
        assert_eq!(20000, sum_of_weights(0x8000_0000_0000_0001));
        assert_eq!(100, sum_of_weights(0x0000_0010_0800_0000));
    }

    #[test]
    fn openness_test() {
        assert_eq!(
            2,
            openness(
                0x0000_0008_0828_0000,
                0x0000_0010_3010_1000,
                0x0000_0000_0010_0000
            )
        );
        assert_eq!(
            5,
            openness(
                0x0000_0008_0828_0000,
                0x0000_0010_3010_1000,
                0x0000_0000_3000_0000
            )
        );
    }

    #[test]
    fn full_search_test() {
        assert_eq!(
            position_to_index(0, 7),
            full_search(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C)
        );
        assert_eq!(
            position_to_index(1, 1),
            full_search(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4)
        );
        assert_eq!(
            position_to_index(0, 7),
            full_search(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F)
        );
        assert_eq!(
            position_to_index(1, 6),
            full_search(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E)
        );
        assert_eq!(
            position_to_index(0, 7),
            full_search_parallel(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C)
        );
        assert_eq!(
            position_to_index(1, 1),
            full_search_parallel(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4)
        );
        assert_eq!(
            position_to_index(0, 7),
            full_search_parallel(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F)
        );
        assert_eq!(
            position_to_index(1, 6),
            full_search_parallel(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E)
        );
    }

    #[test]
    fn heuristic_search_test() {
        assert_eq!(
            position_to_index(4, 0),
            heuristic_search(0x0000_0000_0010_0804, 0x0000_1038_7E6C_3020, 9)
        );
        assert_eq!(
            position_to_index(4, 0),
            heuristic_search_parallel(0x0000_0000_0010_0804, 0x0000_1038_7E6C_3020, 9)
        );
        assert_eq!(
            position_to_index(4, 0),
            heuristic_search_parallel_with(0x0000_0000_0010_0804, 0x0000_1038_7E6C_3020, 9, 1)
        );
        assert_eq!(
            position_to_index(4, 0),
            heuristic_search_parallel_with(0x0000_0000_0010_0804, 0x0000_1038_7E6C_3020, 9, 2)
        );
        assert_eq!(
            heuristic_search(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C, 7),
            heuristic_search_parallel(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C, 7)
        );
        assert_eq!(
            heuristic_search(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4, 7),
            heuristic_search_parallel(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4, 7)
        );
        assert_eq!(
            heuristic_search(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F, 7),
            heuristic_search_parallel(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F, 7)
        );
        assert_eq!(
            heuristic_search(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E, 7),
            heuristic_search_parallel(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E, 7)
        );
        assert_eq!(
            heuristic_search(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C, 9),
            heuristic_search_parallel(0x4000_0810_2C44_6073, 0xBCFD_F7EF_D3BB_9F8C, 9)
        );
        assert_eq!(
            heuristic_search(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4, 9),
            heuristic_search_parallel(0xFE04_3878_1850_3818, 0x00F8_C687_E7AF_C0E4, 9)
        );
        assert_eq!(
            heuristic_search(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F, 9),
            heuristic_search_parallel(0x8080_908F_B388_9C80, 0x7E7C_6F70_4C77_637F, 9)
        );
        assert_eq!(
            heuristic_search(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E, 9),
            heuristic_search_parallel(0x0010_6341_6D29_0721, 0xBCAC_9CBE_92D6_381E, 9)
        );
    }
}
