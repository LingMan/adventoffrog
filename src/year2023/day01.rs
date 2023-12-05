use crate::Puzzle;

use anyhow::Result;

pub struct Day01<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Puzzle<'a> for Day01<'a> {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &'a str) -> Result<Self> {
        let lines = input.lines().collect();
        Ok(Self { lines })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        self.lines
            .iter()
            .map(|line| {
                let mut digits = line.bytes().filter(|c| c.is_ascii_digit());

                let first_digit = digits.next().map(|digit| digit - b'0').unwrap_or_default();

                let last_digit = digits
                    .next_back()
                    .map(|digit| digit - b'0')
                    .unwrap_or(first_digit);

                Self::Sol1Type::from(first_digit * 10 + last_digit)
            })
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        trait Dfa {
            fn advance(&mut self, c: u8) -> Option<u8>;
        }

        macro_rules! next {
            ($c:ident, $final_src:literal => Some($final_dest:literal), $($src:literal => $dest:expr),+) => {
                match $c {
                    $final_src => return Some($final_dest),
                    $($src => $dest,)+
                    _ => S,
                }
            };
            ($c:ident, $($src:literal => $dest:expr),+) => {
                match $c {
                   $($src => $dest,)+
                    _ => S,
                }
            };
        }

        enum DfaOneState {
            S,
            O_,
            ON_,
        }

        enum DfaTwoState {
            S,
            T_,
            TW_,
        }

        enum DfaThreeState {
            S,
            T_,
            TH_,
            THR_,
            THRE_,
        }

        enum DfaFourState {
            S,
            F_,
            FO_,
            FOU_,
        }

        enum DfaFiveState {
            S,
            F_,
            FI_,
            FIV_,
        }

        enum DfaSixState {
            S,
            S_,
            SI_,
        }

        enum DfaSevenState {
            S,
            S_,
            SE_,
            SEV_,
            SEVE_,
        }

        enum DfaEightState {
            S,
            E_,
            EI_,
            EIG_,
            EIGH_,
        }

        enum DfaNineState {
            S,
            N_,
            NI_,
            NIN_,
        }

        struct DfaDigit();

        struct DfaOne(DfaOneState);
        struct DfaTwo(DfaTwoState);
        struct DfaThree(DfaThreeState);
        struct DfaFour(DfaFourState);
        struct DfaFive(DfaFiveState);
        struct DfaSix(DfaSixState);
        struct DfaSeven(DfaSevenState);
        struct DfaEight(DfaEightState);
        struct DfaNine(DfaNineState);

        struct DfaOneBack(DfaOneState);
        struct DfaTwoBack(DfaTwoState);
        struct DfaThreeBack(DfaThreeState);
        struct DfaFourBack(DfaFourState);
        struct DfaFiveBack(DfaFiveState);
        struct DfaSixBack(DfaSixState);
        struct DfaSevenBack(DfaSevenState);
        struct DfaEightBack(DfaEightState);
        struct DfaNineBack(DfaNineState);

        impl Dfa for DfaDigit {
            fn advance(&mut self, c: u8) -> Option<u8> {
                match c {
                    b'0'..=b'9' => Some(c - b'0'),
                    _ => None,
                }
            }
        }

