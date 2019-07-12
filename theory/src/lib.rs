use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::ops;
use strum_macros::Display;

#[derive(Clone, Copy, Debug, Display)]
pub enum PitchBase {
    #[strum(serialize="C")]
    C,
    #[strum(serialize="D")]
    D,
    #[strum(serialize="E")]
    E,
    #[strum(serialize="F")]
    F,
    #[strum(serialize="G")]
    G,
    #[strum(serialize="A")]
    A,
    #[strum(serialize="B")]
    B,
}

#[derive(Clone, Copy, Debug, Display)]
pub enum PitchModifier {
    #[strum(serialize="𝄫")]
    DoubleFlat,
    #[strum(serialize="♭")]
    Flat,
    #[strum(serialize="")]
    Natural,
    #[strum(serialize="♯")]
    Sharp,
    #[strum(serialize="𝄪")]
    DoubleSharp,
}

#[derive(Clone, Copy, Debug)]
pub struct Note(pub PitchBase, pub PitchModifier);

impl Note {
    pub fn semitones_from_c(&self) -> i8 {
        let base = match self.0 {
            PitchBase::C => 0,
            PitchBase::D => 2,
            PitchBase::E => 4,
            PitchBase::F => 5,
            PitchBase::G => 7,
            PitchBase::A => 9,
            PitchBase::B => 11,
        };
        let modifier = match self.1 {
            PitchModifier::DoubleFlat => -2,
            PitchModifier::Flat => -1,
            PitchModifier::Natural => 0,
            PitchModifier::Sharp => 1,
            PitchModifier::DoubleSharp => 2,
        };
        base + modifier
    }

    /// Gets a note from the semitones above C. The notes are spelled using sharps.
    pub fn from_semitones_from_c(semitones: i8) -> Self {
        let semitones = if semitones < 0 { semitones + 12 } else { semitones };
        let semitones = semitones % 12;
        match semitones {
            0 => Note(PitchBase::C, PitchModifier::Natural),
            1 => Note(PitchBase::C, PitchModifier::Sharp),
            2 => Note(PitchBase::D, PitchModifier::Natural),
            3 => Note(PitchBase::D, PitchModifier::Sharp),
            4 => Note(PitchBase::E, PitchModifier::Natural),
            5 => Note(PitchBase::F, PitchModifier::Natural),
            6 => Note(PitchBase::F, PitchModifier::Sharp),
            7 => Note(PitchBase::G, PitchModifier::Natural),
            8 => Note(PitchBase::G, PitchModifier::Sharp),
            9 => Note(PitchBase::A, PitchModifier::Natural),
            10 => Note(PitchBase::A, PitchModifier::Sharp),
            11 => Note(PitchBase::B, PitchModifier::Natural),
            _ => unreachable!()
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl PartialEq for Note {
    fn eq(&self, other: &Note) -> bool {
        self.semitones_from_c() == other.semitones_from_c()
    }
}

impl Eq for Note {}

#[derive(Clone, Copy, Debug)]
/// Pitch base, pitch modifier, and octave. For example, A♭3 would be `Pitch(PitchBase::A, PitchModifier::Flat, 3)`
pub struct Pitch(pub Note, pub i8);

impl Pitch {
    pub fn semitones_from_middle_c(&self) -> i8 {
        let octave_difference = (self.1 - 4) * 12;
        self.0.semitones_from_c() + octave_difference
    }
    pub fn from_semitones_from_middle_c(semitones: i8) -> Self {
        let mut octave_difference = 0;
        let mut semitones = semitones;
        while semitones < 0 {
            semitones += 12;
            octave_difference -= 1;
        }
        while semitones > 12 {
            semitones -= 12;
            octave_difference += 1;
        }
        Pitch(Note::from_semitones_from_c(semitones), 4 + octave_difference)
    }
}

impl fmt::Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        self.semitones_from_middle_c() == other.semitones_from_middle_c()
    }
}

impl Eq for Pitch {}

impl PartialOrd for Pitch {
    fn partial_cmp(&self, other: &Pitch) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pitch {
    fn cmp(&self, other: &Pitch) -> std::cmp::Ordering {
        self.semitones_from_middle_c().cmp(&other.semitones_from_middle_c())
    }
}

// TODO: Enharmonic intervals
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Ord, PartialOrd)]
pub enum Interval {
    #[strum(serialize="unison")]
    Unison,
    #[strum(serialize="minor second")]
    MinorSecond,
    #[strum(serialize="major second")]
    MajorSecond,
    #[strum(serialize="minor third")]
    MinorThird,
    #[strum(serialize="major third")]
    MajorThird,
    #[strum(serialize="perfect fourth")]
    PerfectFourth,
    #[strum(serialize="tritone")]
    Tritone,
    #[strum(serialize="perfect fifth")]
    PerfectFifth,
    #[strum(serialize="minor sixth")]
    MinorSixth,
    #[strum(serialize="major sixth")]
    MajorSixth,
    #[strum(serialize="minor seventh")]
    MinorSeventh,
    #[strum(serialize="major seventh")]
    MajorSeventh,
}

