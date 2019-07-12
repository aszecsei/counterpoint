use rand::prelude::*;
use theory::*;

fn sign(a: i8) -> i8 {
    if a >= 0 {
        1
    } else {
        -1
    }
}

fn shuffle<T>(val: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in (1..val.len()).into_iter().rev() {
        let idx = rng.gen_range(0, i);
        val.swap(i, idx)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Above,
    Below,
}

fn counterpoint(notes: &[Pitch], scale: &Scale, direction: Direction) -> Option<Vec<Pitch>> {
    // The first note must be a perfect octave, unison, or fifth.
    
    let mut opening_pitches = if direction == Direction::Above {
        vec![notes[0] + Interval::Unison, notes[0] + Interval::PerfectFifth, notes[0] + 12]
    } else {
        vec![notes[0] - Interval::Unison, notes[0] - Interval::PerfectFifth, notes[0] - 12]
    };
    

    // We want only notes in the scale.
    let scale_notes = scale.notes();
    for idx in (0..opening_pitches.len()).into_iter().rev() {
        if !scale_notes.contains(&opening_pitches[idx].0) {
            opening_pitches.remove(idx);
        }
    }

    shuffle(&mut opening_pitches);

    for opening in opening_pitches {
        let res = counterpoint_helper(notes, &vec![opening], scale, direction);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn counterpoint_helper(notes: &[Pitch], so_far: &[Pitch], scale: &Scale, direction: Direction) -> Option<Vec<Pitch>> {
    if so_far.len() == notes.len() {
        return Some(Vec::from(so_far))
    }

    let other_note = notes[so_far.len()];

    // If this is the ending, we must choose a unison or octave.
    let mut options = if so_far.len() == notes.len() - 1 {
        if direction == Direction::Above {
            vec![other_note + Interval::Unison, other_note + 12]
        } else {
            vec![other_note - Interval::Unison, other_note - 12]
        }
    } else {
        // Otherwise, we want a consonant interval.
        if direction == Direction::Above {
            vec![other_note + Interval::PerfectFifth, other_note + Interval::MinorThird, other_note + Interval::MajorThird, other_note + Interval::MinorSixth, other_note + Interval::MajorSixth, other_note + 12, other_note + 12 + Interval::MinorThird, other_note + 12 + Interval::MajorThird]
        } else {
            vec![other_note - Interval::PerfectFifth, other_note - Interval::MinorThird, other_note - Interval::MajorThird, other_note - Interval::MinorSixth, other_note - Interval::MajorSixth, other_note - 12, other_note - 12 + Interval::MinorThird, other_note - 12 - Interval::MajorThird]
        }
    };

    // We only want notes from the scale.
    let scale_notes = scale.notes();
    for idx in (0..options.len()).into_iter().rev() {
        if !scale_notes.contains(&options[idx].0) {
            options.remove(idx);
        }
    }

    // We don't want direct or parallel fifths or octaves.
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        if option - other_note == Interval::PerfectFifth || option - other_note == Interval::Unison {
            let prev_note = so_far[so_far.len() - 1];
            let other_prev_note = notes[so_far.len() - 1];

            let motion = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c();
            let other_motion = other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c();

            if sign(motion) == sign(other_motion) {
                options.remove(idx);
            }
        }
    }

    // Don't exceed a tenth from the other line
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx].semitones_from_middle_c();
        let other = other_note.semitones_from_middle_c();
        if (option - other).abs() as u8 > 12 + Interval::MajorThird.semitones() {
            options.remove(idx);
        }
    }

    // Don't move in parallel sixths or thirds more than three notes at a time.
    for idx in (0..options.len()).into_iter().rev() {
        let interval = options[idx] - other_note;
        let mut count = 1;
        if interval == Interval::MinorThird || interval == Interval::MajorThird {
            for m_idx in (0..so_far.len()).into_iter().rev() {
                let interval = so_far[m_idx] - notes[m_idx];
                if interval != Interval::MinorThird && interval != Interval::MajorThird {
                    break;
                } else {
                    count += 1;
                }
            }
        } else if interval == Interval::MinorSixth || interval == Interval::MajorSixth {
            for m_idx in (0..so_far.len()).into_iter().rev() {
                let interval = so_far[m_idx] - notes[m_idx];
                if interval != Interval::MinorSixth && interval != Interval::MajorSixth {
                    break;
                } else {
                    count += 1;
                }
            }
        }
        if count > 3 {
            options.remove(idx);
        }
    }

    // Don't have both voices skip in the same direction
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        let prev_note = so_far[so_far.len() - 1];

        let is_skip = (option.semitones_from_middle_c() - prev_note.semitones_from_middle_c()).abs() as u8 > Interval::MajorSecond.semitones();

        let other_prev_note = notes[so_far.len() - 1];
        let is_other_skip = (other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c()).abs() as u8 > Interval::MajorSecond.semitones();

        if is_skip && is_other_skip {
            let motion = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c();
            let other_motion = other_note.semitones_from_middle_c() - other_prev_note.semitones_from_middle_c();

            if sign(motion) == sign(other_motion) {
                options.remove(idx);
            }
        }
    }

    // Don't repeat the same note more than twice
    for idx in (0..options.len()).into_iter().rev() {
        if so_far.len() > 1 {
            if options[idx].0 == so_far[so_far.len() - 1].0 && so_far[so_far.len() - 1].0 == so_far[so_far.len() - 2].0 {
                options.remove(idx);
            }
        }
    }


    // Don't leap more than an octave
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        let prev_note = so_far[so_far.len() - 1];
        let leap = (option.semitones_from_middle_c() - prev_note.semitones_from_middle_c()).abs() as u8;
        if leap > 12 {
            options.remove(idx);
        }
    }

    // Don't leap by a tritone
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        let prev_note = so_far[so_far.len() - 1];
        let leap = (option.semitones_from_middle_c() - prev_note.semitones_from_middle_c()).abs() as u8;
        if leap == Interval::Tritone.semitones() {
            options.remove(idx);
        }
    }

    // Approach the last note via stepwise motion
    if so_far.len() == notes.len() - 1 {
        for idx in (0..options.len()).into_iter().rev() {
            let option = options[idx];
            let prev_note = so_far[so_far.len() - 1];
            let leap = (option.semitones_from_middle_c() - prev_note.semitones_from_middle_c()).abs() as u8;
            if leap > Interval::MajorSecond.semitones() {
                options.remove(idx);
            }
        }
    }

    // If you leap, you must go the opposite direction by step
    for idx in (0..options.len()).into_iter().rev() {
        let option = options[idx];
        let prev_note = so_far[so_far.len() - 1];
        if so_far.len() > 1 {
            let prev_prev_note = so_far[so_far.len() - 2];

            let motion = prev_note.semitones_from_middle_c() - prev_prev_note.semitones_from_middle_c();
            if motion.abs() as u8 > Interval::MajorThird.semitones() {
                let curr_motion = option.semitones_from_middle_c() - prev_note.semitones_from_middle_c();
                if curr_motion.abs() as u8 > Interval::MajorSecond.semitones() || sign(curr_motion) == sign(motion) {
                    options.remove(idx);
                }
            }
        }
    }


    shuffle(&mut options);

    for option in options {
        let mut r = Vec::from(so_far);
        r.push(option);

        let res = counterpoint_helper(notes, &r, scale, direction);
        if res.is_some() {
            return res;
        }
    }
    None
}