        impl Dfa for DfaOne {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaOneState::*;
                self.0 = match self.0 {
                    S => next!(c, b'o' => O_),
                    O_ => next!(c, b'n' => ON_, b'o' => O_),
                    ON_ => next!(c, b'e' => Some(1), b'o' => O_),
                };
                None
            }
        }

        impl Dfa for DfaOneBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaOneState::*;
                self.0 = match self.0 {
                    S => next!(c, b'e' => ON_),
                    ON_ => next!(c, b'n' => O_, b'e' => ON_),
                    O_ => next!(c, b'o' => Some(1), b'e' => ON_),
                };
                None
            }
        }

        impl Dfa for DfaTwo {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaTwoState::*;
                self.0 = match self.0 {
                    S => next!(c, b't' => T_),
                    T_ => next!(c, b'w' => TW_, b't' => T_),
                    TW_ => next!(c, b'o' => Some(2), b't' => T_),
                };
                None
            }
        }

        impl Dfa for DfaTwoBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaTwoState::*;
                self.0 = match self.0 {
                    S => next!(c, b'o' => TW_),
                    TW_ => next!(c, b'w' => T_, b'o' => TW_),
                    T_ => next!(c, b't' => Some(2), b'o' => TW_),
                };
                None
            }
        }

        impl Dfa for DfaThree {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaThreeState::*;
                self.0 = match self.0 {
                    S => next!(c, b't' => T_),
                    T_ => next!(c, b'h' => TH_, b't' => T_),
                    TH_ => next!(c, b'r' => THR_, b't' => T_),
                    THR_ => next!(c, b'e' => THRE_, b't' => T_),
                    THRE_ => next!(c, b'e' => Some(3), b't' => T_),
                };
                None
            }
        }

        impl Dfa for DfaThreeBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaThreeState::*;
                self.0 = match self.0 {
                    S => next!(c, b'e' => THRE_),
                    THRE_ => next!(c, b'e' => THR_),
                    THR_ => next!(c, b'r' => TH_, b'e' => THRE_),
                    TH_ => next!(c, b'h' => T_, b'e' => THRE_),
                    T_ => next!(c, b't' => Some(3), b'e' => THRE_),
                };
                None
            }
        }

        impl Dfa for DfaFour {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaFourState::*;
                self.0 = match self.0 {
                    S => next!(c, b'f' => F_),
                    F_ => next!(c, b'o' => FO_, b'f' => F_),
                    FO_ => next!(c, b'u' => FOU_, b'f' => F_),
                    FOU_ => next!(c, b'r' => Some(4), b'f' => F_),
                };
                None
            }
        }

        impl Dfa for DfaFourBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaFourState::*;
                self.0 = match self.0 {
                    S => next!(c, b'r' => FOU_),
                    FOU_ => next!(c, b'u' => FO_, b'r' => FOU_),
                    FO_ => next!(c, b'o' => F_, b'r' => FOU_),
                    F_ => next!(c, b'f' => Some(4), b'r' => FOU_),
                };
                None
            }
        }

        impl Dfa for DfaFive {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaFiveState::*;
                self.0 = match self.0 {
                    S => next!(c, b'f' => F_),
                    F_ => next!(c, b'i' => FI_, b'f' => F_),
                    FI_ => next!(c, b'v' => FIV_, b'f' => F_),
                    FIV_ => next!(c, b'e' => Some(5), b'f' => F_),
                };
                None
            }
        }

        impl Dfa for DfaFiveBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaFiveState::*;
                self.0 = match self.0 {
                    S => next!(c, b'e' => FIV_),
                    FIV_ => next!(c, b'v' => FI_, b'e' => FIV_),
                    FI_ => next!(c, b'i' => F_, b'e' => F_),
                    F_ => next!(c, b'f' => Some(5), b'e' => F_),
                };
                None
            }
        }

        impl Dfa for DfaSix {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaSixState::*;
                self.0 = match self.0 {
                    S => next!(c, b's' => S_),
                    S_ => next!(c, b'i' => SI_, b's' => S_),
                    SI_ => next!(c, b'x' => Some(6), b's' => S_),
                };
                None
            }
        }

        impl Dfa for DfaSixBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaSixState::*;
                self.0 = match self.0 {
                    S => next!(c, b'x' => SI_),
                    SI_ => next!(c, b'i' => S_, b'x' => SI_),
                    S_ => next!(c, b's' => Some(6), b'x' => SI_),
                };
                None
            }
        }

        impl Dfa for DfaSeven {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaSevenState::*;
                self.0 = match self.0 {
                    S => next!(c, b's' => S_),
                    S_ => next!(c, b'e' => SE_, b's' => S_),
                    SE_ => next!(c, b'v' => SEV_, b's' => S_),
                    SEV_ => next!(c, b'e' => SEVE_, b's' => S_),
                    SEVE_ => next!(c, b'n' => Some(7), b's' => S_),
                };
                None
            }
        }

        impl Dfa for DfaSevenBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaSevenState::*;
                self.0 = match self.0 {
                    S => next!(c, b'n' => SEVE_),
                    SEVE_ => next!(c, b'e' => SEV_, b'n' => SEVE_),
                    SEV_ => next!(c, b'v' => SE_, b'n' => SEVE_),
                    SE_ => next!(c, b'e' => S_, b'n' => SEVE_),
                    S_ => next!(c, b's' => Some(7), b'n' => SEVE_),
                };
                None
            }
        }

        impl Dfa for DfaEight {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaEightState::*;
                self.0 = match self.0 {
                    S => next!(c, b'e' => E_),
                    E_ => next!(c, b'i' => EI_, b'e' => E_),
                    EI_ => next!(c, b'g' => EIG_, b'e' => E_),
                    EIG_ => next!(c, b'h' => EIGH_, b'e' => E_),
                    EIGH_ => next!(c, b't' => Some(8), b'e' => E_),
                };
                None
            }
        }

        impl Dfa for DfaEightBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaEightState::*;
                self.0 = match self.0 {
                    S => next!(c, b't' => EIGH_),
                    EIGH_ => next!(c, b'h' => EIG_, b't' => EIGH_),
                    EIG_ => next!(c, b'g' => EI_, b't' => EIGH_),
                    EI_ => next!(c, b'i' => E_, b't' => EIGH_),
                    E_ => next!(c, b'e' => Some(8), b't' => EIGH_),
                };
                None
            }
        }

        impl Dfa for DfaNine {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaNineState::*;
                self.0 = match self.0 {
                    S => next!(c, b'n' => N_),
                    N_ => next!(c, b'i' => NI_, b'n' => N_),
                    NI_ => next!(c, b'n' => NIN_),
                    NIN_ => next!(c, b'e' => Some(9), b'n' => N_),
                };
                None
            }
        }

        impl Dfa for DfaNineBack {
            fn advance(&mut self, c: u8) -> Option<u8> {
                use DfaNineState::*;
                self.0 = match self.0 {
                    S => next!(c, b'e' => NIN_),
                    NIN_ => next!(c, b'n' => NI_, b'e' => NIN_),
                    NI_ => next!(c, b'i' => N_, b'e' => NIN_),
                    N_ => next!(c, b'n' => Some(9), b'e' => NIN_),
                };
                None
            }
        }

        self.lines
            .iter()
            .map(|line| {
                let mut automata: [&mut dyn Dfa; 10] = [
                    &mut DfaDigit(),
                    &mut DfaOne(DfaOneState::S),
                    &mut DfaTwo(DfaTwoState::S),
                    &mut DfaThree(DfaThreeState::S),
                    &mut DfaFour(DfaFourState::S),
                    &mut DfaFive(DfaFiveState::S),
                    &mut DfaSix(DfaSixState::S),
                    &mut DfaSeven(DfaSevenState::S),
                    &mut DfaEight(DfaEightState::S),
                    &mut DfaNine(DfaNineState::S),
                ];
                let mut automata_back: [&mut dyn Dfa; 10] = [
                    &mut DfaDigit(),
                    &mut DfaOneBack(DfaOneState::S),
                    &mut DfaTwoBack(DfaTwoState::S),
                    &mut DfaThreeBack(DfaThreeState::S),
                    &mut DfaFourBack(DfaFourState::S),
                    &mut DfaFiveBack(DfaFiveState::S),
                    &mut DfaSixBack(DfaSixState::S),
                    &mut DfaSevenBack(DfaSevenState::S),
                    &mut DfaEightBack(DfaEightState::S),
                    &mut DfaNineBack(DfaNineState::S),
                ];

                let mut first_digit = 0;
                for c in line.bytes() {
                    if let Some(n) = automata.iter_mut().find_map(|a| a.advance(c)) {
                        first_digit = n;
                        break;
                    }
                }

                let mut last_digit = 0;
                for c in line.bytes().rev() {
                    if let Some(n) = automata_back.iter_mut().find_map(|a| a.advance(c)) {
                        last_digit = n;
                        break;
                    }
                }

                Self::Sol2Type::from(first_digit * 10 + last_digit)
            })
            .sum()
    }
}