impl Interval {
    pub fn from_semitones(semitones: u8) -> Self {
        let semitones = semitones % 12;

        match semitones {
            0 => Interval::Unison,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::Tritone,
            7 => Interval::PerfectFifth,
            8 => Interval::MinorSixth,
            9 => Interval::MajorSixth,
            10 => Interval::MinorSeventh,
            11 => Interval::MajorSeventh,
            _ => unreachable!()
        }
    }

    pub fn semitones(&self) -> u8 {
        match *self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::Tritone => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
        }
    }

    pub fn inverse(&self) -> Self {
        let semitones = self.semitones();
        Self::from_semitones(12 - semitones)
    }
}

impl ops::Add<i8> for Pitch {
    type Output = Pitch;
    fn add(self, other: i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other)
    }
}
impl ops::Add<i8> for &Pitch {
    type Output = Pitch;
    fn add(self, other: i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other)
    }
}
impl ops::Add<&i8> for Pitch {
    type Output = Pitch;
    fn add(self, other: &i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other)
    }
}
impl ops::Add<&i8> for &Pitch {
    type Output = Pitch;
    fn add(self, other: &i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other)
    }
}

impl ops::Sub<i8> for Pitch {
    type Output = Pitch;
    fn sub(self, other: i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other)
    }
}
impl ops::Sub<i8> for &Pitch {
    type Output = Pitch;
    fn sub(self, other: i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other)
    }
}
impl ops::Sub<&i8> for Pitch {
    type Output = Pitch;
    fn sub(self, other: &i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other)
    }
}
impl ops::Sub<&i8> for &Pitch {
    type Output = Pitch;
    fn sub(self, other: &i8) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other)
    }
}

impl ops::Add for Interval {
    type Output = Interval;
    fn add(self, other: Interval) -> Self::Output {
        let semitones = self.semitones() + other.semitones();
        Interval::from_semitones(semitones)
    }
}
impl ops::Add<&Interval> for Interval {
    type Output = Interval;
    fn add(self, other: &Interval) -> Self::Output {
        let semitones = self.semitones() + other.semitones();
        Interval::from_semitones(semitones)
    }
}
impl ops::Add<Interval> for &Interval {
    type Output = Interval;
    fn add(self, other: Interval) -> Self::Output {
        let semitones = self.semitones() + other.semitones();
        Interval::from_semitones(semitones)
    }
}
impl ops::Add<&Interval> for &Interval {
    type Output = Interval;
    fn add(self, other: &Interval) -> Self::Output {
        let semitones = self.semitones() + other.semitones();
        Interval::from_semitones(semitones)
    }
}