fn parse_music(data: &mut std::str::Chars) -> Vec<Pitch> {
    let mut result = vec![];

    loop {
        let mut c = data.next();

        while c.map_or(false, |f| { f.is_ascii_whitespace() }) {
            c = data.next();
        }

        if let Some(c) = c {
            let pitch_base = match c.to_ascii_lowercase() {
                'a' => PitchBase::A,
                'b' => PitchBase::B,
                'c' => PitchBase::C,
                'd' => PitchBase::D,
                'e' => PitchBase::E,
                'f' => PitchBase::F,
                'g' => PitchBase::G,
                _ => panic!("Unexpected pitch base")
            };

            let mut c = data.next().expect("Unexpected end of file");
            let pitch_modifier = if !c.is_numeric() {
                let res = match c {
                    '#' => PitchModifier::Sharp,
                    'b' => PitchModifier::Flat,
                    _ => panic!("Unexpected pitch modifier")
                };
                c = data.next().expect("Unexpected end of file");
                res
            } else {
                PitchModifier::Natural
            };

            let octave = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                _ => panic!("Unexpected octave value")
            };

            result.push(Pitch(Note(pitch_base, pitch_modifier), octave));
        } else {
            break;
        }
    }
    result
}

fn main() {
    let cantus_firmus = include_str!("../cantus.txt");
    let cantus_firmus = parse_music(&mut cantus_firmus.chars());
    if let Some(notes) = counterpoint(&cantus_firmus, &Scale(Note(PitchBase::C, PitchModifier::Natural), ScaleType::Ionian), Direction::Below) {
        for note in cantus_firmus {
            print!("{} ", note);
        }
        println!();
        for note in notes {
            print!("{} ", note);
        }
        println!();
    } else {
        println!("Error: No counterpoint :(");
    }
}