impl ops::Add<Pitch> for Interval {
    type Output = Pitch;
    fn add(self, other: Pitch) -> Self::Output {
        Pitch::from_semitones_from_middle_c(other.semitones_from_middle_c() + self.semitones() as i8)
    }
}
impl ops::Add<&Pitch> for Interval {
    type Output = Pitch;
    fn add(self, other: &Pitch) -> Self::Output {
        Pitch::from_semitones_from_middle_c(other.semitones_from_middle_c() + self.semitones() as i8)
    }
}
impl ops::Add<Pitch> for &Interval {
    type Output = Pitch;
    fn add(self, other: Pitch) -> Self::Output {
        Pitch::from_semitones_from_middle_c(other.semitones_from_middle_c() + self.semitones() as i8)
    }
}
impl ops::Add<&Pitch> for &Interval {
    type Output = Pitch;
    fn add(self, other: &Pitch) -> Self::Output {
        Pitch::from_semitones_from_middle_c(other.semitones_from_middle_c() + self.semitones() as i8)
    }
}

impl ops::Add<Interval> for Pitch {
    type Output = Pitch;
    fn add(self, other: Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other.semitones() as i8)
    }
}
impl ops::Add<&Interval> for Pitch {
    type Output = Pitch;
    fn add(self, other: &Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other.semitones() as i8)
    }
}
impl ops::Add<Interval> for &Pitch {
    type Output = Pitch;
    fn add(self, other: Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other.semitones() as i8)
    }
}
impl ops::Add<&Interval> for &Pitch {
    type Output = Pitch;
    fn add(self, other: &Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() + other.semitones() as i8)
    }
}

impl ops::Sub<Interval> for Pitch {
    type Output = Pitch;
    fn sub(self, other: Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other.semitones() as i8)
    }
}
impl ops::Sub<&Interval> for Pitch {
    type Output = Pitch;
    fn sub(self, other: &Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other.semitones() as i8)
    }
}
impl ops::Sub<Interval> for &Pitch {
    type Output = Pitch;
    fn sub(self, other: Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other.semitones() as i8)
    }
}
impl ops::Sub<&Interval> for &Pitch {
    type Output = Pitch;
    fn sub(self, other: &Interval) -> Self::Output {
        Pitch::from_semitones_from_middle_c(self.semitones_from_middle_c() - other.semitones() as i8)
    }
}

impl ops::Add<Note> for Interval {
    type Output = Note;
    fn add(self, other: Note) -> Self::Output {
        Note::from_semitones_from_c(other.semitones_from_c() + self.semitones() as i8)
    }
}
impl ops::Add<&Note> for Interval {
    type Output = Note;
    fn add(self, other: &Note) -> Self::Output {
        Note::from_semitones_from_c(other.semitones_from_c() + self.semitones() as i8)
    }
}
impl ops::Add<Note> for &Interval {
    type Output = Note;
    fn add(self, other: Note) -> Self::Output {
        Note::from_semitones_from_c(other.semitones_from_c() + self.semitones() as i8)
    }
}
impl ops::Add<&Note> for &Interval {
    type Output = Note;
    fn add(self, other: &Note) -> Self::Output {
        Note::from_semitones_from_c(other.semitones_from_c() + self.semitones() as i8)
    }
}

impl ops::Add<Interval> for Note {
    type Output = Note;
    fn add(self, other: Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() + other.semitones() as i8)
    }
}
impl ops::Add<&Interval> for Note {
    type Output = Note;
    fn add(self, other: &Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() + other.semitones() as i8)
    }
}
impl ops::Add<Interval> for &Note {
    type Output = Note;
    fn add(self, other: Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() + other.semitones() as i8)
    }
}
impl ops::Add<&Interval> for &Note {
    type Output = Note;
    fn add(self, other: &Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() + other.semitones() as i8)
    }
}

impl ops::Sub<Interval> for Note {
    type Output = Note;
    fn sub(self, other: Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() - other.semitones() as i8)
    }
}
impl ops::Sub<&Interval> for Note {
    type Output = Note;
    fn sub(self, other: &Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() - other.semitones() as i8)
    }
}
impl ops::Sub<Interval> for &Note {
    type Output = Note;
    fn sub(self, other: Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() - other.semitones() as i8)
    }
}
impl ops::Sub<&Interval> for &Note {
    type Output = Note;
    fn sub(self, other: &Interval) -> Self::Output {
        Note::from_semitones_from_c(self.semitones_from_c() - other.semitones() as i8)
    }
}

impl ops::Sub<Pitch> for Pitch {
    type Output = Interval;
    fn sub(self, other: Pitch) -> Self::Output {
        let (bottom, top) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        let semitones = top.semitones_from_middle_c() - bottom.semitones_from_middle_c();
        Interval::from_semitones(semitones as u8)
    }
}
impl ops::Sub<&Pitch> for Pitch {
    type Output = Interval;
    fn sub(self, other: &Pitch) -> Self::Output {
        let (bottom, top) = if &self < other {
            (&self, other)
        } else {
            (other, &self)
        };
        let semitones = top.semitones_from_middle_c() - bottom.semitones_from_middle_c();
        Interval::from_semitones(semitones as u8)
    }
}
impl ops::Sub<Pitch> for &Pitch {
    type Output = Interval;
    fn sub(self, other: Pitch) -> Self::Output {
        let (bottom, top) = if self < &other {
            (self, &other)
        } else {
            (&other, self)
        };
        let semitones = top.semitones_from_middle_c() - bottom.semitones_from_middle_c();
        Interval::from_semitones(semitones as u8)
    }
}
impl ops::Sub<&Pitch> for &Pitch {
    type Output = Interval;
    fn sub(self, other: &Pitch) -> Self::Output {
        let (bottom, top) = if self < other {
            (self, other)
        } else {
            (other, self)
        };
        let semitones = top.semitones_from_middle_c() - bottom.semitones_from_middle_c();
        Interval::from_semitones(semitones as u8)
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ScaleType {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    MelodicMinor,
    HarmonicMinor,
    WholeTone,
    Pentatonic,
    PhrygianDominant,
    HungarianMinor,
}

lazy_static! {
    static ref SCALES_MAP: HashMap<ScaleType, Vec<Interval>> = {
        let mut map = HashMap::new();
        map.insert(ScaleType::Ionian, vec![Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond]);
        map.insert(ScaleType::Dorian, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond]);
        map.insert(ScaleType::Phrygian, vec![Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond]);
        map.insert(ScaleType::Lydian, vec![Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond]);
        map.insert(ScaleType::Mixolydian, vec![Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond]);
        map.insert(ScaleType::Aeolian, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond]);
        map.insert(ScaleType::Locrian, vec![Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond]);

        map.insert(ScaleType::MelodicMinor, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond]);
        map.insert(ScaleType::HarmonicMinor, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MinorThird, Interval::MinorSecond]);

        map.insert(ScaleType::PhrygianDominant, vec![Interval::MinorSecond, Interval::MinorThird, Interval::MinorSecond, Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond]);
        map.insert(ScaleType::HungarianMinor, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MinorThird, Interval::MinorSecond, Interval::MinorSecond, Interval::MinorThird, Interval::MinorSecond]);

        map.insert(ScaleType::WholeTone, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond]);
        map.insert(ScaleType::Pentatonic, vec![Interval::MajorSecond, Interval::MinorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MajorSecond, Interval::MinorSecond]);
        map
    };
}

pub struct Scale(pub Note, pub ScaleType);

impl Scale {
    pub fn notes(&self) -> Vec<Note> {
        let intervals = SCALES_MAP.get(&self.1).unwrap();
        let mut result = Vec::with_capacity(intervals.len() + 1);

        result.push(self.0);
        let mut last_note = self.0;
        for interval in intervals {
            let new_note = last_note + interval;
            result.push(new_note);
            last_note = new_note;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semitones_from_c() {
        // C natural is 0 semitones above C
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 4).0.semitones_from_c(), 0);
        // D natural is 2 semitones above C
        assert_eq!(Pitch(Note(PitchBase::D, PitchModifier::Natural), 0).0.semitones_from_c(), 2);
        // D# is 3 semitones above C
        assert_eq!(Pitch(Note(PitchBase::D, PitchModifier::Sharp), 4).0.semitones_from_c(), 3);
    }

    #[test]
    fn enharmonic_equivalents() {
        // C natural is enharmonic to D double flat
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 4), Pitch(Note(PitchBase::D, PitchModifier::DoubleFlat), 4));
        // E natural is enharmonic to F flat
        assert_eq!(Pitch(Note(PitchBase::E, PitchModifier::Natural), 2), Pitch(Note(PitchBase::F, PitchModifier::Flat), 2));
        // D sharp is enharmonic to E flat
        assert_eq!(Pitch(Note(PitchBase::D, PitchModifier::Sharp), 2), Pitch(Note(PitchBase::E, PitchModifier::Flat), 2));
        // C natural is enharmonic to B sharp
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 3), Pitch(Note(PitchBase::B, PitchModifier::Sharp), 2));
        // Enharmonic pitches at different octaves are not equal
        assert_ne!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 2), Pitch(Note(PitchBase::B, PitchModifier::Sharp), 2));
    }

    #[test]
    fn inversions() {
        // The inversion of the unison is the unison
        assert_eq!(Interval::Unison.inverse(), Interval::Unison);
        // The inversion of a major third is a minor sixth
        assert_eq!(Interval::MajorThird.inverse(), Interval::MinorSixth);
        // The inversion of a tritone is the tritone
        assert_eq!(Interval::Tritone.inverse(), Interval::Tritone);
        // The inversion of an inversion is itself
        assert_eq!(Interval::MajorSeventh.inverse().inverse(), Interval::MajorSeventh);
    }

    #[test]
    fn intervals_of_pitches() {
        // The same notes are in unison
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 4) - Pitch(Note(PitchBase::C, PitchModifier::Natural), 4), Interval::Unison);
        // C and E are a major third apart
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 4) - Pitch(Note(PitchBase::E, PitchModifier::Natural), 4), Interval::MajorThird);
        // E and G are a minor third apart
        assert_eq!(Pitch(Note(PitchBase::E, PitchModifier::Natural), 3) - Pitch(Note(PitchBase::G, PitchModifier::Natural), 4), Interval::MinorThird);
        // C and G are a perfect fifth apart
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 2) - Pitch(Note(PitchBase::G, PitchModifier::Natural), 4), Interval::PerfectFifth);
        // C and B are a minor second apart
        assert_eq!(Pitch(Note(PitchBase::C, PitchModifier::Natural), 4) - Pitch(Note(PitchBase::B, PitchModifier::Natural), 3), Interval::MinorSecond);
    }

    #[test]
    fn scales() {
        // C major/ionian scale
        assert_eq!(Scale(Note(PitchBase::C, PitchModifier::Natural), ScaleType::Ionian).notes(), vec![
            Note(PitchBase::C, PitchModifier::Natural),
            Note(PitchBase::D, PitchModifier::Natural),
            Note(PitchBase::E, PitchModifier::Natural),
            Note(PitchBase::F, PitchModifier::Natural),
            Note(PitchBase::G, PitchModifier::Natural),
            Note(PitchBase::A, PitchModifier::Natural),
            Note(PitchBase::B, PitchModifier::Natural),
            Note(PitchBase::C, PitchModifier::Natural),
        ]);

        // F melodic minor scale
        assert_eq!(Scale(Note(PitchBase::F, PitchModifier::Natural), ScaleType::MelodicMinor).notes(), vec![
            Note(PitchBase::F, PitchModifier::Natural),
            Note(PitchBase::G, PitchModifier::Natural),
            Note(PitchBase::A, PitchModifier::Flat),
            Note(PitchBase::B, PitchModifier::Flat),
            Note(PitchBase::C, PitchModifier::Natural),
            Note(PitchBase::D, PitchModifier::Natural),
            Note(PitchBase::E, PitchModifier::Natural),
            Note(PitchBase::F, PitchModifier::Natural),
        ]);
    }

    #[test]
    fn below_middle_c() {
        assert_eq!(Pitch::from_semitones_from_middle_c(-1), Pitch(Note(PitchBase::B, PitchModifier::Natural), 3));
    }
}
